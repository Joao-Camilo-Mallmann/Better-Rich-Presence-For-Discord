# Project Customization Rules

- **Language Policy**: Always communicate, reply, and interact in English.
- **Documentation & Code**: Write all code comments, commit messages, and markdown documentation files (.md) exclusively in English.

## Frontend (React Compiler)

`src/` is built with the React Compiler (`babel-plugin-react-compiler`, enabled in `vite.config.ts`), which auto-memoizes every component and hook.

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
