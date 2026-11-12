use crate::models::types::AppError;
use crate::services::detection_status::PlatformStatus;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemProcessInfo {
    pub process_name: String,
    pub display_name: String,
}

fn format_display_name(process_name: &str) -> String {
    let name_without_ext = process_name.replace(".exe", "").replace(".EXE", "");
    
    // Capitalize first letter of each word
    let parts: Vec<&str> = name_without_ext.split(|c| c == '-' || c == '_' || c == ' ').collect();
    let capitalized: Vec<String> = parts.into_iter().map(|p| {
        let mut chars = p.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str()
        }
    }).collect();
    
    capitalized.join(" ")
}

#[tauri::command]
pub async fn get_running_processes() -> Result<Vec<SystemProcessInfo>, AppError> {
    let mut processes = list_processes();
    processes.sort_by(|a, b| a.process_name.cmp(&b.process_name));
    processes.dedup_by(|a, b| a.process_name == b.process_name);
    Ok(processes)
}

/// Platform and activity-detection status for the Settings page.
#[tauri::command]
pub async fn get_platform_status() -> Result<PlatformStatus, AppError> {
    Ok(crate::services::detection_status::platform_status())
}

#[cfg(not(target_os = "linux"))]
fn list_processes() -> Vec<SystemProcessInfo> {
    use sysinfo::System;
    let mut sys = System::new();

    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        sysinfo::ProcessRefreshKind::nothing()
    );

    let mut processes = Vec::new();
    for (_pid, process) in sys.processes() {
        let name = process.name().to_string_lossy().into_owned();
        if !name.is_empty() {
            let lower_name = name.to_lowercase();

            // Filter common noisy OS background processes to keep picker clean
            if lower_name == "svchost.exe" || lower_name == "conhost.exe" || lower_name == "taskhostw.exe"
               || lower_name == "system idle process" || lower_name == "system" || lower_name == "registry"
               || lower_name == "smss.exe" || lower_name == "csrss.exe" || lower_name == "wininit.exe"
               || lower_name == "services.exe" || lower_name == "lsass.exe" || lower_name == "winlogon.exe"
               || lower_name == "better-rich-presence.exe" || lower_name == "better_rich_presence.exe"
               || lower_name == "runtimebroker.exe" || lower_name == "ctfmon.exe" || lower_name == "searchhost.exe"
               || lower_name == "shellexperiencehost.exe" || lower_name == "startmenuexperiencehost.exe" {
                continue;
            }

            let display_name = format_display_name(&name);
            processes.push(SystemProcessInfo {
                process_name: lower_name,
                display_name,
            });
        }
    }
    processes
}

/// Linux: the current user's processes that have an executable (kernel
/// threads have none), named by executable file name, without desktop daemons.
#[cfg(target_os = "linux")]
fn list_processes() -> Vec<SystemProcessInfo> {
    use crate::services::linux::process::exe_name;
    use std::os::unix::fs::MetadataExt;

    let Ok(my_uid) = std::fs::metadata("/proc/self").map(|m| m.uid()) else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return Vec::new();
    };

    let mut processes = Vec::new();
    for entry in entries.flatten() {
        let Some(pid) = entry.file_name().to_str().and_then(|s| s.parse::<u32>().ok()) else {
            continue;
        };
        // `/proc/<pid>` is owned by the process's user.
        if entry.metadata().map(|m| m.uid()).ok() != Some(my_uid) {
            continue;
        }
        let Some(name) = exe_name(pid) else {
            continue;
        };
        if is_linux_background_process(&name) {
            continue;
        }
        processes.push(SystemProcessInfo {
            // `foo.bin` wrappers display as "Foo"; rules still match `foo.bin`.
            display_name: format_display_name(name.trim_end_matches(".bin")),
            process_name: name,
        });
    }
    processes
}

/// Desktop daemons and helpers that never make sense as presence rules.
#[cfg(target_os = "linux")]
fn is_linux_background_process(name: &str) -> bool {
    const EXACT: &[&str] = &[
        "systemd", "sh", "bash", "zsh", "fish", "dash", "sudo", "bwrap", "gjs", "gjs-console",
        "wireplumber", "dconf-service", "gnome-shell", "gnome-session-binary", "gnome-session-ctl",
        "gnome-keyring-daemon", "gnome-shell-calendar-server", "mutter-x11-frames", "xwayland",
        "ssh-agent", "gpg-agent", "mpris-proxy", "update-notifier", "user-session-helper",
        "localsearch-3", "goa-daemon", "goa-identity-service", "kded5", "kded6", "kwin_wayland",
        "kwin_x11", "plasmashell", "better-rich-presence", "better_rich_presence",
        "chrome_crashpad_handler", "crashpad_handler", "qtwebengineprocess",
    ];
    const PREFIXES: &[&str] = &[
        "dbus-", "pipewire", "gsd-", "gvfs", "xdg-", "at-spi", "ibus", "evolution-", "tracker-",
        "snapd", "zeitgeist", "p11-kit", "polkit", "flatpak-", "obex", "gnome-remote-desktop",
        "gdm-", "gnome-session", "gcr-",
    ];
    EXACT.contains(&name) || PREFIXES.iter().any(|prefix| name.starts_with(prefix))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn linux_background_processes_are_filtered() {
        assert!(is_linux_background_process("dbus-daemon"));
        assert!(is_linux_background_process("xdg-desktop-portal-gnome"));
        assert!(is_linux_background_process("gsd-power"));
        assert!(is_linux_background_process("gnome-session-service"));
        assert!(is_linux_background_process("qtwebengineprocess"));
        assert!(!is_linux_background_process("firefox"));
        assert!(!is_linux_background_process("gnome-text-editor"));
        assert!(!is_linux_background_process("code"));
    }

    #[test]
    fn linux_process_list_has_no_kernel_threads() {
        let processes = list_processes();
        assert!(processes.iter().all(|p| !p.process_name.starts_with("kworker")));
        assert!(processes.iter().all(|p| !p.process_name.contains('/')));
    }
}
