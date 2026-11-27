use super::ide_registry::IdeRegistry;
use crate::domain::language::{detect_language, language_icon_url};
use crate::domain::presence::{ide_details, ide_state};
use crate::models::types::PresenceData;
use crate::models::IdeContext;

pub struct ActivityBuilder;

impl ActivityBuilder {
    /// Presence for a parsed IDE window: `details` pairs the IDE with the
    /// workspace, `state` describes the open file and detected language,
    /// `large_image` is the IDE's catalog id, and `small_image` is the detected
    /// language icon URL (empty when no language matches).
    pub fn build(ide_ctx: &IdeContext, timestamp: i64) -> PresenceData {
        let ide_name = IdeRegistry::display_name(&ide_ctx.ide);
        let language = ide_ctx.file_name.as_deref().and_then(detect_language);

        let (small_image, small_text) = match language {
            Some(lang) => (language_icon_url(lang.icon), lang.name.to_string()),
            None => (String::new(), String::new()),
        };

        PresenceData {
            details: ide_details(ide_name, ide_ctx.workspace_name.as_deref()),
            state: ide_state(
                ide_ctx.file_name.as_deref(),
                ide_ctx.workspace_name.as_deref(),
                language.map(|lang| lang.name),
            ),
            large_image: IdeRegistry::catalog_id(&ide_ctx.ide)
                .unwrap_or_default()
                .to_string(),
            large_text: ide_name.to_string(),
            small_image,
            small_text,
            timestamp,
            ..PresenceData::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Ide;

    fn context(ide: Ide, file: Option<&str>, workspace: Option<&str>) -> IdeContext {
        IdeContext {
            ide,
            file_name: file.map(ToString::to_string),
            workspace_name: workspace.map(ToString::to_string),
            file_extension: file.and_then(|f| f.rsplit_once('.').map(|(_, ext)| ext.to_string())),
        }
    }

    #[test]
    fn test_activity_builder_with_known_language() {
        let ide_ctx = context(Ide::VisualStudioCode, Some("main.rs"), Some("my-project"));
        let data = ActivityBuilder::build(&ide_ctx, 123456789);

        assert_eq!(data.details, "Visual Studio Code · my-project");
        assert_eq!(data.state, "Editing main.rs (Rust)");
        assert_eq!(data.large_image, "vscode");
        assert_eq!(data.large_text, "Visual Studio Code");
        assert_eq!(data.small_image, language_icon_url("rust"));
        assert_eq!(data.small_text, "Rust");
        assert_eq!(data.timestamp, 123456789);
    }

    #[test]
    fn test_activity_builder_with_unknown_language() {
        let data = ActivityBuilder::build(
            &context(Ide::Cursor, Some("test.unknown"), Some("workspace")),
            0,
        );
        assert_eq!(data.details, "Cursor · workspace");
        assert_eq!(data.state, "Editing test.unknown");
        assert_eq!(data.large_image, "cursor");
        assert_eq!(data.large_text, "Cursor");
        assert_eq!(data.small_image, "");
        assert_eq!(data.small_text, "");
    }

    #[test]
    fn test_activity_builder_vscodium_uses_catalog_id() {
        let data = ActivityBuilder::build(&context(Ide::VSCodium, Some("a.ts"), Some("w")), 0);
        assert_eq!(data.details, "VSCodium · w");
        assert_eq!(data.state, "Editing a.ts (TypeScript)");
        assert_eq!(data.large_image, "vscodium");
        assert_eq!(data.large_text, "VSCodium");
        assert_eq!(data.small_image, language_icon_url("typescript"));
        assert_eq!(data.small_text, "TypeScript");
    }

    #[test]
    fn test_activity_builder_workspace_only() {
        let data = ActivityBuilder::build(&context(Ide::VisualStudioCode, None, Some("my-app")), 0);
        assert_eq!(data.details, "Visual Studio Code · my-app");
        assert_eq!(data.state, "Working on my-app");
        assert_eq!(data.large_image, "vscode");
        assert_eq!(data.large_text, "Visual Studio Code");
        assert_eq!(data.small_image, "");
        assert_eq!(data.small_text, "");
    }

    #[test]
    fn test_activity_builder_without_file_or_workspace() {
        let data = ActivityBuilder::build(&context(Ide::VisualStudioCode, None, None), 0);
        assert_eq!(data.details, "Visual Studio Code");
        assert_eq!(data.state, "Active");
        assert_eq!(data.large_image, "vscode");
        assert_eq!(data.small_image, "");
        assert_eq!(data.small_text, "");
    }
}
