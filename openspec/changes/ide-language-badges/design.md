## Context

Better Rich Presence already detects programming languages from filenames (`domain/language.rs`) and renders `small_image` / `small_text` corner badges in both Discord Rich Presence (`services/discord.rs`) and the React UI (`PresenceCard.tsx`, `AppRuleCard.tsx`). However:
1. `TitleParser` only handles VS Code-family titles ending in ` - <IDE>`, and only runs when a matching rule has default text (`rule_has_default_text`).
2. `build_presence_from_rule` always leaves `small_image` and `small_text` empty, and uses generic `format_app_presence` for `{file}` instead of parsed IDE filenames.
3. Zed uses `<project> — <file>` (em-dash without a ` - Zed` suffix by default), and JetBrains/Sublime/Notepad++/Neovim/Visual Studio have distinct title conventions.
4. On Windows, Max Priority mode (`win32::get_priority_window_info`) stores only one PID per matched process name; multi-process Electron IDEs (`code.exe`, `cursor.exe`) frequently pick a background helper PID with no visible window and fall back to `"code.exe"` as the title.
5. In `AppRuleCard.tsx`, `isLiveFor` checks `app.name === presenceName`, which fails when the user renames the rule (e.g., `"VScode"` sets `large_text = "VScode"`).

## Goals / Non-Goals

**Goals:**
- Keep all new editor title parsing pure in `src-tauri/src/domain/editor.rs` (`std` only) following Clean Architecture.
- Support rich presence and language badges for VS Code-family IDEs, Zed, and all `category == "editor"` entries in `apps.json`.
- Keep custom rule text (`details`, `state`, `display_name`) intact while attaching the live language badge and resolving `{file}` from the parsed editor filename.
- Fix Windows multi-PID window title resolution in `watcher.rs` and custom-rule badge matching in `AppRuleCard.tsx`.

**Non-Goals:**
- Adding IDE plugins, extensions, or IPC protocols inside VS Code or Zed; detection remains strictly OS window-title + process-based.
- Changing the `apps.json` schema or regenerating icon assets (all 98 language icons already exist in `public/assets/icons/`).

## Decisions

### 1. Pure Domain Parser `domain/editor.rs`
- Placed in `domain/` with zero Tauri/OS dependencies so all parsing rules are unit-tested via `cargo test`.
- Strips editor suffixes (`" - Zed"`, `" - Microsoft Visual Studio"`, `" - Sublime Text"`, `" - Notepad++"`, `" - NVIM"`, etc.) and leading prefixes (`"Spyder (Python ...) - ..."`).
- Splits remaining text by highest-precedence separator (`" — "`, `" – "`, then `" - "`), strips dirty markers (`●`, `•`, `*`, trailing ` +`), VS Code remote suffixes, and JetBrains `[path]` brackets, and resolves `(file_name, workspace_name, language)` using language detection, path/dirty/extension heuristics, and editor-specific ordering tie-breakers (`zed` and JetBrains IDEs prefer `workspace — file`).

### 2. Process-Gated Editor Detection
- Zed is intentionally **not** added to title-only `IdeRegistry::IDES`, because a browser tab titled `"Article - Zed"` would otherwise be misclassified as the Zed editor.
- `TitleParser::parse` is guarded by `app.is_none_or(|a| a.category == "editor")` (keeping Linux `"electron"` processes working while preventing browsers/non-editors from matching).
- Catalog editor parsing (`parse_editor_title`) only runs when `app.category == "editor"`.

### 3. Custom Rule Language Badges
- `build_presence_from_rule` runs `extract_editor_context` when the matched app is an editor/IDE, populates `small_image` and `small_text` from the detected language (using `language_icon_url` + `lang.name`), and uses the parsed `file_name` for `{file}` template replacement while keeping `details`, `state`, `large_image`, and `large_text` governed by the user's rule.

### 4. Multi-PID Windows Priority Lookup & Frontend Matching
- `win32::get_priority_window_info` collects all PIDs sharing the highest-priority executable name, prefers the foreground window title directly when that process is already focused, and enumerates across all matching PIDs otherwise.
- `AppRuleCard.tsx` matches live presences by either normalized `display_name` (case-insensitive) or normalized catalog executable name, and preserves custom rule subtitle text when custom text is configured.

## Risks / Trade-offs

- **Ambiguous two-part titles without extensions**: A file without an extension (e.g., `README` vs `my-project`) uses editor-specific tie-breakers (`workspace — file` for Zed/JetBrains, `file - workspace` otherwise), which could swap file/workspace if the user customized their editor's `window.title` format to reverse the order without extensions. Acceptable trade-off since any recognized extension or language overrides the tie-breaker.
- **Standalone file vs workspace in VS Code**: `my-app - Cursor` (no recognized language, no dirty marker) must continue to parse as workspace `my-app` with no file, whereas `main.rs - Visual Studio Code` parses as standalone file `main.rs`. Using `detect_language` + dirty marker avoids misclassifying workspace names like `my-app` as files.
