use crate::models::Ide;

pub struct IdeDefinition {
    pub display_name: &'static str,
    pub title_identifier: &'static str,
    pub small_image_asset: &'static str,
}

pub struct IdeRegistry;

impl IdeRegistry {
    const IDES: &'static [(Ide, IdeDefinition)] = &[
        (Ide::VisualStudioCode, IdeDefinition {
            display_name: "Visual Studio Code",
            title_identifier: "Visual Studio Code",
            small_image_asset: "visual-studio-code",
        }),
        (Ide::Cursor, IdeDefinition {
            display_name: "Cursor",
            title_identifier: "Cursor",
            small_image_asset: "cursor-logo",
        }),
        (Ide::VSCodium, IdeDefinition {
            display_name: "VSCodium",
            title_identifier: "VSCodium",
            small_image_asset: "vscodium",
        }),
        (Ide::Windsurf, IdeDefinition {
            display_name: "Windsurf",
            title_identifier: "Windsurf",
            small_image_asset: "windsurf",
        }),
        (Ide::Antigravity, IdeDefinition {
            display_name: "Antigravity",
            title_identifier: "Antigravity",
            small_image_asset: "antigravity",
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

    pub fn get_asset(ide: &Ide) -> &'static str {
        for (i, def) in Self::IDES {
            if i == ide {
                return def.small_image_asset;
            }
        }
        "visual-studio-code" // fallback, though it shouldn't happen
    }
}
