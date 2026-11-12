//! Pure parsers for the output of the external tools used on Linux
//! (`busctl`, `hyprctl`, `swaymsg`, `xprop`, `xprintidle`).
//!
//! Everything here is side-effect free so it can be unit-tested with
//! captured sample outputs.

use serde_json::Value;

/// A window as reported by a detection backend.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WindowInfo {
    /// Owning process ID, when the backend knows it.
    pub pid: Option<u32>,
    /// Window title (may be empty).
    pub title: String,
    /// Lowercase window class / Wayland app id (may be empty).
    pub class: String,
    /// Backend-specific window ID (used by Window Calls `GetTitle`).
    pub id: Option<u64>,
}

/// Outcome of a failed backend call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallFailure {
    /// The backend is not usable (tool missing, D-Bus service/object/method absent).
    Unavailable,
    /// The backend answered but has no focused window (lock screen, empty workspace…).
    NoWindow,
}

// ---------------------------------------------------------------------------
// busctl
// ---------------------------------------------------------------------------

/// Extracts the first reply value from `busctl … --json=short` output,
/// e.g. `{"type":"s","data":["…"]}` → `"…"`.
pub fn busctl_payload(stdout: &str) -> Option<Value> {
    let reply: Value = serde_json::from_str(stdout.trim()).ok()?;
    reply.get("data")?.as_array()?.first().cloned()
}

/// The reply as a string (D-Bus signature `s`).
pub fn busctl_string(stdout: &str) -> Option<String> {
    busctl_payload(stdout)?.as_str().map(str::to_string)
}

/// The reply as an unsigned integer (D-Bus signature `t` or `u`).
pub fn busctl_u64(stdout: &str) -> Option<u64> {
    busctl_payload(stdout)?.as_u64()
}

/// Classifies a failed `busctl call` by its stderr. Messages meaning the
/// service, object, interface or method does not exist make the backend
/// unavailable; any other error (e.g. a GNOME extension throwing
/// "No window in focus") means "no focused window right now".
pub fn classify_busctl_error(stderr: &str) -> CallFailure {
    const UNAVAILABLE_MARKERS: &[&str] = &[
        "does not exist",
        "not provided by",
        "no such method",
        "no such interface",
        "no such object",
        "unknown object",
        "unknown method",
        "unknown interface",
        "serviceunknown",
        "unknownobject",
        "unknownmethod",
        "unknowninterface",
        "failed to connect",
        "failed to get",
    ];
    let lower = stderr.to_lowercase();
    if UNAVAILABLE_MARKERS.iter().any(|m| lower.contains(m)) {
        CallFailure::Unavailable
    } else {
        CallFailure::NoWindow
    }
}

// ---------------------------------------------------------------------------
// Shared JSON helpers
// ---------------------------------------------------------------------------

fn json_str(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or_default().to_string()
}

fn json_pid(v: &Value, key: &str) -> Option<u32> {
    v.get(key)
        .and_then(Value::as_i64)
        .filter(|p| *p > 0)
        .and_then(|p| u32::try_from(p).ok())
}

// ---------------------------------------------------------------------------
// GNOME Shell extensions
// ---------------------------------------------------------------------------

/// Parses the JSON returned by the "Window Calls" extension's `List` method.
/// Returns each window with its `focus` flag when the extension version
/// includes it (older versions). Current versions omit both `focus` and
/// `title`: the caller then uses `Details(id)` and `GetTitle(id)`.
pub fn parse_window_calls_list(json: &str) -> Option<Vec<(WindowInfo, Option<bool>)>> {
    let windows: Vec<Value> = serde_json::from_str(json).ok()?;
    Some(
        windows
            .iter()
            .map(|w| {
                let info = WindowInfo {
                    pid: json_pid(w, "pid"),
                    title: json_str(w, "title"),
                    class: json_str(w, "wm_class").to_lowercase(),
                    id: w.get("id").and_then(Value::as_u64),
                };
                let focused = w.get("focus").and_then(Value::as_bool);
                (info, focused)
            })
            .collect(),
    )
}

/// Parses the `focus` flag from the Window Calls `Details(id)` JSON.
pub fn parse_window_calls_details_focus(json: &str) -> Option<bool> {
    let details: Value = serde_json::from_str(json).ok()?;
    details.get("focus").and_then(Value::as_bool)
}

/// Window IDs to probe with Window Calls `Details`, the last focused window
/// first (it usually still has focus), then the rest in list order.
pub fn focus_probe_order(windows: &[WindowInfo], last_focused: Option<u64>) -> Vec<u64> {
    let mut ids: Vec<u64> = windows.iter().filter_map(|w| w.id).collect();
    if let Some(last) = last_focused {
        if let Some(pos) = ids.iter().position(|id| *id == last) {
            let id = ids.remove(pos);
            ids.insert(0, id);
        }
    }
    ids
}

/// Parses the JSON returned by the "Focused Window D-Bus" extension's `Get` method.
pub fn parse_focused_window_dbus(json: &str) -> Option<WindowInfo> {
    let w: Value = serde_json::from_str(json).ok()?;
    if !w.is_object() {
        return None;
    }
    let info = WindowInfo {
        pid: json_pid(&w, "pid"),
        title: json_str(&w, "title"),
        class: json_str(&w, "wm_class").to_lowercase(),
        id: w.get("id").and_then(Value::as_u64),
    };
    if info.pid.is_none() && info.class.is_empty() {
        return None;
    }
    Some(info)
}

// ---------------------------------------------------------------------------
// Hyprland
// ---------------------------------------------------------------------------

fn hyprland_window(w: &Value) -> Option<WindowInfo> {
    let info = WindowInfo {
        pid: json_pid(w, "pid"),
        title: json_str(w, "title"),
        class: json_str(w, "class").to_lowercase(),
        id: None,
    };
    if info.pid.is_none() && info.class.is_empty() {
        return None;
    }
    Some(info)
}

/// Parses `hyprctl activewindow -j` (`{}` when nothing is focused).
pub fn parse_hyprland_active(json: &str) -> Option<WindowInfo> {
    let w: Value = serde_json::from_str(json).ok()?;
    hyprland_window(&w)
}

/// Parses `hyprctl clients -j`.
pub fn parse_hyprland_clients(json: &str) -> Vec<WindowInfo> {
    serde_json::from_str::<Vec<Value>>(json)
        .map(|clients| clients.iter().filter_map(hyprland_window).collect())
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Sway
// ---------------------------------------------------------------------------

fn collect_sway_windows(node: &Value, out: &mut Vec<(WindowInfo, bool)>) {
    if let Some(pid) = json_pid(node, "pid") {
        let class = node
            .get("app_id")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .or_else(|| {
                node.get("window_properties")
                    .and_then(|p| p.get("class"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
            })
            .unwrap_or_default()
            .to_lowercase();
        let focused = node.get("focused").and_then(Value::as_bool).unwrap_or(false);
        out.push((
            WindowInfo {
                pid: Some(pid),
                title: json_str(node, "name"),
                class,
                id: node.get("id").and_then(Value::as_u64),
            },
            focused,
        ));
    }
    for key in ["nodes", "floating_nodes"] {
        if let Some(children) = node.get(key).and_then(Value::as_array) {
            for child in children {
                collect_sway_windows(child, out);
            }
        }
    }
}

/// Parses `swaymsg -t get_tree -r` into every window and its focus flag.
pub fn parse_sway_tree(json: &str) -> Option<Vec<(WindowInfo, bool)>> {
    let root: Value = serde_json::from_str(json).ok()?;
    let mut windows = Vec::new();
    collect_sway_windows(&root, &mut windows);
    Some(windows)
}

// ---------------------------------------------------------------------------
// X11 (xprop)
// ---------------------------------------------------------------------------

/// Extracts window IDs from an xprop `WINDOW` property line, e.g.
/// `_NET_CLIENT_LIST(WINDOW): window id # 0x800004, 0x1000009`.
pub fn parse_xprop_window_ids(stdout: &str, property: &str) -> Vec<String> {
    stdout
        .lines()
        .find(|line| line.starts_with(&format!("{}(", property)))
        .and_then(|line| line.split("window id #").nth(1))
        .map(|ids| {
            ids.split(',')
                .map(|id| id.trim().to_string())
                .filter(|id| id.starts_with("0x") && id != "0x0")
                .collect()
        })
        .unwrap_or_default()
}

/// Parses a C-style quoted xprop string starting at the opening quote,
/// returning the unescaped content and the rest of the input.
fn parse_quoted(input: &str) -> Option<(String, &str)> {
    let rest = input.trim_start().strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.char_indices();
    while let Some((i, c)) = chars.next() {
        match c {
            '"' => return Some((out, &rest[i + 1..])),
            '\\' => match chars.next() {
                Some((_, 'n')) => out.push('\n'),
                Some((_, 't')) => out.push('\t'),
                Some((_, other)) => out.push(other),
                None => break,
            },
            _ => out.push(c),
        }
    }
    None
}

/// Parses the output of `xprop -id <id> _NET_WM_PID _NET_WM_NAME WM_NAME WM_CLASS`.
/// `_NET_WM_NAME` wins over `WM_NAME`; the class is the second `WM_CLASS` value.
pub fn parse_xprop_window_props(stdout: &str) -> WindowInfo {
    let mut info = WindowInfo::default();
    let mut legacy_title = String::new();

    for line in stdout.lines() {
        let Some((key, value)) = line.split_once(" = ") else {
            continue;
        };
        if key.starts_with("_NET_WM_PID(") {
            info.pid = value.trim().parse::<u32>().ok().filter(|p| *p > 0);
        } else if key.starts_with("_NET_WM_NAME(") {
            if let Some((title, _)) = parse_quoted(value) {
                info.title = title;
            }
        } else if key.starts_with("WM_NAME(") {
            if let Some((title, _)) = parse_quoted(value) {
                legacy_title = title;
            }
        } else if key.starts_with("WM_CLASS(") {
            let mut values = Vec::new();
            let mut rest = value;
            while let Some((v, tail)) = parse_quoted(rest) {
                values.push(v);
                rest = tail.trim_start().strip_prefix(',').unwrap_or(tail);
            }
            if let Some(class) = values.last() {
                info.class = class.to_lowercase();
            }
        }
    }

    if info.title.is_empty() {
        info.title = legacy_title;
    }
    info
}

/// Parses `xprintidle` output (milliseconds).
pub fn parse_xprintidle(stdout: &str) -> Option<u64> {
    stdout.trim().parse::<u64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn busctl_string_and_number_payloads() {
        assert_eq!(
            busctl_string(r#"{"type":"s","data":["[{\"pid\":1}]"]}"#).as_deref(),
            Some(r#"[{"pid":1}]"#)
        );
        assert_eq!(busctl_u64(r#"{"type":"t","data":[123456]}"#), Some(123456));
        assert_eq!(busctl_u64("not json"), None);
        assert_eq!(busctl_string(r#"{"type":"s","data":[]}"#), None);
    }

    #[test]
    fn busctl_errors_are_classified() {
        // Captured on GNOME Shell 50 without the extensions installed.
        assert_eq!(
            classify_busctl_error("Call failed: Object does not exist at path “/org/gnome/Shell/Extensions/Windows”"),
            CallFailure::Unavailable
        );
        assert_eq!(
            classify_busctl_error("Call failed: The name org.example.Nope was not provided by any .service files"),
            CallFailure::Unavailable
        );
        assert_eq!(
            classify_busctl_error("Call failed: No such method “NoSuchMethod”"),
            CallFailure::Unavailable
        );
        assert_eq!(
            classify_busctl_error("Call failed: No such interface “org.gnome.Shell.Extensions.Windows” on object at path /"),
            CallFailure::Unavailable
        );
        assert_eq!(
            classify_busctl_error("Call failed: No window in focus"),
            CallFailure::NoWindow
        );
    }

    #[test]
    fn window_calls_list_current_version_without_focus_or_title() {
        // Field set from the current Window Calls README `List` example.
        let json = r#"[
            {"wm_class":"Code","wm_class_instance":"code","pid":7890,"id":821316218,"frame_type":0,"window_type":0,"width":1920,"height":1080,"x":0,"y":0,"in_current_workspace":true,"workspace":0,"monitor":0}
        ]"#;
        let windows = parse_window_calls_list(json).unwrap();
        assert_eq!(windows.len(), 1);
        let (window, focus) = &windows[0];
        assert_eq!(*focus, None);
        assert_eq!(window.pid, Some(7890));
        assert_eq!(window.class, "code");
        assert_eq!(window.title, "");
        assert_eq!(window.id, Some(821316218));
        assert!(parse_window_calls_list("garbage").is_none());
    }

    #[test]
    fn window_calls_list_legacy_version_with_focus_and_title() {
        let json = r#"[
            {"wm_class":"firefox","pid":3456,"id":2034567890,"focus":false,"title":"Mozilla Firefox"},
            {"wm_class":"Code","pid":7890,"id":1234,"focus":true}
        ]"#;
        let windows = parse_window_calls_list(json).unwrap();
        assert_eq!(windows[0].0.title, "Mozilla Firefox");
        assert_eq!(windows[0].1, Some(false));
        assert_eq!(windows[1].1, Some(true));
    }

    #[test]
    fn focus_probe_order_puts_last_focused_first() {
        let windows: Vec<WindowInfo> = [Some(1), None, Some(2), Some(3)]
            .into_iter()
            .map(|id| WindowInfo { id, ..WindowInfo::default() })
            .collect();
        assert_eq!(focus_probe_order(&windows, Some(3)), vec![3, 1, 2]);
        assert_eq!(focus_probe_order(&windows, Some(99)), vec![1, 2, 3]);
        assert_eq!(focus_probe_order(&windows, None), vec![1, 2, 3]);
    }

    #[test]
    fn window_calls_details_focus() {
        assert_eq!(parse_window_calls_details_focus(r#"{"id":1,"focus":true,"pid":2}"#), Some(true));
        assert_eq!(parse_window_calls_details_focus(r#"{"id":1,"focus":false}"#), Some(false));
        assert_eq!(parse_window_calls_details_focus(r#"{"id":1}"#), None);
    }

    #[test]
    fn focused_window_dbus_object() {
        // `Get` returns the full window object (README "Return format example").
        let json = r#"{"title":"main.rs - app - Visual Studio Code","wm_class":"Code","wm_class_instance":"code","pid":4242,"id":99,"frame_type":0,"window_type":0,"width":800,"height":600,"x":0,"y":0,"focus":true,"in_current_workspace":true,"workspace":0}"#;
        let w = parse_focused_window_dbus(json).unwrap();
        assert_eq!(w.pid, Some(4242));
        assert_eq!(w.class, "code");
        assert_eq!(w.title, "main.rs - app - Visual Studio Code");
        assert!(parse_focused_window_dbus("{}").is_none());
        assert!(parse_focused_window_dbus("[]").is_none());
    }

    #[test]
    fn hyprland_active_window_and_clients() {
        let active = r#"{"address":"0x5c2b1e0","mapped":true,"class":"firefox","title":"GitHub — Mozilla Firefox","pid":2211,"xwayland":false}"#;
        let w = parse_hyprland_active(active).unwrap();
        assert_eq!(w.pid, Some(2211));
        assert_eq!(w.class, "firefox");
        assert_eq!(w.title, "GitHub — Mozilla Firefox");
        assert!(parse_hyprland_active("{}").is_none());

        let clients = format!("[{}, {{\"class\":\"kitty\",\"title\":\"~\",\"pid\":10}}]", active);
        let all = parse_hyprland_clients(&clients);
        assert_eq!(all.len(), 2);
        assert_eq!(all[1].class, "kitty");
        assert!(parse_hyprland_clients("oops").is_empty());
    }

    #[test]
    fn sway_tree_finds_focused_wayland_and_xwayland_windows() {
        let tree = r#"{"id":1,"type":"root","focused":false,"nodes":[
            {"id":2,"type":"output","nodes":[
                {"id":3,"type":"workspace","focused":false,"nodes":[
                    {"id":10,"type":"con","focused":true,"name":"main.rs - app - Visual Studio Code","pid":500,"app_id":null,"window_properties":{"class":"Code","instance":"code"}},
                    {"id":11,"type":"con","focused":false,"name":"Terminal","pid":600,"app_id":"foot"}
                ],"floating_nodes":[
                    {"id":12,"type":"floating_con","focused":false,"name":"Picture-in-Picture","pid":700,"app_id":"firefox"}
                ]}
            ]}
        ]}"#;
        let windows = parse_sway_tree(tree).unwrap();
        assert_eq!(windows.len(), 3);
        let (focused, _) = windows.iter().find(|(_, f)| *f).unwrap();
        assert_eq!(focused.pid, Some(500));
        assert_eq!(focused.class, "code");
        assert_eq!(focused.title, "main.rs - app - Visual Studio Code");
        assert!(windows.iter().any(|(w, _)| w.class == "foot"));
        assert!(windows.iter().any(|(w, _)| w.class == "firefox"));
    }

    #[test]
    fn xprop_window_ids() {
        // Captured from XWayland on GNOME Shell 50.
        let out = "_NET_ACTIVE_WINDOW(WINDOW): window id # 0x600003\n_NET_CLIENT_LIST(WINDOW): window id # 0x800004, 0x1000009\n";
        assert_eq!(parse_xprop_window_ids(out, "_NET_ACTIVE_WINDOW"), vec!["0x600003"]);
        assert_eq!(parse_xprop_window_ids(out, "_NET_CLIENT_LIST"), vec!["0x800004", "0x1000009"]);
        assert!(parse_xprop_window_ids("_NET_ACTIVE_WINDOW(WINDOW): window id # 0x0", "_NET_ACTIVE_WINDOW").is_empty());
        assert!(parse_xprop_window_ids("_NET_ACTIVE_WINDOW:  no such atom on any window.", "_NET_ACTIVE_WINDOW").is_empty());
    }

    #[test]
    fn xprop_window_props() {
        // Captured from XWayland on GNOME Shell 50.
        let out = "_NET_WM_PID(CARDINAL) = 6089\n_NET_WM_NAME(UTF8_STRING) = \"Daily notes | Notion\"\nWM_NAME(UTF8_STRING) = \"Daily notes | Notion\"\nWM_CLASS(STRING) = \"notion-desktop\", \"notion-desktop\"\n";
        let w = parse_xprop_window_props(out);
        assert_eq!(w.pid, Some(6089));
        assert_eq!(w.title, "Daily notes | Notion");
        assert_eq!(w.class, "notion-desktop");

        let escaped = "_NET_WM_PID:  not found.\n_NET_WM_NAME(UTF8_STRING) = \"say \\\"hi\\\" - Code\"\nWM_CLASS(STRING) = \"code\", \"Code\"\n";
        let w = parse_xprop_window_props(escaped);
        assert_eq!(w.pid, None);
        assert_eq!(w.title, "say \"hi\" - Code");
        assert_eq!(w.class, "code");

        let legacy = "WM_NAME(STRING) = \"xterm\"\nWM_CLASS:  not found.\n";
        let w = parse_xprop_window_props(legacy);
        assert_eq!(w.title, "xterm");
        assert_eq!(w.class, "");
    }

    #[test]
    fn xprintidle_output() {
        assert_eq!(parse_xprintidle("61234\n"), Some(61234));
        assert_eq!(parse_xprintidle(""), None);
    }
}
