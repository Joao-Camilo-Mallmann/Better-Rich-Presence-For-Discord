## Why

The icon resolution pipeline currently has three different sources: GitHub Raw PNGs (primary), Iconify API (fallback for 31 apps), and a Lucide `<Monitor/>` generic. This creates unnecessary external dependencies, inconsistent icon styles (SVG vs PNG), and dead-code paths in both the Rust backend and React frontend. Consolidating to a single source (GitHub Raw `icon_url`) simplifies the codebase, removes the Iconify dependency, and makes the icon contract trivial: if a PNG exists in `public/assets/icons/`, it works; if not, you get a generic fallback.

## What Changes

- **BREAKING**: Remove the `icon` field (Iconify key like `simple-icons:visualstudiocode`) from all 286 entries in `apps.json`
- Remove `icon` from the Rust `AppEntry` struct and all fallback branches that use it in `engine.rs`
- Remove the Iconify colon-parsing branch from `get_discord_asset_key()` in Rust
- Remove the Iconify API fallback from the React `AppIcon` component
- Remove the Iconify favicon fallback from `App.tsx`
- Remove `icon` from the TypeScript `AppDefinition` interface
- The 31 apps currently pointing to Iconify URLs in `icon_url` will have that field cleared — they'll fall back to `"default"` in Discord and `<Monitor/>` in the UI until local PNGs are added

## Capabilities

### New Capabilities

- `icon-url-only`: Single-source icon resolution using only `icon_url` (GitHub Raw / jsDelivr CDN). Eliminates the Iconify API as a data source and removes the `icon` field from the app catalog schema.

### Modified Capabilities

_(none — no existing spec-level requirements change, this is an internal simplification)_

## Impact

- **`apps.json`**: Schema change — `icon` field removed from all 286 entries. 31 entries lose their `icon_url` (were Iconify URLs, not local PNGs).
- **Rust backend** (`app_registry.rs`, `engine.rs`): `AppEntry` struct loses `icon` field. `get_discord_asset_key()` simplified (no more colon-parsing). Two fallback chains in `build_presence_from_rule` and `build_fallback_presence` simplified.
- **React frontend** (`AppIcon.tsx`, `App.tsx`, `app-types.ts`): `AppDefinition` loses `icon` field. `AppIcon` no longer calls Iconify API. `App.tsx` favicon logic simplified.
- **Tests**: `test_get_discord_asset_key_iconify_colon_format` in `engine.rs` must be removed or rewritten.
- **31 apps temporarily lose their icons** until local PNGs are created in `public/assets/icons/`.
