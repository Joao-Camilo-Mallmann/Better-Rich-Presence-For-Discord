//! # Commands Module
//!
//! Exposes Tauri IPC commands to the frontend and manages shared state.

use crate::types::{AppRule, ConnectionInfo, PresenceData, PresenceSource, PresenceState, Settings};
use std::sync::Arc;
use tauri::{State, AppHandle};
use tokio::sync::RwLock;
use tauri_plugin_store::StoreExt;

pub struct AppState {
    pub app_rules: Arc<RwLock<Vec<AppRule>>>,
    pub current_presence: Arc<RwLock<Option<PresenceData>>>,
    pub presence_state: Arc<RwLock<PresenceState>>,
    pub current_source: Arc<RwLock<PresenceSource>>,
    pub connection_info: Arc<RwLock<ConnectionInfo>>,
    pub settings: Arc<RwLock<Settings>>,
}

// ---------------------------------------------------------------------------
// State Queries
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn get_current_presence(state: State<'_, AppState>) -> Result<Option<PresenceData>, String> {
    Ok(state.current_presence.read().await.clone())
}

#[tauri::command]
pub async fn get_presence_state(state: State<'_, AppState>) -> Result<PresenceState, String> {
    Ok(state.presence_state.read().await.clone())
}

#[tauri::command]
pub async fn get_current_source(state: State<'_, AppState>) -> Result<PresenceSource, String> {
    Ok(state.current_source.read().await.clone())
}

#[tauri::command]
pub async fn get_connection_status(state: State<'_, AppState>) -> Result<ConnectionInfo, String> {
    Ok(state.connection_info.read().await.clone())
}

// ---------------------------------------------------------------------------
// App Rules Management
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn get_app_rules(state: State<'_, AppState>) -> Result<Vec<AppRule>, String> {
    Ok(state.app_rules.read().await.clone())
}

#[tauri::command]
pub async fn update_app_rule(
    app: AppHandle,
    state: State<'_, AppState>,
    rule: AppRule,
) -> Result<(), String> {
    let mut rules = state.app_rules.write().await;
    if let Some(pos) = rules.iter().position(|r| r.process_name == rule.process_name) {
        rules[pos] = rule;
        save_rules(&app, &rules)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn add_app_rule(
    app: AppHandle,
    state: State<'_, AppState>,
    rule: AppRule,
) -> Result<(), String> {
    let mut rules = state.app_rules.write().await;
    if !rules.iter().any(|r| r.process_name == rule.process_name) {
        rules.push(rule);
        save_rules(&app, &rules)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_app_rule(
    app: AppHandle,
    state: State<'_, AppState>,
    process_name: String,
) -> Result<(), String> {
    let mut rules = state.app_rules.write().await;
    rules.retain(|r| r.process_name != process_name);
    save_rules(&app, &rules)?;
    Ok(())
}

#[tauri::command]
pub async fn reset_app_rules_to_defaults(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut rules = state.app_rules.write().await;
    *rules = crate::presets::default_app_rules();
    save_rules(&app, &rules)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Settings Management
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    Ok(state.settings.read().await.clone())
}

#[tauri::command]
pub async fn update_settings(
    app: AppHandle,
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<(), String> {
    let mut current = state.settings.write().await;
    *current = settings.clone();
    
    // Save to store
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    store.set("config", serde_json::to_value(&settings).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn save_rules(app: &AppHandle, rules: &[AppRule]) -> Result<(), String> {
    let store = app.store("rules.json").map_err(|e| e.to_string())?;
    store.set("app_rules", serde_json::to_value(rules).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

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
pub async fn get_running_processes() -> Result<Vec<SystemProcessInfo>, String> {
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
    
    processes.sort_by(|a, b| a.process_name.cmp(&b.process_name));
    processes.dedup_by(|a, b| a.process_name == b.process_name);
    
    Ok(processes)
}
