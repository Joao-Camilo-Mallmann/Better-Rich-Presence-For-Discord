use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub struct ProcessService;

impl ProcessService {
    /// Get the path to the local icon cache directory (e.g. app_cache_dir/icons).
    /// Creates the directory if it does not exist.
    pub fn get_icon_cache_dir(app_handle: &AppHandle) -> Result<PathBuf, String> {
        let mut cache_dir = app_handle
            .path()
            .app_cache_dir()
            .map_err(|e| format!("Failed to get app cache dir: {}", e))?;
        
        cache_dir.push("icons");
        
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .map_err(|e| format!("Failed to create icon cache directory: {}", e))?;
        }
        
        Ok(cache_dir)
    }

    /// Check if an icon is cached.
    pub fn is_icon_cached(app_handle: &AppHandle, icon_id: &str) -> bool {
        if let Ok(mut path) = Self::get_icon_cache_dir(app_handle) {
            path.push(format!("{}.svg", icon_id));
            path.exists()
        } else {
            false
        }
    }

    /// Write an icon to the cache.
    pub fn write_cached_icon(app_handle: &AppHandle, icon_id: &str, content: &str) -> Result<(), String> {
        let mut path = Self::get_icon_cache_dir(app_handle)?;
        path.push(format!("{}.svg", icon_id));
        
        fs::write(&path, content)
            .map_err(|e| format!("Failed to write cached icon to file: {}", e))?;
            
        Ok(())
    }

    /// Get the absolute path to a cached icon file.
    pub fn get_cached_icon_path(app_handle: &AppHandle, icon_id: &str) -> Option<String> {
        if let Ok(mut path) = Self::get_icon_cache_dir(app_handle) {
            path.push(format!("{}.svg", icon_id));
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
        }
        None
    }
}
