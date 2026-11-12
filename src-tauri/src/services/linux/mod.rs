//! # Linux Activity Detection
//!
//! Detects the focused window, running rule apps and idle time on Linux,
//! across X11 and Wayland desktops.
//!
//! Layers:
//! - [`parsers`]: pure parsers for tool output (unit-tested with captured samples).
//! - [`backends`]: adapters that run desktop tools (`busctl`, `xprop`, `hyprctl`, …).
//! - [`process`]: `/proc` process identity and the rule-driven process scan.
//! - [`LinuxDetector`]: picks backends for the session, falls back to the
//!   process scan, and publishes the detection status.

pub mod backends;
pub mod parsers;
pub mod process;

use crate::models::types::AppRule;
use crate::services::detection_status::{self, DetectionStatus};
use crate::services::engine::normalize_process_name;
use backends::{BackendUnavailable, IdleBackend, WindowBackend};
use log::info;
use parsers::WindowInfo;
use process::ProcessScanner;

/// Re-detect backends every 30 ticks (60 s at the 2 s watcher interval)
/// while detection is degraded.
const REDETECT_EVERY_TICKS: u32 = 30;
/// Re-detect after this many consecutive "unavailable" answers.
const MAX_UNAVAILABLE_STREAK: u32 = 3;
/// Window backend label used when only the process scan is available.
pub const PROCESS_SCAN_LABEL: &str = "process-scan";

/// Desktop session facts that decide which backends to try.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SessionEnv {
    pub wayland: bool,
    pub x11: bool,
    pub gnome: bool,
    pub hyprland: bool,
    pub sway: bool,
}

impl SessionEnv {
    /// Builds the session facts from an environment lookup (testable).
    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> Self {
        let var = |key: &str| get(key).filter(|v| !v.trim().is_empty());
        let session_type = var("XDG_SESSION_TYPE").unwrap_or_default().to_lowercase();
        let wayland = session_type == "wayland"
            || (session_type != "x11" && var("WAYLAND_DISPLAY").is_some());
        let x11 = !wayland && (session_type == "x11" || var("DISPLAY").is_some());
        let desktop = var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase();
        Self {
            wayland,
            x11,
            gnome: desktop.contains("gnome"),
            hyprland: var("HYPRLAND_INSTANCE_SIGNATURE").is_some(),
            sway: var("SWAYSOCK").is_some(),
        }
    }

    /// Reads the session facts from the process environment.
    pub fn from_env() -> Self {
        Self::from_lookup(|key| std::env::var(key).ok())
    }
}

/// Focused-window backends to try, in order. X11 hints are never used on
/// Wayland: they only describe XWayland windows and go stale.
pub fn window_candidates(env: &SessionEnv) -> Vec<WindowBackend> {
    let mut candidates = Vec::new();
    if env.hyprland {
        candidates.push(WindowBackend::Hyprland);
    }
    if env.sway {
        candidates.push(WindowBackend::Sway);
    }
    if env.wayland && env.gnome {
        // Focused Window D-Bus answers in one call; current Window Calls
        // versions need `List` plus one `Details` call per window.
        candidates.push(WindowBackend::GnomeFocusedWindow);
        candidates.push(WindowBackend::GnomeWindowCalls);
    }
    if env.x11 {
        candidates.push(WindowBackend::X11);
    }
    candidates
}

/// Idle backends to try, in order.
pub fn idle_candidates(env: &SessionEnv) -> Vec<IdleBackend> {
    let mut candidates = Vec::new();
    if env.gnome {
        candidates.push(IdleBackend::GnomeMutter);
    }
    if env.x11 {
        candidates.push(IdleBackend::XPrintIdle);
    }
    candidates
}

/// User-facing hint for degraded detection, `None` when detection is complete.
pub fn detection_hint(
    env: &SessionEnv,
    window: Option<WindowBackend>,
    idle: Option<IdleBackend>,
    tool_available: impl Fn(&str) -> bool,
) -> Option<String> {
    const RULES_ONLY: &str =
        "Until then, only apps from your rules are detected while they are running.";
    let mut parts: Vec<String> = Vec::new();

    if window.is_none() {
        if env.wayland && env.gnome && !tool_available("busctl") {
            parts.push(format!(
                "Focused-window detection on GNOME needs busctl (systemd). {}",
                RULES_ONLY
            ));
        } else if env.wayland && env.gnome {
            parts.push(format!(
                "Install and enable the \"Focused Window D-Bus\" GNOME Shell extension \
                 (extensions.gnome.org/extension/5592/focused-window-d-bus) to detect the focused window. {}",
                RULES_ONLY
            ));
        } else if env.x11 && !tool_available("xprop") {
            parts.push(format!(
                "Install xprop (package x11-utils) to detect the focused window. {}",
                RULES_ONLY
            ));
        } else {
            parts.push(
                "Focused-window detection is not supported on this desktop yet. \
                 Only apps from your rules are detected while they are running."
                    .to_string(),
            );
        }
    }

    if idle.is_none() {
        if env.x11 {
            parts.push("Install xprintidle to enable idle detection.".to_string());
        } else {
            parts.push("Idle detection is not available on this desktop.".to_string());
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

/// Whether two process names refer to the same app (`.exe`-insensitive,
/// ignoring a `.bin` wrapper suffix on either side).
fn same_app_name(a: &str, b: &str) -> bool {
    process::rule_targets(a, b) || process::rule_targets(b, a)
}

fn is_self_name(name: &str) -> bool {
    matches!(
        normalize_process_name(name).as_str(),
        "better-rich-presence" | "better_rich_presence"
    )
}

/// A window resolved to an app identity, ready for the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedWindow {
    pub process_name: String,
    pub aliases: Vec<String>,
    pub title: String,
    pub pid: Option<u32>,
}

/// Stateful Linux detector owned by the watcher task.
pub struct LinuxDetector {
    env: SessionEnv,
    window_backend: Option<WindowBackend>,
    idle_backend: Option<IdleBackend>,
    unavailable_streak: u32,
    ticks_since_detect: u32,
    detected_once: bool,
    scanner: ProcessScanner,
    self_pid: u32,
}

impl LinuxDetector {
    /// Creates the detector and runs the first backend detection.
    pub fn new() -> Self {
        let mut detector = Self {
            env: SessionEnv::from_env(),
            window_backend: None,
            idle_backend: None,
            unavailable_streak: 0,
            ticks_since_detect: 0,
            detected_once: false,
            scanner: ProcessScanner::default(),
            self_pid: std::process::id(),
        };
        detector.detect();
        detector
    }

    /// Probes the candidate backends and publishes the detection status.
    fn detect(&mut self) {
        let previous = (self.window_backend, self.idle_backend);
        self.window_backend = window_candidates(&self.env)
            .into_iter()
            .find(|backend| backend.focused().is_ok());
        self.idle_backend = idle_candidates(&self.env)
            .into_iter()
            .find(|backend| backend.idle_ms().is_ok());
        self.unavailable_streak = 0;
        self.ticks_since_detect = 0;

        let window_label = self
            .window_backend
            .map(WindowBackend::label)
            .unwrap_or(PROCESS_SCAN_LABEL);
        let idle_label = self.idle_backend.map(IdleBackend::label);

        if !self.detected_once || previous != (self.window_backend, self.idle_backend) {
            info!(
                "[Linux] Detection backends: window={}, idle={} ({:?})",
                window_label,
                idle_label.unwrap_or("none"),
                self.env
            );
            self.detected_once = true;
        }

        detection_status::set(DetectionStatus {
            window_backend: Some(window_label.to_string()),
            idle_backend: idle_label.map(str::to_string),
            hint: detection_hint(
                &self.env,
                self.window_backend,
                self.idle_backend,
                backends::command_available,
            ),
        });
    }

    /// Call once per watcher tick; re-detects backends when needed.
    pub fn tick(&mut self) {
        self.ticks_since_detect = self.ticks_since_detect.saturating_add(1);
        let degraded = self.window_backend.is_none() || self.idle_backend.is_none();
        if self.unavailable_streak >= MAX_UNAVAILABLE_STREAK
            || (degraded && self.ticks_since_detect >= REDETECT_EVERY_TICKS)
        {
            self.detect();
        }
    }

    /// Milliseconds since the last user input (0 when unknown).
    pub fn idle_ms(&self) -> u64 {
        self.idle_backend
            .and_then(|backend| backend.idle_ms().ok())
            .unwrap_or(0)
    }

    /// Resolves a backend window to an app identity; `None` for our own window.
    fn identify(&self, window: WindowInfo) -> Option<DetectedWindow> {
        if window.pid == Some(self.self_pid) {
            return None;
        }
        let primary = window
            .pid
            .and_then(process::process_name)
            .or_else(|| (!window.class.is_empty()).then(|| window.class.clone()))?;
        if is_self_name(&primary) {
            return None;
        }
        let comm = window.pid.and_then(process::comm_name);
        let aliases = process::build_aliases(&primary, &window.class, comm.as_deref());
        Some(DetectedWindow {
            process_name: primary,
            aliases,
            title: window.title,
            pid: window.pid,
        })
    }

    /// Focused window from the active backend (no fallback).
    fn backend_foreground(&mut self) -> Option<DetectedWindow> {
        let backend = self.window_backend?;
        match backend.focused() {
            Ok(window) => {
                self.unavailable_streak = 0;
                window.and_then(|w| self.identify(w))
            }
            Err(BackendUnavailable) => {
                self.unavailable_streak += 1;
                None
            }
        }
    }

    /// Highest-priority running rule app as `(pid, process name)`.
    fn scan_rule_app(&mut self, rules: &[AppRule]) -> Option<(u32, String)> {
        if !rules.iter().any(|r| r.enabled) {
            return None;
        }
        let processes = self.scanner.scan();
        process::pick_priority_rule(&processes, rules, self.self_pid)
    }

    /// Title of a window that belongs to the given app, if the backend can list windows.
    fn title_for_app(&self, backend: WindowBackend, pid: u32, name: &str) -> Option<String> {
        let mut windows = backend.windows();
        let index = windows.iter().position(|w| w.pid == Some(pid)).or_else(|| {
            windows.iter().position(|w| {
                w.pid
                    .and_then(process::process_name)
                    .map_or(false, |n| same_app_name(&n, name))
                    || same_app_name(&w.class, name)
            })
        })?;
        let mut window = windows.swap_remove(index);
        backend.complete_title(&mut window);
        (!window.title.is_empty()).then_some(window.title)
    }

    /// Normal mode: the focused window, or the process-scan fallback when no
    /// focused-window backend is available.
    pub fn foreground_window(&mut self, rules: &[AppRule]) -> Option<DetectedWindow> {
        if self.window_backend.is_some() {
            return self.backend_foreground();
        }
        self.scan_rule_app(rules).map(|(pid, name)| DetectedWindow {
            title: name.clone(),
            process_name: name,
            aliases: Vec::new(),
            pid: Some(pid),
        })
    }

    /// Max Priority Mode: the highest-priority running rule app, flagged as
    /// prioritized (with the foreground app) when another app has focus.
    pub fn priority_window(
        &mut self,
        rules: &[AppRule],
    ) -> Option<(DetectedWindow, bool, Option<String>)> {
        let foreground = self.backend_foreground();
        let Some((pid, name)) = self.scan_rule_app(rules) else {
            return foreground.map(|f| (f, false, None));
        };

        if let Some(fg) = &foreground {
            let same_app = fg.pid == Some(pid)
                || same_app_name(&fg.process_name, &name)
                || fg.aliases.iter().any(|alias| same_app_name(alias, &name));
            if same_app {
                return foreground.map(|f| (f, false, None));
            }
        }

        let title = self
            .window_backend
            .and_then(|backend| self.title_for_app(backend, pid, &name))
            .unwrap_or_else(|| name.clone());
        let foreground_app = foreground.map(|f| f.process_name);
        let is_prioritized = foreground_app.is_some();

        Some((
            DetectedWindow {
                process_name: name,
                aliases: Vec::new(),
                title,
                pid: Some(pid),
            },
            is_prioritized,
            foreground_app,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn env_from(vars: &[(&str, &str)]) -> SessionEnv {
        let map: HashMap<String, String> = vars
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        SessionEnv::from_lookup(|key| map.get(key).cloned())
    }

    fn gnome_wayland() -> SessionEnv {
        env_from(&[
            ("XDG_SESSION_TYPE", "wayland"),
            ("XDG_CURRENT_DESKTOP", "ubuntu:GNOME"),
            ("WAYLAND_DISPLAY", "wayland-0"),
            ("DISPLAY", ":0"),
        ])
    }

    #[test]
    fn gnome_wayland_uses_extensions_and_never_x11() {
        let env = gnome_wayland();
        assert!(env.wayland && env.gnome && !env.x11);
        assert_eq!(
            window_candidates(&env),
            vec![WindowBackend::GnomeFocusedWindow, WindowBackend::GnomeWindowCalls]
        );
        assert_eq!(idle_candidates(&env), vec![IdleBackend::GnomeMutter]);
    }

    #[test]
    fn x11_session_uses_xprop() {
        let env = env_from(&[("XDG_SESSION_TYPE", "x11"), ("XDG_CURRENT_DESKTOP", "XFCE"), ("DISPLAY", ":0")]);
        assert_eq!(window_candidates(&env), vec![WindowBackend::X11]);
        assert_eq!(idle_candidates(&env), vec![IdleBackend::XPrintIdle]);

        let gnome_x11 = env_from(&[("XDG_SESSION_TYPE", "x11"), ("XDG_CURRENT_DESKTOP", "GNOME"), ("DISPLAY", ":0")]);
        assert_eq!(window_candidates(&gnome_x11), vec![WindowBackend::X11]);
        assert_eq!(
            idle_candidates(&gnome_x11),
            vec![IdleBackend::GnomeMutter, IdleBackend::XPrintIdle]
        );
    }

    #[test]
    fn display_without_session_type_counts_as_x11() {
        let env = env_from(&[("DISPLAY", ":0")]);
        assert!(env.x11 && !env.wayland);
    }

    #[test]
    fn wlroots_compositors() {
        let hyprland = env_from(&[
            ("XDG_SESSION_TYPE", "wayland"),
            ("WAYLAND_DISPLAY", "wayland-1"),
            ("DISPLAY", ":1"),
            ("HYPRLAND_INSTANCE_SIGNATURE", "abc"),
        ]);
        assert_eq!(window_candidates(&hyprland), vec![WindowBackend::Hyprland]);
        assert!(idle_candidates(&hyprland).is_empty());

        let sway = env_from(&[("WAYLAND_DISPLAY", "wayland-1"), ("SWAYSOCK", "/run/user/1000/sway.sock")]);
        assert_eq!(window_candidates(&sway), vec![WindowBackend::Sway]);
    }

    #[test]
    fn kde_wayland_has_no_window_backend() {
        let env = env_from(&[("XDG_SESSION_TYPE", "wayland"), ("XDG_CURRENT_DESKTOP", "KDE"), ("WAYLAND_DISPLAY", "wayland-0")]);
        assert!(window_candidates(&env).is_empty());
    }

    #[test]
    fn hint_for_gnome_wayland_without_extension() {
        let hint = detection_hint(&gnome_wayland(), None, Some(IdleBackend::GnomeMutter), |_| true).unwrap();
        assert!(hint.contains("Focused Window D-Bus"));
        assert!(hint.contains("extension/5592"));
        assert!(hint.contains("only apps from your rules"));
    }

    #[test]
    fn no_hint_when_detection_is_complete() {
        assert_eq!(
            detection_hint(
                &gnome_wayland(),
                Some(WindowBackend::GnomeFocusedWindow),
                Some(IdleBackend::GnomeMutter),
                |_| true
            ),
            None
        );
    }

    #[test]
    fn hint_for_unsupported_wayland_desktop() {
        let env = env_from(&[("XDG_SESSION_TYPE", "wayland"), ("XDG_CURRENT_DESKTOP", "KDE")]);
        let hint = detection_hint(&env, None, None, |_| true).unwrap();
        assert!(hint.contains("not supported on this desktop"));
        assert!(hint.contains("Idle detection is not available"));
    }

    #[test]
    fn hint_for_x11_missing_tools() {
        let env = env_from(&[("XDG_SESSION_TYPE", "x11"), ("DISPLAY", ":0")]);
        let hint = detection_hint(&env, None, None, |tool| tool != "xprop").unwrap();
        assert!(hint.contains("x11-utils"));
        assert!(hint.contains("xprintidle"));
    }

    #[test]
    fn same_app_name_is_symmetric() {
        assert!(same_app_name("code.exe", "code"));
        assert!(same_app_name("rocketchat-desktop", "rocketchat-desktop.bin"));
        assert!(same_app_name("rocketchat-desktop.bin", "rocketchat-desktop"));
        assert!(!same_app_name("code", "cursor"));
    }

    #[test]
    fn self_names_are_recognized() {
        assert!(is_self_name("better-rich-presence"));
        assert!(is_self_name("Better_Rich_Presence.exe"));
        assert!(!is_self_name("code"));
    }
}
