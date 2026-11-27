## 1. Generator Pipeline Update

- [x] 1.1 Update `scripts/generate-colored-icons.mjs` to output 512×512 PNGs (16px margin), allow `auth-badges` in `ALLOWED_COLLECTIONS`, resolve `currentColor` to `#ffffff` before rasterization, and add retry logic to `fetchOk`

## 2. Icon Source Map & App Catalog

- [x] 2.1 Update `scripts/icon-sources.json` to pin `glincker/thesvg` commit `e851d81457442fed8f391f5a2ae0103d28329ac5`, upgrade single-color/monochrome icons to full-color theSVG slugs/variants, move `files` into `icons`, move `expressvpn` into `keep`, and add entries for `fusion360`, `tortoisegit`, `lens`, `tabby`, `gitextensions`, `standardnotes`, and `pcloud`
- [x] 2.2 Update `apps.json` to configure dedicated `icon_url` entries for `claude_desktop`, `fusion360`, `tortoisegit`, `lens`, `tabby`, `gitextensions`, `standardnotes`, and `pcloud`

## 3. Asset Generation & Visual Review

- [x] 3.1 Run `npm run assets:generate` to regenerate all mapped icons at 512×512 PNG with zero failures and zero unmapped catalog icons
- [x] 3.2 Visually inspect contact sheets of all upgraded and newly added icons on a dark background to confirm readability and color accuracy

## 4. Verification

- [x] 4.1 Verify all mapped PNGs in `public/assets/icons/` are 512×512 RGBA PNGs and every `icon_url` in `apps.json` and icon slug in `src-tauri/src/domain/language.rs` resolves to an existing PNG
- [x] 4.2 Run `cargo test` in `src-tauri/`, `npm run build`, and `npm run compiler:check` and confirm all checks pass

## 5. Documentation

- [x] 5.1 Update `docs/Icons.md` to reflect 512×512 PNG output, `currentColor` handling, `auth-badges` support, and updated icon counts
- [x] 5.2 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (N/A — `.agents/AGENTS.md` already references `docs/Icons.md` and `npm run assets:generate` without hardcoding dimensions or collections)
