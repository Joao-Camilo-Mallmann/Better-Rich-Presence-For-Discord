pub struct ExtensionMapping;

impl ExtensionMapping {
    pub fn get_asset_for_extension(ext: Option<&str>) -> &'static str {
        match ext {
            Some("rs") => "rust-logo",
            Some("ts") | Some("tsx") => "typescript-logo",
            Some("js") | Some("jsx") => "javascript-logo",
            Some("md") => "markdown-logo",
            _ => "file-icon", // default
        }
    }
}
