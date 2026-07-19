use crate::services::presence_db::get_client_id_or_default;
use crate::services::process_service::ProcessService;
use tauri::{command, AppHandle};

#[command]
pub fn get_client_id_for_process(process_name: String) -> u64 {
    get_client_id_or_default(&process_name)
}

#[command]
pub fn is_icon_cached(app_handle: AppHandle, icon_id: String) -> bool {
    ProcessService::is_icon_cached(&app_handle, &icon_id)
}

#[command]
pub fn write_cached_icon(app_handle: AppHandle, icon_id: String, svg_content: String) -> Result<(), String> {
    ProcessService::write_cached_icon(&app_handle, &icon_id, &svg_content)
}

#[command]
pub fn get_cached_icon_path(app_handle: AppHandle, icon_id: String) -> Option<String> {
    ProcessService::get_cached_icon_path(&app_handle, &icon_id)
}
