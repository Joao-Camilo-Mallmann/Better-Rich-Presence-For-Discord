pub mod commands;
pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod models;
pub mod services;

use crate::models::types::{AppRule, AppState, EngineEvent, Settings};
use tauri::Manager;
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
            services::logger::init(app.handle().clone());

            // Disable native window decorations (removes window border/titlebar).
            // Window starts hidden in tauri.conf.json so `--minimized` launches never flash.
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_decorations(false);

                if !domain::startup::should_start_minimized(std::env::args()) {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }

            infrastructure::autostart::sync_autostart_entry();

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

            // Claude Code ranks against other apps through its own list entry.
            let mut modified = models::presets::ensure_claude_code_rule(&mut app_rules);

            // Proactively merge any missing default rules into the user's rules
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
            interface::tray::setup_tray(app, settings.global_enabled)?;

            // 4. Create mpsc channel
            let (tx, rx) = mpsc::channel::<EngineEvent>(100);

            // 5. Create DiscordManager
            let (_discord_manager, discord_handle) =
                services::discord::DiscordManager::new(app.handle().clone());

            // 6. Shared State (Consolidated)
            let claude_code_port = settings.claude_code_port;
            let app_state =
                AppState::new(app.handle().clone(), discord_handle, app_rules, settings);

            app.manage(app_state.clone());
            app.manage(tx.clone());

            // 7. Spawn Watcher Task
            services::watcher::start_window_watcher(tx.clone(), app_state.clone());

            // 8. Spawn Claude Code HTTP Hook Receiver
            services::claude_code_server::start_claude_code_server(
                claude_code_port,
                tx.clone(),
                app_state.clone(),
            );

            // 9. Spawn Engine Task
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
            commands::get_platform_status,
            commands::get_claude_code_status,
            commands::connect_claude_code,
            commands::disconnect_claude_code,
            commands::select_claude_code_session,
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
