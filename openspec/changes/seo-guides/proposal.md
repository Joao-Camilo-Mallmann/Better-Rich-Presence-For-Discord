## Why

The landing page targets one broad query ("Discord Rich Presence for developers"). People search for narrower intents instead: "VS Code Discord Rich Presence", "Claude Code Discord status", "Discord Rich Presence on Linux", "add a custom app without the Developer Portal". Each of those intents deserves its own indexable, answer-first page, which also gives LLM-based answer engines (GEO) concise, citable passages that point back to the product.

## What Changes

- Add a static `/guides/` section to the public site: a hub page and six article pages, one search intent each. URLs are directory-style (`/guides/<slug>/`) and served from `public/guides/<slug>/index.html`.
- Add one shared stylesheet `public/guides/_shared/guide.css` (Neo-Brutalist Arcade tokens from `docs/DESIGN.md`) that every guide links, so pages stay small and consistent.
- Add a fixed article template (header nav, breadcrumb, answer-first intro, table of contents, step sections, screenshot, FAQ, related guides, footer) with Article, BreadcrumbList, and FAQPage JSON-LD. The first article (`vscode-discord-rich-presence`) is the exemplar other writers copy.
- Register all seven URLs in `public/sitemap.xml`, add a "Guides" section to `public/llms.txt`, and add Guides links to the landing page nav and footer.
- Guides: `vscode-discord-rich-presence`, `claude-code-discord-status`, `discord-rich-presence-linux`, `custom-discord-rich-presence-any-app`, `jetbrains-neovim-discord-presence`, `lightweight-discord-rich-presence`.

## Capabilities

### New Capabilities
- `guides`: The static guides section: hub page, article template and shared stylesheet, per-page SEO/GEO metadata and structured data, factual-sourcing rules, and registry in sitemap, `llms.txt`, and the landing page.

### Modified Capabilities
<!-- none: the landing page spec in `open-source-relaunch` is not changed; only a nav and footer link are added. -->

## Impact

- New files: `public/guides/_shared/guide.css`, `public/guides/index.html`, `public/guides/<slug>/index.html` (six).
- Edited files: `public/sitemap.xml`, `public/llms.txt`, `public/LandingPage.html` (nav and footer link only), `.agents/AGENTS.md` (reference to the guides section).
- No change to `netlify.toml`, Rust, frontend app, Tauri commands, or `apps.json`.
