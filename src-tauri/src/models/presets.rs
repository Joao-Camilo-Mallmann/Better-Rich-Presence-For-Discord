//! # Presets Module
//!
//! Contains default app rules to seed the store on first run.

use crate::models::types::{AppRule, PresenceSource};

/// Returns the default list of AppRules.
pub fn default_app_rules() -> Vec<AppRule> {
    vec![
        // --- IDEs & Code Editors ---
        AppRule::new("code.exe", "VSCode", "Editing {file}", "In VS Code", "vscode", PresenceSource::Work),
        AppRule::new("cursor.exe", "Cursor", "Editing {file}", "In Cursor", "cursor", PresenceSource::Work),
        AppRule::new("antigravity ide.exe", "Antigravity IDE", "Editing {file}", "In Antigravity IDE", "antigravity", PresenceSource::Work),
        AppRule::new("antigravity-ide.exe", "Antigravity IDE", "Editing {file}", "In Antigravity IDE", "antigravity", PresenceSource::Work),
        AppRule::new("idea64.exe", "IntelliJ IDEA", "Programming in IntelliJ", "Developing", "intellij", PresenceSource::Work),
        AppRule::new("studio64.exe", "Android Studio", "Programming in Android Studio", "Developing", "androidstudio", PresenceSource::Work),
        AppRule::new("devenv.exe", "Visual Studio", "Programming in Visual Studio", "Developing", "visualstudio", PresenceSource::Work),
        AppRule::new("pycharm64.exe", "PyCharm", "Programming in PyCharm", "Developing", "auto", PresenceSource::Work),
        AppRule::new("webstorm64.exe", "WebStorm", "Programming in WebStorm", "Developing", "auto", PresenceSource::Work),
        AppRule::new("clion64.exe", "CLion", "Programming in CLion", "Developing", "auto", PresenceSource::Work),
        AppRule::new("sublime_text.exe", "Sublime Text", "Editing code", "Sublime Text", "auto", PresenceSource::Work),
        AppRule::new("nvim.exe", "Neovim", "Editing in Neovim", "In Terminal", "auto", PresenceSource::Work),

        // --- Development & DevOps Tools ---
        AppRule::new("windowsterminal.exe", "Terminal", "Using Terminal", "CLI", "terminal", PresenceSource::Work),
        AppRule::new("docker desktop.exe", "Docker Desktop", "Managing containers", "DevOps", "docker", PresenceSource::Work),
        AppRule::new("githubdesktop.exe", "GitHub Desktop", "Committing changes", "Git", "github", PresenceSource::Work),
        AppRule::new("gitkraken.exe", "GitKraken", "Managing repositories", "Git", "auto", PresenceSource::Work),
        AppRule::new("postman.exe", "Postman", "Testing APIs", "Postman", "auto", PresenceSource::Work),
        AppRule::new("insomnia.exe", "Insomnia", "Testing APIs", "Insomnia", "auto", PresenceSource::Work),
        AppRule::new("dbeaver.exe", "DBeaver", "Modeling database", "DBeaver", "auto", PresenceSource::Work),
        AppRule::new("pgadmin4.exe", "pgAdmin", "Managing database", "pgAdmin", "auto", PresenceSource::Work),

        // --- Design & Creative ---
        AppRule::new("figma.exe", "Figma", "Designing in Figma", "Design", "figma", PresenceSource::Work),
        AppRule::new("photoshop.exe", "Photoshop", "Editing in Photoshop", "Design", "photoshop", PresenceSource::Work),
        AppRule::new("illustrator.exe", "Illustrator", "Vectorizing in Illustrator", "Design", "auto", PresenceSource::Work),
        AppRule::new("adobe premiere pro.exe", "Premiere Pro", "Editing video in Premiere", "Video", "premiere", PresenceSource::Work),
        AppRule::new("afterfx.exe", "After Effects", "Animating in After Effects", "VFX", "aftereffects", PresenceSource::Work),
        AppRule::new("blender.exe", "Blender", "Modeling in Blender", "3D", "blender", PresenceSource::Work),
        AppRule::new("canva.exe", "Canva", "Designing in Canva", "Design", "auto", PresenceSource::Work),

        // --- Office & Productivity ---
        AppRule::new("excel.exe", "Excel", "Working on spreadsheets", "Office", "excel", PresenceSource::Work),
        AppRule::new("winword.exe", "Word", "Writing documents", "Office", "word", PresenceSource::Work),
        AppRule::new("powerpnt.exe", "PowerPoint", "Creating presentations", "Office", "powerpoint", PresenceSource::Work),
        AppRule::new("notion.exe", "Notion", "Organizing tasks", "Office", "notion", PresenceSource::Work),
        AppRule::new("obsidian.exe", "Obsidian", "Writing notes in Obsidian", "Office", "obsidian", PresenceSource::Work),

        // --- Communication & Collaboration ---
        AppRule::new("slack.exe", "Slack", "Chatting on Slack", "Communication", "slack", PresenceSource::Work),
        AppRule::new("teams.exe", "Microsoft Teams", "In a meeting / Chatting", "Communication", "auto", PresenceSource::Work),
        AppRule::new("zoom.exe", "Zoom", "In Zoom meeting", "Communication", "auto", PresenceSource::Work),
        AppRule::new("discord.exe", "Discord", "Chatting on Discord", "Communication", "auto", PresenceSource::Work),
        AppRule::new("whatsapp.exe", "WhatsApp", "Chatting on WhatsApp", "Communication", "auto", PresenceSource::Work),
        AppRule::new("telegram.exe", "Telegram", "Chatting on Telegram", "Communication", "auto", PresenceSource::Work),

        // --- Browsers & Web ---
        AppRule::new("chrome.exe", "Chrome", "Browsing the web", "Web", "chrome", PresenceSource::Browser),
        AppRule::new("firefox.exe", "Firefox", "Browsing the web", "Web", "firefox", PresenceSource::Browser),
        AppRule::new("msedge.exe", "Edge", "Browsing the web", "Web", "edge", PresenceSource::Browser),
        AppRule::new("opera.exe", "Opera", "Browsing the web", "Web", "auto", PresenceSource::Browser),
        AppRule::new("brave.exe", "Brave", "Browsing the web", "Web", "auto", PresenceSource::Browser),

        // --- Entertainment & Media ---
        AppRule::new("spotify.exe", "Spotify", "Listening to music", "Entertainment", "spotify", PresenceSource::Browser),
        AppRule::new("vlc.exe", "VLC", "Watching media", "Entertainment", "auto", PresenceSource::Browser),

        // --- Games ---
        AppRule::new("javaw.exe", "Minecraft", "Playing Minecraft", "Survival Mode", "auto", PresenceSource::Game),
        AppRule::new("cs2.exe", "Counter-Strike 2", "Playing CS2", "In a match", "auto", PresenceSource::Game),
        AppRule::new("league of legends.exe", "League of Legends", "Playing League of Legends", "In a match", "auto", PresenceSource::Game),
        AppRule::new("valorant.exe", "Valorant", "Playing Valorant", "In a match", "auto", PresenceSource::Game),
        AppRule::new("steam.exe", "Steam", "Browsing Steam", "Library", "auto", PresenceSource::Game),
        AppRule::new("robloxplayerbeta.exe", "Roblox", "Playing Roblox", "Exploring", "auto", PresenceSource::Game),
    ]
}
