## ADDED Requirements

### Requirement: MIT license
The repository SHALL include a `LICENSE` file at its root containing the standard MIT license text with the copyright line `Copyright (c) 2026 João Camilo Mallmann`.

#### Scenario: License is discoverable
- **WHEN** a visitor opens the repository root
- **THEN** a `LICENSE` file with the MIT text and the copyright holder is present

### Requirement: README open-source positioning
`README.md`, `README.en.md`, and `README.pt.md` SHALL each show an MIT license badge, state the "by coders, for coders" positioning near the top, and include a "Contributing and Open Source" section that links to `CONTRIBUTING.md` and `LICENSE`.

#### Scenario: Positioning near the top
- **WHEN** a visitor reads the top of any README
- **THEN** the MIT badge and the "by coders, for coders" tagline are visible before the feature table

#### Scenario: Contribution entry point
- **WHEN** a visitor looks for how to contribute from any README
- **THEN** a section links to `CONTRIBUTING.md` and `LICENSE` and names the main contribution paths (add an app, add an icon, fix a bug, propose a feature)

### Requirement: Contributor guide
The repository SHALL include `CONTRIBUTING.md` that explains how to build and run the app (`bun install`, `bun run tauri dev`), how to add an app to `apps.json`, how to add an icon through `scripts/icon-sources.json` and `npm run assets:generate` (referencing `docs/Icons.md`), the checks to run before opening a pull request, and the OpenSpec workflow for larger changes.

#### Scenario: Add an app
- **WHEN** a contributor wants to add a supported application
- **THEN** `CONTRIBUTING.md` lists the `apps.json` fields and the steps, including the icon steps, to open a pull request

#### Scenario: Larger change
- **WHEN** a contributor plans a feature, behavior fix, or refactor
- **THEN** `CONTRIBUTING.md` tells them to create an OpenSpec change (`/opsx:propose`) with a `tasks.md` before writing code, and names the trivial edits that may skip it

### Requirement: Agent rules reference the contribution docs
`.agents/AGENTS.md` SHALL point to `CONTRIBUTING.md` and `LICENSE` in its documentation references.

#### Scenario: Agent finds the contributor guide
- **WHEN** an agent reads `.agents/AGENTS.md`
- **THEN** it finds links to `CONTRIBUTING.md` and `LICENSE`
