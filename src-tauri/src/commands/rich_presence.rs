use crate::models::types::{EngineEvent, PresenceData};
use tauri::{command, State};
use tokio::sync::mpsc::Sender;

#[command]
pub async fn submit_resolved_presence(
    tx: State<'_, Sender<EngineEvent>>,
    client_id: u64,
    data: PresenceData,
) -> Result<(), String> {
    tx.send(EngineEvent::ResolvedPresence { client_id, data })
        .await
        .map_err(|e| format!("Failed to send resolved presence to engine: {}", e))?;
    Ok(())
}

#[command]
pub async fn clear_rich_presence(
    tx: State<'_, Sender<EngineEvent>>,
) -> Result<(), String> {
    tx.send(EngineEvent::ClearPresence)
        .await
        .map_err(|e| format!("Failed to send clear command to engine: {}", e))?;
    Ok(())
}
