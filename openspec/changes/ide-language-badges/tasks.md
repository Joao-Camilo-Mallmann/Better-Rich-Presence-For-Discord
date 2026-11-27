## 1. Pure Domain Editor Parser

- [x] 1.1 Create `src-tauri/src/domain/editor.rs` (`parse_editor_title`, `EditorTitleContext`, `language_badge`) and export `pub mod editor;` in `src-tauri/src/domain/mod.rs`
- [x] 1.2 Add comprehensive unit tests in `src-tauri/src/domain/editor.rs` covering Zed, JetBrains IDEs, Sublime Text, Notepad++, Neovim/Vim, Visual Studio, Eclipse/NetBeans/Code::Blocks/RStudio/Spyder/Atom/Emacs/Jupyter/Xcode, and custom VS Code titles

## 2. VS Code Title Parser & Engine Integration

- [x] 2.1 Update `src-tauri/src/services/title_parser.rs` to detect standalone files (`main.rs - Visual Studio Code`) while keeping two-part workspace titles (`my-app - Cursor`) intact, with unit tests
- [x] 2.2 Update `src-tauri/src/services/engine.rs` with process-gated `extract_editor_context`, `build_editor_presence`, workspace timestamp tracking for all editors, and custom-rule `{file}` + `small_image`/`small_text` language badge attachment in `build_presence_from_rule`
- [x] 2.3 Add unit tests in `src-tauri/src/services/engine.rs` for VS Code/Zed/JetBrains rich presence, custom-rule language badges, workspace-only fallback, and non-editor browser isolation

## 3. Windows Multi-PID Priority Lookup & Frontend Apps Badge

- [x] 3.1 Update `win32::get_priority_window_info` and `EnumData` in `src-tauri/src/services/watcher.rs` to check all PIDs of the prioritized process and prefer the foreground window title when focused
- [x] 3.2 Update `src/components/AppRuleCard.tsx` (`isLiveFor` and preview selection) so custom-named editor rules display the live language corner badge in the Apps list while preserving custom rule preview text

## 4. Verification

- [x] 4.1 Run `bun run compiler:check` and `bun run build`
- [x] 4.2 Run `cargo test --manifest-path src-tauri/Cargo.toml`
- [x] 4.3 Run `openspec validate "ide-language-badges"`

## 5. Documentation

- [x] 5.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise)
