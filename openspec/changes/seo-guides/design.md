## Context

The public site is static: Netlify publishes `dist/`, Vite copies `public/` into it, and `netlify.toml` rewrites `/` to `/LandingPage.html` (forced) and has a non-forced `/*` rule that returns the landing page with status 404. Non-forced rules only apply when no static file matches, so `public/guides/<slug>/index.html` is served at `/guides/<slug>/` (Netlify resolves a directory request to its `index.html`) and no redirect change is needed.

## Goals / Non-Goals

**Goals:**
- One page per search intent, answer in the first two sentences, structured so search engines and LLMs can quote it.
- Visual consistency with the landing page through one shared stylesheet; no build step, no JavaScript required.
- Facts come only from the project's own sources.

**Non-Goals:**
- A blog engine, a static site generator, or client-side routing.
- Translations, comments, analytics, or newsletter forms.
- Any change to the app, `apps.json`, or redirects.

## Decisions

- **Directory URLs** (`/guides/<slug>/`), canonical with trailing slash. Rejected: `/guides/<slug>.html` (uglier, and the repo already serves `/` through a rewrite).
- **Plain static HTML, no framework.** Each page repeats the header and footer markup. Duplication is accepted for six pages; a generator would add a build step the site does not have. The exemplar carries a comment block at the top of `<body>` that documents each section so writers copy the structure.
- **One shared stylesheet** `guides/_shared/guide.css` (linked, not inlined, so it is cached across guides). Tokens mirror `docs/DESIGN.md`: Blurple, Green (download CTA only), Magenta, Yellow; 3px black borders; 6px hard shadows; Space Grotesk display and Inter body with system fallbacks. Article text is capped at about 68ch. Press effects respect `prefers-reduced-motion`. Mobile-first.
- **Fonts:** same non-blocking Google Fonts link as the landing page (`media="print"` swap) with system fallbacks, so text renders immediately.
- **Metadata per page:** unique `<title>` under 60 characters, meta description under 155, canonical, Open Graph and Twitter tags (image `/img/og.jpg`), JSON-LD `Article` (author and publisher "Better Rich Presence", `datePublished` 2026-11-28), `BreadcrumbList`, and `FAQPage` whose Q&A text equals the visible FAQ.
- **Page skeleton (fixed order):** header nav, breadcrumb, `h1`, short answer paragraph, table of contents, step sections (`h2`/`h3`), screenshot from `/img/app/` with width, height and alt, FAQ (3 to 5 Q&A), related guides, footer.
- **Sourcing rule:** statements come from `README.md`, `CONTRIBUTING.md`, `docs/*.md`, `apps.json`, `public/llms.txt`, and the landing page. No invented features, numbers, or comparisons with other products. Anything not covered is omitted or phrased as general advice.
- **Registry:** `sitemap.xml` lists the hub and six articles; `llms.txt` gets a "Guides" section; the landing page gets a "Guides" link in the nav and footer (small edits only).
- **Hub** (`/guides/`) lists all six guides with one-line summaries, so the hub is not an orphan and each article is one link from it.

## Risks / Trade-offs

- Header and footer duplicated across seven pages can drift → the exemplar is the reference; the verification task diffs the shared blocks.
- Pages for guides 2 to 6 are linked from the hub and registry before they exist → all links are registered up front, and the verification task fails if any registered URL does not resolve to a file.
- Claims about Discord's own settings are outside the project's sources → writers keep them generic or omit them.
