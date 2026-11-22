use crate::models::types::{AppError, AppState, EngineEvent, Settings};
use tauri::State;
use tokio::sync::mpsc::Sender;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, AppError> {
    Ok(state.get_settings().await)
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    tx: State<'_, Sender<EngineEvent>>,
    settings: Settings,
) -> Result<(), AppError> {
    state.update_settings(settings).await?;
    let _ = tx.send(EngineEvent::ClaudeCodeConfigChanged).await;
    Ok(())
}
