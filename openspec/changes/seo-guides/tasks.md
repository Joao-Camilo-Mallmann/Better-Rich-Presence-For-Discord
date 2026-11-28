## 1. Template, hub, and exemplar

- [x] 1.1 Create `public/guides/_shared/guide.css` (design tokens, mobile-first, ~68ch article column, code blocks, callouts, FAQ, focus styles, reduced motion)
- [x] 1.2 Create the hub `public/guides/index.html` listing all six guides with one-line summaries
- [x] 1.3 Create the exemplar `public/guides/vscode-discord-rich-presence/index.html` (900 to 1400 words) with the documented section-by-section comment block, JSON-LD, FAQ, screenshot, and related links

## 2. Articles (other writers; copy the exemplar structure)

- [x] 2.1 `claude-code-discord-status`: show a live Claude Code session as Discord status
- [x] 2.2 `discord-rich-presence-linux`: X11, GNOME Wayland, Hyprland, Sway
- [x] 2.3 `custom-discord-rich-presence-any-app`: add any app without the Discord Developer Portal
- [x] 2.4 `jetbrains-neovim-discord-presence`: JetBrains IDEs, Neovim, Zed, Sublime Text, Cursor
- [x] 2.5 `lightweight-discord-rich-presence`: why a Rust and Tauri core stays under 20 MB RAM

## 3. Registry

- [x] 3.1 Add the hub and all six article URLs to `public/sitemap.xml`
- [x] 3.2 Add a "Guides" section with the six article links to `public/llms.txt`
- [x] 3.3 Add a Guides link to the nav and footer of `public/LandingPage.html`
- [x] 3.4 Confirm `netlify.toml` serves `/guides/<slug>/` from the static file (no change needed)

## 4. Verification

- [x] 4.1 Every registered URL resolves to an existing `public/guides/**/index.html`
- [x] 4.2 JSON-LD in each page parses and the `FAQPage` equals the visible FAQ
- [x] 4.3 Title under 60 characters and meta description under 155 characters on every page; one `h1`; every image has `width`, `height`, and `alt`
- [ ] 4.4 Every claim traces to the allowed sources; no comparisons with other products
- [x] 4.5 `npm run build` succeeds and `dist/guides/` contains all pages and the stylesheet
- [x] 4.6 Open the hub and an article at mobile (375px) and desktop widths

## 6. Search Console alignment (data: last 3 months to 2026-11-26)

Only the home page has impressions (1,348; 64 clicks; avg position 11.6). Traffic comes from "custom" queries ("custom rpc discord" 8.9, "custom rich presence" 8.4, "discord custom rpc" 9.9); the head term "discord rich presence" sits at ~41.

- [x] 6.1 Keep "Custom Discord Rich Presence" and "custom RPC" in the landing page title, meta description, Open Graph/Twitter titles, and h1 (the relaunch draft had dropped them)
- [x] 6.2 Retarget `custom-discord-rich-presence-any-app` at "custom RPC for Discord" (title, headline, breadcrumb, lead)
- [x] 6.3 Add `what-is-discord-rich-presence` for the head term and beginner queries; register it in the sitemap, llms.txt, hub, and related lists
- [ ] 6.4 After deploy: request indexing for `/` and `/guides/` in Search Console, resubmit the sitemap, and compare query positions after 4 weeks

## 5. Documentation

- [x] 5.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise)
