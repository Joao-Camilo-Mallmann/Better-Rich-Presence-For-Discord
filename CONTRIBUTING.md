# Contributing to Better Rich Presence for Discord

Better Rich Presence is open source (MIT), built by coders, for coders, and open to everyone who wants to make it better. This guide gets you from clone to pull request.

## Ways to contribute

- **Add an app** to the catalog (`apps.json`), the quickest way to help. See [Add an app](#add-an-app-to-appsjson).
- **Add or fix an icon.** See [Add an icon](#add-an-icon).
- **Fix a bug** or **improve Linux and Windows detection.**
- **Propose or build a feature.** Larger work goes through OpenSpec. See [OpenSpec workflow](#openspec-workflow-for-bigger-changes).
- **Improve the docs.**

Open an issue first if you are unsure whether a change fits. For a new app or icon, a pull request is enough.

## Build and run

Prerequisites:

- [Bun](https://bun.sh/) (or Node.js 18+)
- [Rust](https://www.rust-lang.org/tools/install)
- **Windows:** [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with **Desktop development with C++**
- **Linux:** the Tauri v2 system libraries (`libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, and others) listed in the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

```bash
git clone https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord.git
cd Better-Rich-Presence-For-Discord
bun install
bun run tauri dev
```

Production builds are described in the [README](README.md#build-from-source). Discord must be running for presence to show up.

## Add an app to `apps.json`

`apps.json` is the app catalog: one object per supported application.

| Field | Meaning |
| --- | --- |
| `id` | Unique lowercase identifier, for example `epicgames`. |
| `name` | Display name shown in the app and in Discord, for example `Epic Games Launcher`. |
| `executables` | Process names to match, lowercase. Include the Windows name (`code.exe`) and the Linux name (`code`). Matching ignores case and a trailing `.exe`. |
| `client_id` | Discord application ID. Reuse the one every other entry uses. |
| `category` | One of the categories already in the file, for example `editor`, `developer`, `browser`, `media`, `communication`, `design`, `productivity`, `database`, `security`, `data`. It decides the presence text. |
| `icon_url` | URL of the PNG in `public/assets/icons/` (optional, but strongly recommended). |

Steps:

1. Copy a similar entry in `apps.json` and edit it. Keep the file's formatting.
2. Add the icon (next section) and point `icon_url` at it. Use the same raw GitHub URL form as the other entries, with the PNG name from step 1 of the icon section.
3. Run the app, open the target application, and check that the Apps tab and the Status Preview show it.
4. Open a pull request.

## Add an icon

Icons are 512x512 PNGs with a transparent background in `public/assets/icons/`, generated from [theSVG](https://thesvg.org) logos. The full pipeline is in [docs/Icons.md](docs/Icons.md); the short version:

1. Find the logo on [thesvg.org](https://thesvg.org) and note its slug (`thesvg.org/icon/<slug>`).
2. Add it to `scripts/icon-sources.json` under `icons`, keyed by the PNG name (lowercase letters and digits):
   ```json
   "epicgames": "epic-games"
   ```
3. Generate it:
   ```bash
   npm run assets:generate -- --only epicgames
   ```
4. Check the PNG on a dark background. If it is hard to see, pin a `variant` in the map and regenerate.
5. Reference it from the `icon_url` of the app entry in `apps.json`.

If theSVG has no logo, add a 512x512 transparent PNG by hand and list its name under `keep` in `scripts/icon-sources.json`.

Logos are trademarks of their owners and are used only to identify the running application. See [docs/Icons.md](docs/Icons.md#4-licenses-and-trademarks). Icons appear in Discord only after the pull request is merged to `main` and the CDN cache refreshes.

## Architecture rules

The full rules live in [.agents/AGENTS.md](.agents/AGENTS.md). The essentials:

- **Rust backend (`src-tauri/src/`)** follows Clean Architecture: `interface` depends on `application`, `application` on `domain`, and `infrastructure` on `application` and `domain`. `domain/` and `application/` never import `tauri`, `windows`, `reqwest`, or other adapters. OS-specific code lives only in `infrastructure/`, behind a port. Domain rules are pure and unit-tested.
- **Frontend (`src/`)** is built with the React Compiler. Follow the Rules of React and do not add `memo`, `useMemo`, or `useCallback` for performance.
- **UI** follows the design tokens in [docs/DESIGN.md](docs/DESIGN.md).
- **English only.** Code, comments, log messages, UI strings, commit messages, and docs are written in English.

## Checks before you open a pull request

```bash
# Rust backend (from src-tauri/)
cargo test
cargo clippy

# Frontend (from the repo root)
npm run build
npm run compiler:check
```

Run the ones that match what you changed. Catalog and icon changes only need the app to start and show the new entry.

## OpenSpec workflow for bigger changes

Features, behavior fixes, refactors, and multi-file changes are planned with [OpenSpec](openspec/) before code is written. Tasks live in the change's `tasks.md`, never only in a chat or an issue.

1. **Propose:** `/opsx:propose` creates `openspec/changes/<name>/` with `proposal.md`, `design.md`, `specs/`, and `tasks.md`. Use `/opsx:explore` first if the idea needs thinking through.
2. **Apply:** `/opsx:apply` implements the tasks. Tick `- [x]` in `tasks.md` as each one is done.
3. **Update:** if the scope changes, run `/opsx:update` before writing more code.
4. **Archive:** `/opsx:archive` merges the delta specs into `openspec/specs/` when all tasks are done.

Every `tasks.md` includes a "Verification" group and ends with a "Documentation" group. Trivial edits (typos, one-line fixes, wording tweaks), catalog entries, and icons can skip OpenSpec. Current capability specs are in [openspec/specs/](openspec/specs/).

## Pull requests

- Keep each pull request focused on one change.
- Describe what changed and why, and how you checked it.
- Write commit messages in English, in the style of the existing history (`feat: ...`, `fix(scope): ...`, `docs: ...`).

## License

By contributing, you agree that your contributions are licensed under the [MIT License](LICENSE).
