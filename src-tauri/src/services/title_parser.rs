use super::ide_registry::IdeRegistry;
use crate::models::IdeContext;
use std::path::Path;

const REMOTE_SUFFIX_PREFIXES: [&str; 4] = [
    "[WSL:",
    "[SSH:",
    "[Dev Container:",
    "[Codespaces:",
];

fn clean_file_name(raw: &str) -> Option<String> {
    let mut s = raw.trim();
    loop {
        if let Some(rest) = s.strip_prefix('●').or_else(|| s.strip_prefix('•')) {
            s = rest.trim_start();
            continue;
        }
        break;
    }
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

fn strip_workspace_suffixes(raw: &str) -> &str {
    let mut s = raw.trim();
    loop {
        if let Some(stripped) = s.strip_suffix("(Workspace)") {
            s = if stripped.ends_with(" - ") {
                stripped
            } else {
                stripped.trim_end()
            };
            continue;
        }
        if s.ends_with(']') {
            if let Some(open_idx) = s.rfind('[') {
                let bracketed = &s[open_idx..];
                if REMOTE_SUFFIX_PREFIXES
                    .iter()
                    .any(|prefix| bracketed.starts_with(prefix))
                {
                    let before = &s[..open_idx];
                    s = if before.ends_with(" - ") {
                        before
                    } else {
                        before.trim_end()
                    };
                    continue;
                }
            }
        }
        break;
    }
    s
}

pub struct TitleParser;

impl TitleParser {
    pub fn parse(title: &str) -> Option<IdeContext> {
        // VS Code appends ` - Modified` while the active file has unsaved changes.
        let title = title.trim();
        let title = title.strip_suffix(" - Modified").unwrap_or(title);
        let (prefix, ide) = IdeRegistry::strip_ide_suffix(title)?;
        if prefix.trim().is_empty() {
            return None;
        }

        let cleaned_prefix = strip_workspace_suffixes(prefix);
        let (file_name, workspace_name) = match cleaned_prefix.rsplit_once(" - ") {
            Some((raw_file, raw_workspace)) => {
                let file = clean_file_name(raw_file);
                let ws = raw_workspace.trim();
                let workspace = if ws.is_empty() { None } else { Some(ws.to_string()) };
                (file, workspace)
            }
            None => {
                let ws = cleaned_prefix.trim();
                let workspace = if ws.is_empty() { None } else { Some(ws.to_string()) };
                (None, workspace)
            }
        };

        let file_extension = file_name.as_deref().and_then(|name| {
            Path::new(name)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_string())
        });

        Some(IdeContext {
            ide,
            file_name,
            workspace_name,
            file_extension,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Ide;

    #[test]
    fn modified_suffix_is_ignored() {
        let ctx = TitleParser::parse(
            "AppIcon.tsx - Better-Rich-Presence-For-Discord - Visual Studio Code - Modified",
        )
        .unwrap();
        assert_eq!(ctx.ide, Ide::VisualStudioCode);
        assert_eq!(ctx.file_name.as_deref(), Some("AppIcon.tsx"));
        assert_eq!(
            ctx.workspace_name.as_deref(),
            Some("Better-Rich-Presence-For-Discord")
        );
    }

    #[test]
    fn test_valid_vscode_window() {
        let title = "main.rs - my-project - Visual Studio Code";
        let ctx = TitleParser::parse(title).unwrap();
        assert_eq!(ctx.ide, Ide::VisualStudioCode);
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-project"));
        assert_eq!(ctx.file_name.as_deref(), Some("main.rs"));
        assert_eq!(ctx.file_extension.as_deref(), Some("rs"));
    }

    #[test]
    fn test_valid_forked_ide_window() {
        let title = "app.ts - ui-workspace - Cursor";
        let ctx = TitleParser::parse(title).unwrap();
        assert_eq!(ctx.ide, Ide::Cursor);
        assert_eq!(ctx.workspace_name.as_deref(), Some("ui-workspace"));
        assert_eq!(ctx.file_name.as_deref(), Some("app.ts"));
        assert_eq!(ctx.file_extension.as_deref(), Some("ts"));
    }

    #[test]
    fn test_complex_filename() {
        let title = "my-file - test.rs - workspace - VSCodium";
        let ctx = TitleParser::parse(title).unwrap();
        assert_eq!(ctx.ide, Ide::VSCodium);
        assert_eq!(ctx.workspace_name.as_deref(), Some("workspace"));
        assert_eq!(ctx.file_name.as_deref(), Some("my-file - test.rs"));
        assert_eq!(ctx.file_extension.as_deref(), Some("rs"));
    }

    #[test]
    fn test_no_extension() {
        let title = "Makefile - project - Windsurf";
        let ctx = TitleParser::parse(title).unwrap();
        assert_eq!(ctx.ide, Ide::Windsurf);
        assert_eq!(ctx.file_name.as_deref(), Some("Makefile"));
        assert_eq!(ctx.file_extension, None);
    }

    #[test]
    fn test_dirty_indicator_and_remote_suffix_cleanup() {
        let ctx = TitleParser::parse("● main.rs - my-app [WSL: Ubuntu] - Visual Studio Code")
            .unwrap();
        assert_eq!(ctx.ide, Ide::VisualStudioCode);
        assert_eq!(ctx.file_name.as_deref(), Some("main.rs"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-app"));
        assert_eq!(ctx.file_extension.as_deref(), Some("rs"));

        let ctx = TitleParser::parse("• app.ts - api [SSH: prod-box] - Cursor").unwrap();
        assert_eq!(ctx.file_name.as_deref(), Some("app.ts"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("api"));

        let ctx = TitleParser::parse(
            "lib.rs - crate [Dev Container: Rust - 1.80] - Visual Studio Code",
        )
        .unwrap();
        assert_eq!(ctx.file_name.as_deref(), Some("lib.rs"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("crate"));

        let ctx = TitleParser::parse(
            "index.ts - web [Codespaces: octo-space] - Visual Studio Code",
        )
        .unwrap();
        assert_eq!(ctx.file_name.as_deref(), Some("index.ts"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("web"));

        let ctx = TitleParser::parse(
            "main.py - ml (Workspace) [WSL: Debian] - Visual Studio Code",
        )
        .unwrap();
        assert_eq!(ctx.file_name.as_deref(), Some("main.py"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("ml"));
    }

    #[test]
    fn test_two_part_workspace_only_title() {
        let ctx = TitleParser::parse("my-app - Cursor").unwrap();
        assert_eq!(ctx.ide, Ide::Cursor);
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-app"));
        assert_eq!(ctx.file_name, None);
        assert_eq!(ctx.file_extension, None);

        let ctx = TitleParser::parse("my-app [WSL: Ubuntu] - Visual Studio Code").unwrap();
        assert_eq!(ctx.ide, Ide::VisualStudioCode);
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-app"));
        assert_eq!(ctx.file_name, None);
    }

    #[test]
    fn test_vscode_insiders() {
        let ctx =
            TitleParser::parse("main.rs - my-project - Visual Studio Code - Insiders").unwrap();
        assert_eq!(ctx.ide, Ide::VisualStudioCode);
        assert_eq!(ctx.file_name.as_deref(), Some("main.rs"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-project"));

        let ctx = TitleParser::parse("my-project - Visual Studio Code - Insiders").unwrap();
        assert_eq!(ctx.ide, Ide::VisualStudioCode);
        assert_eq!(ctx.file_name, None);
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-project"));

        assert!(TitleParser::parse("Visual Studio Code - Insiders").is_none());
    }

    #[test]
    fn test_non_ide_and_bare_titles_return_none() {
        assert!(TitleParser::parse("GitHub - Mozilla Firefox").is_none());
        assert!(TitleParser::parse("Visual Studio Code").is_none());
        assert!(TitleParser::parse(" - Visual Studio Code").is_none());
    }
}
