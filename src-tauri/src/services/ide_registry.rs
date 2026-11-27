use crate::models::Ide;

pub struct IdeDefinition {
    pub display_name: &'static str,
    pub title_identifier: &'static str,
    pub title_aliases: &'static [&'static str],
    /// Catalog (`apps.json`) id whose `icon_url` is the IDE's image, if any.
    pub catalog_id: Option<&'static str>,
}

pub struct IdeRegistry;

impl IdeRegistry {
    const IDES: &'static [(Ide, IdeDefinition)] = &[
        (
            Ide::VisualStudioCode,
            IdeDefinition {
                display_name: "Visual Studio Code",
                title_identifier: "Visual Studio Code",
                title_aliases: &["Visual Studio Code - Insiders"],
                catalog_id: Some("vscode"),
            },
        ),
        (
            Ide::Cursor,
            IdeDefinition {
                display_name: "Cursor",
                title_identifier: "Cursor",
                title_aliases: &[],
                catalog_id: Some("cursor"),
            },
        ),
        (
            Ide::VSCodium,
            IdeDefinition {
                display_name: "VSCodium",
                title_identifier: "VSCodium",
                title_aliases: &[],
                catalog_id: Some("vscodium"),
            },
        ),
        (
            Ide::Windsurf,
            IdeDefinition {
                display_name: "Windsurf",
                title_identifier: "Windsurf",
                title_aliases: &[],
                catalog_id: Some("windsurf"),
            },
        ),
        (
            Ide::Antigravity,
            IdeDefinition {
                display_name: "Antigravity",
                title_identifier: "Antigravity",
                title_aliases: &[],
                catalog_id: Some("antigravity"),
            },
        ),
    ];

    pub fn detect_from_title(title: &str) -> Option<Ide> {
        for (ide, def) in Self::IDES {
            if title == def.title_identifier || def.title_aliases.contains(&title) {
                return Some(ide.clone());
            }
        }
        None
    }

    /// Strips a trailing ` - <IDE>` suffix (checking longer aliases such as
    /// `Visual Studio Code - Insiders` first) and returns `(prefix, ide)`.
    pub fn strip_ide_suffix(title: &str) -> Option<(&str, Ide)> {
        for (ide, def) in Self::IDES {
            for alias in def.title_aliases {
                if let Some(rest) = title.strip_suffix(alias) {
                    if let Some(prefix) = rest.strip_suffix(" - ") {
                        return Some((prefix, ide.clone()));
                    }
                }
            }
            if let Some(rest) = title.strip_suffix(def.title_identifier) {
                if let Some(prefix) = rest.strip_suffix(" - ") {
                    return Some((prefix, ide.clone()));
                }
            }
        }
        None
    }

    fn definition(ide: &Ide) -> Option<&'static IdeDefinition> {
        Self::IDES
            .iter()
            .find(|(i, _)| i == ide)
            .map(|(_, def)| def)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_vscode_insiders_as_vscode() {
        assert_eq!(
            IdeRegistry::detect_from_title("Visual Studio Code - Insiders"),
            Some(Ide::VisualStudioCode)
        );
        assert_eq!(
            IdeRegistry::detect_from_title("Visual Studio Code"),
            Some(Ide::VisualStudioCode)
        );
    }

    #[test]
    fn vscodium_has_catalog_id() {
        assert_eq!(IdeRegistry::catalog_id(&Ide::VSCodium), Some("vscodium"));
    }

    #[test]
    fn strip_ide_suffix_handles_insiders_and_requires_separator() {
        assert_eq!(
            IdeRegistry::strip_ide_suffix("main.rs - app - Visual Studio Code - Insiders"),
            Some(("main.rs - app", Ide::VisualStudioCode))
        );
        assert_eq!(
            IdeRegistry::strip_ide_suffix("my-app - Cursor"),
            Some(("my-app", Ide::Cursor))
        );
        assert_eq!(
            IdeRegistry::strip_ide_suffix("Visual Studio Code - Insiders"),
            None
        );
        assert_eq!(IdeRegistry::strip_ide_suffix("Visual Studio Code"), None);
    }
}
