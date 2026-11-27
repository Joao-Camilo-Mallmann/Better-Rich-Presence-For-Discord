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

/// Returns `true` when `path` sits in a Cargo build output directory
/// (`target\debug` or `target\release`). Such executables may be dev builds
/// that load the Vite dev server instead of an embedded frontend.
pub fn is_cargo_target_path(path: &Path) -> bool {
    let raw = path
        .to_string_lossy()
        .replace('/', "\\")
        .to_ascii_lowercase();
    raw.contains(r"\target\debug\") || raw.contains(r"\target\release\")
}

/// Picks the executable to register for autostart, or `None` when no
/// standalone (frontend-embedded) build is available.
///
/// The installed executable always wins. Otherwise the running executable is
/// used only when it is not a dev build and not inside a Cargo `target`
/// directory.
pub fn select_startup_executable(
    current_exe: &Path,
    is_dev: bool,
    installed_exe: Option<&Path>,
) -> Option<PathBuf> {
    if let Some(installed) = installed_exe {
        return Some(installed.to_path_buf());
    }
    if is_dev || is_cargo_target_path(current_exe) {
        return None;
    }
    Some(current_exe.to_path_buf())
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
        let with_spaces = Path::new(
            r"C:\Users\Alice Smith\AppData\Local\Better Rich Presence\better-rich-presence.exe",
        );
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
        let enabled = [
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let disabled = [
            0x03, 0x00, 0x00, 0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F, 0x01, 0x01,
        ];
        let alt_enabled = [
            0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert!(is_startup_approved(&enabled));
        assert!(is_startup_approved(&alt_enabled));
        assert!(!is_startup_approved(&disabled));
        assert!(is_startup_approved(&[]));
    }

    #[test]
    fn detects_cargo_target_paths() {
        assert!(is_cargo_target_path(Path::new(
            r"D:\repo\src-tauri\target\release\app.exe"
        )));
        assert!(is_cargo_target_path(Path::new(
            r"D:\repo\target\debug\app.exe"
        )));
        assert!(is_cargo_target_path(Path::new("/repo/target/release/app")));
        assert!(!is_cargo_target_path(Path::new(
            r"C:\Users\A\AppData\Local\Better Rich Presence\better-rich-presence.exe"
        )));
    }

    #[test]
    fn installed_exe_wins() {
        let debug_exe = PathBuf::from(r"D:\repo\target\debug\app.exe");
        let installed = PathBuf::from(r"C:\Users\A\AppData\Local\Better Rich Presence\app.exe");
        assert_eq!(
            select_startup_executable(&debug_exe, true, Some(&installed)),
            Some(installed)
        );
    }

    #[test]
    fn dev_build_without_installed_selects_nothing() {
        let debug_exe = PathBuf::from(r"D:\repo\target\debug\app.exe");
        assert_eq!(select_startup_executable(&debug_exe, true, None), None);
    }

    #[test]
    fn target_release_exe_is_never_selected() {
        let release_exe = PathBuf::from(r"D:\repo\target\release\app.exe");
        assert_eq!(select_startup_executable(&release_exe, false, None), None);
    }

    #[test]
    fn standalone_current_exe_is_used() {
        let exe = PathBuf::from(r"E:\Apps\Better Rich Presence\app.exe");
        assert_eq!(
            select_startup_executable(&exe, false, None),
            Some(exe.clone())
        );
    }
}
