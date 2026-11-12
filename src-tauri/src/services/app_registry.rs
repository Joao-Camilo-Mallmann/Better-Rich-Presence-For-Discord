use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub executables: Vec<String>,
    pub client_id: String,
    pub category: String,
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

    /// Finds an app by catalog id, else by executable. Mirrors the React
    /// `AppIcon` lookup so a rule image like `firefox` resolves the same way.
    pub fn find_by_id_or_executable(&self, key: &str) -> Option<&AppEntry> {
        let key = key.trim();
        self.apps
            .iter()
            .find(|app| app.id == key)
            .or_else(|| self.find_app(key))
    }
}

/// The text before the last ` - ` in a window title (apps usually append their
/// own name there), trimmed. The whole title when there is no separator.
fn title_subject(title: &str) -> &str {
    match title.rfind(" - ") {
        Some(idx) => title[..idx].trim(),
        None => title.trim(),
    }
}

/// Describes what the user is doing in an app, from its catalog category and
/// window title. This is the presence's `state` line; `details` is the app name.
pub fn format_app_presence(category: &str, app_name: &str, window_title: &str) -> String {
    let title = window_title.trim();
    let subject = title_subject(title);
    let has_subject = !subject.is_empty() && !subject.eq_ignore_ascii_case(app_name);

    match category {
        "editor" => {
            let file_name = match title.split(" - ").next() {
                Some(first) if title.contains(" - ") => first.trim().trim_start_matches('●').trim_start_matches('*').trim(),
                _ => title,
            };
            if file_name.is_empty() || file_name.eq_ignore_ascii_case(app_name) {
                "No file open".to_string()
            } else {
                format!("Editing {}", file_name)
            }
        }
        "browser" if has_subject => format!("Browsing {}", subject),
        "browser" => "Browsing the web".to_string(),
        "communication" => "Chatting / In a call".to_string(),
        "productivity" if has_subject => format!("Working on {}", subject),
        "productivity" => "Working".to_string(),
        "design" if has_subject => format!("Designing {}", subject),
        "design" => "Designing".to_string(),
        "developer" if has_subject => format!("Developing {}", subject),
        "developer" => "Developing".to_string(),
        "media" if has_subject => format!("Watching / Listening to {}", subject),
        "media" => "Playing media".to_string(),
        "gaming" => "Playing".to_string(),
        _ if title.is_empty() || title.eq_ignore_ascii_case(app_name) => "Active".to_string(),
        _ => title.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_names_the_page() {
        assert_eq!(format_app_presence("browser", "Firefox", "GitHub - Mozilla Firefox"), "Browsing GitHub");
        assert_eq!(format_app_presence("browser", "Firefox", "Firefox"), "Browsing the web");
        assert_eq!(format_app_presence("browser", "Firefox", ""), "Browsing the web");
    }

    #[test]
    fn productivity_without_useful_title_is_generic() {
        assert_eq!(format_app_presence("productivity", "Notion", "Notion"), "Working");
        assert_eq!(format_app_presence("productivity", "Notion", "Roadmap - Notion"), "Working on Roadmap");
    }

    #[test]
    fn editor_names_the_file() {
        assert_eq!(format_app_presence("editor", "Zed", "● main.rs - project - Zed"), "Editing main.rs");
        assert_eq!(format_app_presence("editor", "Zed", "notes.md"), "Editing notes.md");
        assert_eq!(format_app_presence("editor", "Zed", "Zed"), "No file open");
        assert_eq!(format_app_presence("editor", "Zed", ""), "No file open");
    }

    #[test]
    fn media_names_the_track() {
        assert_eq!(format_app_presence("media", "VLC", "song.mp3 - VLC media player"), "Watching / Listening to song.mp3");
        assert_eq!(format_app_presence("media", "VLC", "VLC"), "Playing media");
    }

    #[test]
    fn generic_uses_the_window_title() {
        assert_eq!(format_app_presence("generic", "my-tool", "Report Q3"), "Report Q3");
        assert_eq!(format_app_presence("generic", "my-tool", ""), "Active");
        assert_eq!(format_app_presence("database", "DBeaver", "DBeaver"), "Active");
    }

    #[test]
    fn find_by_id_or_executable_matches_app_icon_lookup() {
        let registry = AppRegistry::new();
        assert_eq!(registry.find_by_id_or_executable("firefox").map(|a| a.id.as_str()), Some("firefox"));
        assert_eq!(registry.find_by_id_or_executable("vscode").map(|a| a.id.as_str()), Some("vscode"));
        assert_eq!(registry.find_by_id_or_executable("code.exe").map(|a| a.id.as_str()), Some("vscode"));
        assert!(registry.find_by_id_or_executable("rust-logo").is_none());
    }
}
