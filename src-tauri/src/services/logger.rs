//! Structured, target-filtered, colorized terminal logger.
//!
//! Filters out noisy third-party `TRACE`/`DEBUG`/`INFO` logs (such as
//! `discord_presence` frame-polling loops) while keeping internal app logs
//! at `DEBUG` and external warnings/errors visible. Supports `RUST_LOG`,
//! `NO_COLOR`, `CLICOLOR`, `CLICOLOR_FORCE`, and `TERM=dumb`, while emitting
//! plain-text `"app-log"` events to the Tauri frontend.

use log::{Level, LevelFilter, Log, Metadata, Record};
use std::borrow::Cow;
use std::sync::OnceLock;
use tauri::Emitter;

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
static LOGGER: OnceLock<AppLogger> = OnceLock::new();

const INTERNAL_ROOTS: &[&str] = &["better_rich_presence_lib", "better_rich_presence"];

#[derive(Clone, serde::Serialize)]
struct LogPayload {
    level: String,
    message: String,
    timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogFilter {
    default_level: LevelFilter,
    rules: Vec<(String, LevelFilter)>,
}

impl Default for LogFilter {
    fn default() -> Self {
        Self {
            default_level: LevelFilter::Warn,
            rules: INTERNAL_ROOTS
                .iter()
                .map(|&root| (root.to_string(), LevelFilter::Debug))
                .collect(),
        }
    }
}

impl LogFilter {
    /// Builds a filter from the current `RUST_LOG` environment variable,
    /// falling back to the built-in defaults when unset or empty.
    pub fn from_env() -> Self {
        match std::env::var("RUST_LOG") {
            Ok(spec) => Self::from_env_str(&spec),
            Err(_) => Self::default(),
        }
    }

    /// Parses a `RUST_LOG`-style comma-separated filter specification.
    ///
    /// Supports global levels (e.g. `warn`, `debug`) and target-specific
    /// directives (e.g. `discord_presence=trace`). When only target directives
    /// are given, the built-in internal `DEBUG` / external `WARN` defaults are
    /// preserved for unspecified targets.
    pub fn from_env_str(spec: &str) -> Self {
        let trimmed = spec.trim();
        if trimmed.is_empty() {
            return Self::default();
        }

        let mut global_override: Option<LevelFilter> = None;
        let mut parsed_rules: Vec<(String, LevelFilter)> = Vec::new();

        for token in trimmed.split(',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            if let Some((target, level_str)) = token.split_once('=') {
                let target = target.trim();
                if target.is_empty() {
                    continue;
                }
                if let Some(level) = parse_level_filter(level_str) {
                    if let Some(existing) = parsed_rules.iter_mut().find(|(t, _)| t == target) {
                        existing.1 = level;
                    } else {
                        parsed_rules.push((target.to_string(), level));
                    }
                }
            } else if let Some(level) = parse_level_filter(token) {
                global_override = Some(level);
            }
        }

        if global_override.is_none() && parsed_rules.is_empty() {
            return Self::default();
        }

        let mut filter = match global_override {
            Some(level) => Self {
                default_level: level,
                rules: Vec::new(),
            },
            None => Self::default(),
        };

        for (target, level) in parsed_rules {
            if let Some(existing) = filter.rules.iter_mut().find(|(t, _)| t == &target) {
                existing.1 = level;
            } else {
                filter.rules.push((target, level));
            }
        }

        filter
    }

    /// Returns whether a log record with `target` and `level` should be logged.
    pub fn enabled(&self, target: &str, level: Level) -> bool {
        let threshold = self
            .rules
            .iter()
            .filter(|(rule_target, _)| target_matches(rule_target, target))
            .max_by_key(|(rule_target, _)| rule_target.len())
            .map(|(_, lvl)| *lvl)
            .unwrap_or(self.default_level);

        level <= threshold
    }

    /// Returns the highest `LevelFilter` required across all configured rules.
    pub fn max_level(&self) -> LevelFilter {
        self.rules
            .iter()
            .map(|(_, lvl)| *lvl)
            .chain(std::iter::once(self.default_level))
            .max()
            .unwrap_or(LevelFilter::Warn)
    }
}

fn target_matches(rule_target: &str, record_target: &str) -> bool {
    record_target == rule_target
        || record_target
            .strip_prefix(rule_target)
            .is_some_and(|rest| rest.starts_with("::"))
}

fn parse_level_filter(raw: &str) -> Option<LevelFilter> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "off" => Some(LevelFilter::Off),
        "error" => Some(LevelFilter::Error),
        "warn" => Some(LevelFilter::Warn),
        "info" => Some(LevelFilter::Info),
        "debug" => Some(LevelFilter::Debug),
        "trace" => Some(LevelFilter::Trace),
        _ => None,
    }
}

/// Determines whether ANSI color escape sequences should be emitted based on
/// standard environment variables.
pub fn should_use_color(
    no_color: Option<&str>,
    clicolor: Option<&str>,
    clicolor_force: Option<&str>,
    term: Option<&str>,
) -> bool {
    if clicolor_force.is_some_and(|v| !v.is_empty() && v != "0") {
        return true;
    }
    if no_color.is_some_and(|v| !v.is_empty()) {
        return false;
    }
    if clicolor == Some("0") {
        return false;
    }
    if term == Some("dumb") {
        return false;
    }
    true
}

fn should_use_color_from_env() -> bool {
    let no_color = std::env::var("NO_COLOR").ok();
    let clicolor = std::env::var("CLICOLOR").ok();
    let clicolor_force = std::env::var("CLICOLOR_FORCE").ok();
    let term = std::env::var("TERM").ok();
    should_use_color(
        no_color.as_deref(),
        clicolor.as_deref(),
        clicolor_force.as_deref(),
        term.as_deref(),
    )
}

#[cfg(target_os = "windows")]
fn enable_windows_ansi() {
    use windows::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, CONSOLE_MODE,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE,
    };

    unsafe {
        if let Ok(handle) = GetStdHandle(STD_OUTPUT_HANDLE) {
            let mut mode = CONSOLE_MODE(0);
            if GetConsoleMode(handle, &mut mode).is_ok() {
                let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
}

/// Maps a Rust module target to a concise display tag and ANSI color code.
pub fn component_tag(target: &str) -> (Cow<'static, str>, &'static str) {
    let is_internal = INTERNAL_ROOTS
        .iter()
        .any(|root| target_matches(root, target));

    if is_internal {
        if target.contains("::services::watcher") {
            return (Cow::Borrowed("Watcher"), "\x1b[94m");
        }
        if target.contains("::services::linux") {
            return (Cow::Borrowed("Linux"), "\x1b[32m");
        }
        if target.contains("::services::engine") {
            return (Cow::Borrowed("Engine"), "\x1b[95m");
        }
        if target.contains("::services::discord") {
            return (Cow::Borrowed("Discord"), "\x1b[96m");
        }
        if target.contains("::services::claude_code") {
            return (Cow::Borrowed("Claude"), "\x1b[93m");
        }
        return (Cow::Borrowed("App"), "\x1b[1;37m");
    }

    let crate_name = target
        .split("::")
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("unknown");
    (Cow::Owned(crate_name.to_string()), "\x1b[90m")
}

/// Trims whitespace, flattens newlines to spaces, and truncates safely at a
/// UTF-8 character boundary.
pub fn truncate_for_log(text: &str, max_chars: usize) -> String {
    let normalized = text.replace(['\r', '\n'], " ");
    let trimmed = normalized.trim();
    if max_chars == 0 {
        return String::new();
    }
    let char_count = trimmed.chars().count();
    if char_count <= max_chars {
        return trimmed.to_string();
    }
    if max_chars == 1 {
        return "…".to_string();
    }
    let mut out: String = trimmed.chars().take(max_chars - 1).collect();
    out.push('…');
    out
}

/// Formats a single terminal log line with aligned columns and optional ANSI colors.
pub fn format_terminal_line(
    timestamp: &str,
    level: Level,
    target: &str,
    message: &str,
    color: bool,
) -> String {
    let level_label = match level {
        Level::Error => "ERROR",
        Level::Warn => "WARN ",
        Level::Info => "INFO ",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE",
    };

    let (tag_name, tag_color) = component_tag(target);
    let bracketed_tag = format!("[{tag_name}]");
    let tag_pad_len = 9usize.saturating_sub(bracketed_tag.len());
    let tag_pad = " ".repeat(tag_pad_len);

    if !color {
        return format!("{timestamp}  {level_label}  {bracketed_tag}{tag_pad}  {message}");
    }

    let level_color = match level {
        Level::Error => "\x1b[1;91m",
        Level::Warn => "\x1b[1;93m",
        Level::Info => "\x1b[1;92m",
        Level::Debug => "\x1b[36m",
        Level::Trace => "\x1b[90m",
    };

    let styled_message = style_message(level, message);

    format!(
        "\x1b[90m{timestamp}\x1b[0m  {level_color}{level_label}\x1b[0m  {tag_color}{bracketed_tag}\x1b[0m{tag_pad}  {styled_message}"
    )
}

fn style_message(level: Level, message: &str) -> Cow<'_, str> {
    match level {
        Level::Error => Cow::Owned(format!("\x1b[31m{message}\x1b[0m")),
        Level::Warn => Cow::Owned(format!("\x1b[33m{message}\x1b[0m")),
        Level::Debug | Level::Trace => Cow::Owned(format!("\x1b[2m{message}\x1b[0m")),
        Level::Info => style_info_message(message),
    }
}

fn style_info_message(message: &str) -> Cow<'_, str> {
    if let Some(rest) = message.strip_prefix("Switch: ") {
        let (transition, details) = split_pipe(rest);
        let transition_styled = if let Some((from, to)) = transition.split_once(" → ") {
            format!("\x1b[33m{from}\x1b[0m \x1b[2m→\x1b[0m \x1b[1;97m{to}\x1b[0m")
        } else {
            format!("\x1b[1;97m{transition}\x1b[0m")
        };
        let suffix = details
            .map(|d| format!(" \x1b[2m|\x1b[0m {d}"))
            .unwrap_or_default();
        return Cow::Owned(format!(
            "\x1b[1;96mSwitch:\x1b[0m {transition_styled}{suffix}"
        ));
    }

    if let Some(rest) = message.strip_prefix("Active: ") {
        let (app, details) = split_pipe(rest);
        let suffix = details
            .map(|d| format!(" \x1b[2m|\x1b[0m {d}"))
            .unwrap_or_default();
        return Cow::Owned(format!(
            "\x1b[1;92mActive:\x1b[0m \x1b[1;97m{app}\x1b[0m{suffix}"
        ));
    }

    if let Some(rest) = message.strip_prefix("Update: ") {
        let (app, details) = split_pipe(rest);
        let suffix = details
            .map(|d| format!(" \x1b[2m|\x1b[0m {d}"))
            .unwrap_or_default();
        return Cow::Owned(format!(
            "\x1b[1;36mUpdate:\x1b[0m \x1b[1;97m{app}\x1b[0m{suffix}"
        ));
    }

    Cow::Borrowed(message)
}

fn split_pipe(text: &str) -> (&str, Option<&str>) {
    match text.split_once(" | ") {
        Some((head, tail)) => (head, Some(tail)),
        None => (text, None),
    }
}

struct AppLogger {
    filter: LogFilter,
    use_color: bool,
}

impl Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata.target(), metadata.level())
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        let message = format!("{}", record.args());
        let line = format_terminal_line(
            &timestamp,
            record.level(),
            record.target(),
            &message,
            self.use_color,
        );
        println!("{line}");

        if let Some(app) = APP_HANDLE.get() {
            let _ = app.emit(
                "app-log",
                LogPayload {
                    level: record.level().to_string(),
                    message,
                    timestamp,
                },
            );
        }
    }

    fn flush(&self) {}
}

/// Initializes the global backend logger and binds it to the Tauri `AppHandle`.
pub fn init(app_handle: tauri::AppHandle) {
    let _ = APP_HANDLE.set(app_handle);

    #[cfg(target_os = "windows")]
    enable_windows_ansi();

    let filter = LogFilter::from_env();
    let max_level = filter.max_level();
    let logger = LOGGER.get_or_init(|| AppLogger {
        filter,
        use_color: should_use_color_from_env(),
    });

    let _ = log::set_logger(logger);
    log::set_max_level(max_level);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filter_allows_internal_debug_and_suppresses_external_trace() {
        let filter = LogFilter::default();

        assert!(filter.enabled("better_rich_presence_lib::services::engine", Level::Debug));
        assert!(filter.enabled("better_rich_presence_lib::services::watcher", Level::Info));
        assert!(!filter.enabled("better_rich_presence_lib::services::engine", Level::Trace));

        assert!(filter.enabled("discord_presence::connection", Level::Warn));
        assert!(filter.enabled("discord_presence::connection", Level::Error));
        assert!(!filter.enabled("discord_presence::connection", Level::Info));
        assert!(!filter.enabled("discord_presence::connection", Level::Debug));
        assert!(!filter.enabled("discord_presence::connection", Level::Trace));
    }

    #[test]
    fn rust_log_parsing_supports_global_target_prefix_and_invalid_tokens() {
        let custom = LogFilter::from_env_str(
            "warn, better_rich_presence_lib::services::engine = TRACE, better_rich_presence_lib::services = info, bad=nope, , discord_presence=debug",
        );

        assert!(custom.enabled("better_rich_presence_lib::services::engine", Level::Trace));
        assert!(custom.enabled("better_rich_presence_lib::services::watcher", Level::Info));
        assert!(!custom.enabled("better_rich_presence_lib::services::watcher", Level::Debug));
        assert!(custom.enabled("better_rich_presence_lib::models", Level::Warn));
        assert!(!custom.enabled("better_rich_presence_lib::models", Level::Info));
        assert!(custom.enabled("discord_presence::client", Level::Debug));
        assert!(!custom.enabled("discord_presence::client", Level::Trace));

        let target_only = LogFilter::from_env_str("discord_presence=trace");
        assert!(target_only.enabled("better_rich_presence_lib::services::engine", Level::Debug));
        assert!(target_only.enabled("discord_presence::client", Level::Trace));
        assert!(!target_only.enabled("reqwest::connect", Level::Info));

        let invalid_only = LogFilter::from_env_str("not_a_level, =debug, foo=invalid");
        assert_eq!(invalid_only, LogFilter::default());
    }

    #[test]
    fn should_use_color_respects_standard_env_vars() {
        assert!(should_use_color(None, None, None, None));
        assert!(!should_use_color(Some("1"), None, None, None));
        assert!(should_use_color(Some(""), None, None, None));
        assert!(!should_use_color(None, Some("0"), None, None));
        assert!(!should_use_color(None, None, None, Some("dumb")));
        assert!(should_use_color(
            Some("1"),
            Some("0"),
            Some("1"),
            Some("dumb")
        ));
        assert!(!should_use_color(Some("1"), None, Some("0"), None));
    }

    #[test]
    fn component_tag_maps_internal_and_external_targets() {
        assert_eq!(
            component_tag("better_rich_presence_lib::services::watcher").0,
            "Watcher"
        );
        assert_eq!(
            component_tag("better_rich_presence_lib::services::engine").0,
            "Engine"
        );
        assert_eq!(
            component_tag("better_rich_presence_lib::services::discord").0,
            "Discord"
        );
        assert_eq!(
            component_tag("better_rich_presence_lib::services::claude_code_server").0,
            "Claude"
        );
        assert_eq!(
            component_tag("better_rich_presence_lib::services::linux::backends").0,
            "Linux"
        );
        assert_eq!(component_tag("better_rich_presence_lib").0, "App");
        assert_eq!(
            component_tag("discord_presence::connection::manager").0,
            "discord_presence"
        );
    }

    #[test]
    fn format_terminal_line_handles_plain_and_colored_transitions() {
        let plain = format_terminal_line(
            "21:15:00",
            Level::Info,
            "better_rich_presence_lib::services::engine",
            "Switch: Discord → Visual Studio Code | Coding",
            false,
        );
        assert!(!plain.contains("\x1b["));
        assert_eq!(
            plain,
            "21:15:00  INFO   [Engine]   Switch: Discord → Visual Studio Code | Coding"
        );

        let colored = format_terminal_line(
            "21:15:00",
            Level::Info,
            "better_rich_presence_lib::services::engine",
            "Switch: Discord → Visual Studio Code | Coding",
            true,
        );
        assert!(colored.contains("\x1b["));
        assert!(colored.contains("\x1b[0m"));
        assert!(colored.contains("Switch:"));
        assert!(colored.contains("Discord"));
        assert!(colored.contains("Visual Studio Code"));
    }

    #[test]
    fn truncate_for_log_preserves_utf8_and_flattens_newlines() {
        assert_eq!(truncate_for_log("  hello\nworld\r\n", 20), "hello world");
        assert_eq!(truncate_for_log("#🌍│geral-do-geral", 6), "#🌍│ge…");
        assert_eq!(truncate_for_log("abc", 1), "…");
        assert_eq!(truncate_for_log("abc", 0), "");
    }
}
