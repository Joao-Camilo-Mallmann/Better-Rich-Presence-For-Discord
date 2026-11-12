//! # Detection Status Module
//!
//! Records which window and idle detection backends the watcher is using,
//! plus an optional setup hint, so the frontend can show how activity is
//! detected on the current platform.

use serde::Serialize;
use std::sync::RwLock;

/// Backends currently used by the watcher, written on every (re)detection.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct DetectionStatus {
    /// Focused-window backend label (e.g. `"win32"`, `"gnome-window-calls"`),
    /// `"process-scan"` when only running rule apps can be detected.
    pub window_backend: Option<String>,
    /// Idle backend label, `None` when idle time cannot be measured.
    pub idle_backend: Option<String>,
    /// User-facing setup hint when detection is degraded.
    pub hint: Option<String>,
}

/// Platform status returned to the frontend by `get_platform_status`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlatformStatus {
    /// `std::env::consts::OS` (`"windows"`, `"linux"`, …).
    pub os: String,
    /// `XDG_SESSION_TYPE` on Linux (`"wayland"`, `"x11"`), `None` elsewhere.
    pub session_type: Option<String>,
    /// `XDG_CURRENT_DESKTOP` on Linux (e.g. `"ubuntu:GNOME"`), `None` elsewhere.
    pub desktop: Option<String>,
    pub window_backend: Option<String>,
    pub idle_backend: Option<String>,
    pub hint: Option<String>,
}

static STATUS: RwLock<DetectionStatus> = RwLock::new(DetectionStatus {
    window_backend: None,
    idle_backend: None,
    hint: None,
});

/// Replaces the current detection status.
pub fn set(status: DetectionStatus) {
    let mut guard = STATUS.write().unwrap_or_else(|e| e.into_inner());
    *guard = status;
}

/// Returns a copy of the current detection status.
pub fn get() -> DetectionStatus {
    STATUS.read().unwrap_or_else(|e| e.into_inner()).clone()
}

fn non_empty_env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.trim().is_empty())
}

/// Combines the detection status with the OS and desktop session details.
pub fn platform_status() -> PlatformStatus {
    let detection = get();
    let is_linux = cfg!(target_os = "linux");
    PlatformStatus {
        os: std::env::consts::OS.to_string(),
        session_type: if is_linux { non_empty_env("XDG_SESSION_TYPE") } else { None },
        desktop: if is_linux { non_empty_env("XDG_CURRENT_DESKTOP") } else { None },
        window_backend: detection.window_backend,
        idle_backend: detection.idle_backend,
        hint: detection.hint,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_then_get_round_trips() {
        let status = DetectionStatus {
            window_backend: Some("x11".to_string()),
            idle_backend: None,
            hint: Some("Install xprintidle".to_string()),
        };
        set(status.clone());
        assert_eq!(get(), status);
        assert_eq!(platform_status().window_backend.as_deref(), Some("x11"));
    }
}
