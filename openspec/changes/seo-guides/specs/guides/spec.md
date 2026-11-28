## ADDED Requirements

### Requirement: Guides hub and article URLs
The site SHALL serve a guides hub at `/guides/` and one article per search intent at `/guides/<slug>/`, each from `public/guides/<slug>/index.html`, for exactly these slugs: `vscode-discord-rich-presence`, `claude-code-discord-status`, `discord-rich-presence-linux`, `custom-discord-rich-presence-any-app`, `jetbrains-neovim-discord-presence`, `lightweight-discord-rich-presence`. The hub SHALL link to every article with a one-line summary.

#### Scenario: Directory URL resolves
- **WHEN** a visitor requests `/guides/vscode-discord-rich-presence/`
- **THEN** the static file `public/guides/vscode-discord-rich-presence/index.html` is served with status 200 and not the 404 fallback

#### Scenario: Hub lists all guides
- **WHEN** a visitor opens `/guides/`
- **THEN** the page links to all six article URLs

### Requirement: Shared stylesheet
All guide pages SHALL link the single stylesheet `public/guides/_shared/guide.css`, which implements the design tokens of `docs/DESIGN.md`, a mobile-first layout, an article column of about 68 characters, styled code blocks, callouts, and FAQ, visible focus styles, and no animation or transition when `prefers-reduced-motion: reduce` is set.

#### Scenario: Reduced motion
- **WHEN** the visitor's system requests reduced motion
- **THEN** button press transforms and transitions are disabled

### Requirement: Article structure
Each article SHALL contain, in this order: a header navigation (logo to `/`, Guides to `/guides/`, Download to the latest GitHub release), a breadcrumb, one `h1`, an answer paragraph whose first two sentences answer the page's query, a table of contents, step sections with `h2`/`h3`, at least one real screenshot from `/img/app/` with `width`, `height`, and `alt`, a FAQ with 3 to 5 questions, links to the other five guides, and a footer.

#### Scenario: Answer first
- **WHEN** a crawler or LLM reads the text after the `h1`
- **THEN** the first paragraph answers the query in its first two sentences

### Requirement: Page metadata and structured data
Each guide page SHALL have a unique `<title>` of fewer than 60 characters, a meta description of fewer than 155 characters, a canonical URL ending in a trailing slash, Open Graph and Twitter tags using `/img/og.jpg`, and JSON-LD for `Article` (author and publisher "Better Rich Presence", `datePublished` `2026-11-28`), `BreadcrumbList`, and `FAQPage`. The `FAQPage` questions and answers SHALL equal the visible FAQ text.

#### Scenario: Structured data parses
- **WHEN** the JSON-LD blocks of a guide are parsed
- **THEN** they are valid JSON and the FAQ entries equal the visible FAQ entries

### Requirement: Factual sourcing
Guide content SHALL state only facts found in `README.md`, `CONTRIBUTING.md`, `docs/*.md`, `apps.json`, `public/llms.txt`, or the landing page, and SHALL NOT invent features or numbers or compare Better Rich Presence with other products.

#### Scenario: Unsupported claim
- **WHEN** a statement cannot be traced to one of the listed sources
- **THEN** it is removed or rewritten as general advice before the page ships

### Requirement: Registry
`public/sitemap.xml` SHALL list the hub and the six articles, `public/llms.txt` SHALL have a "Guides" section linking all six articles, and `public/LandingPage.html` SHALL link to `/guides/` from its navigation and footer.

#### Scenario: Sitemap complete
- **WHEN** a crawler reads `/sitemap.xml`
- **THEN** it finds seven guide-section URLs (hub plus six articles) on the production domain
