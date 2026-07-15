---
version: alpha
name: Better RPC Design System
description: A Neo-Brutalist Arcade design system inspired by Discord's gaming-native palette. Solid blocks of Blurple, Magenta, Green, and Yellow replace soft gradients. Thick black borders (3px), hard offset shadows (6px 6px 0 #000), and bold all-caps display type create a tactile, retro-gaming feel — punchy, playful, and unmistakably not a template.

colors:
  primary: "#5865f2"
  on-primary: "#ffffff"
  green: "#35ed7e"
  magenta: "#ec48bd"
  yellow: "#fee75c"
  red: "#ed4245"
  link: "#00b0f4"
  canvas: "#23272a"
  surface-indigo: "#1e2353"
  surface-onyx: "#23272a"
  surface-black: "#000000"
  ink: "#ffffff"
  ink-dark: "#000000"
  muted: "#333333"
  hairline: "#23272a"

typography:
  display-xl:
    fontFamily: ABC Ginto Nord
    fontSize: 82px
    fontWeight: 800
    lineHeight: 1.0
    letterSpacing: 0
  display-lg:
    fontFamily: ABC Ginto Nord
    fontSize: 62px
    fontWeight: 800
    lineHeight: 1.05
    letterSpacing: 0
  display-md:
    fontFamily: ABC Ginto Nord
    fontSize: 56px
    fontWeight: 700
    lineHeight: 1.05
    letterSpacing: 0
  heading-lg:
    fontFamily: ABC Ginto Nord
    fontSize: 48px
    fontWeight: 700
    lineHeight: 1.1
    letterSpacing: 0
  heading-sm:
    fontFamily: ABC Ginto Nord
    fontSize: 22px
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: 0
  body-lg:
    fontFamily: ABC Ginto
    fontSize: 20px
    fontWeight: 500
    lineHeight: 1.4
    letterSpacing: 0
  link-lg:
    fontFamily: ABC Ginto
    fontSize: 18px
    fontWeight: 500
    lineHeight: 1.4
    letterSpacing: 0
  body:
    fontFamily: ggsans
    fontSize: 16px
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: 0
  link:
    fontFamily: ABC Ginto
    fontSize: 16px
    fontWeight: 500
    lineHeight: 1.4
    letterSpacing: 0
  link-sm:
    fontFamily: ABC Ginto
    fontSize: 14px
    fontWeight: 500
    lineHeight: 1.4
    letterSpacing: 0

rounded:
  xs: 6px
  sm: 12px
  md: 14px
  lg: 16px
  xl: 40px
  pill: 50px
  jumbo: 120px
  full: 9999px

spacing:
  xxs: 4px
  xs: 8px
  sm: 12px
  md: 16px
  lg: 20px
  xl: 24px
  xxl: 32px
  section: 40px

components:
  nav-bar:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.link}"
    padding: "{spacing.md} {spacing.xl}"
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.on-primary}"
    typography: "{typography.link-lg}"
    rounded: "{rounded.sm}"
    padding: "{spacing.lg} {spacing.xl}"
  button-green:
    backgroundColor: "{colors.green}"
    textColor: "{colors.ink-dark}"
    typography: "{typography.link-lg}"
    rounded: "{rounded.sm}"
    padding: "{spacing.sm} {spacing.xl}"
  button-white:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.ink-dark}"
    typography: "{typography.link}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xs} {spacing.md}"
  button-ghost:
    backgroundColor: "{colors.surface-indigo}"
    textColor: "{colors.ink}"
    typography: "{typography.link}"
    rounded: "{rounded.lg}"
    padding: "{spacing.md}"
  button-ghost-sm:
    backgroundColor: "{colors.surface-indigo}"
    textColor: "{colors.ink}"
    typography: "{typography.link-sm}"
    rounded: "{rounded.xs}"
    padding: "{spacing.sm} {spacing.xxl}"
  hero:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.display-xl}"
    padding: "{spacing.section}"
  feature-card-gradient:
    backgroundColor: "{colors.magenta}"
    textColor: "{colors.ink}"
    rounded: "{rounded.xl}"
    padding: "{spacing.section}"
  feature-card-dark:
    backgroundColor: "{colors.surface-indigo}"
    textColor: "{colors.ink}"
    rounded: "{rounded.xl}"
    padding: "{spacing.xxl}"
  showcase-band-black:
    backgroundColor: "{colors.surface-black}"
    textColor: "{colors.ink}"
    rounded: "{rounded.xl}"
    padding: "{spacing.section}"
  stat-card:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.ink}"
    typography: "{typography.display-md}"
    rounded: "{rounded.xl}"
    padding: "{spacing.xxl}"
  step-card:
    backgroundColor: "{colors.magenta}"
    textColor: "{colors.ink}"
    typography: "{typography.heading-sm}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
  cta-band:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.ink}"
    typography: "{typography.display-md}"
    rounded: "{rounded.xl}"
    padding: "{spacing.section}"
  marquee-band:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.ink}"
    typography: "{typography.display-lg}"
    padding: "{spacing.lg}"
  pricing-table:
    backgroundColor: "{colors.surface-indigo}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
  game-rank-feature:
    backgroundColor: "{colors.surface-indigo}"
    textColor: "{colors.ink}"
    typography: "{typography.heading-sm}"
    rounded: "{rounded.lg}"
    padding: "{spacing.md}"
  game-rank-row:
    backgroundColor: "{colors.surface-indigo}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "{spacing.sm} {spacing.md}"
  faq-accordion:
    backgroundColor: "{colors.surface-indigo}"
    textColor: "{colors.ink}"
    typography: "{typography.link-lg}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
  badge:
    backgroundColor: "{colors.magenta}"
    textColor: "{colors.ink}"
    typography: "{typography.link-sm}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xxs} {spacing.sm}"
  footer:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.link}"
    padding: "{spacing.section}"

  # ─── Examples (illustrative) — auto-derived; resolve any TO_FILL markers below ───
  ex-pricing-tier:
    description: "Default Pricing tier card. Re-uses feature-card chrome with brand canvas-soft surface."
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    borderColor: "{colors.hairline}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
  ex-pricing-tier-featured:
    description: "Featured/highlighted tier — polarity-flipped surface (dark fill + light text in light mode, light fill + dark text in dark mode)."
    backgroundColor: "{colors.ink}"
    textColor: "{colors.on-primary}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
  ex-product-selector:
    description: "What's Included summary card — re-purposed for SaaS / B2B verticals (NOT a literal product gallery)."
    backgroundColor: "{colors.canvas}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
  ex-cart-drawer:
    description: "Subscription summary — re-purposed for SaaS / B2B (line items per add-on, not literal cart)."
    backgroundColor: "{colors.canvas}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
    item-divider: "{colors.hairline}"
  ex-app-shell-row:
    description: "Sidebar nav row inside the App Shell example. Active state uses brand primary as the indicator."
    backgroundColor: "{colors.canvas}"
    activeIndicator: "{colors.primary}"
    rounded: "{rounded.md}"
    padding: "{spacing.sm} {spacing.md}"
  ex-data-table-cell:
    description: "Default data-table th + td chrome. Header uses small link-caps typography; body uses body."
    headerBackground: "{colors.surface-indigo}"
    headerTypography: "{typography.link-sm}"
    bodyTypography: "{typography.body}"
    cellPadding: "{spacing.sm} {spacing.md}"
    rowBorder: "{colors.hairline}"
  ex-auth-form-card:
    description: "Sign-in / sign-up card. Re-uses feature-card chrome with text-input primitives inside."
    backgroundColor: "{colors.canvas}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
  ex-modal-card:
    description: "Modal dialog surface — same chrome as feature-card with elevated shadow."
    backgroundColor: "{colors.canvas}"
    rounded: "{rounded.lg}"
    padding: "{spacing.xl}"
  ex-empty-state-card:
    description: "Empty-state illustration frame."
    backgroundColor: "{colors.canvas}"
    rounded: "{rounded.lg}"
    padding: "{spacing.section}"
    captionTypography: "{typography.body}"
  ex-toast:
    description: "Toast notification surface — feature-card shape + medium shadow."
    backgroundColor: "{colors.canvas}"
    rounded: "{rounded.lg}"
    padding: "{spacing.sm} {spacing.md}"
    typography: "{typography.body}"
---

## Overview

Better RPC's design system follows a **Neo-Brutalist Arcade** aesthetic built on Discord's gaming-native palette. Instead of the soft glowing gradients and glassmorphism clichés of typical dark-mode tech sites, this system uses solid, aggressive blocks of saturated color — Blurple for hero sections, Magenta for marquees, Yellow for the footer — bounded by thick black borders and punctuated by hard, offset black shadows. The result is tactile, playful, and unmistakably retro-gaming: it feels like an arcade cabinet UI, not a SaaS template.

The colour anchor is **Blurple** (`{colors.primary}` — #5865f2). It owns the hero background, stat cards, CTA bands, and the nav bar. Around it orbit three supporting accents: electric **Green** (`{colors.green}`) for highest-intent CTAs (download/get started), vibrant **Magenta** (`{colors.magenta}`) for marquee bands and feature cards, and **Yellow** (`{colors.yellow}`) for the footer and highlight badges. The canvas body is **Onyx** (`{colors.canvas}` — #23272a) — Discord's classic dark surface for content areas between the colour bands.

Every interactive element has physical weight. Buttons, cards, and containers are bounded by `border: 3px solid #000000` and cast a hard `box-shadow: 6px 6px 0px #000000` that compresses on hover/click (`translate(4px, 4px)` → `2px 2px 0 #000`), simulating an arcade button press. Giant wordmarks use `-webkit-text-stroke` outlines. Section eyebrow labels are rendered as tilted solid-color tags (`transform: rotate(-2deg)`).

**Key Characteristics:**

- **Solid Color Canvas**: No soft gradient meshes. Hero, performance, CTA, and footer sections use full solid-color backgrounds (Blurple, Magenta, Yellow).
- **Hard Shadows**: All cards, buttons, and elevated elements use solid, offset black shadows (`box-shadow: 6px 6px 0px #000000`). Soft glows are banned.
- **Thick Black Borders**: Every interactive container gets `border: 3px solid #000000` for high-contrast definition.
- **Retro Display Elements**: Giant text uses `-webkit-text-stroke: 3px #000`; section labels are tilted solid-color tags with hard shadows.
- **Interactive Press Effects**: Hover translates the element `4px, 4px` and reduces the shadow to `2px 2px 0`. Active press translates `6px, 6px` and zeroes the shadow.
- **Dot-grid Textures**: Content sections on the Onyx canvas use a subtle `radial-gradient` dot-grid pattern (`rgba(255,255,255,0.1) 2px`, 32px spacing) for texture.
- **Diagonal Stripe Overlays**: Feature cards and CTA bands use repeating `45deg` stripe patterns (`rgba(0,0,0,0.05)`) for depth without gradients.
- **Page rhythm**: Blurple hero → Magenta marquee → Onyx feature cards → Black demo band → Blurple performance → Magenta CTA → Yellow footer.

## Colors

> Source pages analyzed: home, ads/quests, nitro, trending-games. Blurple, green, magenta, white display type, and the deep-indigo canvas recur on every page; nitro adds the pricing table, trending-games adds the ranked game list.

### Brand & Accent

- **Blurple** (`{colors.primary}` — #5865f2): The iconic brand colour. Primary CTA fill, marquee and CTA bands, stat cards, brand mark. The single most-used action colour.
- **Electric Green** (`{colors.green}` — #35ed7e): Reserved for the highest-intent CTA ("get started" / "download"), always paired with `{colors.ink-dark}` text.
- **Vibrant Magenta** (`{colors.magenta}` — #ec48bd): The saturated pink that fills gradient feature panels, step cards, and badges — the playful counterweight to the cool indigo.
- **Link Cyan** (`{colors.link}` — #00b0f4): Inline text-link colour on dark surfaces.

### Surface

- **Indigo Canvas** (`{colors.canvas}` — #0a0d3a): The deep-indigo page base, washed by the animated brand-gradient mesh.
- **Raised Indigo** (`{colors.surface-indigo}` — #1e2353): One step up from canvas — dark feature cards, pricing table, game-rank rows, ghost buttons.
- **Onyx** (`{colors.surface-onyx}` — #23272a): Discord's classic dark-UI surface; product-chrome cards and dividers.
- **Black** (`{colors.surface-black}` — #000000): Full-black showcase bands framing product media.

### Text

- **White** (`{colors.ink}` — #ffffff): All display and body text on the dark canvas. The dominant text colour.
- **Ink** (`{colors.ink-dark}` — #000000): Text on light fills — the white button and green CTA.
- **Muted Ink** (`{colors.muted}` — #333333): Secondary text on the occasional light surface (white product-mockup cards).

### Brand Gradient

The hero and feature panels ride an animated mesh that sweeps from `{colors.primary}` (Blurple) through a deep violet into the vibrant `{colors.magenta}`, resolving back into the `{colors.canvas}` indigo at the edges. It is the brand's defining atmospheric signature — always in motion, never a flat fill.

## Typography

### Font Family

- **ABC Ginto Nord** — the heavy display face. All marketing headlines, set in 700–800 weight, frequently all-caps. Wide, confident, slightly condensed character that reads as "gaming."
- **ABC Ginto** — the lighter companion for lead paragraphs, links, and buttons (weight 500).
- **ggsans** — Discord's in-product UI sans, used for dense body copy (16px / 400).

**Note on font substitutes:** ABC Ginto Nord and ggsans are proprietary. For an open-source rebuild, pair a heavy geometric grotesque — **Hanken Grotesk** or **Space Grotesk** at 700–800 — for display, with **Inter** or **Plus Jakarta Sans** for body and UI. Keep headlines bold and tracked tight; the loud, confident display weight is the brand's voice.

### Hierarchy

| Token                     | Size | Weight | Line Height | Letter Spacing | Use                                |
| ------------------------- | ---- | ------ | ----------- | -------------- | ---------------------------------- |
| `{typography.display-xl}` | 82px | 800    | 1.0         | 0              | Hero headline (all-caps)           |
| `{typography.display-lg}` | 62px | 800    | 1.05        | 0              | Marquee band, major headline       |
| `{typography.display-md}` | 56px | 700    | 1.05        | 0              | Section headline, CTA band         |
| `{typography.heading-lg}` | 48px | 700    | 1.1         | 0              | Sub-section heading                |
| `{typography.heading-sm}` | 22px | 700    | 1.2         | 0              | Card heading, step label           |
| `{typography.body-lg}`    | 20px | 500    | 1.4         | 0              | Lead paragraph                     |
| `{typography.link-lg}`    | 18px | 500    | 1.4         | 0              | Large button label, prominent link |
| `{typography.body}`       | 16px | 400    | 1.5         | 0              | Default body copy (ggsans)         |
| `{typography.link}`       | 16px | 500    | 1.4         | 0              | Nav link, button label             |
| `{typography.link-sm}`    | 14px | 500    | 1.4         | 0              | Small link, badge, fine print      |

### Principles

- Headlines are short, declarative, and frequently ALL-CAPS in ABC Ginto Nord 800 — the loudest element on every page.
- Body copy drops to the lighter ABC Ginto / ggsans 400–500 so the display type stays the hero.
- The display-to-body weight jump (800 → 400/500) is dramatic on purpose; there is no timid mid-weight in between.

## Layout

### Spacing System

- **Base unit**: 8px.
- **Tokens**: `{spacing.xxs}` 4px · `{spacing.xs}` 8px · `{spacing.sm}` 12px · `{spacing.md}` 16px · `{spacing.lg}` 20px · `{spacing.xl}` 24px · `{spacing.xxl}` 32px · `{spacing.section}` 40px.
- Card interiors run `{spacing.xl}`–`{spacing.section}`; buttons pad `{spacing.sm}`–`{spacing.lg}` vertical by `{spacing.xl}` horizontal.

### Grid & Container

- Centered max-width content column (~1200px) on the full-bleed indigo canvas.
- Feature sections alternate a two-column split (text + media) with stacked full-width gradient/dark cards.
- Marquee, CTA, and showcase bands are full-bleed colour fields with their own rounded inner containers.

### Whitespace Philosophy

Sections breathe through large vertical gaps of indigo, then collide with saturated colour bands for rhythm. Inside cards, generous padding lets oversized 3D art and product mockups float with air.

### Responsive Strategy

#### Breakpoints

| Name    | Width       | Key Changes                                                             |
| ------- | ----------- | ----------------------------------------------------------------------- |
| Mobile  | < 768px     | Single column; nav collapses to logo + hamburger; CTAs stack full-width |
| Tablet  | 768–1023px  | Two-column splits begin stacking; gradient cards go full-width          |
| Laptop  | 1024–1279px | Container narrows; multi-column grids retained                          |
| Desktop | ≥ 1280px    | Full multi-column grids; centered ~1200px column                        |

(Discord ships an unusually dense breakpoint ladder — dozens of stops between 240px and ~2000px — to keep the oversized display type and 3D art balanced at every width.)

#### Touch Targets

`{components.button-primary}` and `{components.button-green}` clear ≥44px tap height via their vertical padding. Nav links and game-rank rows meet the same minimum on mobile.

#### Collapsing Strategy

The dark top nav (logo · links · Login · Download CTA) collapses to logo + hamburger below 768px. Two-column feature rows stack media-over-text; gradient and dark cards span full width. The nitro pricing table becomes horizontally scrollable; the trending-games ranked list keeps its row layout but drops secondary columns.

#### Image Behavior

Product mockups and 3D character art sit inside rounded media frames (`{rounded.lg}`–`{rounded.xl}`) or bleed past card edges as decorative props. Media scales fluidly within its container and keeps its corner radius at every width.

## Elevation & Depth

| Level              | Treatment                                              | Use                                       |
| ------------------ | ------------------------------------------------------ | ----------------------------------------- |
| 0 — Flat           | No shadow; separation by solid colour field + border   | Full-bleed colour bands (hero, marquee)   |
| 1 — Hard offset    | `6px 6px 0px #000000` — solid black, no blur           | Cards, buttons, containers, mockups       |
| 2 — Heavy offset   | `12px 12px 0px #000000` — larger offset for key pieces | Hero mockup frame, demo container         |
| Hover — Compressed | `2px 2px 0px #000000` + `translate(4px, 4px)`          | Interactive hover state on all Level 1/2  |
| Active — Flat      | `0px 0px 0px #000000` + `translate(6px, 6px)`          | Active/pressed state                      |

Depth in this system comes from **hard offset shadows and thick borders**, not blur or transparency. Soft drop shadows, glassmorphism, and gradient glows are not used.

### Decorative Depth

- **Dot-grid patterns**: `radial-gradient(rgba(255,255,255,0.1) 2px, transparent 2px)` at 32px spacing on Onyx content sections.
- **Diagonal stripes**: `repeating-linear-gradient(45deg, transparent, transparent 10px, rgba(0,0,0,0.05) 10px, rgba(0,0,0,0.05) 20px)` overlaid on feature cards and CTA bands.
- **Colour-field contrast**: Full-bleed solid-colour sections (Blurple, Magenta, Yellow, Black) collide to create visual rhythm without shadows.
- **Tilted labels**: Section eyebrow tags use `transform: rotate(-2deg)` with a hard shadow for a playful, hand-placed feel.

## Shapes

### Border Radius Scale

| Token             | Value  | Use                                             |
| ----------------- | ------ | ----------------------------------------------- |
| `{rounded.xs}`    | 6px    | Small ghost buttons, compact chips              |
| `{rounded.sm}`    | 12px   | Primary / green CTA buttons, links, table cells |
| `{rounded.md}`    | 14px   | Game-rank rows, mid controls                    |
| `{rounded.lg}`    | 16px   | White / ghost buttons, cards, media frames      |
| `{rounded.xl}`    | 40px   | Gradient feature panels, large media tiles      |
| `{rounded.pill}`  | 50px   | Pill caps, badges, avatar chips                 |
| `{rounded.jumbo}` | 120px  | Signature oversized rounded shape cards         |
| `{rounded.full}`  | 9999px | Circular avatars and icon buttons               |

### Photography Geometry

Media is presented at soft-cornered rectangles (`{rounded.lg}`–`{rounded.xl}`), never hard-edged. The hero media block uses a directional bottom-only radius (88px bottom corners) for a swooping base. Avatars and circular icon controls are fully round.

## Components

> No hover states documented. Component specs cover Default and Active/Pressed only; variants are separate `components:` entries.

### Buttons

**`button-primary`** — the Blurple pill CTA

- Background `{colors.primary}`, text `{colors.on-primary}`, type `{typography.link-lg}`, rounded `{rounded.sm}`, padding `{spacing.lg} {spacing.xl}`. The everyday action button across hero and feature sections.

**`button-green`** — the electric-green high-intent CTA ("Get Started" / "Download")

- Background `{colors.green}`, text `{colors.ink-dark}`, type `{typography.link-lg}`, rounded `{rounded.sm}`, padding `{spacing.sm} {spacing.xl}`. The highest-visibility action; black label for contrast.

**`button-white`** — white solid button

- Background `{colors.ink}`, text `{colors.ink-dark}`, type `{typography.link}`, rounded `{rounded.lg}`, padding `{spacing.xs} {spacing.md}`.

**`button-ghost`** — translucent indigo button on dark surfaces

- Background `{colors.surface-indigo}`, text `{colors.ink}`, type `{typography.link}`, rounded `{rounded.lg}`, padding `{spacing.md}`.

**`button-ghost-sm`** — compact ghost button (quests CTA row)

- Background `{colors.surface-indigo}`, text `{colors.ink}`, type `{typography.link-sm}`, rounded `{rounded.xs}`, padding `{spacing.sm} {spacing.xxl}`.

### Cards & Containers

**`hero`** — dark-indigo hero

- Indigo `{colors.canvas}` field with the animated gradient mesh, white `{colors.ink}` all-caps headline at `{typography.display-xl}`, lead paragraph, and a CTA pair. The hero media block carries a swooping bottom-only radius.

**`feature-card-gradient`** — vibrant magenta gradient feature panel

- Background `{colors.magenta}` gradient, white text, `{rounded.xl}` (40px), padding `{spacing.section}`. Frames a product mockup or 3D prop.

**`feature-card-dark`** — raised dark feature card

- Background `{colors.surface-indigo}`, white text, `{rounded.xl}`, padding `{spacing.xxl}`. Holds product screenshots / chat mockups.

**`showcase-band-black`** — full-black product showcase band

- Background `{colors.surface-black}`, white text, `{rounded.xl}`, padding `{spacing.section}`. Frames a hero product demo.

**`stat-card`** — big-number stat card (quests)

- Background `{colors.primary}` (Blurple), white text, headline at `{typography.display-md}`, `{rounded.xl}`, padding `{spacing.xxl}`.

**`step-card`** — numbered step panel (1/2/3 process)

- Background `{colors.magenta}` gradient, white text, label at `{typography.heading-sm}`, `{rounded.lg}`, padding `{spacing.xl}`.

**`cta-band`** — full-bleed Blurple CTA band

- Background `{colors.primary}`, white headline at `{typography.display-md}`, `{rounded.xl}`, padding `{spacing.section}`, with a `button-white` or `button-green` CTA.

**`marquee-band`** — scrolling all-caps marquee ("PLAY · CHAT · HANG OUT")

- Background `{colors.primary}`, white display text at `{typography.display-lg}`, padding `{spacing.lg}`.

**`faq-accordion`** — collapsible FAQ row (nitro)

- Background `{colors.surface-indigo}`, white text, question at `{typography.link-lg}`, `{rounded.lg}`, padding `{spacing.xl}`.

### Inputs & Forms

> Discord's marketing pages surface no standalone text inputs; forms route to the app. The kit-mirror `ex-*` form surfaces below model inputs against the brand's `{rounded.lg}` surfaces and `{colors.surface-indigo}` fills.

### Navigation

**`nav-bar`** — dark top navigation

- Indigo `{colors.canvas}` bar, white `{colors.ink}` links at `{typography.link}`, padding `{spacing.md} {spacing.xl}`. Slots: Discord logo · text links · "Login" · a Blurple/green Download CTA. Collapses to logo + hamburger on mobile.

**`footer`** — dark link footer

- Indigo `{colors.canvas}`, white links at `{typography.link}`, padding `{spacing.section}`, organized into multi-column link groups above a giant "Discord" wordmark.

### Signature Components

**`pricing-table`** — nitro plan comparison table

- Raised-indigo `{colors.surface-indigo}` surface, body text at `{typography.body}`, `{rounded.lg}`, padding `{spacing.xl}`. Two plan columns (Nitro Basic / Nitro) with the popular tier carrying a `{colors.magenta}` badge, each row ending in a `button-primary` "Subscribe".

**`game-rank-feature`** — top-ranked game card (trending-games #1/#2/#3)

- Raised-indigo `{colors.surface-indigo}` card, large media, rank number + title at `{typography.heading-sm}`, `{rounded.lg}`, padding `{spacing.md}`.

**`game-rank-row`** — ranked game list row

- Raised-indigo `{colors.surface-indigo}`, body text at `{typography.body}`, `{rounded.md}`, padding `{spacing.sm} {spacing.md}`. Rank · icon · title · metadata columns.

**`badge`** — small rounded tag / category chip

- Background `{colors.magenta}`, white text at `{typography.link-sm}`, `{rounded.lg}`, padding `{spacing.xxs} {spacing.sm}`.

### Examples (illustrative)

> Auto-derived kit-mirror demonstration surfaces (`scripts/derive-examples-block.mjs`). Each `ex-*` entry references brand-native primitives so downstream consumers (`/preview-design`, `/generate-kit`) re-skin the same 10 surfaces consistently. `TO_FILL` markers indicate missing primitives — resolve in the LLM judgment pass.

**`ex-pricing-tier`** — Default Pricing tier card. Re-uses feature-card chrome with brand canvas-soft surface.

- Properties: `backgroundColor`, `textColor`, `borderColor`, `rounded`, `padding`

**`ex-pricing-tier-featured`** — Featured/highlighted tier — polarity-flipped surface (dark fill + light text in light mode, light fill + dark text in dark mode).

- Properties: `backgroundColor`, `textColor`, `rounded`, `padding`

**`ex-product-selector`** — What's Included summary card — re-purposed for SaaS / B2B verticals (NOT a literal product gallery).

- Properties: `backgroundColor`, `rounded`, `padding`

**`ex-cart-drawer`** — Subscription summary — re-purposed for SaaS / B2B (line items per add-on, not literal cart).

- Properties: `backgroundColor`, `rounded`, `padding`, `item-divider`

**`ex-app-shell-row`** — Sidebar nav row inside the App Shell example. Active state uses brand primary as the indicator.

- Properties: `backgroundColor`, `activeIndicator`, `rounded`, `padding`

**`ex-data-table-cell`** — Default data-table th + td chrome. Header uses mono-caps eyebrow typography; body uses body-sm.

- Properties: `headerBackground`, `headerTypography`, `bodyTypography`, `cellPadding`, `rowBorder`

**`ex-auth-form-card`** — Sign-in / sign-up card. Re-uses feature-card chrome with text-input primitives inside.

- Properties: `backgroundColor`, `rounded`, `padding`

**`ex-modal-card`** — Modal dialog surface — same chrome as feature-card with elevated shadow.

- Properties: `backgroundColor`, `rounded`, `padding`

**`ex-empty-state-card`** — Empty-state illustration frame.

- Properties: `backgroundColor`, `rounded`, `padding`, `captionTypography`

**`ex-toast`** — Toast notification surface — feature-card shape + medium shadow.

- Properties: `backgroundColor`, `rounded`, `padding`, `typography`

## Do's and Don'ts

### Do

- Use solid blocks of saturated colour (Blurple, Magenta, Yellow) as section backgrounds — never soft gradient meshes.
- Apply `border: 3px solid #000` and `box-shadow: 6px 6px 0 #000` to every card, button, and container.
- Implement press effects: hover → `translate(4px,4px)` + `2px 2px 0 #000`; active → `translate(6px,6px)` + `0 0 0 #000`.
- Reserve `{colors.green}` for the single highest-intent CTA on a page; use `{colors.primary}` Blurple for everything else.
- Shout with heavy all-caps display type (weight 700–800) and drop hard to body weight (400–500) for copy.
- Use dot-grid patterns and diagonal stripes for texture instead of blurred gradients or noise.
- Make section eyebrow labels with solid-fill tags, slight rotation (`rotate(-2deg)`), and hard shadows.
- Use `-webkit-text-stroke` on giant decorative wordmarks for an outlined, retro-arcade feel.
- Frame product mockups inside hard-bordered containers with heavy offset shadows.

### Don't

- Don't use soft drop shadows, blur filters, glassmorphism, or `backdrop-filter: blur()` — depth is from hard offsets only.
- Don't use gradient fills on backgrounds or text — solid colours only. No radial-gradient meshes or animated glowing orbs.
- Don't use `{colors.green}` as a general accent; it is the high-intent CTA only.
- Don't set headlines in a timid mid-weight — display type is 700–800 or it loses the brand voice.
- Don't remove the thick black borders — they are the signature of the Neo-Brutalist style.
- Don't introduce a fifth accent beyond Blurple, Green, Magenta, and Yellow — those four are the full chord.
- Don't make elements feel "floaty" with subtle opacity shadows — everything should feel physical and mechanical.
