//! # Presets Module
//!
//! Contains default app rules to seed the store on first run.

use crate::models::types::AppRule;

/// Returns the default list of AppRules.
pub fn default_app_rules() -> Vec<AppRule> {
    vec![
        // --- IDEs & Code Editors ---
        AppRule::new("code.exe", "VSCode", "Editing {file}", "In VS Code", "vscode"),
        AppRule::new("cursor.exe", "Cursor", "Editing {file}", "In Cursor", "cursor"),
        AppRule::new("antigravity ide.exe", "Antigravity IDE", "Editing {file}", "In Antigravity IDE", "antigravity"),
        AppRule::new("antigravity-ide.exe", "Antigravity IDE", "Editing {file}", "In Antigravity IDE", "antigravity"),
        AppRule::new("idea64.exe", "IntelliJ IDEA", "Programming in IntelliJ", "Developing", "intellij"),
        AppRule::new("studio64.exe", "Android Studio", "Programming in Android Studio", "Developing", "androidstudio"),
        AppRule::new("devenv.exe", "Visual Studio", "Programming in Visual Studio", "Developing", "visualstudio"),
        AppRule::new("pycharm64.exe", "PyCharm", "Programming in PyCharm", "Developing", "auto"),
        AppRule::new("webstorm64.exe", "WebStorm", "Programming in WebStorm", "Developing", "auto"),
        AppRule::new("clion64.exe", "CLion", "Programming in CLion", "Developing", "auto"),
        AppRule::new("sublime_text.exe", "Sublime Text", "Editing code", "Sublime Text", "auto"),
        AppRule::new("nvim.exe", "Neovim", "Editing in Neovim", "In Terminal", "auto"),

        // --- Development & DevOps Tools ---
        AppRule::new("windowsterminal.exe", "Terminal", "Using Terminal", "CLI", "terminal"),
        AppRule::new("docker desktop.exe", "Docker Desktop", "Managing containers", "DevOps", "docker"),
        AppRule::new("githubdesktop.exe", "GitHub Desktop", "Committing changes", "Git", "github"),
        AppRule::new("gitkraken.exe", "GitKraken", "Managing repositories", "Git", "auto"),
        AppRule::new("postman.exe", "Postman", "Testing APIs", "Postman", "auto"),
        AppRule::new("insomnia.exe", "Insomnia", "Testing APIs", "Insomnia", "auto"),
        AppRule::new("dbeaver.exe", "DBeaver", "Modeling database", "DBeaver", "auto"),
        AppRule::new("pgadmin4.exe", "pgAdmin", "Managing database", "pgAdmin", "auto"),

        // --- Design & Creative ---
        AppRule::new("figma.exe", "Figma", "Designing in Figma", "Design", "figma"),
        AppRule::new("photoshop.exe", "Photoshop", "Editing in Photoshop", "Design", "photoshop"),
        AppRule::new("illustrator.exe", "Illustrator", "Vectorizing in Illustrator", "Design", "auto"),
        AppRule::new("adobe premiere pro.exe", "Premiere Pro", "Editing video in Premiere", "Video", "premiere"),
        AppRule::new("afterfx.exe", "After Effects", "Animating in After Effects", "VFX", "aftereffects"),
        AppRule::new("blender.exe", "Blender", "Modeling in Blender", "3D", "blender"),
        AppRule::new("canva.exe", "Canva", "Designing in Canva", "Design", "auto"),

        // --- Office & Productivity ---
        AppRule::new("excel.exe", "Excel", "Working on spreadsheets", "Office", "excel"),
        AppRule::new("winword.exe", "Word", "Writing documents", "Office", "word"),
        AppRule::new("powerpnt.exe", "PowerPoint", "Creating presentations", "Office", "powerpoint"),
        AppRule::new("notion.exe", "Notion", "Organizing tasks", "Office", "notion"),
        AppRule::new("obsidian.exe", "Obsidian", "Writing notes in Obsidian", "Office", "obsidian"),

        // --- Communication & Collaboration ---
        AppRule::new("slack.exe", "Slack", "Chatting on Slack", "Communication", "slack"),
        AppRule::new("teams.exe", "Microsoft Teams", "In a meeting / Chatting", "Communication", "auto"),
        AppRule::new("zoom.exe", "Zoom", "In Zoom meeting", "Communication", "auto"),
        AppRule::new("discord.exe", "Discord", "Chatting on Discord", "Communication", "auto"),
        AppRule::new("whatsapp.exe", "WhatsApp", "Chatting on WhatsApp", "Communication", "auto"),
        AppRule::new("telegram.exe", "Telegram", "Chatting on Telegram", "Communication", "auto"),

        // --- Entertainment & Media ---
        AppRule::new("spotify.exe", "Spotify", "Listening to music", "Entertainment", "spotify"),

    ]
}
