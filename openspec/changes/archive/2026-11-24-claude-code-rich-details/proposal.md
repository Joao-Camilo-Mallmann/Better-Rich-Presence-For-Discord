# Proposal

## Why

The current Claude Code integration only displays `"Claude Code"` and `"<action> in <project>"` on Discord, and only exposes a single active summary string in Settings. Users running multiple Claude Code chats cannot see or choose which chat is shown, and Discord Rich Presence lacks useful context such as the chat title, active model, git branch, and active chat count.

## What Changes

- Enrich tracked Claude Code sessions with local metadata read from the session's `transcript_path` (chat title, model name, git branch) and `.git/HEAD` fallback, plus `prompt` fallback from `UserPromptSubmit` hook payloads.
- Format Claude Code Discord Rich Presence with richer layout:
  - `details`: `Claude Code · <project>` (or `Claude Code` when no project is known)
  - `state`: `<action> · <chat_title>` (when enabled and a chat title exists, else `<action>`)
  - `large_text`: `<model> · <git_branch> · <N> chats` (omitting missing parts and chat count when `<= 1`)
- Add multi-chat tracking and session pinning (`select_claude_code_session` Tauri command) so users can view all active Claude Code chats in the **Now** view and pin a specific chat or return to auto-follow.
- Add a `claude_code_show_chat_title` setting toggle in Settings (enabled by default) to control whether the chat title is published to Discord.
- Support rendering `small_image` / `small_text` badges in `PresenceCard` for full preview parity with Discord.

## Capabilities

### New Capabilities

(None)

### Modified Capabilities

- `claude-code-integration`: Add local transcript/git metadata enrichment, multi-chat session listing, session pinning, and `claude_code_show_chat_title` configuration.
- `presence-display`: Update Claude Code `details`, `state`, and `large_text` layout and add `small_image`/`small_text` rendering to `PresenceCard`.
- `main-window`: Display active Claude Code chats with pinning controls in the Now view when sessions are live.

## Impact

- Backend (`src-tauri/src/`): `domain/claude_code.rs`, new `services/claude_code_metadata.rs`, `models/types.rs`, `commands/claude_code.rs`, `services/engine.rs`, `lib.rs`.
- Frontend (`src/`): `types.ts`, `hooks/useClaudeCode.ts`, `components/PresenceCard.tsx`, new `components/ClaudeCodeChats.tsx`, `components/ClaudeCodeSection.tsx`, `pages/Dashboard.tsx`, `pages/Settings.tsx`.
- No new Cargo or npm dependencies required.
