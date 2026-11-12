//! Linux window and idle backends. Each backend shells out to a desktop tool
//! and hands the output to [`super::parsers`].

use super::parsers::{self, CallFailure, WindowInfo};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

const GNOME_SHELL: &str = "org.gnome.Shell";
const WINDOW_CALLS_PATH: &str = "/org/gnome/Shell/Extensions/Windows";
const WINDOW_CALLS_IFACE: &str = "org.gnome.Shell.Extensions.Windows";
const FOCUSED_WINDOW_PATH: &str = "/org/gnome/shell/extensions/FocusedWindow";
const FOCUSED_WINDOW_IFACE: &str = "org.gnome.shell.extensions.FocusedWindow";
const MUTTER_IDLE: &str = "org.gnome.Mutter.IdleMonitor";
const MUTTER_IDLE_PATH: &str = "/org/gnome/Mutter/IdleMonitor/Core";

/// The backend cannot be used right now (tool missing, service absent, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendUnavailable;

/// Whether `program` exists as a file in one of the `PATH` directories.
pub fn command_available(program: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join(program).is_file()))
        .unwrap_or(false)
}

/// Runs a command and returns its stdout; a spawn failure or non-zero exit
/// status means the backend is unavailable.
fn command_stdout(program: &str, args: &[&str]) -> Result<String, BackendUnavailable> {
    let output = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|_| BackendUnavailable)?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(BackendUnavailable)
    }
}

/// `busctl --user --json=short call …`, returning stdout or a classified failure.
fn busctl_call(
    destination: &str,
    path: &str,
    interface: &str,
    method: &str,
    args: &[&str],
) -> Result<String, CallFailure> {
    let mut argv = vec!["--user", "--json=short", "call", destination, path, interface, method];
    argv.extend_from_slice(args);
    let output = Command::new("busctl")
        .args(&argv)
        .stdin(Stdio::null())
        .output()
        .map_err(|_| CallFailure::Unavailable)?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(parsers::classify_busctl_error(&String::from_utf8_lossy(&output.stderr)))
    }
}

/// Window Calls ID of the last window found focused through `Details` (0 = none).
static LAST_WINDOW_CALLS_FOCUS: AtomicU64 = AtomicU64::new(0);

/// Window Calls `List`: every window, with its focus flag on older versions.
fn window_calls_list() -> Result<Vec<(WindowInfo, Option<bool>)>, BackendUnavailable> {
    match busctl_call(GNOME_SHELL, WINDOW_CALLS_PATH, WINDOW_CALLS_IFACE, "List", &[]) {
        Ok(out) => parsers::busctl_string(&out)
            .and_then(|json| parsers::parse_window_calls_list(&json))
            .ok_or(BackendUnavailable),
        Err(CallFailure::NoWindow) => Ok(Vec::new()),
        Err(CallFailure::Unavailable) => Err(BackendUnavailable),
    }
}

/// Window Calls `Details(u id)` focus flag; `false` when the call fails.
fn window_calls_is_focused(id: u64) -> bool {
    let id = id.to_string();
    busctl_call(GNOME_SHELL, WINDOW_CALLS_PATH, WINDOW_CALLS_IFACE, "Details", &["u", &id])
        .ok()
        .and_then(|out| parsers::busctl_string(&out))
        .and_then(|json| parsers::parse_window_calls_details_focus(&json))
        .unwrap_or(false)
}

/// The focused window from a Window Calls `List` reply. Older versions carry
/// `focus` in `List`; current ones need one `Details` call per window, starting
/// with the window that was focused last time.
fn window_calls_focused(windows: Vec<(WindowInfo, Option<bool>)>) -> Option<WindowInfo> {
    if windows.iter().any(|(_, focus)| focus.is_some()) {
        return windows
            .into_iter()
            .find(|(_, focus)| *focus == Some(true))
            .map(|(w, _)| w);
    }

    let windows: Vec<WindowInfo> = windows.into_iter().map(|(w, _)| w).collect();
    let last = LAST_WINDOW_CALLS_FOCUS.load(Ordering::Relaxed);
    for id in parsers::focus_probe_order(&windows, (last != 0).then_some(last)) {
        if window_calls_is_focused(id) {
            LAST_WINDOW_CALLS_FOCUS.store(id, Ordering::Relaxed);
            return windows.into_iter().find(|w| w.id == Some(id));
        }
    }
    None
}

/// Window Calls `GetTitle(u id)`, for versions whose `List` omits titles.
fn window_calls_title(id: u64) -> Option<String> {
    let id = id.to_string();
    busctl_call(GNOME_SHELL, WINDOW_CALLS_PATH, WINDOW_CALLS_IFACE, "GetTitle", &["u", &id])
        .ok()
        .and_then(|out| parsers::busctl_string(&out))
}

/// Reads one X11 window's PID, title and class with `xprop`.
fn x11_window(id: &str) -> Result<WindowInfo, BackendUnavailable> {
    let out = command_stdout(
        "xprop",
        &["-id", id, "_NET_WM_PID", "_NET_WM_NAME", "WM_NAME", "WM_CLASS"],
    )?;
    Ok(parsers::parse_xprop_window_props(&out))
}

/// Source of the focused window on Linux.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowBackend {
    Hyprland,
    Sway,
    GnomeWindowCalls,
    GnomeFocusedWindow,
    X11,
}

impl WindowBackend {
    /// Stable label reported to the frontend.
    pub fn label(self) -> &'static str {
        match self {
            Self::Hyprland => "hyprland",
            Self::Sway => "sway",
            Self::GnomeWindowCalls => "gnome-window-calls",
            Self::GnomeFocusedWindow => "gnome-focused-window",
            Self::X11 => "x11",
        }
    }

    /// The focused window; `Ok(None)` when the backend works but nothing is focused.
    pub fn focused(self) -> Result<Option<WindowInfo>, BackendUnavailable> {
        match self {
            Self::Hyprland => {
                let out = command_stdout("hyprctl", &["activewindow", "-j"])?;
                Ok(parsers::parse_hyprland_active(&out))
            }
            Self::Sway => {
                let out = command_stdout("swaymsg", &["-t", "get_tree", "-r"])?;
                let windows = parsers::parse_sway_tree(&out).ok_or(BackendUnavailable)?;
                Ok(windows.into_iter().find(|(_, focused)| *focused).map(|(w, _)| w))
            }
            Self::GnomeWindowCalls => {
                let focused = window_calls_focused(window_calls_list()?);
                Ok(focused.map(|mut window| {
                    self.complete_title(&mut window);
                    window
                }))
            }
            Self::GnomeFocusedWindow => {
                match busctl_call(GNOME_SHELL, FOCUSED_WINDOW_PATH, FOCUSED_WINDOW_IFACE, "Get", &[]) {
                    Ok(out) => Ok(parsers::busctl_string(&out)
                        .and_then(|json| parsers::parse_focused_window_dbus(&json))),
                    Err(CallFailure::NoWindow) => Ok(None),
                    Err(CallFailure::Unavailable) => Err(BackendUnavailable),
                }
            }
            Self::X11 => {
                let out = command_stdout("xprop", &["-root", "_NET_ACTIVE_WINDOW"])?;
                let Some(id) = parsers::parse_xprop_window_ids(&out, "_NET_ACTIVE_WINDOW")
                    .into_iter()
                    .next()
                else {
                    return Ok(None);
                };
                let window = x11_window(&id)?;
                Ok(if window.class.is_empty() { None } else { Some(window) })
            }
        }
    }

    /// Every window the backend can list (empty when it cannot list windows).
    /// Titles may be empty; see [`WindowBackend::complete_title`].
    pub fn windows(self) -> Vec<WindowInfo> {
        match self {
            Self::Hyprland => command_stdout("hyprctl", &["clients", "-j"])
                .map(|out| parsers::parse_hyprland_clients(&out))
                .unwrap_or_default(),
            Self::Sway => command_stdout("swaymsg", &["-t", "get_tree", "-r"])
                .ok()
                .and_then(|out| parsers::parse_sway_tree(&out))
                .map(|windows| windows.into_iter().map(|(w, _)| w).collect())
                .unwrap_or_default(),
            Self::GnomeWindowCalls => window_calls_list()
                .map(|windows| windows.into_iter().map(|(w, _)| w).collect())
                .unwrap_or_default(),
            Self::GnomeFocusedWindow => Vec::new(),
            Self::X11 => command_stdout("xprop", &["-root", "_NET_CLIENT_LIST"])
                .map(|out| parsers::parse_xprop_window_ids(&out, "_NET_CLIENT_LIST"))
                .unwrap_or_default()
                .iter()
                .filter_map(|id| x11_window(id).ok())
                .collect(),
        }
    }

    /// Fills in a missing title (Window Calls versions whose `List` omits it).
    pub fn complete_title(self, window: &mut WindowInfo) {
        if self == Self::GnomeWindowCalls && window.title.is_empty() {
            if let Some(id) = window.id {
                window.title = window_calls_title(id).unwrap_or_default();
            }
        }
    }
}

/// Source of the user's idle time on Linux.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdleBackend {
    /// `org.gnome.Mutter.IdleMonitor` (GNOME on X11 and Wayland).
    GnomeMutter,
    /// `xprintidle` (any X11 session).
    XPrintIdle,
}

impl IdleBackend {
    /// Stable label reported to the frontend.
    pub fn label(self) -> &'static str {
        match self {
            Self::GnomeMutter => "gnome-mutter",
            Self::XPrintIdle => "xprintidle",
        }
    }

    /// Milliseconds since the last user input.
    pub fn idle_ms(self) -> Result<u64, BackendUnavailable> {
        match self {
            Self::GnomeMutter => busctl_call(MUTTER_IDLE, MUTTER_IDLE_PATH, MUTTER_IDLE, "GetIdletime", &[])
                .ok()
                .and_then(|out| parsers::busctl_u64(&out))
                .ok_or(BackendUnavailable),
            Self::XPrintIdle => command_stdout("xprintidle", &[])
                .ok()
                .and_then(|out| parsers::parse_xprintidle(&out))
                .ok_or(BackendUnavailable),
        }
    }
}
