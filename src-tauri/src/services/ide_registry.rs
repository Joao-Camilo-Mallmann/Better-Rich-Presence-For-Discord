use crate::models::Ide;

pub struct IdeDefinition {
    pub display_name: &'static str,
    pub title_identifier: &'static str,
    /// Catalog (`apps.json`) id whose `icon_url` is the IDE's image, if any.
    pub catalog_id: Option<&'static str>,
}

pub struct IdeRegistry;

impl IdeRegistry {
    const IDES: &'static [(Ide, IdeDefinition)] = &[
        (Ide::VisualStudioCode, IdeDefinition {
            display_name: "Visual Studio Code",
            title_identifier: "Visual Studio Code",
            catalog_id: Some("vscode"),
        }),
        (Ide::Cursor, IdeDefinition {
            display_name: "Cursor",
            title_identifier: "Cursor",
            catalog_id: Some("cursor"),
        }),
        (Ide::VSCodium, IdeDefinition {
            display_name: "VSCodium",
            title_identifier: "VSCodium",
            catalog_id: None,
        }),
        (Ide::Windsurf, IdeDefinition {
            display_name: "Windsurf",
            title_identifier: "Windsurf",
            catalog_id: Some("windsurf"),
        }),
        (Ide::Antigravity, IdeDefinition {
            display_name: "Antigravity",
            title_identifier: "Antigravity",
            catalog_id: Some("antigravity"),
        }),
    ];

    pub fn detect_from_title(title: &str) -> Option<Ide> {
        // Because titles can be "filename - workspace - IDE",
        // we should check if the title ends with the IDE title_identifier.
        // Or we can just do exact matching on the parsed ide part.
        for (ide, def) in Self::IDES {
            if title == def.title_identifier {
                return Some(ide.clone());
            }
        }
        None
    }

    fn definition(ide: &Ide) -> Option<&'static IdeDefinition> {
        Self::IDES.iter().find(|(i, _)| i == ide).map(|(_, def)| def)
    }

    /// The IDE's display name, e.g. `Visual Studio Code`.
    pub fn display_name(ide: &Ide) -> &'static str {
        Self::definition(ide).map_or("IDE", |def| def.display_name)
    }

    /// The catalog id that holds the IDE's icon, if the catalog has one.
    pub fn catalog_id(ide: &Ide) -> Option<&'static str> {
        Self::definition(ide).and_then(|def| def.catalog_id)
    }
}
