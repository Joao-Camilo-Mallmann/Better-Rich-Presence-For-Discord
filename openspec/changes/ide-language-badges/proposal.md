## Why

Currently, adding a custom rule for VS Code (such as `VScode - Coding` / `Working`) causes the presence engine to bypass IDE parsing entirely and drop the programming language mini badge (`small_image` / `small_text`). In addition, Zed and other catalog editors/IDEs (JetBrains IDEs, Visual Studio, Sublime Text, Notepad++, Neovim, Vim, etc.) have no IDE title parser or language badge support, and Windows Max Priority mode only inspects a single PID for multi-process IDEs like `code.exe`, often losing the window title.

## What Changes

- Add a pure domain editor title parser (`domain/editor.rs`) supporting Zed (including its native em-dash `<project> — <file>` title without a `- Zed` suffix) and all catalog editors/IDEs (`category == "editor"` in `apps.json`).
- Refine `TitleParser` for standalone open files in VS Code-family titles (e.g., `main.rs - Visual Studio Code`) while preserving two-part workspace-only behavior (`my-app - Cursor`).
- Gate title-only IDE parsing on non-editor processes so browsers or other non-editor apps whose page titles end in ` - Cursor` or ` - Zed` are never misclassified as IDEs.
- Preserve the detected language mini badge (`small_image` / `small_text`) and `{file}` template value even when an editor/IDE has custom text in its Apps rule, while keeping the user's custom `details` and `state` text.
- Fix Windows Max Priority mode (`win32::get_priority_window_info`) to inspect all PIDs matching the prioritized executable and prefer the foreground window title when focused.
- Update `AppRuleCard` in the frontend so custom-named editor rules also show the live language corner badge in the Apps list without overwriting custom rule preview text.

## Capabilities

### New Capabilities
<!-- None; all changes extend existing IDE and presence-display capabilities -->

### Modified Capabilities
- `vsc-universal-presence`: Extend IDE title parsing and language badge generation to standalone VS Code files, Zed, and all catalog editors (`category == "editor"`), gated by process category.
- `presence-display`: Preserve the detected programming language `small_image` / `small_text` badge and parsed `{file}` substitution for matching editor/IDE rules that carry custom text.

## Impact

- Backend (`src-tauri/`): `src/domain/editor.rs`, `src/domain/mod.rs`, `src/services/title_parser.rs`, `src/services/engine.rs`, `src/services/watcher.rs`.
- Frontend (`src/`): `src/components/AppRuleCard.tsx`.
- Documentation & Specs: `openspec/changes/ide-language-badges/`, `.agents/AGENTS.md`.
