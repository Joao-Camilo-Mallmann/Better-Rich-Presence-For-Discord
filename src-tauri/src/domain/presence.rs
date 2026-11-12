//! Pure rules for what a Discord activity contains: its title, text lines and
//! large image. The engine applies them; the Status Preview renders the result.

/// Registered name of the shared Discord application (`DEFAULT_CLIENT_ID`).
/// Discord always shows it as the activity's bold title.
pub const DEFAULT_ACTIVITY_NAME: &str = "Better Rich Presence";

/// The shared Discord application has no uploaded art assets, so every image
/// sent to it must be a URL. This is the fallback when no app icon is known.
pub const LOGO_IMAGE_URL: &str =
    "https://cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/public/logo-256.png";

const JSDELIVR_BASE: &str = "cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/";
const RAW_GITHUB_BASES: [&str; 2] = [
    "raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/refs/heads/main/",
    "raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/main/",
];

/// Rule details that older versions saved as "no details given".
const LEGACY_DETAIL_PLACEHOLDERS: [&str; 2] = ["Using the computer", "Usando o computador"];

/// Whether `value` is an HTTP(S) URL rather than a Discord asset key.
pub fn is_image_url(value: &str) -> bool {
    let value = value.trim();
    value.starts_with("http://") || value.starts_with("https://")
}

/// Rewrites an image URL for Discord: `.svg` becomes its `.png` sibling (Discord
/// cannot render SVG) and GitHub Raw becomes jsDelivr. Other strings are only trimmed.
pub fn discord_image_url(icon: &str) -> String {
    let icon = icon.trim();
    if !is_image_url(icon) {
        return icon.to_string();
    }

    let mut url = match icon.strip_suffix(".svg") {
        Some(stem) => format!("{stem}.png"),
        None => icon.to_string(),
    };
    for raw in RAW_GITHUB_BASES {
        url = url.replace(raw, JSDELIVR_BASE);
    }
    url
}

/// Values that mean "no specific image": the app's own icon, if any, applies.
fn is_placeholder_image(value: &str) -> bool {
    value.is_empty() || value.eq_ignore_ascii_case("auto") || value.eq_ignore_ascii_case("default")
}

/// Resolves a presence's large image to something Discord can render.
///
/// URLs are kept (rewritten by [`discord_image_url`]). Any other value is looked
/// up in the catalog by app id or executable through `catalog_icon`. On the shared
/// client, which has no uploaded assets, everything else becomes [`LOGO_IMAGE_URL`].
/// An app's own Discord application may have uploaded assets, so there an
/// unknown key is passed through.
pub fn resolve_large_image(
    value: &str,
    shared_client: bool,
    catalog_icon: impl Fn(&str) -> Option<String>,
) -> String {
    let value = value.trim();
    if is_image_url(value) {
        return discord_image_url(value);
    }
    if !is_placeholder_image(value) {
        if let Some(url) = catalog_icon(value) {
            return discord_image_url(&url);
        }
        if !shared_client {
            return value.to_string();
        }
    }
    LOGO_IMAGE_URL.to_string()
}

/// The bold title Discord shows: the shared application's name, else the name
/// Discord reported for the app's own application, else the app name.
pub fn activity_name(shared_client: bool, discord_app_name: Option<&str>, app_name: &str) -> String {
    if shared_client {
        return DEFAULT_ACTIVITY_NAME.to_string();
    }
    match discord_app_name.map(str::trim).filter(|n| !n.is_empty()) {
        Some(name) => name.to_string(),
        None => app_name.trim().to_string(),
    }
}

/// The `details` line of a rule presence. The rule's own text wins; on the shared
/// client it is prefixed with the app name so Discord names the app. Empty text
/// and legacy placeholders become the app name.
pub fn rule_details(display_name: &str, rendered_details: &str, shared_client: bool) -> String {
    let name = display_name.trim();
    let details = rendered_details.trim();

    let is_placeholder = details.is_empty()
        || LEGACY_DETAIL_PLACEHOLDERS
            .iter()
            .any(|p| details.eq_ignore_ascii_case(p));
    if is_placeholder {
        return if name.is_empty() { details.to_string() } else { name.to_string() };
    }

    if !shared_client || name.is_empty() || details.to_lowercase().contains(&name.to_lowercase()) {
        return details.to_string();
    }
    format!("{name} - {details}")
}

/// The `state` line of an IDE presence, from the parsed file and workspace.
pub fn ide_activity(file_name: Option<&str>, workspace: Option<&str>) -> String {
    let file = file_name.map(str::trim).filter(|s| !s.is_empty());
    let workspace = workspace.map(str::trim).filter(|s| !s.is_empty());
    match (file, workspace) {
        (Some(file), Some(workspace)) => format!("Editing {file} in {workspace}"),
        (Some(file), None) => format!("Editing {file}"),
        (None, Some(workspace)) => format!("In {workspace}"),
        (None, None) => "Active".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog(id: &str) -> Option<String> {
        match id {
            "firefox" => Some("https://example.com/firefox.png".to_string()),
            "cursor" => Some(
                "https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/refs/heads/main/public/assets/icons/cursor.png"
                    .to_string(),
            ),
            _ => None,
        }
    }

    #[test]
    fn image_url_rewrites_raw_github_to_jsdelivr() {
        let input = "https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/main/public/assets/icons/simple-icons-zenbrowser.png";
        let expected = "https://cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/public/assets/icons/simple-icons-zenbrowser.png";
        assert_eq!(discord_image_url(input), expected);
    }

    #[test]
    fn image_url_turns_svg_into_png() {
        assert_eq!(discord_image_url("https://example.com/a.svg"), "https://example.com/a.png");
    }

    #[test]
    fn image_url_passes_plain_strings_through() {
        assert_eq!(discord_image_url(" default "), "default");
        assert_eq!(discord_image_url("simple-icons:zenbrowser"), "simple-icons:zenbrowser");
    }

    #[test]
    fn large_image_keeps_urls() {
        assert_eq!(
            resolve_large_image("https://example.com/x.png", true, catalog),
            "https://example.com/x.png"
        );
    }

    #[test]
    fn large_image_placeholders_become_logo() {
        for value in ["", "auto", "default", "DEFAULT", "  "] {
            assert_eq!(resolve_large_image(value, true, catalog), LOGO_IMAGE_URL);
            assert_eq!(resolve_large_image(value, false, catalog), LOGO_IMAGE_URL);
        }
    }

    #[test]
    fn large_image_catalog_id_becomes_its_icon_url() {
        assert_eq!(
            resolve_large_image("firefox", true, catalog),
            "https://example.com/firefox.png"
        );
        assert_eq!(
            resolve_large_image("cursor", true, catalog),
            "https://cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/public/assets/icons/cursor.png"
        );
    }

    #[test]
    fn large_image_unknown_key_becomes_logo_on_shared_client() {
        for key in ["idle", "rust-logo", "file-icon", "simple-icons:vscode", "vscodium"] {
            assert_eq!(resolve_large_image(key, true, catalog), LOGO_IMAGE_URL);
        }
    }

    #[test]
    fn large_image_unknown_key_passes_through_on_own_client() {
        assert_eq!(resolve_large_image("my-uploaded-asset", false, catalog), "my-uploaded-asset");
    }

    #[test]
    fn activity_name_on_shared_client_is_the_shared_app_name() {
        assert_eq!(activity_name(true, Some("Other"), "Firefox"), DEFAULT_ACTIVITY_NAME);
    }

    #[test]
    fn activity_name_on_own_client_prefers_the_discord_name() {
        assert_eq!(activity_name(false, Some("Spotify Premium"), "Spotify"), "Spotify Premium");
        assert_eq!(activity_name(false, Some("  "), "Spotify"), "Spotify");
        assert_eq!(activity_name(false, None, "Spotify"), "Spotify");
    }

    #[test]
    fn rule_details_prefixes_the_app_name_on_shared_client() {
        assert_eq!(rule_details("Notes", "Writing Plan", true), "Notes - Writing Plan");
    }

    #[test]
    fn rule_details_keeps_text_that_names_the_app() {
        assert_eq!(rule_details("Notes", "Taking notes", true), "Taking notes");
    }

    #[test]
    fn rule_details_keeps_text_on_own_client() {
        assert_eq!(rule_details("Notes", "Writing Plan", false), "Writing Plan");
    }

    #[test]
    fn rule_details_empty_or_legacy_becomes_the_app_name() {
        assert_eq!(rule_details("Notes", "", true), "Notes");
        assert_eq!(rule_details("Notes", "Using the computer", true), "Notes");
        assert_eq!(rule_details("Notes", "usando o computador", false), "Notes");
    }

    #[test]
    fn ide_activity_sentences() {
        assert_eq!(ide_activity(Some("main.rs"), Some("my-project")), "Editing main.rs in my-project");
        assert_eq!(ide_activity(Some("main.rs"), Some(" ")), "Editing main.rs");
        assert_eq!(ide_activity(None, Some("my-project")), "In my-project");
        assert_eq!(ide_activity(None, None), "Active");
    }
}
