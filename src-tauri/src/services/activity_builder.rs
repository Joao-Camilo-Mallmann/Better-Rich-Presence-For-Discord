use crate::models::IdeContext;
use crate::models::types::PresenceData;
use super::ide_registry::IdeRegistry;
use super::extension_mapping::ExtensionMapping;

pub struct ActivityBuilder;

impl ActivityBuilder {
    pub fn build(ide_ctx: &IdeContext, timestamp: i64) -> PresenceData {
        let details = if let Some(ref file_name) = ide_ctx.file_name {
            format!("Editing {}", file_name)
        } else {
            "Idle".to_string()
        };

        let state = if let Some(ref workspace) = ide_ctx.workspace_name {
            format!("Workspace: {}", workspace)
        } else {
            "No Workspace".to_string()
        };

        let large_image = ExtensionMapping::get_asset_for_extension(ide_ctx.file_extension.as_deref()).to_string();
        let large_text = ide_ctx.file_name.clone().unwrap_or_else(|| "Unknown File".to_string());

        let small_image = IdeRegistry::get_asset(&ide_ctx.ide).to_string();
        let small_text = IdeRegistry::get_asset(&ide_ctx.ide).replace("-logo", "");
        
        PresenceData {
            details,
            state,
            large_image,
            large_text,
            small_image,
            small_text,
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Ide;

    #[test]
    fn test_activity_builder_with_valid_context() {
        let ide_ctx = IdeContext {
            ide: Ide::VisualStudioCode,
            file_name: Some("main.rs".to_string()),
            workspace_name: Some("my-workspace".to_string()),
            file_extension: Some("rs".to_string()),
        };

        let timestamp = 123456789;
        let data = ActivityBuilder::build(&ide_ctx, timestamp);

        assert_eq!(data.details, "Editing main.rs");
        assert_eq!(data.state, "Workspace: my-workspace");
        assert_eq!(data.large_image, "rust-logo");
        assert_eq!(data.large_text, "main.rs");
        assert_eq!(data.small_image, "visual-studio-code");
        assert_eq!(data.small_text, "visual-studio-code");
        assert_eq!(data.timestamp, 123456789);
    }

    #[test]
    fn test_activity_builder_unknown_extension() {
        let ide_ctx = IdeContext {
            ide: Ide::Cursor,
            file_name: Some("test.unknown".to_string()),
            workspace_name: Some("workspace".to_string()),
            file_extension: Some("unknown".to_string()),
        };

        let data = ActivityBuilder::build(&ide_ctx, 0);

        assert_eq!(data.large_image, "file-icon");
        assert_eq!(data.small_image, "cursor-logo");
        assert_eq!(data.small_text, "cursor");
    }
}
