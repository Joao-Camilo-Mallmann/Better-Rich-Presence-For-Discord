//! # Parser Module
//!
//! Provides utility functions for parsing window titles and resolving favicon URLs.

/// Resolve a process or display name to a favicon URL using a lookup table and guessing.
pub fn resolve_auto_image(process_name: &str, display_name: &str) -> String {
    let name_to_search = if !display_name.is_empty() {
        display_name.to_lowercase()
    } else {
        process_name.to_lowercase()
    };

    let guessed_domain: String;
    let domain = match name_to_search.as_str() {
        // Dev / Editors
        n if n.contains("vscode") || n.contains("visual studio code") => "visualstudio.com",
        n if n.contains("cursor") => "cursor.com",
        n if n.contains("antigravity") => "deepmind.com",
        n if n.contains("intellij") => "jetbrains.com",
        n if n.contains("pycharm") => "jetbrains.com",
        n if n.contains("android studio") => "developer.android.com",
        n if n.contains("visual studio") => "visualstudio.com",
        n if n.contains("sublime") => "sublimetext.com",
        n if n.contains("webstorm") => "jetbrains.com",
        n if n.contains("rider") => "jetbrains.com",
        n if n.contains("clion") => "jetbrains.com",
        n if n.contains("datagrip") => "jetbrains.com",
        n if n.contains("postman") => "postman.com",
        n if n.contains("insomnia") => "insomnia.rest",
        n if n.contains("pgadmin") => "pgadmin.org",
        n if n.contains("dbeaver") => "dbeaver.io",
        
        // Browsers
        n if n.contains("chrome") => "google.com",
        n if n.contains("firefox") => "mozilla.org",
        n if n.contains("edge") => "microsoft.com",
        n if n.contains("opera") => "opera.com",
        n if n.contains("brave") => "brave.com",
        n if n.contains("safari") => "apple.com",

        // Office / Productivity
        n if n.contains("notion") => "notion.so",
        n if n.contains("obsidian") => "obsidian.md",
        n if n.contains("excel") => "microsoft.com",
        n if n.contains("word") => "microsoft.com",
        n if n.contains("powerpoint") => "microsoft.com",
        n if n.contains("teams") => "microsoft.com",
        n if n.contains("trello") => "trello.com",
        n if n.contains("asana") => "asana.com",
        n if n.contains("jira") => "atlassian.com",
        n if n.contains("confluence") => "atlassian.com",
        n if n.contains("slack") => "slack.com",
        n if n.contains("discord") => "discord.com",
        n if n.contains("telegram") => "telegram.org",
        n if n.contains("whatsapp") => "whatsapp.com",
        n if n.contains("zoom") => "zoom.us",
        n if n.contains("skype") => "skype.com",

        // Design / Media
        n if n.contains("figma") => "figma.com",
        n if n.contains("photoshop") => "adobe.com",
        n if n.contains("illustrator") => "adobe.com",
        n if n.contains("premiere") => "adobe.com",
        n if n.contains("after effects") => "adobe.com",
        n if n.contains("canva") => "canva.com",
        n if n.contains("blender") => "blender.org",
        n if n.contains("unity") => "unity.com",
        n if n.contains("unreal") => "unrealengine.com",
        n if n.contains("vlc") => "videolan.org",
        n if n.contains("obs") || n.contains("obs64") => "obsproject.com",

        // Entertainment / Gaming
        n if n.contains("spotify") => "spotify.com",
        n if n.contains("steam") => "steampowered.com",
        n if n.contains("github") => "github.com",
        n if n.contains("docker") => "docker.com",
        n if n.contains("netflix") => "netflix.com",
        n if n.contains("youtube") => "youtube.com",
        n if n.contains("twitch") => "twitch.tv",
        n if n.contains("minecraft") => "minecraft.net",
        n if n.contains("roblox") => "roblox.com",
        n if n.contains("league of legends") => "leagueoflegends.com",
        n if n.contains("valorant") => "playvalorant.com",
        n if n.contains("counter-strike") || n.contains("cs2") || n.contains("csgo") => "counter-strike.net",
        
        // Default guessing
        _ => {
            // Remove ".exe" and sanitize
            let clean_name = name_to_search
                .replace(".exe", "")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
                .collect::<String>();
            
            // Try to extract the first word or join words
            let first_word = clean_name.split_whitespace().next().unwrap_or("").trim().to_string();
            guessed_domain = if !first_word.is_empty() {
                format!("{}.com", first_word)
            } else {
                format!("{}.com", clean_name.replace(' ', ""))
            };
            &guessed_domain
        }
    };

    format!("https://icons.duckduckgo.com/ip3/{}.ico", domain)
}

/// Helper to extract the active file name from an editor window title.
pub fn parse_file_name(window_title: &str, process_name: &str) -> String {
    // If it's Cursor or VS Code, the title is usually: "● filename.ext - folder - Cursor"
    let parts: Vec<&str> = window_title.split(" - ").collect();
    if parts.len() >= 2 {
        let first_part = parts[0].trim();
        // Remove unsaved indicator dot or asterisk if present
        let clean_part = first_part
            .trim_start_matches('●')
            .trim_start_matches('*')
            .trim();
        
        if clean_part.is_empty() {
            "Sem arquivo aberto".to_string()
        } else {
            clean_part.to_string()
        }
    } else {
        // Fallback: clean the window title a bit by removing the process suffix
        let suffix = match process_name.to_lowercase().as_str() {
            "cursor.exe" | "cursor" => " - Cursor",
            "code.exe" | "code" => " - Visual Studio Code",
            "antigravity ide.exe" | "antigravity-ide.exe" | "antigravity-ide" | "antigravity ide" => " - Antigravity IDE",
            _ => "",
        };
        if !suffix.is_empty() && window_title.ends_with(suffix) {
            let clean = window_title[..window_title.len() - suffix.len()].trim();
            if clean.is_empty() {
                "Sem arquivo aberto".to_string()
            } else {
                clean.to_string()
            }
        } else {
            let clean = window_title.trim();
            if clean.is_empty() {
                "Sem arquivo aberto".to_string()
            } else {
                clean.to_string()
            }
        }
    }
}

pub struct CatalogApp {
    pub id: &'static str,
    pub name: &'static str,
    pub executables: &'static [&'static str],
    pub icon: &'static str,
    pub discord_asset: &'static str,
}

pub static APP_CATALOG: &[CatalogApp] = &[
    CatalogApp {
        id: "visual-studio-code",
        name: "Visual Studio Code",
        executables: &["Code.exe", "code"],
        icon: "simple-icons:visualstudiocode",
        discord_asset: "vscode",
    },
    CatalogApp {
        id: "google-chrome",
        name: "Google Chrome",
        executables: &["chrome.exe"],
        icon: "simple-icons:googlechrome",
        discord_asset: "chrome",
    },
    CatalogApp {
        id: "spotify",
        name: "Spotify",
        executables: &["Spotify.exe"],
        icon: "simple-icons:spotify",
        discord_asset: "spotify",
    },
    CatalogApp {
        id: "cursor",
        name: "Cursor",
        executables: &["cursor.exe", "Cursor.exe"],
        icon: "lucide:square-terminal",
        discord_asset: "cursor",
    },
    CatalogApp {
        id: "antigravity-ide",
        name: "Antigravity IDE",
        executables: &["antigravity ide.exe", "antigravity-ide.exe"],
        icon: "lucide:orbit",
        discord_asset: "antigravity",
    },
    CatalogApp {
        id: "zen-browser",
        name: "Zen Browser",
        executables: &["zen.exe"],
        icon: "lucide:compass",
        discord_asset: "zen",
    },
    CatalogApp {
        id: "intellij-idea",
        name: "IntelliJ IDEA",
        executables: &["idea64.exe"],
        icon: "simple-icons:intellijidea",
        discord_asset: "intellij",
    },
    CatalogApp {
        id: "webstorm",
        name: "WebStorm",
        executables: &["webstorm64.exe"],
        icon: "simple-icons:webstorm",
        discord_asset: "webstorm",
    },
    CatalogApp {
        id: "pycharm",
        name: "PyCharm",
        executables: &["pycharm64.exe"],
        icon: "simple-icons:pycharm",
        discord_asset: "pycharm",
    },
    CatalogApp {
        id: "phpstorm",
        name: "PhpStorm",
        executables: &["phpstorm64.exe"],
        icon: "simple-icons:phpstorm",
        discord_asset: "phpstorm",
    },
];

pub fn detect_catalog_app(process_name: &str) -> Option<&'static CatalogApp> {
    if process_name.is_empty() {
        return None;
    }
    
    // Extract file name
    let file_name = process_name.split(['\\', '/']).next_back().unwrap_or(process_name);
    let normalized = file_name.to_lowercase().trim().to_string();

    // 1. Try matching by executables exactly
    for app in APP_CATALOG {
        if app.executables.iter().any(|exe| exe.to_lowercase().trim() == normalized) {
            return Some(app);
        }
    }

    // 2. Try matching by ID or discord_asset exactly
    for app in APP_CATALOG {
        if app.id.to_lowercase() == normalized || app.discord_asset.to_lowercase() == normalized {
            return Some(app);
        }
    }

    // 3. Try fuzzy matching by checking if the app name matches or contains the name
    for app in APP_CATALOG {
        let clean_app_name = app.name.to_lowercase().trim().to_string();
        if normalized.contains(&clean_app_name) || clean_app_name.contains(&normalized) {
            return Some(app);
        }
    }

    // 4. Special manual aliases for editors
    if normalized.contains("vscode") || normalized.contains("visual studio code") || normalized == "code" {
        return APP_CATALOG.iter().find(|app| app.id == "visual-studio-code");
    }
    if normalized.contains("cursor") {
        return APP_CATALOG.iter().find(|app| app.id == "cursor");
    }
    if normalized.contains("antigravity") {
        return APP_CATALOG.iter().find(|app| app.id == "antigravity-ide");
    }
    if normalized.contains("intellij") || normalized.contains("idea") || normalized == "idea64" {
        return APP_CATALOG.iter().find(|app| app.id == "intellij-idea");
    }

    None
}

pub fn resolve_iconify_url(icon: &str) -> String {
    let parts: Vec<&str> = icon.split(':').collect();
    if parts.len() == 2 {
        let collection = parts[0];
        let name = parts[1];
        format!(
            "https://images.weserv.nl/?url=https://api.iconify.design/{}/{}.svg&output=png&w=512&h=512",
            collection, name
        )
    } else {
        "default".to_string()
    }
}

