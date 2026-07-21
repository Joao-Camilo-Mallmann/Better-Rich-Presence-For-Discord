## Why

Currently, `apps.json` relies on external remote icon URLs (e.g., `api.iconify.design`). Storing these icon assets directly within the repository removes external network runtime dependencies, improves load reliability, and enables serving assets locally or directly via GitHub Raw.

## What Changes

- Create a download script (`scripts/download-app-assets.ts`) to automatically fetch all remote icon URLs defined in `apps.json`.
- Store downloaded SVG/PNG icon assets in `public/assets/icons/`.
- Update `apps.json` entries to point to the localized icon asset paths (`/assets/icons/<icon-name>.svg`).
- Add an npm script command (`npm run assets:download`) to allow easy updating and re-syncing of icon assets.

## Capabilities

### New Capabilities
- `asset-localization`: Downloads remote icon assets referenced in `apps.json` into the local repository and updates `apps.json` references.

### Modified Capabilities
<!-- None -->

## Impact

- `apps.json`: `icon_url` paths updated to point to local `/assets/icons/` files.
- `public/assets/icons/`: New directory created containing downloaded icon SVG files.
- Dependencies: `package.json` updated with `assets:download` script.
