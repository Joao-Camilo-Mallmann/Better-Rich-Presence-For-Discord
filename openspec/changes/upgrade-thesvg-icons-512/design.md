## Context

Better Rich Presence sources brand logos from theSVG (`glincker/thesvg`) as vector SVGs and rasterizes them via `sharp` into transparent PNG files committed under `public/assets/icons/*.png`. Discord Rich Presence cannot render raw `.svg` files directly, and `apps.json`, `src-tauri/src/domain/language.rs`, and `src/components/AppIcon.tsx` all reference `.png` assets.

## Goals / Non-Goals

**Goals:**
- Replace single-color and monochrome-white icons with full-color logos available in the latest theSVG catalog (`e851d81457442fed8f391f5a2ae0103d28329ac5`).
- Configure dedicated icons in `apps.json` for catalog apps that previously lacked an `icon_url` or shared a generic icon.
- Upgrade generated PNGs from 256×256 to 512×512 with proportional 16px margins.
- Keep every icon legible on Discord's dark theme and the app's dark UI.

**Non-Goals:**
- Committing raw `.svg` files to `public/assets/icons/` (Discord RPC requires raster PNGs).
- Modifying `keep` icons (`abletonlive`, `expressvpn`, `helium`, `shotcut`, `windowsterminal`, `wondersharefilmora`), which are managed manually or come from external sources.
- Adding icons from outside theSVG for the 21 catalog apps that still have no entry in theSVG.

## Decisions

### Decision 1: 512×512 transparent PNG output from vector SVGs
Increase `SIZE` from `256` to `512` and `MARGIN` from `8` to `16` (preserving the 3.125% inset padding) in `scripts/generate-colored-icons.mjs`. Keeping `RASTER_SIZE = 1024` provides 2× supersampling before Lanczos downscaling to 512×512, matching Discord's recommended minimum 512×512 asset resolution and theSVG's 512px PNG export size while maintaining `.png` compatibility across Discord RPC and `AppIcon.tsx`.

### Decision 2: Resolve `currentColor` to `#ffffff` before rasterization
Some theSVG brand logos combine `fill="currentColor"` with colored brand accents (for example, `aws` combines `currentColor` letters with a `#F90` orange smile). Because `librsvg` evaluates `currentColor` as black `#000000`, the dark-neutral share detector previously threw away the orange accent and rendered a pure-white silhouette. Replacing `currentColor` with `#ffffff` (case-insensitive) prior to rasterization preserves brand accent colors while making foreground strokes crisp on dark backgrounds.

### Decision 3: Allow `auth-badges` selectively when no colored brand logo exists
Expand `ALLOWED_COLLECTIONS` in `scripts/generate-colored-icons.mjs` to `new Set(['brands', 'community', 'auth-badges'])`. Brand and community logos remain the default choice; `auth-badges` entries are mapped explicitly in `scripts/icon-sources.json` only for apps that otherwise have no icon on theSVG (`standardnotes`, `pcloud`) or whose brand mark on theSVG is monochrome-only while a crisp colored badge exists (`1password`, `basecamp`, `teamviewer`, `render`, `termius`).

### Decision 4: Visual-inspection filtering for dark-background readability
Every candidate replacement is previewed on a `#1e1f22` Discord-dark canvas before pinning:
- Keep `sqlite` pinned to `mono-white` (its `default` SVG is a wide wordmark with dim `#003B57` text).
- Keep `pgadmin` on `postgresql.png` (its theSVG entry is a 600×130 wordmark with unreadable dark subtitle text).
- Keep `spyderide` on `spyder-ide` (`spyder` has a low-contrast `#303030` letter `S`).
- Keep `opencl` on automatic dark-fallback (`default` has black `#010101` text).
- Pin `eclipseide` (`eclipse-ide`) and `julia` (`julia`) to `default` so both preserve their full colors despite sitting right around the 60% dark-neutral threshold at 512px.
- Paint pure-black single-variant silhouettes (`darkShare >= 0.98`, no `dark` or `mono` variant) white (`default-white`) so `awk`, `circuitpython`, `micropython`, `putty`, `typora`, and `xtend` remain visible on dark backgrounds while keeping dark-tile brand squares (`Ai`, `Lr`, `Ps`, `Pr`) untouched.
- Use `n8n-legacy` for `n8n` (the iconic `#EA4B71` node graph reads much more clearly than the abstract 3-bar `n8n` mark).
- Use `google-antigravity` for `antigravity` (clean vector render of the rainbow "A" mark).
- Move `expressvpn` to `keep` because upstream theSVG removed that slug between `985e12a0` and `e851d814`.

### Decision 5: Retry transient jsDelivr requests with GitHub Raw fallback
Wrap `fetchOk` in up to 3 attempts (retrying transient `403`/`429`/`5xx` and network errors) and fall back to `https://raw.githubusercontent.com/<repo>/<commit>` if jsDelivr rate-limits a request, so regenerating all ~391 icons completes reliably without failures.

## Risks / Trade-offs

- [512×512 PNGs increase `public/assets/icons/` disk size compared to 256×256] → Acceptable per user preference for 512×512 resolution; `compressionLevel: 9` and `adaptiveFiltering: true` remain enabled.
- [jsDelivr caches `main` for up to 7 days after merge] → Updated PNGs appear immediately via the local `/assets/icons/` fallback during dev and on Discord once pushed to `main` and cached/purged.
