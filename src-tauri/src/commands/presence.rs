use crate::models::types::{
    AppError, AppState, ConnectionInfo, PresenceData, PresenceState,
};
use crate::services::presence_manager::PresenceManager;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn get_current_presence(
    state: State<'_, AppState>,
) -> Result<Option<PresenceData>, AppError> {
    Ok(state.get_current_presence().await)
}

#[tauri::command]
pub async fn get_presence_state(state: State<'_, AppState>) -> Result<PresenceState, AppError> {
    Ok(state.get_presence_state().await)
}


#[tauri::command]
pub async fn get_connection_status(state: State<'_, AppState>) -> Result<ConnectionInfo, AppError> {
    Ok(state.get_connection_status().await)
}

#[tauri::command]
pub async fn update_presence(
    process: String,
    title: String,
    presence_manager: State<'_, Mutex<PresenceManager>>,
) -> Result<(), ()> {
    let mut manager = presence_manager.lock().await;
    manager.switch_to(&process, &title).await;
    Ok(())
}
