use super::ide_registry::IdeRegistry;
use crate::domain::presence::ide_activity;
use crate::models::types::PresenceData;
use crate::models::IdeContext;

pub struct ActivityBuilder;

impl ActivityBuilder {
    /// Presence for a parsed IDE window: the IDE is the app (`details`, image
    /// text), the file and workspace are the activity (`state`). `large_image` is
    /// the IDE's catalog id; the engine resolves it to the catalog icon URL, or to
    /// the logo when the catalog has none.
    pub fn build(ide_ctx: &IdeContext, timestamp: i64) -> PresenceData {
        let ide_name = IdeRegistry::display_name(&ide_ctx.ide).to_string();

        PresenceData {
            details: ide_name.clone(),
            state: ide_activity(ide_ctx.file_name.as_deref(), ide_ctx.workspace_name.as_deref()),
            large_image: IdeRegistry::catalog_id(&ide_ctx.ide).unwrap_or_default().to_string(),
            large_text: ide_name,
            small_image: String::new(),
            small_text: String::new(),
            timestamp,
            ..PresenceData::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Ide;

    fn context(ide: Ide, file: &str, workspace: &str) -> IdeContext {
        IdeContext {
            ide,
            file_name: Some(file.to_string()),
            workspace_name: Some(workspace.to_string()),
            file_extension: file.rsplit_once('.').map(|(_, ext)| ext.to_string()),
        }
    }

    #[test]
    fn test_activity_builder_with_valid_context() {
        let ide_ctx = context(Ide::VisualStudioCode, "main.rs", "my-project");
        let data = ActivityBuilder::build(&ide_ctx, 123456789);

        assert_eq!(data.details, "Visual Studio Code");
        assert_eq!(data.state, "Editing main.rs in my-project");
        assert_eq!(data.large_image, "vscode");
        assert_eq!(data.large_text, "Visual Studio Code");
        assert_eq!(data.small_image, "");
        assert_eq!(data.small_text, "");
        assert_eq!(data.timestamp, 123456789);
    }

    #[test]
    fn test_activity_builder_uses_the_ide_catalog_id() {
        let data = ActivityBuilder::build(&context(Ide::Cursor, "test.unknown", "workspace"), 0);
        assert_eq!(data.details, "Cursor");
        assert_eq!(data.large_image, "cursor");
    }

    #[test]
    fn test_activity_builder_ide_without_catalog_entry() {
        let data = ActivityBuilder::build(&context(Ide::VSCodium, "a.ts", "w"), 0);
        assert_eq!(data.details, "VSCodium");
        assert_eq!(data.large_image, "");
    }
}
