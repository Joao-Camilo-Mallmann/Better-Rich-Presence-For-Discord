## Why

Better Rich Presence for Discord is being relaunched as an open source project positioned "by coders, for coders": a developer-focused Discord Rich Presence app (VS Code and other editors, language icons, Claude Code integration) that the community can read, edit, and contribute to. Today the repository has no license, no contributor guide, and the READMEs and landing page do not say the project is open source or how to help. Without a license the code is legally not reusable, and without a guide the main contribution paths (adding an app to `apps.json`, adding an icon) stay undiscoverable.

## What Changes

- Add an MIT `LICENSE` (copyright 2026 João Camilo Mallmann).
- Add an MIT badge, the "by coders, for coders" positioning, and a "Contributing and Open Source" section to `README.md`, `README.en.md`, and `README.pt.md` (all three currently carry identical English content, so the additions are English in all three).
- Add `CONTRIBUTING.md`: how to build and run (`bun install`, `bun run tauri dev`), how to add an app to `apps.json`, how to add an icon (`scripts/icon-sources.json` + `npm run assets:generate`, per `docs/Icons.md`), the architecture rules contributors must respect, and the OpenSpec workflow for larger changes.
- Redesign the static landing page `public/LandingPage.html` (served by `netlify.toml`): real app screenshots in `public/img/`, a YouTube trailer embed slot ready for a video ID, open-source and coders-for-coders messaging, and strong SEO and GEO (meta tags, Open Graph and Twitter cards, JSON-LD `SoftwareApplication` and `FAQPage`, `llms.txt`, `sitemap.xml`, `robots.txt`). This part is planned here and built later in a separate pass.
- Point `.agents/AGENTS.md` at `CONTRIBUTING.md` and `LICENSE`.

## Capabilities

### New Capabilities
- `open-source-project`: The license, contributor guide, and README open-source positioning that make the repository ready for community contributions.
- `landing-page`: The static marketing page and its SEO/GEO metadata, screenshots, and trailer slot.

### Modified Capabilities
<!-- none -->

## Impact

- New files: `LICENSE`, `CONTRIBUTING.md`.
- Edited files: `README.md`, `README.en.md`, `README.pt.md`, `.agents/AGENTS.md`.
- Planned (landing page pass): `public/LandingPage.html`, `public/img/*`, `public/llms.txt`, `public/sitemap.xml`, `public/robots.txt`.
- No Rust, frontend app, Tauri command, or `apps.json` schema changes.
