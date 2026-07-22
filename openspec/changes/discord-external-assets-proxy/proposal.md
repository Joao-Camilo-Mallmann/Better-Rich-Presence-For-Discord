## Why

Discord Rich Presence activity updates currently replace HTTP/HTTPS image URLs with a fallback string ("default"). Consequently, applications using online icons (such as Iconify or raw GitHub URLs) fail to render custom logos in Discord profiles, displaying a generic question mark placeholder image (`?`).

Implementing Discord External Assets Proxy support (or proper URL/asset key resolution) allows rich presence icons to be rendered dynamically via external HTTPS image assets without requiring manual asset uploads for every application to the Discord Developer Portal.

## What Changes

- **External Asset Handling in Discord RPC**: Update `discord.rs` to format and send external image URLs using Discord's external proxy format or appropriate asset resolution instead of dropping HTTP URLs to `"default"`.
- **Icon URL Processing in Engine**: Modify `engine.rs` so that HTTP/HTTPS image URLs (including Iconify or CDN assets) are preserved when building `PresenceData`.
- **Application Registry Integration**: Ensure applications with remote `icon_url` definitions can pass their image URLs directly to Discord Rich Presence payload formatting.

## Capabilities

### New Capabilities

- `discord-external-assets`: Support for dynamic external HTTP/HTTPS image rendering in Discord Rich Presence without requiring static developer portal asset keys.

### Modified Capabilities

- None.

## Impact

- `src-tauri/src/services/discord.rs`: Update activity asset mapping logic for `large_image` and `small_image`.
- `src-tauri/src/services/engine.rs`: Ensure `icon_url` or HTTPS icon strings are preserved rather than trimmed to icon keys when external icons are requested.
- `apps.json`: App icon entries with HTTP/HTTPS URLs will correctly display rich presence images.
