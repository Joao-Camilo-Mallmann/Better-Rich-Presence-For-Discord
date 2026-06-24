use crate::models::types::{AppError, AppRule, AppState};
use tauri::State;

#[tauri::command]
pub async fn get_app_rules(state: State<'_, AppState>) -> Result<Vec<AppRule>, AppError> {
    Ok(state.get_app_rules().await)
}

#[tauri::command]
pub async fn update_app_rule(
    state: State<'_, AppState>,
    rule: AppRule,
) -> Result<(), AppError> {
    state.update_app_rule(rule).await
}

#[tauri::command]
pub async fn add_app_rule(
    state: State<'_, AppState>,
    rule: AppRule,
) -> Result<(), AppError> {
    state.add_app_rule(rule).await
}

#[tauri::command]
pub async fn delete_app_rule(
    state: State<'_, AppState>,
    process_name: String,
) -> Result<(), AppError> {
    state.delete_app_rule(&process_name).await
}

#[tauri::command]
pub async fn reset_app_rules_to_defaults(
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.reset_app_rules_to_defaults().await
}

#[tauri::command]
pub async fn reorder_app_rules(
    state: State<'_, AppState>,
    process_names_order: Vec<String>,
) -> Result<(), AppError> {
    state.reorder_app_rules(process_names_order).await
}
