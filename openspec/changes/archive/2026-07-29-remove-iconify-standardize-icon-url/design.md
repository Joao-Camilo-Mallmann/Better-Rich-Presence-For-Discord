## Context

The project currently resolves app icons through three sources:

1. **GitHub Raw / jsDelivr CDN** (`icon_url` field) — 255 of 286 apps point to `raw.githubusercontent.com/.../public/assets/icons/*.png`, which `get_discord_asset_key()` rewrites to jsDelivr for Discord consumption.
2. **Iconify API** (`icon` field, e.g. `simple-icons:visualstudiocode`) — Used as a fallback in the Rust engine's colon-parsing branch and in the React `AppIcon` component. 31 apps still have `icon_url` pointing to `api.iconify.design`.
3. **Lucide `<Monitor/>`** — Generic React fallback when both sources fail.

The `icon` field is vestigial: every app already has `icon_url`, so `icon` is never the primary source. The Iconify colon-parsing in `get_discord_asset_key()` is dead code for apps that have `icon_url` set. The 31 Iconify-pointed apps are the only real users of external icon sources.

## Goals / Non-Goals

**Goals:**
- Eliminate the `icon` field from the entire stack (JSON schema, Rust struct, TypeScript interface)
- Remove all Iconify API fallback paths from Rust backend and React frontend
- Simplify `get_discord_asset_key()` to only handle URL inputs (no colon-parsing)
- Establish a single icon contract: `icon_url` → GitHub Raw PNG → jsDelivr CDN rewrite → Discord

**Non-Goals:**
- Creating PNG icons for the 31 apps currently on Iconify (separate effort)
- Changing the jsDelivr CDN rewrite logic in `get_discord_asset_key()`
- Modifying how Discord Rich Presence consumes the final URL
- Changing the `<Monitor/>` generic fallback behavior

## Decisions

### Decision 1: Remove `icon` field entirely rather than deprecate

**Choice**: Hard removal from `apps.json`, Rust `AppEntry`, and TypeScript `AppDefinition`.

**Why**: The field is already vestigial — `icon_url` takes priority everywhere. Keeping it as deprecated adds maintenance burden with zero benefit. All 286 apps have `icon_url` set.

**Alternative considered**: Soft deprecation (keep field, ignore it). Rejected because it creates confusion for contributors who might think it's still used.

### Decision 2: Clear `icon_url` from the 31 Iconify-pointed apps

**Choice**: Remove `icon_url` from apps that currently point to `api.iconify.design/*`, making them fall through to the `"default"` path in Rust and `<Monitor/>` in React.

**Why**: Keeping Iconify URLs in `icon_url` contradicts the goal of eliminating external icon dependencies. These apps will temporarily show a generic icon, which is acceptable.

**Alternative considered**: Convert Iconify URLs to GitHub Raw URLs pointing to non-existent files. Rejected because it would cause 404s and the generic fallback handles missing icons gracefully.

### Decision 3: Simplify `get_discord_asset_key()` — keep only URL branch

**Choice**: Remove the `icon_trimmed.contains(':')` branch entirely. The function only needs to handle:
- HTTP(S) URLs → apply jsDelivr rewrite → return
- Non-URL strings → return as-is (for `"default"` and similar)

**Why**: With `icon` removed, no colon-formatted strings will ever reach this function. The only input path is `app.icon_url` which is always a full URL or absent.

## Risks / Trade-offs

- **31 apps temporarily lose visual identity** → Acceptable. They get `<Monitor/>` in UI and `"default"` in Discord. PNGs can be added later independently.
- **Breaking change for anyone reading `icon` from `apps.json`** → Low risk. This is an internal schema, not a public API. The field was never documented as stable.
- **Rust test removal** → `test_get_discord_asset_key_iconify_colon_format` must be removed. Add a test for the "non-URL passthrough" case to maintain coverage.
