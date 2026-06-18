//! # Presets Module
//!
//! Contains default app rules to seed the store on first run.

use crate::types::{AppRule, PresenceSource};

/// Returns the default list of AppRules.
pub fn default_app_rules() -> Vec<AppRule> {
    vec![
        AppRule::new("code.exe", "VSCode", "Programando no VSCode", "Desenvolvendo", "vscode", PresenceSource::Work),
        AppRule::new("cursor.exe", "Cursor", "Programando no Cursor", "Desenvolvendo", "cursor", PresenceSource::Work),
        AppRule::new("idea64.exe", "IntelliJ IDEA", "Programando no IntelliJ", "Desenvolvendo", "intellij", PresenceSource::Work),
        AppRule::new("studio64.exe", "Android Studio", "Programando no Android Studio", "Desenvolvendo", "androidstudio", PresenceSource::Work),
        AppRule::new("devenv.exe", "Visual Studio", "Programando no Visual Studio", "Desenvolvendo", "visualstudio", PresenceSource::Work),
        AppRule::new("figma.exe", "Figma", "Desenhando no Figma", "Design", "figma", PresenceSource::Work),
        AppRule::new("photoshop.exe", "Photoshop", "Editando no Photoshop", "Design", "photoshop", PresenceSource::Work),
        AppRule::new("adobe premiere pro.exe", "Premiere Pro", "Editando vídeo no Premiere", "Video", "premiere", PresenceSource::Work),
        AppRule::new("afterfx.exe", "After Effects", "Criando animações no AE", "VFX", "aftereffects", PresenceSource::Work),
        AppRule::new("blender.exe", "Blender", "Modelando no Blender", "3D", "blender", PresenceSource::Work),
        AppRule::new("excel.exe", "Excel", "Trabalhando em planilhas", "Office", "excel", PresenceSource::Work),
        AppRule::new("winword.exe", "Word", "Escrevendo documentos", "Office", "word", PresenceSource::Work),
        AppRule::new("powerpnt.exe", "PowerPoint", "Criando apresentações", "Office", "powerpoint", PresenceSource::Work),
        AppRule::new("notion.exe", "Notion", "Organizando tarefas", "Office", "notion", PresenceSource::Work),
        AppRule::new("obsidian.exe", "Obsidian", "Anotando no Obsidian", "Office", "obsidian", PresenceSource::Work),
        AppRule::new("slack.exe", "Slack", "Conversando no Slack", "Comunicação", "slack", PresenceSource::Work),
        AppRule::new("windowsterminal.exe", "Terminal", "Usando o Terminal", "CLI", "terminal", PresenceSource::Work),
        AppRule::new("docker desktop.exe", "Docker Desktop", "Gerenciando containers", "DevOps", "docker", PresenceSource::Work),
        AppRule::new("githubdesktop.exe", "GitHub Desktop", "Fazendo commits", "Git", "github", PresenceSource::Work),
        AppRule::new("spotify.exe", "Spotify", "Ouvindo música", "Entretenimento", "spotify", PresenceSource::Browser),
        AppRule::new("chrome.exe", "Chrome", "Navegando na web", "Web", "chrome", PresenceSource::Browser),
        AppRule::new("firefox.exe", "Firefox", "Navegando na web", "Web", "firefox", PresenceSource::Browser),
        AppRule::new("msedge.exe", "Edge", "Navegando na web", "Web", "edge", PresenceSource::Browser),
    ]
}
