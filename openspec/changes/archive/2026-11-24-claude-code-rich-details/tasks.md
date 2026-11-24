# Tasks

## 1. Domain Model & Rich Formatting

- [x] 1.1 Extend `ClaudeHookPayload`, `ClaudeSession`, and `ClaudeSessionTracker` in `src-tauri/src/domain/claude_code.rs` with `transcript_path`, `prompt`, `permission_mode`, `ClaudeSessionMetadata`, `has_explicit_title`, `pinned_session_id`, `apply_metadata`, `select_session`, `sessions_ordered`, and auto-unpin on end/expiry; verify with unit tests in `domain::claude_code::tests`.
- [x] 1.2 Add `format_model_name`, `sanitize_chat_title`, `action_text`, `details_text`, `state_text_rich`, and `large_text` in `src-tauri/src/domain/claude_code.rs` with UTF-8 boundary truncation; verify with unit tests in `domain::claude_code::tests`.

## 2. Local Metadata Reader Adapter

- [x] 2.1 Create `src-tauri/src/services/claude_code_metadata.rs` (exported in `src-tauri/src/services/mod.rs`) with `parse_transcript_metadata`, `parse_git_head`, and bounded `read_session_metadata`; verify with unit tests for transcript parsing, `.git/HEAD` / worktree `gitdir:` parsing, and bounded temp-file reads.

## 3. Shared Types, Commands & Engine Wiring

- [x] 3.1 Add `ClaudeSessionSummary`, extend `ClaudeCodeStatus`, add `claude_code_show_chat_title` to `Settings`, add `EngineEvent::SelectClaudeCodeSession`, and update `AppState::set_claude_session_snapshot` in `src-tauri/src/models/types.rs`; verify legacy `Settings` deserialization test passes.
- [x] 3.2 Add `select_claude_code_session` in `src-tauri/src/commands/claude_code.rs`, update `disconnect_claude_code`, and register `commands::select_claude_code_session` in `src-tauri/src/lib.rs`.
- [x] 3.3 Update `build_claude_code_presence`, `sync_claude_status`, `ClaudeCodeEvent` metadata enrichment (`spawn_blocking`), and `SelectClaudeCodeSession` handling in `src-tauri/src/services/engine.rs`; verify engine unit tests pass.

## 4. Frontend Types, Preview & Live Chats UI

- [x] 4.1 Update `src/types.ts` and `src/hooks/useClaudeCode.ts` with `ClaudeSessionSummary`, extended `ClaudeCodeStatus`, `claude_code_show_chat_title`, `small_image`/`small_text`, and `selectSession`.
- [x] 4.2 Add `small_image` / `small_text` corner badge rendering to `src/components/PresenceCard.tsx`.
- [x] 4.3 Create `src/components/ClaudeCodeChats.tsx` and render it in `src/pages/Dashboard.tsx` to list active chats with metadata, status indicators, and click-to-pin / Auto-follow controls.
- [x] 4.4 Update `src/components/ClaudeCodeSection.tsx` (chat count in live summary) and `src/pages/Settings.tsx` (`Show chat title on Discord` toggle).

## 5. Verification

- [x] 5.1 Run `cargo test` in `src-tauri/` and verify all tests pass.
- [x] 5.2 Run `npm run compiler:check` and verify 0 skipped functions.
- [x] 5.3 Run `npm run build` and verify TypeScript and Vite production build succeed.

## 6. Documentation

- [x] 6.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise) — N/A.
