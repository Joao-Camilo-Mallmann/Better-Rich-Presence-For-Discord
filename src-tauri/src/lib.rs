pub mod commands;
pub mod models;
pub mod services;

use crate::models::types::{
    AppRule, AppState, EngineEvent, Settings,
};
use log::{Level, Log, Metadata, Record};
use std::sync::OnceLock;
use tauri::Emitter;

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

struct AppLogger;

impl Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_str = record.level().to_string();
            let msg = format!("{}", record.args());

            // Print to standard output
            println!("[{}] {}", level_str, msg);

            // Emit to frontend if AppHandle is set
            if let Some(app) = APP_HANDLE.get() {
                let _ = app.emit(
                    "app-log",
                    LogPayload {
                        level: level_str,
                        message: msg,
                        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                    },
                );
            }
        }
    }

    fn flush(&self) {}
}

#[derive(Clone, serde::Serialize)]
struct LogPayload {
    level: String,
    message: String,
    timestamp: String,
}

static LOGGER: AppLogger = AppLogger;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_store::StoreExt;
use tokio::sync::mpsc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .setup(|app| {
            let _ = APP_HANDLE.set(app.handle().clone());
            let _ = log::set_logger(&LOGGER);
            log::set_max_level(log::LevelFilter::Trace);

            // Disable native window decorations (removes window border/titlebar)
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_decorations(false);
            }

            // 1. Load settings from store
            let settings_store = app
                .store("settings.json")
                .expect("Failed to create settings store");
            let settings = if let Some(config) = settings_store.get("config") {
                serde_json::from_value::<Settings>(config).unwrap_or_default()
            } else {
                Settings::default()
            };

            // 2. Load app_rules from store
            let rules_store = app
                .store("rules.json")
                .expect("Failed to create rules store");
            let mut app_rules = if let Some(rules_val) = rules_store.get("app_rules") {
                serde_json::from_value::<Vec<AppRule>>(rules_val)
                    .unwrap_or_else(|_| models::presets::default_app_rules())
            } else {
                models::presets::default_app_rules()
            };

            // Proactively merge any missing default rules into the user's rules
            let mut modified = false;
            for default_rule in models::presets::default_app_rules() {
                if !app_rules.iter().any(|r| {
                    r.process_name.to_lowercase() == default_rule.process_name.to_lowercase()
                }) {
                    app_rules.push(default_rule);
                    modified = true;
                }
            }

            if modified || rules_store.get("app_rules").is_none() {
                let _ = rules_store.set("app_rules", serde_json::to_value(&app_rules).unwrap());
                let _ = rules_store.save();
            }

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
            let (_discord_manager, discord_handle) = services::discord::DiscordManager::new(app.handle().clone());

            // 6. Shared State (Consolidated)
            let app_state =
                AppState::new(app.handle().clone(), discord_handle, app_rules, settings);

            app.manage(app_state.clone());
            app.manage(tx.clone());

            // 7. Spawn Watcher Task
            services::watcher::start_window_watcher(tx.clone(), app_state.clone());

            // 8. Spawn Engine Task
            services::engine::start_engine(rx, app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_current_presence,
            commands::get_presence_state,
            commands::get_discord_user,
            commands::get_connection_status,
            commands::get_app_rules,
            commands::update_app_rule,
            commands::add_app_rule,
            commands::delete_app_rule,
            commands::reset_app_rules_to_defaults,
            commands::reorder_app_rules,
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
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::Exit => {
            if let Some(state) = app_handle.try_state::<AppState>() {
                log::info!("Application exiting, cleanly disconnecting Discord RPC...");
                state
                    .discord_handle
                    .send(crate::models::types::EngineCommand::Disconnect);
            }
        }
        _ => {}
    });
}
