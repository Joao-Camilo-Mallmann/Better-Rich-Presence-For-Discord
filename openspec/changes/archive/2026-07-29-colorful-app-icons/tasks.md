## 1. Create the Colored Icon Generation Script

- [x] 1.1 Create `scripts/generate-colored-icons.mjs` with slug extraction logic that parses the `icon` field from `apps.json` (extracting slug from `simple-icons:{slug}` and `{prefix}:{name}` patterns)
- [x] 1.2 Implement Simple Icons CDN download: fetch colored SVG from `https://cdn.simpleicons.org/{slug}` for `simple-icons:*` entries
- [x] 1.3 Implement Iconify API fallback: fetch SVG from `https://api.iconify.design/{prefix}/{name}.svg` for non-`simple-icons` entries (e.g., `material-symbols:*`)
- [x] 1.4 Implement SVG→PNG conversion using `sharp`: resize to 1024×1024 with `fit: contain` and transparent background, output to `public/assets/icons/{slug}.png`
- [x] 1.5 Add error handling: log warnings on failure, skip to next icon, preserve existing PNG
- [x] 1.6 Add summary report at end: total processed, successful, failed, skipped counts

## 2. Wire Up and Clean Up

- [x] 2.1 Add `assets:generate` script to `package.json` pointing to `scripts/generate-colored-icons.mjs`
- [x] 2.2 Remove `scripts/convert-svgs-to-png.js` (functionality merged into new script)
- [x] 2.3 Update or remove the old `scripts/download-app-assets.mjs` if fully superseded

## 3. Generate and Commit Icons

- [x] 3.1 Run `npm run assets:generate` to regenerate all 232 icon PNGs with brand colors
- [x] 3.2 Visually verify a sample of generated PNGs (Discord, VS Code, Chrome, Figma) to confirm colors are correct
- [x] 3.3 Remove any leftover intermediate SVG files from `public/assets/icons/`
- [x] 3.4 Stage and commit the regenerated PNGs
