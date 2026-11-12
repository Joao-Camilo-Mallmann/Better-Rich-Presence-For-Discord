# Project Customization Rules

- **Language Policy**: Always communicate, reply, and interact in English, even when the user writes in another language.
- **Documentation & Code**: Everything that lands in the repository is English only: identifiers, code comments, log messages, user-facing strings (UI, tray menu, Discord presence text), commit messages, OpenSpec artifacts, and markdown documentation (.md). When touching code that still has non-English text, translate it.

## Rust Backend Architecture (Clean Architecture)

`src-tauri/src/` follows Clean Architecture. Dependencies point inward only: `interface` → `application` → `domain`, and `infrastructure` → `application`/`domain`.

| Layer | Contains | May depend on |
|---|---|---|
| `domain/` | Entities and pure rules: `AppRule`, `PresenceData`, `Settings`, rule/catalog matching, window-title parsing, presence formatting | `std`, `serde` only |
| `application/` | Use cases (presence engine, rule and settings management) and the **ports** (traits) they need: window source, idle source, presence publisher, rule/settings repositories | `domain` |
| `infrastructure/` | **Adapters** implementing ports: Discord IPC, Win32 and Linux activity detection, `tauri-plugin-store` repositories, HTTP | `application`, `domain`, external crates |
| `interface/` | Tauri commands and event emission: thin mapping between the frontend and use cases, no business logic | `application`, `domain` |
| `lib.rs` | Composition root: builds adapters and wires them into use cases | everything |

- `domain/` and `application/` never import `tauri`, `windows`, `reqwest`, `discord_presence`, `std::process`, `std::fs`, or any adapter. `application/` may use the async primitives `tokio::sync` and `tokio::time`; spawning tasks and runtime access stay in `infrastructure/` and `lib.rs`.
- The frontend contract (Tauri command names, argument names, payload shapes, event names) is owned by `interface/` and only changes through an OpenSpec change.
- OS-specific code lives only in `infrastructure/`, behind a port, gated with `#[cfg(target_os = "…")]` at module level.
- Parsers for external output (D-Bus replies, CLI output, Win32 strings) are pure functions next to their adapter, unit-tested with captured samples.
- Every domain and application rule has unit tests that run with `cargo test` without Tauri, Discord, or a desktop session.
- Migration status: only `domain/` exists so far (`domain/presence.rs`); the rest of the backend still lives in `models/` / `services/` / `commands/`. The full migration plan was archived without being implemented in `openspec/changes/archive/2026-11-12-rust-clean-architecture/`; resuming it needs a new OpenSpec change. Until the migration is done, new code goes into the layer it belongs to and must not add new inward-pointing violations.

## Frontend (React Compiler)

`src/` is built with the React Compiler (`babel-plugin-react-compiler`), which auto-memoizes every component and hook. On Vite 8 / `@vitejs/plugin-react` 6 it runs through `@rolldown/plugin-babel` with `reactCompilerPreset()` in `vite.config.ts`; plugin-react 6 silently ignores the old `react({ babel })` option.

- An `assert-react-compiler` plugin in `vite.config.ts` fails `vite build` (and `tauri build`) when no module imports `react/compiler-runtime`. If it fires after a dependency bump, fix the compiler wiring; never remove the guard.
- Keep `@babel/core` on 7.x: compiler 1.0 skips components with destructuring defaults (`{ size = 24 }`) under Babel 8. Revisit when a compiler release supports Babel 8.
- Do not add `memo`, `useMemo`, or `useCallback` for performance; the compiler handles it. Existing calls may stay.
- Follow the Rules of React: no mutation of props, state, or values used in render; hooks only at the top level.
- The compiler silently skips any function it cannot compile, which drops its memoization. Compiler 1.0 cannot compile `try { … } finally { … }` or conditional / logical expressions (`?:`, `&&`, `||`, `??`, `?.`) inside a `try` block: move the `finally` body after the `try/catch`, and compute conditions before the `try`.
- Run `bun run compiler:check` after frontend changes; it fails and lists the file and line of any skipped function.

## Planning & Task Workflow (OpenSpec)

All planning in this repository goes through OpenSpec (`openspec/`). Tasks live in the change's `tasks.md` — never only in a chat plan, a TODO list, or an ad-hoc file.

- **Explore** (optional): `/opsx:explore` (skill `openspec-explore`) to think through an idea before committing to a plan.
- **Plan → propose**: any change that needs a plan (feature, behavior fix, refactor, multi-file change) starts with `/opsx:propose` (skill `openspec-propose`), producing `openspec/changes/<name>/` with `proposal.md`, `design.md`, `specs/`, and `tasks.md`.
- **Plan mode**: when an agent plan mode is used (e.g. Claude Code), the approved plan's first step is creating the OpenSpec change; from then on `tasks.md` is the source of truth for the work.
- **Implement → apply**: `/opsx:apply` (skill `openspec-apply-change`); tick `- [x]` in `tasks.md` as each task is completed.
- **Scope changes**: update the artifacts with `/opsx:update` (skill `openspec-update-change`) before writing more code.
- **Finish → archive**: `/opsx:archive` (skill `openspec-archive-change`) once all tasks are done, so delta specs merge into `openspec/specs/`.
- **Keep this file current**: every `tasks.md` ends with a task to update `.agents/AGENTS.md` when conventions, workflows, or reference docs changed (enforced by the `rules.tasks` in `openspec/config.yaml`).
- **Exempt**: trivial edits (typos, one-line fixes, comment or wording tweaks) may skip OpenSpec.

## Project Documentation & Guidelines

Here are the key reference documents for this project:

- [DESIGN.md](../docs/DESIGN.md): Defines the visual design system tokens (colors, typography, spacing, etc.) using a Neo-Brutalist Arcade theme inspired by Discord's gaming-native palette.
- [SettingRichPresence.md](../docs/SettingRichPresence.md): **IMPORTANT** Contains instructions and reference documentation on how to set up and update Discord Rich Presence within the application.
- [Icons.md](../docs/Icons.md): Explains the single-source icon architecture — how `icon_url` in `apps.json` is rendered by the React UI and sent to Discord by the Rust engine — and how icons are generated from theSVG through `scripts/icon-sources.json` (`npm run assets:generate`).
- [openspec/specs/](../openspec/specs/): Current capability specs; archived changes live in `openspec/changes/archive/`.
