use crate::models::types::{AppError, AppState, ConnectionInfo, PresenceData, PresenceSource, PresenceState};
use tauri::State;

#[tauri::command]
pub async fn get_current_presence(state: State<'_, AppState>) -> Result<Option<PresenceData>, AppError> {
    Ok(state.get_current_presence().await)
}

#[tauri::command]
pub async fn get_presence_state(state: State<'_, AppState>) -> Result<PresenceState, AppError> {
    Ok(state.get_presence_state().await)
}

#[tauri::command]
pub async fn get_current_source(state: State<'_, AppState>) -> Result<PresenceSource, AppError> {
    Ok(state.get_current_source().await)
}

#[tauri::command]
pub async fn get_connection_status(state: State<'_, AppState>) -> Result<ConnectionInfo, AppError> {
    Ok(state.get_connection_status().await)
}
