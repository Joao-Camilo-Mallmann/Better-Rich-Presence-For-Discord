//! OS-specific autostart registry synchronization.

#[cfg(target_os = "windows")]
mod windows_impl {
    use crate::domain::startup::{
        format_windows_run_command, is_cargo_target_path, is_startup_approved,
        select_startup_executable, STARTUP_APP_NAME,
    };
    use log::warn;
    use std::path::{Path, PathBuf};
    use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE};
    use winreg::RegKey;

    const RUN_KEY_PATH: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
    const STARTUP_APPROVED_KEY_PATH: &str =
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";

    /// Returns the installer-placed executable when it exists. Builds inside a
    /// Cargo `target` directory are never considered: a plain
    /// `cargo build --release` leaves the frontend out of the binary.
    fn find_installed_exe() -> Option<PathBuf> {
        let local_app_data = std::env::var("LOCALAPPDATA").ok()?;
        let trimmed = local_app_data.trim();
        if trimmed.is_empty() {
            return None;
        }
        let installed = PathBuf::from(trimmed)
            .join("Better Rich Presence")
            .join("better-rich-presence.exe");
        installed.is_file().then_some(installed)
    }

    /// Normalizes an existing Windows `Run` entry for Better Rich Presence without
    /// re-enabling entries the user disabled in MSConfig or Task Manager.
    pub fn sync_autostart_entry() {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let Ok(run_key) = hkcu.open_subkey_with_flags(RUN_KEY_PATH, KEY_READ | KEY_SET_VALUE)
        else {
            return;
        };

        let Ok(existing_cmd) = run_key.get_value::<String, _>(STARTUP_APP_NAME) else {
            // Autostart is not enabled in Run; nothing to normalize.
            return;
        };

        if let Ok(approved_key) = hkcu.open_subkey_with_flags(STARTUP_APPROVED_KEY_PATH, KEY_READ) {
            if let Ok(raw) = approved_key.get_raw_value(STARTUP_APP_NAME) {
                if !is_startup_approved(&raw.bytes) {
                    return;
                }
            }
        }

        let Ok(current_exe) = std::env::current_exe() else {
            return;
        };
        let installed_exe = find_installed_exe();
        let Some(selected_exe) =
            select_startup_executable(&current_exe, cfg!(dev), installed_exe.as_deref())
        else {
            let existing_path = existing_cmd.trim().trim_start_matches('"');
            let existing_path = existing_path.split('"').next().unwrap_or(existing_path);
            if is_cargo_target_path(Path::new(existing_path)) {
                warn!(
                    "Autostart entry points to a build in a Cargo target directory ({existing_path}); \
                     it may not include the frontend. Install the app with the Tauri installer to fix startup."
                );
            }
            return;
        };
        let desired_cmd = format_windows_run_command(&selected_exe);

        if existing_cmd != desired_cmd {
            if let Err(err) = run_key.set_value(STARTUP_APP_NAME, &desired_cmd) {
                warn!("Failed to normalize Windows autostart registry entry: {err}");
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub use windows_impl::sync_autostart_entry;

#[cfg(not(target_os = "windows"))]
pub fn sync_autostart_entry() {}
