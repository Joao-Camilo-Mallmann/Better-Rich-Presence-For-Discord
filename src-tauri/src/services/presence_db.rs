//! presence_db.rs
//!
//! Lookup table mapping process names to their respective Discord Client IDs.
//! This allows the Rich Presence to show the actual Application Name (e.g., "Visual Studio Code" instead of a generic name).

pub const DEFAULT_CLIENT_ID: u64 = 1517170930764480552;

fn process_file_stem(process_name: &str) -> String {
    let file_name = process_name
        .rsplit(['\\', '/'])
        .next()
        .unwrap_or(process_name)
        .trim();

    let stem = if file_name.to_ascii_lowercase().ends_with(".exe") {
        &file_name[..file_name.len() - 4]
    } else {
        file_name
    };

    stem.trim().to_string()
}

fn process_lookup_key(process_name: &str) -> String {
    process_file_stem(process_name).to_lowercase()
}

pub fn display_name_from_process_name(process_name: &str) -> String {
    let stem = process_file_stem(process_name);
    if stem.is_empty() {
        return String::new();
    }

    match stem.to_lowercase().as_str() {
        "code" | "vscode" => "VS Code".to_string(),
        "msedge" => "Edge".to_string(),
        "obs64" | "obs32" | "obs" => "OBS Studio".to_string(),
        "cs2" => "Counter-Strike 2".to_string(),
        "windowsterminal" => "Terminal".to_string(),
        "githubdesktop" => "GitHub Desktop".to_string(),
        other => other
            .split(['-', '_', ' '])
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

/// Get the Discord Client ID for a given process name.
/// Returns `None` if the process is not recognized, in which case a generic App ID can be used.
pub fn get_client_id(process_name: &str) -> Option<&'static str> {
    let name = process_lookup_key(process_name);

    // Mappings of process name -> Discord Application ID
    // Note: These are example community IDs, you might need to update them with actual ones
    match name.as_str() {
        "code" | "vscode" => Some("383226320970055681"), // VS Code
        "firefox" => Some("383227926214344715"),         // Firefox
        "chrome" => Some("383226655516311552"),          // Google Chrome
        "spotify" => Some("361424911850110996"),         // Spotify (Custom/Fallback)
        "discord" | "discordptb" | "discordcanary" => Some("383227320576311306"), // Discord
        "telegram" => Some("383227568589701121"),        // Telegram
        "slack" => Some("383227663456337920"),           // Slack
        "figma" => Some("763435133604085800"),           // Figma
        "obs64" | "obs32" | "obs" => Some("366579227523121153"), // OBS Studio
        "steam" | "steamwebhelper" => Some("366578051662577665"), // Steam
        _ => None,
    }
}

pub fn get_client_id_or_default(process_name: &str) -> u64 {
    get_client_id(process_name)
        .and_then(|id| id.parse::<u64>().ok())
        .unwrap_or(DEFAULT_CLIENT_ID)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_id_lookup_normalizes_case_extension_and_paths() {
        assert_eq!(get_client_id("Code.exe"), Some("383226320970055681"));
        assert_eq!(
            get_client_id("C:\\Program Files\\Google\\Chrome.EXE"),
            Some("383226655516311552")
        );
    }

    #[test]
    fn display_name_formats_process_names_for_fallback_presence() {
        assert_eq!(
            display_name_from_process_name("custom_tool.exe"),
            "Custom Tool"
        );
        assert_eq!(
            display_name_from_process_name("githubdesktop.exe"),
            "GitHub Desktop"
        );
        assert_eq!(
            display_name_from_process_name("C:\\Tools\\obs64.exe"),
            "OBS Studio"
        );
    }
}
