//! Pure browser disambiguation and window-title parsing rules.
//!
//! Chromium-based and Firefox-based forks frequently share an upstream binary
//! name (`chrome.exe`, `opera.exe`, `firefox.exe`). This module identifies the
//! actual browser from window-title suffixes and executable paths without any
//! OS or Tauri dependencies.

/// Title separators used between a page title and the trailing browser name.
/// Chromium-based browsers typically use `" - "`; Firefox-family browsers
/// (Firefox, Zen, LibreWolf, Tor Browser) commonly use `" — "` (em-dash).
const TITLE_SEPARATORS: [&str; 3] = [" - ", " — ", " – "];

/// Normalizes a process name for browser checks: trimmed, lowercase, without
/// `.exe` or `.bin` wrapper suffixes.
fn normalize_browser_exe(process_name: &str) -> String {
    let lower = process_name.trim().to_lowercase();
    let without_exe = lower.strip_suffix(".exe").unwrap_or(&lower);
    let without_bin = without_exe.strip_suffix(".bin").unwrap_or(without_exe);
    without_bin.to_string()
}

/// Whether `process_name` belongs to a known browser or upstream browser base.
/// Disambiguation only runs when this returns `true`, preventing non-browser
/// apps (such as editors editing `helium.rs`) from ever being reclassified.
pub fn is_browser_process(process_name: &str) -> bool {
    matches!(
        normalize_browser_exe(process_name).as_str(),
        "chrome"
            | "google-chrome"
            | "google-chrome-stable"
            | "google-chrome-beta"
            | "google-chrome-unstable"
            | "chromium"
            | "chromium-browser"
            | "ungoogled-chromium"
            | "firefox"
            | "firefox-esr"
            | "firefox-developer-edition"
            | "firefox-nightly"
            | "opera"
            | "opera-stable"
            | "opera-beta"
            | "opera-developer"
            | "operagx"
            | "opera-gx"
            | "brave"
            | "brave-browser"
            | "brave-browser-stable"
            | "brave-browser-beta"
            | "brave-browser-nightly"
            | "msedge"
            | "edge"
            | "microsoft-edge"
            | "microsoft-edge-stable"
            | "microsoft-edge-beta"
            | "microsoft-edge-dev"
            | "vivaldi"
            | "vivaldi-stable"
            | "vivaldi-snapshot"
            | "arc"
            | "zen"
            | "zen-browser"
            | "zen-alpha"
            | "zen-twilight"
            | "librewolf"
            | "torbrowser"
            | "tor-browser"
            | "helium"
            | "helium-browser"
            | "safari"
    )
}

/// Whether a resolved browser id is a generic upstream base that forks often
/// impersonate in process names or default window titles.
fn is_generic_base(browser_id: &str) -> bool {
    matches!(browser_id, "chrome" | "firefox" | "opera")
}

/// Finds the byte index and byte length of the rightmost title separator.
fn rfind_title_separator(title: &str) -> Option<(usize, usize)> {
    TITLE_SEPARATORS
        .iter()
        .filter_map(|sep| title.rfind(sep).map(|idx| (idx, sep.len())))
        .max_by_key(|(idx, _)| *idx)
}

/// Maps a cleaned browser label (e.g. from a window title suffix) to its
/// canonical catalog id.
fn match_browser_label(raw: &str) -> Option<&'static str> {
    let cleaned: String = raw
        .chars()
        .filter(|c| !matches!(*c, '\u{200b}' | '\u{feff}'))
        .collect();
    match cleaned.trim().to_lowercase().as_str() {
        "helium" | "helium browser" => Some("helium"),
        "zen" | "zen browser" | "zen twilight" | "zen browser private browsing" => Some("zen"),
        "librewolf" | "librewolf private browsing" => Some("librewolf"),
        "tor browser" | "tor browser private browsing" => Some("torbrowser"),
        "opera gx" => Some("operagx"),
        "opera" => Some("opera"),
        "brave" | "brave browser" => Some("brave"),
        "vivaldi" => Some("vivaldi"),
        "arc" | "arc browser" => Some("arc"),
        "microsoft edge" | "edge" => Some("edge"),
        "google chrome" | "chrome" => Some("chrome"),
        "chromium" => Some("chromium"),
        "mozilla firefox"
        | "firefox"
        | "firefox developer edition"
        | "firefox nightly"
        | "firefox private browsing"
        | "mozilla firefox private browsing" => Some("firefox"),
        "safari" => Some("safari"),
        _ => None,
    }
}

/// Identifies a browser from the trailing suffix of a window title (or from a
/// bare browser title such as `"Helium"` or `"Zen Browser"`).
pub fn browser_from_title_suffix(window_title: &str) -> Option<&'static str> {
    let title = window_title.trim();
    if title.is_empty() {
        return None;
    }
    let candidate = match rfind_title_separator(title) {
        Some((idx, sep_len)) => &title[idx + sep_len..],
        None => title,
    };
    match_browser_label(candidate)
}

/// Identifies a browser from its full executable path by inspecting path
/// segments. Specific forks are checked before generic upstream bases so paths
/// like `...\imput\Helium\Application\chrome.exe` or
/// `...\Tor Browser\Browser\firefox.exe` resolve to the fork.
pub fn browser_from_exe_path(exe_path: &str) -> Option<&'static str> {
    let normalized = exe_path.trim().replace('\\', "/").to_lowercase();
    if normalized.is_empty() {
        return None;
    }
    let segments: Vec<&str> = normalized
        .split('/')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let has = |pred: &dyn Fn(&str) -> bool| segments.iter().any(|seg| pred(seg));

    // Check specific Chromium and Firefox forks first.
    if has(&|s| s == "helium" || s == "helium-browser" || s.starts_with("helium ")) {
        return Some("helium");
    }
    if has(&|s| s == "opera gx" || s == "opera-gx" || s == "operagx" || s.starts_with("opera gx "))
    {
        return Some("operagx");
    }
    if has(&|s| s == "tor browser" || s == "tor-browser" || s == "torbrowser") {
        return Some("torbrowser");
    }
    if has(&|s| s == "librewolf" || s.starts_with("librewolf-")) {
        return Some("librewolf");
    }
    if has(&|s| s == "zen browser" || s == "zen-browser" || s == "zen" || s == "zen-twilight") {
        return Some("zen");
    }
    if has(&|s| s == "bravesoftware" || s == "brave-browser" || s == "brave") {
        return Some("brave");
    }
    if has(&|s| s == "vivaldi" || s.starts_with("vivaldi-")) {
        return Some("vivaldi");
    }
    if has(&|s| s == "arc") {
        return Some("arc");
    }
    if has(&|s| s == "edge" || s == "microsoft-edge" || s.starts_with("microsoft-edge-")) {
        return Some("edge");
    }
    if has(&|s| s == "chromium" || s == "ungoogled-chromium") {
        return Some("chromium");
    }

    // Generic upstream bases.
    if has(&|s| s == "opera" || s == "opera software") {
        return Some("opera");
    }
    if has(&|s| {
        s == "mozilla firefox"
            || s == "firefox"
            || s == "firefox-esr"
            || s == "firefox-nightly"
            || s == "firefox-developer-edition"
    }) {
        return Some("firefox");
    }
    if has(&|s| s == "chrome" || s == "google-chrome" || s.starts_with("google-chrome-")) {
        return Some("chrome");
    }

    None
}

/// Resolves the canonical browser catalog id for a window when `process_name`
/// is a known browser or upstream browser base.
///
/// Precedence:
/// 1. Specific (non-generic) browser from the window title suffix.
/// 2. Browser from the executable path (handles forks like Opera GX whose
///    window title may still say ` - Opera`, or Helium on a bare `New Tab`).
/// 3. Generic browser from the window title suffix.
pub fn disambiguate_browser(
    process_name: &str,
    exe_path: Option<&str>,
    window_title: &str,
) -> Option<&'static str> {
    if !is_browser_process(process_name) {
        return None;
    }

    let from_title = browser_from_title_suffix(window_title);
    if let Some(title_id) = from_title {
        if !is_generic_base(title_id) {
            return Some(title_id);
        }
    }

    if let Some(path_id) = exe_path.and_then(browser_from_exe_path) {
        return Some(path_id);
    }

    from_title
}

/// Extracts the active page subject from a browser window title, stripping
/// trailing browser names separated by `" - "`, `" — "`, or `" – "`.
/// Returns `None` when the title is empty, a blank new-tab label, or only the
/// browser's own name.
pub fn extract_browser_subject(app_name: &str, window_title: &str) -> Option<String> {
    let title = window_title.trim();
    if title.is_empty() {
        return None;
    }

    let (subject, had_separator) = match rfind_title_separator(title) {
        Some((idx, _)) => (title[..idx].trim(), true),
        None => (title, false),
    };

    if subject.is_empty() || subject.eq_ignore_ascii_case(app_name) {
        return None;
    }

    if !had_separator && match_browser_label(subject).is_some() {
        return None;
    }

    let lower = subject.to_lowercase();
    if matches!(
        lower.as_str(),
        "new tab" | "new private tab" | "private browsing" | "start page" | "speed dial"
    ) {
        return None;
    }

    Some(subject.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helium_running_as_chrome_is_disambiguated_by_title_and_path() {
        assert_eq!(
            disambiguate_browser(
                "chrome.exe",
                Some(r"C:\Users\alice\AppData\Local\imput\Helium\Application\chrome.exe"),
                "GitHub - Helium",
            ),
            Some("helium")
        );
        assert_eq!(
            disambiguate_browser(
                "chrome.exe",
                Some(r"C:\Users\alice\AppData\Local\imput\Helium\Application\chrome.exe"),
                "New Tab",
            ),
            Some("helium")
        );
        assert_eq!(
            disambiguate_browser("chrome.exe", None, "Rust Docs - Helium"),
            Some("helium")
        );
    }

    #[test]
    fn opera_gx_running_as_opera_prefers_path_over_generic_opera_title() {
        assert_eq!(
            disambiguate_browser(
                "opera.exe",
                Some(r"C:\Users\alice\AppData\Local\Programs\Opera GX\opera.exe"),
                "YouTube - Opera",
            ),
            Some("operagx")
        );
        assert_eq!(
            disambiguate_browser("opera.exe", None, "YouTube - Opera GX"),
            Some("operagx")
        );
    }

    #[test]
    fn tor_browser_running_as_firefox_is_disambiguated() {
        assert_eq!(
            disambiguate_browser(
                "firefox.exe",
                Some(r"C:\Users\alice\Desktop\Tor Browser\Browser\firefox.exe"),
                "DuckDuckGo — Tor Browser",
            ),
            Some("torbrowser")
        );
        assert_eq!(
            disambiguate_browser(
                "firefox.exe",
                Some(r"C:\Users\alice\Desktop\Tor Browser\Browser\firefox.exe"),
                "",
            ),
            Some("torbrowser")
        );
    }

    #[test]
    fn firefox_and_zen_and_librewolf_and_brave_and_edge_disambiguate_cleanly() {
        assert_eq!(
            disambiguate_browser("zen.exe", None, "GitHub — Zen Browser"),
            Some("zen")
        );
        assert_eq!(
            disambiguate_browser("librewolf.exe", None, "ArchWiki — LibreWolf"),
            Some("librewolf")
        );
        assert_eq!(
            disambiguate_browser("brave.exe", None, "Docs - Brave"),
            Some("brave")
        );
        // Edge inserts a zero-width space between "Microsoft" and "Edge".
        assert_eq!(
            disambiguate_browser("msedge.exe", None, "Portal - Work - Microsoft\u{200b} Edge"),
            Some("edge")
        );
        assert_eq!(
            disambiguate_browser(
                "chrome.exe",
                Some(r"C:\Program Files\Google\Chrome\Application\chrome.exe"),
                "GitHub - Google Chrome",
            ),
            Some("chrome")
        );
    }

    #[test]
    fn non_browser_processes_are_never_disambiguated() {
        assert_eq!(
            disambiguate_browser(
                "code.exe",
                Some(r"C:\Users\alice\Helium\code.exe"),
                "helium.rs - Visual Studio Code",
            ),
            None
        );
        assert_eq!(
            disambiguate_browser("filmora.exe", None, "Project - Helium"),
            None
        );
    }

    #[test]
    fn browser_subject_handles_em_dash_en_dash_and_blank_tabs() {
        assert_eq!(
            extract_browser_subject("Zen Browser", "Rust Documentation — Zen Browser").as_deref(),
            Some("Rust Documentation")
        );
        assert_eq!(
            extract_browser_subject("Firefox", "GitHub – Mozilla Firefox").as_deref(),
            Some("GitHub")
        );
        assert_eq!(
            extract_browser_subject("Helium Browser", "GitHub - Helium").as_deref(),
            Some("GitHub")
        );
        assert_eq!(extract_browser_subject("Helium Browser", "Helium"), None);
        assert_eq!(
            extract_browser_subject("Helium Browser", "New Tab - Helium"),
            None
        );
        assert_eq!(extract_browser_subject("Firefox", "Mozilla Firefox"), None);
        assert_eq!(extract_browser_subject("Firefox", ""), None);
    }
}
