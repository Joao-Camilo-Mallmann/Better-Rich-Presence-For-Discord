## Context

The project has 232 app icon PNGs in `public/assets/icons/`, all monochrome black silhouettes. They were generated from Simple Icons SVGs (which are monochrome by design) via a `convert-svgs-to-png.js` script using `sharp`. The `icon` field in `apps.json` uses `simple-icons:{slug}` notation (e.g., `simple-icons:discord`), and `icon_url` points to the raw GitHub URL of the committed PNG.

These PNGs are consumed in three places:
1. **Discord Rich Presence** — `large_image` asset (Discord recommends 1024×1024)
2. **Frontend UI** — `AppIcon.tsx` renders `<img src={icon_url}>`
3. **Page favicon** — `App.tsx` sets `link[rel=icon]` to the same URL

Simple Icons CDN (`cdn.simpleicons.org/{slug}`) serves the same icons but with the **official brand color** applied as the SVG `fill` attribute. This is a free, no-auth, Cloudflare-cached CDN with the same slug namespace already used in `apps.json`.

## Goals / Non-Goals

**Goals:**
- Replace all 232 monochrome black icon PNGs with colorful brand-colored versions
- Create a single one-shot script that downloads colored SVGs and converts to 1024×1024 PNG
- Handle non-Simple-Icons entries (e.g., `material-symbols:*`) via Iconify API fallback
- Committed PNGs — no build-time or runtime dependency on external CDNs
- Keep `icon_url` format in `apps.json` unchanged

**Non-Goals:**
- Changing the `icon` field format in `apps.json`
- Adding runtime SVG→PNG conversion
- Supporting animated icons or multi-color gradients beyond what the SVG source provides
- Extracting actual executable icons from installed applications

## Decisions

### Decision 1: Simple Icons CDN as primary colored SVG source

**Choice**: Fetch colored SVGs from `https://cdn.simpleicons.org/{slug}` (default brand color)

**Alternatives considered**:
- **VectorLogo.zone**: Only serves SVGs, different slug namespace requiring a manual mapping table, not all icons covered. Higher friction for no clear benefit.
- **logo.dev / Brandfetch**: Require API keys, serve company logos (not app icons), different coverage set.
- **CompanyEnrich**: Returns JPEG at 200×200 — wrong format, too small.
- **Devicon**: SVG-only, different slug namespace, lower coverage for non-dev tools.

**Rationale**: The `apps.json` already uses Simple Icons slugs (`simple-icons:discord` → slug `discord`), so zero mapping is needed. The CDN returns SVGs with `fill="#5865F2"` (brand color). Free, no auth, Cloudflare-cached.

### Decision 2: Iconify API as fallback for non-Simple-Icons entries

**Choice**: For icons with `material-symbols:*` or other non-simple-icons prefixes, fetch from `https://api.iconify.design/{prefix}/{name}.svg`

**Rationale**: The codebase already uses Iconify as a fallback in `AppIcon.tsx` (line 44). Iconify serves colored SVGs for material-symbols and other icon sets.

### Decision 3: Inline SVG→PNG conversion (single script)

**Choice**: Download SVG + convert to PNG in one pass within a single script, rather than separate download and conversion steps.

**Rationale**: The current pipeline uses two scripts (`download-app-assets.mjs` + `convert-svgs-to-png.js`). Merging them eliminates intermediate SVG files and simplifies the workflow. `sharp` is already a project dependency.

### Decision 4: Keep existing PNG filenames

**Choice**: Output PNGs as `{slug}.png` (e.g., `discord.png`, `visualstudiocode.png`) matching the current naming convention.

**Rationale**: `icon_url` in `apps.json` already references these filenames via raw GitHub URLs. Keeping names unchanged means zero changes to `apps.json` URL values.

## Risks / Trade-offs

- **Some icons may look similar in black intentionally** (e.g., GitHub's logo IS black) → No mitigation needed, the CDN returns the correct brand color even when it's black
- **Simple Icons CDN could be temporarily down** → Script is one-shot; re-run later if it fails. Committed PNGs mean the app never depends on the CDN at runtime
- **A slug might not exist on Simple Icons CDN** → Script logs a warning and keeps the existing PNG. The Iconify fallback covers `material-symbols:*` entries
- **Repo size increase** → Negligible. 232 PNGs at ~30-50KB each ≈ ~7-10MB. Already in this range with current black PNGs
