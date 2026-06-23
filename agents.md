# Project Rules & Design Preferences

## General Rules
- **Language**: Always interact, write commits, document, and comment in English. All responses, markdown docs, and communications must be strictly in English.
- **Backend Concurrency**: All shared state mutation/access must use the unified `AppState` helper methods to prevent deadlocks.
- **Cross-Platform Compatibility**: Process comparisons must strip the `.exe` extension before matching.

## Document References (Folder `docs/`)
- [structure.md](file:///home/joao/projects/Better-Rich-Presence-For-Discord/docs/structure.md): Detailed 3-layer architecture, file responsibilities, and data-flow diagrams.
- [DESIGN.md](file:///home/joao/projects/Better-Rich-Presence-For-Discord/docs/DESIGN.md): Visual design system specs (fonts, colors, components).
- [rich-presence-best-practices.md](file:///home/joao/projects/Better-Rich-Presence-For-Discord/docs/rich-presence-best-practices.md): Discord's official guidelines on strings, assets, and limits.

## Design System (Discord Dark Client Theme)
- **Colors**:
  - Background Primary: `#313338` (main content)
  - Background Secondary: `#2b2d31` (cards, containers)
  - Background Tertiary/Darkest: `#1e1f22` (navbars, headers, footers)
  - Accent Color (Blurple): `#5865f2` (primary actions, active states)
  - Success Color (Green): `#248046` or `#23a55a` (saves/downloads)
  - Text: `#dbdee1` (primary), `#949ba4` (muted)
- **Visual Features**:
  - Showcase the `/public/image.png` app mockup inside a 3D Floating Mockup with hover tilt.
  - Show CPU/RAM performance charts (Rust vs Electron).
  - Include an interactive JSON code editor for rule editing.
