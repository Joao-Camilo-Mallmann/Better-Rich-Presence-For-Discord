## Why

Many icons in `public/assets/icons/` still render as flat single-color silhouettes or monochrome white marks because `scripts/icon-sources.json` pointed to single-color Simple Icons-derived slugs on theSVG (such as `google-chrome`, `brave`, `apache-netbeans-ide`, `gnu-emacs`, `phpstorm`, `rider`, `rubymine`, `webstorm`, `r`, `azure`, and `google-docs`) instead of theSVG's full-color logos, or because `currentColor` / dark-default variants triggered monochrome white fallbacks (such as AWS, n8n, Monday.com, Julia, OBS, 1Password, Basecamp, TeamViewer, Render, and Termius). In addition, several catalog apps in `apps.json` either lack an `icon_url` (`standardnotes`, `pcloud`) or reuse generic icons (`fusion360`, `tortoisegit`, `lens`, `tabby`, `gitextensions`, `claude_desktop`) despite having dedicated logos on theSVG. Upgrading the generated PNG output from 256×256 to 512×512 also aligns with Discord Rich Presence's recommended 512×512 asset size while preserving PNG delivery (since Discord RPC cannot render raw SVG assets).

## What Changes

- Update `scripts/generate-colored-icons.mjs` to rasterize theSVG vector SVGs into 512×512 transparent RGBA PNGs (with a 16px margin), allow theSVG's `auth-badges` collection alongside `brands` and `community`, substitute `currentColor` with `#ffffff` so dual-color SVGs retain their brand accent colors on dark backgrounds, and retry transient CDN fetch errors.
- Bump the pinned `glincker/thesvg` commit in `scripts/icon-sources.json` to `e851d81457442fed8f391f5a2ae0103d28329ac5`.
- Upgrade single-color and monochrome mapped icons to full-color theSVG slugs/variants (`chrome`, `brave-browser`, `netbeans`, `emacs`, `jetbrains-phpstorm`, `jetbrains-rider`, `jetbrains-rubymine`, `jetbrains-webstorm`, `r-lang`, `microsoft-azure`, `google-antigravity`, `gmail-2026`, `google-calendar-2026`, `google-docs-2026`, `google-drive-2026`, `google-meet-2026`, `google-sheets-2026`, `google-slides-2026`, `zoom-2025`, `gnu-bash`, `zed-industries`, `vlang`, `n8n-legacy`, `obs`, `monday` `light` variant, `aws` `color` variant, `julia`, and colored `auth-badges` for `1password`, `basecamp`, `teamviewer`, `render`, and `termius`).
- Move `files` from `keep` into `icons` (using theSVG's `files` logo) and move `expressvpn` (removed upstream from theSVG) into `keep` so its existing PNG is preserved.
- Add dedicated icon entries (`fusion360`, `tortoisegit`, `lens`, `tabby`, `gitextensions`, `standardnotes`, `pcloud`) to `scripts/icon-sources.json` and update `apps.json` to configure them (plus point `claude_desktop` to `claude.png` instead of monochrome `anthropic.png`).
- Regenerate all mapped icons in `public/assets/icons/*.png` at 512×512 and update `docs/Icons.md`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `colored-icon-pipeline`: Output dimensions increase from 256×256 to 512×512 transparent PNGs, allowed theSVG collections expand to include `auth-badges` for apps without colored brand marks, and `currentColor` in downloaded SVGs resolves to `#ffffff` for dark-background readability.

## Impact

- `scripts/generate-colored-icons.mjs`, `scripts/icon-sources.json`, `docs/Icons.md`
- `apps.json`: 8 entries updated to use dedicated full-color icons (`claude_desktop`, `fusion360`, `tortoisegit`, `lens`, `tabby`, `gitextensions`, `standardnotes`, `pcloud`).
- `public/assets/icons/*.png`: all mapped icons regenerated at 512×512 PNG; 7 new PNGs added; `keep` icons (`abletonlive`, `expressvpn`, `helium`, `shotcut`, `windowsterminal`, `wondersharefilmora`) preserved untouched.
- No Rust or React runtime behavior changes (`icon_url`, jsDelivr rewrite, and `AppIcon` local fallback continue to serve `.png` files identically).
