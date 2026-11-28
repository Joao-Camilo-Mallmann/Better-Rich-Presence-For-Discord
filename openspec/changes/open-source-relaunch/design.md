## Context

The repo is a Tauri 2 + Rust + React app. It has `docs/` (DESIGN, Icons, SettingRichPresence), OpenSpec under `openspec/`, and a Netlify-served static page (`public/LandingPage.html`; `netlify.toml` rewrites `/` to it). `README.md`, `README.en.md`, and `README.pt.md` are currently byte-identical English files. There is no `.github/` folder, no `LICENSE`, and neither `package.json` nor `src-tauri/Cargo.toml` declares a license.

## Goals / Non-Goals

**Goals:**
- A legally clear MIT license and a contributor guide that gets a newcomer from clone to a merged "add an app" pull request.
- READMEs and landing page that state the positioning ("by coders, for coders") and the open-source status up front.
- A landing page that is discoverable by search engines and by LLM-based answer engines (GEO).

**Non-Goals:**
- Issue and PR templates, CI workflows, a code of conduct, or a security policy (candidates for a follow-up change).
- Translating the READMEs to Portuguese. `README.pt.md` has English content today; this change keeps it in sync with the others.
- Any Rust or frontend behavior change.
- Setting a `license` field in `package.json` / `Cargo.toml` (manifest metadata, left for a follow-up so this change does not touch `src-tauri/`).

## Decisions

- **MIT, standard text.** Copyright line: `Copyright (c) 2026 João Camilo Mallmann`. Chosen by the owner. The MIT license covers the project's own code; logos in `public/assets/icons/` remain trademarks of their owners (already stated in `docs/Icons.md` section 4), so `CONTRIBUTING.md` and the README link to that section instead of restating it.
- **READMEs stay in sync.** All three files get the same additions in the same places: an MIT badge in the badge row, a one-line tagline under the title, a nav link, and a "Contributing and Open Source" section placed before "Documentation". Rejected alternative: editing only `README.md`; the three files would drift further.
- **CONTRIBUTING.md is derived from existing docs, not a copy.** It gives the minimal steps for each path and links to `docs/Icons.md`, `docs/DESIGN.md`, and `.agents/AGENTS.md` for detail. Paths covered: build and run, add an app to `apps.json` (fields `id`, `name`, `executables`, `client_id`, `category`, `icon_url`), add an icon, architecture rules (Clean Architecture layers, React Compiler rules), checks to run, the OpenSpec workflow for bigger changes, and the English-only rule.
- **Landing page is planned, not built here.** The tasks list its deliverables so a separate pass can execute them. Constraints: static HTML only (Netlify publishes `dist`, and Vite copies `public/` into it), no new build step; images in `public/img/` are `webp` with explicit `width`/`height`; the trailer is a click-to-load YouTube facade (a poster that swaps in the `youtube-nocookie.com` iframe) so it costs nothing until clicked, with one constant for the video ID; JSON-LD is `SoftwareApplication` (with `license`, `codeRepository`, `applicationCategory`) plus `FAQPage` whose answers match the visible FAQ text; `llms.txt`, `sitemap.xml`, and `robots.txt` live in `public/`.

## Risks / Trade-offs

- A license cannot be easily changed once outside contributions land → the owner confirmed MIT; `CONTRIBUTING.md` states that contributions are accepted under the same license.
- Bundled third-party brand logos are not MIT → `CONTRIBUTING.md` and the README point to `docs/Icons.md` section 4.
- The trailer video ID is unknown at build time → the slot shows a poster and a link instead of a broken embed until the ID is filled in.
- Structured data can drift from visible content → a task checks that FAQ answers in JSON-LD and in the page match.
