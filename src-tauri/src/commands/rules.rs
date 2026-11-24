use crate::models::types::{AppError, AppRule, AppState, EngineEvent};
use tauri::State;
use tokio::sync::mpsc::Sender;

/// Rule order and switches decide whether Claude Code outranks the current app,
/// so the engine re-evaluates after every change.
async fn notify_engine(tx: &Sender<EngineEvent>) {
    let _ = tx.send(EngineEvent::ClaudeCodeConfigChanged).await;
}

#[tauri::command]
pub async fn get_app_rules(state: State<'_, AppState>) -> Result<Vec<AppRule>, AppError> {
    Ok(state.get_app_rules().await)
}

#[tauri::command]
pub async fn update_app_rule(
    state: State<'_, AppState>,
    tx: State<'_, Sender<EngineEvent>>,
    rule: AppRule,
) -> Result<(), AppError> {
    state.update_app_rule(rule).await?;
    notify_engine(&tx).await;
    Ok(())
}

#[tauri::command]
pub async fn add_app_rule(
    state: State<'_, AppState>,
    tx: State<'_, Sender<EngineEvent>>,
    rule: AppRule,
) -> Result<(), AppError> {
    state.add_app_rule(rule).await?;
    notify_engine(&tx).await;
    Ok(())
}

#[tauri::command]
pub async fn delete_app_rule(
    state: State<'_, AppState>,
    tx: State<'_, Sender<EngineEvent>>,
    process_name: String,
) -> Result<(), AppError> {
    state.delete_app_rule(&process_name).await?;
    notify_engine(&tx).await;
    Ok(())
}

#[tauri::command]
pub async fn reset_app_rules_to_defaults(
    state: State<'_, AppState>,
    tx: State<'_, Sender<EngineEvent>>,
) -> Result<(), AppError> {
    state.reset_app_rules_to_defaults().await?;
    notify_engine(&tx).await;
    Ok(())
}

#[tauri::command]
pub async fn reorder_app_rules(
    state: State<'_, AppState>,
    tx: State<'_, Sender<EngineEvent>>,
    process_names_order: Vec<String>,
) -> Result<(), AppError> {
    state.reorder_app_rules(process_names_order).await?;
    notify_engine(&tx).await;
    Ok(())
}
