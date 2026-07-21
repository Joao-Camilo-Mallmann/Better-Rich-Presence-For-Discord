use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub executables: Vec<String>,
    pub client_id: String,
    pub category: String,
    pub icon: Option<String>,
    pub icon_url: Option<String>,
}

pub struct AppRegistry {
    apps: Vec<AppEntry>,
}

impl AppRegistry {
    pub fn new() -> Self {
        let json_data = include_str!("../../../apps.json");
        let apps: Vec<AppEntry> = serde_json::from_str(json_data).expect("Failed to parse apps.json");
        Self { apps }
    }

    pub fn find_app(&self, process_name: &str) -> Option<&AppEntry> {
        let proc = process_name
            .strip_suffix(".exe")
            .unwrap_or(process_name)
            .to_lowercase();
        
        self.apps.iter().find(|app| {
            app.executables.iter().any(|exe| {
                let e = exe.strip_suffix(".exe").unwrap_or(exe).to_lowercase();
                e == proc
            })
        })
    }
}

pub fn format_app_presence(category: &str, app_name: &str, window_title: &str) -> (String, String) {
    let title = window_title.trim();
    
    match category {
        "editor" => {
            let parts: Vec<&str> = title.split(" - ").collect();
            let file_name = if parts.len() >= 2 {
                parts[0].trim().trim_start_matches('●').trim_start_matches('*').trim()
            } else {
                title
            };
            
            let file_name = if file_name.is_empty() { "No file opened" } else { file_name };
            (format!("Editing {}", file_name), format!("In {}", app_name))
        }
        "browser" => {
            let site = if let Some(idx) = title.rfind(" - ") {
                &title[..idx]
            } else {
                title
            };
            let site = site.trim();
            if site.is_empty() || site.eq_ignore_ascii_case(app_name) {
                (format!("Browsing the web"), app_name.to_string())
            } else {
                (format!("Browsing {}", site), app_name.to_string())
            }
        }
        "communication" => {
            (format!("Chatting / In a call"), app_name.to_string())
        }
        "productivity" => {
            let doc_name = if let Some(idx) = title.rfind(" - ") {
                &title[..idx]
            } else {
                title
            };
            let doc_name = doc_name.trim();
            if doc_name.is_empty() || doc_name.eq_ignore_ascii_case(app_name) {
                (format!("Working"), app_name.to_string())
            } else {
                (format!("Working on {}", doc_name), app_name.to_string())
            }
        }
        "design" => {
            let project = if let Some(idx) = title.rfind(" - ") {
                &title[..idx]
            } else {
                title
            };
            let project = project.trim();
            if project.is_empty() || project.eq_ignore_ascii_case(app_name) {
                (format!("Designing"), app_name.to_string())
            } else {
                (format!("Designing {}", project), app_name.to_string())
            }
        }
        "developer" => {
            let project = if let Some(idx) = title.rfind(" - ") {
                &title[..idx]
            } else {
                title
            };
            let project = project.trim();
            if project.is_empty() || project.eq_ignore_ascii_case(app_name) {
                (format!("Developing"), app_name.to_string())
            } else {
                (format!("Developing {}", project), app_name.to_string())
            }
        }
        "media" => {
            let track = if let Some(idx) = title.rfind(" - ") {
                &title[..idx]
            } else {
                title
            };
            let track = track.trim();
            if track.is_empty() || track.eq_ignore_ascii_case(app_name) {
                (format!("Playing Media"), app_name.to_string())
            } else {
                (format!("Watching / Listening to {}", track), app_name.to_string())
            }
        }
        "gaming" => {
            (format!("Playing"), app_name.to_string())
        }
        _ => {
            let state = if title.is_empty() || title.eq_ignore_ascii_case(app_name) {
                "Active".to_string()
            } else {
                title.to_string()
            };
            (format!("Using {}", app_name), state)
        }
    }
}
