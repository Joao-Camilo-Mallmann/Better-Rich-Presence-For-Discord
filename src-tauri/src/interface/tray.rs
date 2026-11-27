//! System tray icon, menu, and live status synchronization.

use crate::domain::tray::{format_tray_snapshot, TraySnapshot};
use crate::models::types::{AppState, EngineEvent};
use std::sync::Mutex;
use tauri::{
    menu::{
        CheckMenuItem, CheckMenuItemBuilder, MenuBuilder, MenuItem, MenuItemBuilder,
        PredefinedMenuItem,
    },
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tokio::sync::mpsc::Sender;

/// Managed handle to the system tray icon and its mutable menu items.
pub struct TrayController {
    tray: TrayIcon<tauri::Wry>,
    status_item: MenuItem<tauri::Wry>,
    share_item: CheckMenuItem<tauri::Wry>,
    last_snapshot: Mutex<Option<TraySnapshot>>,
}

impl TrayController {
    fn apply(&self, snapshot: &TraySnapshot) {
        let mut last = self
            .last_snapshot
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if last.as_ref() == Some(snapshot) {
            return;
        }
        let _ = self.status_item.set_text(&snapshot.status_text);
        let _ = self.share_item.set_checked(snapshot.sharing_checked);
        let _ = self.tray.set_tooltip(Some(&snapshot.tooltip));
        *last = Some(snapshot.clone());
    }
}

/// Unminimizes, shows, and focuses the main window.
pub fn restore_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// Builds the system tray icon and registers [`TrayController`] on the app.
pub fn setup_tray(app: &tauri::App, sharing_enabled: bool) -> tauri::Result<()> {
    let initial = format_tray_snapshot(sharing_enabled, false, "Disconnected", None);

    let status_item = MenuItemBuilder::with_id("status", &initial.status_text)
        .enabled(false)
        .build(app)?;
    let sep_top = PredefinedMenuItem::separator(app)?;
    let share_item = CheckMenuItemBuilder::with_id("toggle_share", "Share on Discord")
        .checked(initial.sharing_checked)
        .build(app)?;
    let show_item = MenuItemBuilder::with_id("show", "Open Better Rich Presence").build(app)?;
    let sep_bottom = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[
            &status_item,
            &sep_top,
            &share_item,
            &show_item,
            &sep_bottom,
            &quit_item,
        ])
        .build()?;

    let icon = match tauri::image::Image::from_bytes(include_bytes!("../../icons/tray.png")) {
        Ok(img) => img,
        Err(_) => app
            .default_window_icon()
            .expect("default window icon must exist")
            .clone(),
    };

    let tray = TrayIconBuilder::new()
        .icon(icon)
        .tooltip(&initial.tooltip)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                restore_main_window(app);
            }
            "toggle_share" => {
                let app_handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    let Some(state) = app_handle.try_state::<AppState>() else {
                        return;
                    };
                    let mut next = state.get_settings().await;
                    next.global_enabled = !next.global_enabled;
                    if state.update_settings(next).await.is_ok() {
                        if let Some(tx) = app_handle.try_state::<Sender<EngineEvent>>() {
                            let _ = tx.send(EngineEvent::ClaudeCodeConfigChanged).await;
                        }
                    }
                });
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                restore_main_window(tray.app_handle());
            }
        })
        .build(app)?;

    app.manage(TrayController {
        tray,
        status_item,
        share_item,
        last_snapshot: Mutex::new(Some(initial)),
    });

    Ok(())
}

/// Refreshes the tray tooltip, status item, and Share toggle from `AppState`.
pub async fn refresh_tray_from_state(app_state: &AppState) {
    let (sharing_enabled, connected, presence_state, active_app) = {
        let inner = app_state.inner.read().await;
        (
            inner.settings.global_enabled,
            inner.connection_info.connected,
            format!("{:?}", inner.presence_state),
            inner
                .current_presence
                .as_ref()
                .map(|presence| presence.large_text.clone()),
        )
    };

    let snapshot = format_tray_snapshot(
        sharing_enabled,
        connected,
        &presence_state,
        active_app.as_deref(),
    );

    if let Some(controller) = app_state.app_handle.try_state::<TrayController>() {
        controller.apply(&snapshot);
    }
}
