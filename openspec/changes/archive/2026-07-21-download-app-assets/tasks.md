## 1. Setup & Scripting

- [x] 1.1 Create `scripts/download-app-assets.mjs` script to fetch remote icon URLs from `apps.json` and save files to `public/assets/icons/`
- [x] 1.2 Update `package.json` with `assets:download` command

## 2. Asset Execution & Verification

- [x] 2.1 Execute `npm run assets:download` to download icon SVG files into `public/assets/icons/`
- [x] 2.2 Verify `apps.json` entries point to local `/assets/icons/` paths and validate file integrity
