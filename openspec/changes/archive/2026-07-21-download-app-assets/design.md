## Context

The repository contains `apps.json` which specifies application metadata for Discord Rich Presence, including `icon_url` values pointing to external Iconify endpoints (`api.iconify.design`). To make the project self-contained and eliminate network latency/failures when loading app icons, all icons will be downloaded into `public/assets/icons/` and referenced locally.

## Goals / Non-Goals

**Goals:**
- Provide an automated Node/Bun script (`scripts/download-app-assets.mjs`) to download all icon URLs referenced in `apps.json`.
- Store SVG files in `public/assets/icons/` with clean filename mappings.
- Rewrite `apps.json` `icon_url` fields to point to local paths (`/assets/icons/<name>.svg`).
- Add an npm script (`assets:download`) to `package.json`.

**Non-Goals:**
- Building a dynamic backend image proxy server.
- Modifying non-icon properties in `apps.json`.

## Decisions

- **Decision 1: Native Node.js script using `fetch` & `fs/promises`**:
  Use standard Node.js APIs (compatible with Bun/Node 18+) to download files asynchronously in batches without external NPM dependencies.
  *Alternatives considered*: Adding `axios` or `node-fetch` as heavy external dependencies.

- **Decision 2: Icon Filename Mapping**:
  Sanitize the `icon` field (e.g. `simple-icons:visualstudiocode` -> `simple-icons-visualstudiocode.svg`) or use `id` (e.g. `vscode.svg`) to ensure unique, filesystem-safe filenames.

## Risks / Trade-offs

- **[Risk]** Rate limiting or network timeout when fetching multiple icons from `api.iconify.design` → **Mitigation**: Implement concurrency limits and retry handles in the download script.
- **[Risk]** Invalid/404 remote image URLs → **Mitigation**: Log warnings for failed downloads while preserving existing `icon_url` entries for manual review.
