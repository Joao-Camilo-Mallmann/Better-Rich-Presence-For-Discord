//! Pure formatting rules for the system tray tooltip and status row.

/// Snapshot of what the system tray displays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraySnapshot {
    pub tooltip: String,
    pub status_text: String,
    pub sharing_checked: bool,
}

/// Formats the tray tooltip, disabled status item text, and Share check state.
pub fn format_tray_snapshot(
    sharing_enabled: bool,
    connected: bool,
    presence_state: &str,
    active_app: Option<&str>,
) -> TraySnapshot {
    if !sharing_enabled {
        return TraySnapshot {
            tooltip: "Better Rich Presence — Sharing off".to_string(),
            status_text: "Status: Sharing off".to_string(),
            sharing_checked: false,
        };
    }

    if presence_state == "PausedByGame" {
        return TraySnapshot {
            tooltip: "Better Rich Presence — Paused for a game".to_string(),
            status_text: "Status: Paused for a game".to_string(),
            sharing_checked: true,
        };
    }

    if connected || presence_state == "Connected" || presence_state == "Updating" {
        let shown = active_app.map(str::trim).filter(|name| {
            !name.is_empty() && *name != "Better Rich Presence" && *name != "Better RPC"
        });
        if let Some(app) = shown {
            return TraySnapshot {
                tooltip: format!("Better Rich Presence — Showing {app}"),
                status_text: format!("Showing: {app}"),
                sharing_checked: true,
            };
        }
        return TraySnapshot {
            tooltip: "Better Rich Presence — Live on Discord".to_string(),
            status_text: "Status: Live on Discord".to_string(),
            sharing_checked: true,
        };
    }

    if presence_state == "WaitingDiscord" {
        return TraySnapshot {
            tooltip: "Better Rich Presence — Connecting to Discord".to_string(),
            status_text: "Status: Connecting to Discord".to_string(),
            sharing_checked: true,
        };
    }

    TraySnapshot {
        tooltip: "Better Rich Presence — Not connected to Discord".to_string(),
        status_text: "Status: Not connected to Discord".to_string(),
        sharing_checked: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_sharing_off_and_paused_states() {
        let off = format_tray_snapshot(false, true, "Connected", Some("Visual Studio Code"));
        assert_eq!(off.tooltip, "Better Rich Presence — Sharing off");
        assert_eq!(off.status_text, "Status: Sharing off");
        assert!(!off.sharing_checked);

        let paused = format_tray_snapshot(true, true, "PausedByGame", Some("Elden Ring"));
        assert_eq!(paused.tooltip, "Better Rich Presence — Paused for a game");
        assert_eq!(paused.status_text, "Status: Paused for a game");
        assert!(paused.sharing_checked);
    }

    #[test]
    fn formats_live_connecting_and_disconnected_states() {
        let with_app = format_tray_snapshot(true, true, "Connected", Some("Visual Studio Code"));
        assert_eq!(
            with_app.tooltip,
            "Better Rich Presence — Showing Visual Studio Code"
        );
        assert_eq!(with_app.status_text, "Showing: Visual Studio Code");
        assert!(with_app.sharing_checked);

        let idle_placeholder =
            format_tray_snapshot(true, true, "Connected", Some("Better Rich Presence"));
        assert_eq!(
            idle_placeholder.tooltip,
            "Better Rich Presence — Live on Discord"
        );
        assert_eq!(idle_placeholder.status_text, "Status: Live on Discord");

        let waiting = format_tray_snapshot(true, false, "WaitingDiscord", None);
        assert_eq!(
            waiting.tooltip,
            "Better Rich Presence — Connecting to Discord"
        );
        assert_eq!(waiting.status_text, "Status: Connecting to Discord");

        let disconnected = format_tray_snapshot(true, false, "Disconnected", None);
        assert_eq!(
            disconnected.tooltip,
            "Better Rich Presence — Not connected to Discord"
        );
        assert_eq!(disconnected.status_text, "Status: Not connected to Discord");
    }
}
