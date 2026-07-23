#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ide {
    VisualStudioCode,
    Cursor,
    VSCodium,
    Windsurf,
    Antigravity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdeContext {
    pub ide: Ide,
    pub file_name: Option<String>,
    pub workspace_name: Option<String>,
    pub file_extension: Option<String>,
}
