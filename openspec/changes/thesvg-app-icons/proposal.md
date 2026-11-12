## Why

The icons in `public/assets/icons/` are flat, single-color Simple Icons silhouettes at 192×192. Many are black (Microsoft Office, Visual Studio Code, the JetBrains IDEs, GitHub, Notion, Cursor), so they vanish on Discord's dark theme and in the app's dark UI. The generator script that produced them, `scripts/generate-colored-icons.mjs`, no longer runs: it reads the `icon` field that `icon-url-only` removed from `apps.json`.

[theSVG](https://thesvg.org) (`glincker/thesvg`) publishes official full-color brand logos with a per-icon license, a machine-readable manifest, and stable jsDelivr URLs. It covers almost every app in the catalog.

## What Changes

- Regenerate every icon referenced by `apps.json` from theSVG as a full-color 256×256 transparent PNG, keeping the existing filenames so `icon_url` values do not change.
- When a logo's default variant is near-black, use theSVG's dark-mode variant (or a white mono rendering) so it stays readable on dark backgrounds.
- Add `icon_url` to the `apps.json` entries that have no icon but exist on theSVG: Typora, Monday.com, PuTTY, Ahrefs, BrowserStack, FontBase, Midjourney, 7-Zip.
- Add icons for 59 popular desktop apps that are not in the catalog yet (Spotify, Steam, Telegram, WhatsApp, Claude, Gemini, and others), bringing the folder to 300 icons. These are assets only; no catalog entries are added.
- Replace `scripts/generate-colored-icons.mjs` with a theSVG-based generator driven by an explicit icon map (`scripts/icon-sources.json`). `npm run assets:generate` keeps its name.
- Icons that theSVG does not have (Ableton Live, Shotcut, Windows Terminal, the generic "Files" icon) keep their current PNG.
- Update `docs/Icons.md` with the source, the regeneration workflow, and license notes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `colored-icon-pipeline`: the source changes from the Simple Icons CDN / Iconify to theSVG, the output size changes from 1024×1024 to 256×256, icons are selected through an explicit map instead of the removed `icon` field, and near-black logos use a dark-background variant.
- `asset-localization`: its requirements describe the removed `icon` field and a 1024×1024 SVG-path workflow that no longer exists; they are removed in favor of `colored-icon-pipeline`.

## Impact

- `public/assets/icons/*.png`: 233 regenerated (minus the 4 kept), 67 added.
- `apps.json`: `icon_url` added to 8 entries; no other fields change.
- `scripts/generate-colored-icons.mjs` (rewritten), `scripts/icon-sources.json` (new), `docs/Icons.md`.
- No Rust or React code changes. `icon_url` URLs, the jsDelivr rewrite, and the `AppIcon` fallback behave as before.
- Discord and jsDelivr serve icons from `main`, so the new icons show in Discord only after this lands on `main` and the jsDelivr cache refreshes.
