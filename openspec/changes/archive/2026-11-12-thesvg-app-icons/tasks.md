# Tasks

## 1. Icon source map

- [x] 1.1 Create `scripts/icon-sources.json` with the pinned theSVG commit, the 233 catalog icons mapped to theSVG `brands` slugs (with manual fixes for Adobe apps, Terraform, PowerShell, RStudio, Zed, Antigravity, AWS, Nuxt), and the `keep` list (abletonlive, shotcut, windowsterminal, files); verify every slug exists in the pinned manifest
- [x] 1.2 Add the 8 catalog entries without an icon (typora, monday, putty, ahrefs, browserstack, fontbase, midjourney, 7zip) and the 59 extra popular apps to the map; verify the map plus the `keep` list holds exactly 300 names

## 2. Generator

- [x] 2.1 Rewrite `scripts/generate-colored-icons.mjs` to read the map, fetch SVGs from the pinned theSVG commit, trim borders and render 256×256 transparent PNGs; verify by running `npm run assets:generate` and checking a sample file is 256×256 RGBA
- [x] 2.2 Add near-black detection with the `dark` → white `mono` fallback and the per-entry `variant` override; verify `github.png` and `cursor.png` come from a light rendering and `discord.png` from `default`
- [x] 2.3 Add the unmapped-catalog-icon check and the summary report (generated, kept, failed, unmapped) with a non-zero exit on failures or unmapped names; verify by temporarily removing one map entry and confirming the script fails and names it

## 3. Assets and catalog

- [x] 3.1 Regenerate all icons and review a contact sheet on a dark background; fix wrong or unreadable logos through map overrides until every icon is correct and visible
- [x] 3.2 Add `icon_url` to the 8 `apps.json` entries from 1.2; verify `apps.json` parses and every `icon_url` file exists in `public/assets/icons/`

## 4. Verification

- [x] 4.1 Run `cargo test` in `src-tauri/` and `npm run build`; both pass
- [ ] 4.2 Check in the running app that icons render in the Apps list and the presence card, and that an app's presence in Discord shows the new icon once served from `main`

## 5. Documentation

- [x] 5.1 Update `docs/Icons.md`: theSVG source, `scripts/icon-sources.json`, the regeneration command, 256×256 PNG size, dark-variant rule, and license and trademark notes; verify the documented command runs as written
- [x] 5.2 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise)
