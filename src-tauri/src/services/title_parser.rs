use crate::models::IdeContext;
use super::ide_registry::IdeRegistry;
use std::path::Path;

pub struct TitleParser;

impl TitleParser {
    pub fn parse(title: &str) -> Option<IdeContext> {
        let parts: Vec<&str> = title.split(" - ").collect();
        let len = parts.len();

        if len < 3 {
            // Might be just the IDE name or something else, but we need at least 3 parts
            // for File - Workspace - IDE
            // However, what if there's no workspace? Just File - IDE? Let's check the spec.
            // Spec focuses on `<filename> - <workspace> - <ide>`.
            return None;
        }

        let ide_name_candidate = parts[len - 1];
        let ide = IdeRegistry::detect_from_title(ide_name_candidate)?;

        let workspace_name = parts[len - 2].to_string();
        let file_name = parts[0..len - 2].join(" - ");

        // Extract file extension
        let file_extension = Path::new(&file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string());

        Some(IdeContext {
            ide,
            file_name: Some(file_name),
            workspace_name: Some(workspace_name),
            file_extension,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Ide;

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
}
