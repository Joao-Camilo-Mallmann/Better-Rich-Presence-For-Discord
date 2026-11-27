//! Pure startup & autostart rules (no OS APIs or filesystem access).

use std::path::{Path, PathBuf};

/// Registry entry name shown in MSConfig / Task Manager Startup Apps.
pub const STARTUP_APP_NAME: &str = "Better Rich Presence";

/// CLI argument passed when launched automatically at OS login.
pub const MINIMIZED_ARG: &str = "--minimized";

/// Returns `true` when the application was launched with `--minimized`.
pub fn should_start_minimized<I, S>(args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    args.into_iter().any(|arg| arg.as_ref() == MINIMIZED_ARG)
}

/// Strips the Windows verbatim path prefix (`\\?\`) if present.
pub fn strip_verbatim_prefix(raw: &str) -> &str {
    raw.strip_prefix(r"\\?\").unwrap_or(raw)
}

/// Formats a Windows `Run` registry command with a double-quoted executable path
/// followed by `--minimized`.
pub fn format_windows_run_command(exe_path: &Path) -> String {
    let raw = exe_path.to_string_lossy();
    let clean = strip_verbatim_prefix(&raw);
    format!("\"{clean}\" {MINIMIZED_ARG}")
}

/// Evaluates raw Windows `StartupApproved\Run` binary data.
///
/// Windows stores a 12-byte value where bit 0 of byte 0 is `0` when enabled
/// (commonly `0x02`) and `1` when disabled by Task Manager / MSConfig
/// (commonly `0x03`), followed by an 8-byte FILETIME timestamp in bytes `4..12`
/// that is all zeros when enabled. Missing or short values default to approved.
pub fn is_startup_approved(bytes: &[u8]) -> bool {
    if bytes.len() < 12 {
        return true;
    }
    (bytes[0] & 1) == 0 && bytes[4..12].iter().all(|&b| b == 0)
}

/// Picks the executable path to register for autostart.
///
/// In dev builds (`is_dev == true`), prefers the first available release or
/// installed executable so login autostart never points at a debug binary that
/// expects a live Vite dev server.
pub fn select_startup_executable(
    current_exe: &Path,
    is_dev: bool,
    available_release_exes: &[PathBuf],
) -> PathBuf {
    if is_dev {
        if let Some(release_exe) = available_release_exes.first() {
            return release_exe.clone();
        }
    }
    current_exe.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_minimized_flag() {
        assert!(!should_start_minimized(["app.exe"]));
        assert!(should_start_minimized(["app.exe", "--minimized"]));
        assert!(!should_start_minimized(["app.exe", "--minimize"]));
    }

    #[test]
    fn formats_quoted_windows_run_command_and_strips_verbatim_prefix() {
        let with_spaces = Path::new(r"C:\Users\Alice Smith\AppData\Local\Better Rich Presence\better-rich-presence.exe");
        assert_eq!(
            format_windows_run_command(with_spaces),
            r#""C:\Users\Alice Smith\AppData\Local\Better Rich Presence\better-rich-presence.exe" --minimized"#
        );

        let verbatim = Path::new(r"\\?\D:\Codes\Better Rich Presence\better-rich-presence.exe");
        assert_eq!(
            format_windows_run_command(verbatim),
            r#""D:\Codes\Better Rich Presence\better-rich-presence.exe" --minimized"#
        );
    }

    #[test]
    fn parses_startup_approved_enabled_and_disabled_bytes() {
        let enabled = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let disabled = [0x03, 0x00, 0x00, 0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F, 0x01, 0x01];
        let alt_enabled = [0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        assert!(is_startup_approved(&enabled));
        assert!(is_startup_approved(&alt_enabled));
        assert!(!is_startup_approved(&disabled));
        assert!(is_startup_approved(&[]));
    }

    #[test]
    fn selects_release_exe_in_dev_when_available() {
        let debug_exe = PathBuf::from(r"D:\repo\target\debug\better-rich-presence.exe");
        let release_exe = PathBuf::from(r"D:\repo\target\release\better-rich-presence.exe");

        assert_eq!(
            select_startup_executable(&debug_exe, true, &[release_exe.clone()]),
            release_exe
        );
        assert_eq!(
            select_startup_executable(&debug_exe, true, &[]),
            debug_exe
        );
        assert_eq!(
            select_startup_executable(&debug_exe, false, &[release_exe]),
            debug_exe
        );
    }
}
