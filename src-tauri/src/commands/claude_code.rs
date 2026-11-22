//! Tauri commands for managing the Claude Code HTTP hook connection.

use crate::models::types::{AppError, AppState, ClaudeCodeStatus, EngineEvent};
use crate::services::claude_code_config;
use tauri::{Emitter, State};
use tokio::sync::mpsc::Sender;

/// Returns the current Claude Code hook installation, listener, and active session status.
#[tauri::command]
pub async fn get_claude_code_status(
    state: State<'_, AppState>,
) -> Result<ClaudeCodeStatus, AppError> {
    match state.refresh_claude_install_status().await {
        Ok(status) => Ok(status),
        Err(err) => {
            let mut status = state.get_claude_code_status().await;
            if status.server_error.is_none() {
                status.server_error = Some(err.to_string());
            }
            Ok(status)
        }
    }
}

/// Non-destructively installs or updates Better RPC's HTTP hooks in `~/.claude/settings.json`.
#[tauri::command]
pub async fn connect_claude_code(
    state: State<'_, AppState>,
) -> Result<ClaudeCodeStatus, AppError> {
    let port = state.get_settings().await.claude_code_port;
    let path = claude_code_config::resolve_settings_path().map_err(AppError::System)?;
    claude_code_config::install_hooks_at(&path, port).map_err(AppError::System)?;

    let status = state.refresh_claude_install_status().await?;
    let _ = state.app_handle.emit("claude-code-status-changed", &status);
    Ok(status)
}

/// Removes only Better RPC's HTTP hooks from `~/.claude/settings.json` and clears active sessions.
#[tauri::command]
pub async fn disconnect_claude_code(
    state: State<'_, AppState>,
    tx: State<'_, Sender<EngineEvent>>,
) -> Result<ClaudeCodeStatus, AppError> {
    let path = claude_code_config::resolve_settings_path().map_err(AppError::System)?;
    claude_code_config::remove_hooks_at(&path).map_err(AppError::System)?;

    let _ = tx.send(EngineEvent::ClaudeCodeDisconnected).await;
    state.set_claude_session_snapshot(0, None, None).await;

    let status = state.refresh_claude_install_status().await?;
    let _ = state.app_handle.emit("claude-code-status-changed", &status);
    Ok(status)
}
