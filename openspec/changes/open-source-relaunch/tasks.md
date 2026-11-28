## 1. License

- [x] 1.1 Add `LICENSE` at the repo root with the MIT text and `Copyright (c) 2026 João Camilo Mallmann`

## 2. READMEs

- [x] 2.1 `README.md`: add an MIT badge to the badge row, a "by coders, for coders" tagline under the title, an "Open Source" nav link, and a "Contributing and Open Source" section (before "Documentation") linking to `CONTRIBUTING.md` and `LICENSE`
- [x] 2.2 `README.en.md`: same additions as 2.1
- [x] 2.3 `README.pt.md`: same additions as 2.1 (its content is English today, so the additions are English)
- [x] 2.4 Add `CONTRIBUTING.md` and `LICENSE` to each README's "Documentation" list

## 3. Contributor guide

- [x] 3.1 Create `CONTRIBUTING.md`: ways to contribute, prerequisites, build and run (`bun install`, `bun run tauri dev`)
- [x] 3.2 Document adding an app to `apps.json` (`id`, `name`, `executables`, `client_id`, `category`, `icon_url`)
- [x] 3.3 Document adding an icon (`scripts/icon-sources.json`, `npm run assets:generate -- --only <name>`, link to `docs/Icons.md`)
- [x] 3.4 Document architecture rules (Clean Architecture layers, React Compiler rules), the checks to run (`cargo test`, `npm run build`, `npm run compiler:check`), and the English-only rule
- [x] 3.5 Document the OpenSpec workflow (`/opsx:propose`, `/opsx:apply`, `/opsx:archive`, trivial edits exempt) and the contribution license statement

## 4. Landing page (planned; built in a separate pass)

- [x] 4.1 Capture real app screenshots (Apps tab, Status Preview, Settings with Claude Code) and save them as `webp` in `public/img/` with known dimensions
- [x] 4.2 Redesign `public/LandingPage.html` following `docs/DESIGN.md`: hero (product, MIT, "by coders, for coders", download and GitHub CTAs), features, supported-apps strip with language and editor icons, Claude Code section, Linux and Windows section, open-source and contributing section, FAQ
- [x] 4.3 Add the trailer section: click-to-load `youtube-nocookie.com` facade driven by one `VIDEO_ID` constant, with a poster and fallback link while the ID is empty
- [x] 4.4 Add the SEO head: title, description, canonical, `lang`, theme color, Open Graph and Twitter tags (image from `public/img/`)
- [x] 4.5 Add JSON-LD `SoftwareApplication` and `FAQPage`; keep FAQ answers identical to the visible FAQ
- [x] 4.6 Add `public/llms.txt`, `public/sitemap.xml`, and `public/robots.txt`; check that the `netlify.toml` rewrites do not shadow them (they are static files in `dist/`)
- [x] 4.7 Check accessibility and performance: alt text, one `<h1>`, contrast, mobile layout, lazy-loaded images, no render-blocking scripts

## 5. Verification

- [x] 5.1 Check that relative links in `CONTRIBUTING.md` and the READMEs resolve to existing files
- [x] 5.2 Check that the three READMEs carry the same additions
- [x] 5.3 `npm run build` succeeds and `dist/` contains the landing page and crawler files (after section 4)
- [x] 5.4 Validate the JSON-LD and Open Graph tags, and open the page at mobile and desktop widths (after section 4)

## 6. Documentation

- [x] 6.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed: add references to `CONTRIBUTING.md` and `LICENSE`
