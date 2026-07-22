## Context

Discord Rich Presence activity objects support external HTTP/HTTPS assets via the `mp:external/<hash>/<url>` proxy protocol or direct URL formatting in Discord SDK clients.
Currently, `src-tauri/src/services/discord.rs` truncates any asset starting with `http` to `"default"` because arbitrary HTTP URLs previously triggered missing asset errors (`?` icon).

`engine.rs` also strips prefixes (like `simple-icons:zenbrowser`) into short keys (`zenbrowser`). When no static asset with key `zenbrowser` exists in the Discord Developer Portal for the Application ID, Discord renders a `?` fallback icon.

## Goals / Non-Goals

**Goals:**
- Retain external icon HTTP/HTTPS URLs (including Iconify or CDN assets) in `engine.rs` and `discord.rs`.
- Format external image URLs into Discord's external image proxy format or raw image URL parameters recognized by Discord RPC.
- Provide a robust fallback to a default app icon if the external URL fails to load.

**Non-Goals:**
- Manually uploading PNG files to the Discord Developer Portal for all 300+ application entries.
- Hosting an intermediate proxy server for static icons.

## Decisions

### Decision 1: Preserve HTTP URLs in `get_discord_asset_key`
Instead of discarding `http` prefixes or assuming every icon string is a local asset key, `get_discord_asset_key` in `engine.rs` will retain full HTTPS URLs when present.

### Decision 2: External Proxy Format in `discord.rs`
Update `discord.rs` asset payload assembly:
- If `large_image` is a valid HTTP/HTTPS URL, convert or format it using Discord's external proxy URL convention (`mp:external/...`) or pass the verified image URL directly, ensuring Discord's gateway can fetch and cache the image.
- Handle fallback cleanly if an image format is unsupported (e.g. converting or selecting SVG PNG endpoints from Iconify).

## Risks / Trade-offs

- **[Risk]** Discord RPC client versions may reject unproxied external SVG URLs.
  - *Mitigation*: Ensure Iconify or GitHub URLs use raster image endpoints (PNG/WebP format) rather than raw SVG files.

- **[Risk]** Rate limits from external icon CDNs when many games/apps are active.
  - *Mitigation*: Leverage default GitHub raw or Iconify PNG CDN URLs which feature client caching.
