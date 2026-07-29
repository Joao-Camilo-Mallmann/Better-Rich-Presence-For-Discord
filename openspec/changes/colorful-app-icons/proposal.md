## Why

All 232 app icons in `public/assets/icons/` are monochrome black silhouettes sourced from Simple Icons SVGs. This looks lifeless — Discord's blurple logo renders as a black blob, VS Code's blue is gone, Chrome is unrecognizable. The icons are used in Discord Rich Presence (`large_image`), the frontend UI (`AppIcon.tsx`), and as the page favicon. Colorful brand logos would dramatically improve visual quality across all three surfaces.

## What Changes

- Replace the icon generation script with a new one-shot script that downloads **colored** SVGs from the Simple Icons CDN (`cdn.simpleicons.org/{slug}`) and converts them to 1024×1024 PNGs via `sharp`
- Re-generate all 232 icon PNGs as colorful brand-colored versions and commit them to the repo
- For icons using non-Simple-Icons sources (`material-symbols:*`), fall back to the Iconify API which also serves colored SVGs
- Remove the old `convert-svgs-to-png.js` script (its functionality is merged into the new script)
- Keep the existing `icon_url` pattern in `apps.json` unchanged (still points to raw GitHub PNG URLs)

## Capabilities

### New Capabilities
- `colored-icon-pipeline`: A one-shot script that downloads colored SVGs from Simple Icons CDN (with Iconify fallback) and converts them to 1024×1024 PNGs committed to the repository

### Modified Capabilities
- `asset-localization`: The download pipeline now sources colored SVGs instead of monochrome ones, and converts SVG→PNG inline rather than as a separate step

## Impact

- **Files**: `scripts/download-app-assets.mjs` (rewrite), `scripts/convert-svgs-to-png.js` (remove), `public/assets/icons/*.png` (regenerate all 232)
- **Dependencies**: `sharp` (already in `package.json`, no new deps)
- **Build**: No build-time changes — PNGs are committed to repo, not generated during build
- **Downstream**: `icon_url` in `apps.json` stays the same, so Discord Rich Presence and `AppIcon.tsx` need zero changes
