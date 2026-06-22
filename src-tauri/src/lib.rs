pub mod commands;
pub mod discord;
pub mod engine;
pub mod presets;
pub mod types;
pub mod watcher;

use crate::commands::AppState;
use crate::types::{AppRule, ConnectionInfo, EngineEvent, PresenceSource, PresenceState, Settings};
use std::sync::Arc;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_store::StoreExt;
use tokio::sync::{mpsc, RwLock};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--minimized"])))
        .setup(|app| {
            // 1. Load settings from store
            let settings_store = app.store("settings.json").expect("Failed to create settings store");
            let settings = if let Some(config) = settings_store.get("config") {
                serde_json::from_value::<Settings>(config).unwrap_or_default()
            } else {
                Settings::default()
            };
            
            // 2. Load app_rules from store
            let rules_store = app.store("rules.json").expect("Failed to create rules store");
            let app_rules = if let Some(rules_val) = rules_store.get("app_rules") {
                serde_json::from_value::<Vec<AppRule>>(rules_val).unwrap_or_else(|_| presets::default_app_rules())
            } else {
                let default_rules = presets::default_app_rules();
                let _ = rules_store.set("app_rules", serde_json::to_value(&default_rules).unwrap());
                let _ = rules_store.save();
                default_rules
            };

            // 3. Set up tray
            let quit_i = MenuItemBuilder::with_id("quit", "Sair").build(app)?;
            let show_i = MenuItemBuilder::with_id("show", "Abrir").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&show_i, &quit_i]).build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
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
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // 4. Create mpsc channel
            let (tx, rx) = mpsc::channel::<EngineEvent>(100);

            // 5. Create DiscordManager
            let (_discord_manager, discord_handle) = discord::DiscordManager::new();

            // Store handle to cleanly disconnect on exit
            let _discord_handle_clone = discord_handle.clone();
            
            // 6. Shared State
            let app_state = AppState {
                app_rules: Arc::new(RwLock::new(app_rules)),
                current_presence: Arc::new(RwLock::new(None)),
                presence_state: Arc::new(RwLock::new(PresenceState::Disconnected)),
                current_source: Arc::new(RwLock::new(PresenceSource::Idle)),
                connection_info: Arc::new(RwLock::new(ConnectionInfo::default())),
                settings: Arc::new(RwLock::new(settings)),
            };

            let rules_ref = Arc::clone(&app_state.app_rules);
            let settings_ref = Arc::clone(&app_state.settings);

            app.manage(app_state);

            // 7. Spawn Watcher Task
            watcher::start_window_watcher(tx.clone());

            // 8. Spawn Engine Task
            engine::start_engine(
                rx,
                discord_handle,
                app.handle().clone(),
                rules_ref,
                settings_ref,
            );

            // Ensure Discord disconnects cleanly on exit
            let _ = app.handle().clone();
            std::thread::spawn(move || {
                // Not ideal but works for now to catch process termination if possible
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_current_presence,
            commands::get_presence_state,
            commands::get_current_source,
            commands::get_connection_status,
            commands::get_app_rules,
            commands::update_app_rule,
            commands::add_app_rule,
            commands::delete_app_rule,
            commands::reset_app_rules_to_defaults,
            commands::get_settings,
            commands::update_settings,
            commands::get_running_processes,
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
