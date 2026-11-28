## ADDED Requirements

### Requirement: Open-source and coders-first messaging
The landing page `public/LandingPage.html` SHALL present the project as open source (MIT) and "by coders, for coders", highlighting VS Code and editor support, programming-language icons, and Claude Code integration, with links to the GitHub repository, the latest release, and `CONTRIBUTING.md`.

#### Scenario: Hero message
- **WHEN** a visitor opens the landing page
- **THEN** the first screen states the product, the open-source MIT status, and the coders-for-coders positioning, with a download call to action and a GitHub link

### Requirement: Real screenshots
The landing page SHALL show real screenshots of the app stored in `public/img/`, served as `webp` with explicit `width` and `height` attributes and descriptive `alt` text.

#### Scenario: Screenshot rendering
- **WHEN** the page loads
- **THEN** each screenshot has `width`, `height`, and `alt` set, and below-the-fold images use `loading="lazy"`

### Requirement: Trailer embed slot
The landing page SHALL contain a trailer section with a YouTube embed slot driven by a single video-ID value, loaded on click from `youtube-nocookie.com`. While the ID is unset, the section SHALL render without a broken embed.

#### Scenario: Video ID unset
- **WHEN** the video ID placeholder has not been filled in
- **THEN** the trailer section shows a poster and a link to the releases or repository instead of a broken iframe

#### Scenario: Video ID set
- **WHEN** a video ID is set and the visitor clicks the poster
- **THEN** the page loads the YouTube player for that ID, and not before the click

### Requirement: Search engine metadata
The landing page SHALL define a `<title>`, meta description, canonical URL, `lang`, viewport, theme color, Open Graph tags (`og:title`, `og:description`, `og:image`, `og:url`, `og:type`), and Twitter card tags, and use a single `<h1>` with a logical heading hierarchy.

#### Scenario: Social preview
- **WHEN** the page URL is shared on a platform that reads Open Graph or Twitter tags
- **THEN** the preview shows the title, description, and an image from `public/img/`

### Requirement: Structured data
The landing page SHALL embed JSON-LD with a `SoftwareApplication` (name, description, `applicationCategory`, `operatingSystem`, `license`, `codeRepository`, download URL, free offer) and a `FAQPage` whose questions and answers match the visible FAQ text.

#### Scenario: Structured data parses
- **WHEN** the JSON-LD blocks are parsed
- **THEN** they are valid JSON and the FAQ entries equal the visible FAQ entries

### Requirement: Crawler and answer-engine files
The site SHALL serve `public/llms.txt` (a concise Markdown summary with links to the repository, docs, and releases), `public/sitemap.xml` (listing the landing page URL), and `public/robots.txt` (allowing crawling, including common AI crawlers, and pointing to the sitemap).

#### Scenario: robots and sitemap
- **WHEN** a crawler requests `/robots.txt`
- **THEN** it receives rules that allow the landing page and a `Sitemap:` line pointing to `/sitemap.xml`

#### Scenario: llms.txt
- **WHEN** an LLM agent requests `/llms.txt`
- **THEN** it receives a Markdown summary of the product with links to the GitHub repository, `README.md`, `CONTRIBUTING.md`, and the latest release
