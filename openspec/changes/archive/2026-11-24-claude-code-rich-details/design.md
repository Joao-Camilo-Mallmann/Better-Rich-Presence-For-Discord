# Design: Claude Code Rich Details & Live Chats

## Context

See `proposal.md` for motivation. Currently, Better Rich Presence tracks Claude Code sessions purely from HTTP hook payloads (`session_id`, `hook_event_name`, `cwd`, `tool_name`, `tool_input`, `notification_type`, `agent_type`) and exposes only a single active summary string in `ClaudeCodeStatus`. Claude Code also writes local session transcripts (`transcript_path` JSONL) containing `custom-title`, `summary`, `agent-name`, `last-prompt`, `gitBranch`, and `assistant` `message.model`, and hook payloads include `transcript_path`, `prompt` (on `UserPromptSubmit`), and `permission_mode`.

Clean Architecture boundaries in `src-tauri/src/` require `domain/claude_code.rs` to remain `std` + `serde` only, with filesystem I/O and `serde_json` transcript parsing living in `services/` (`infrastructure` adapter).

## Goals / Non-Goals

**Goals:**
- Enrich each tracked Claude Code session with local chat title, formatted model name, and git branch without blocking the HTTP hook receiver or the engine loop.
- Surface all active Claude Code chats in the Now view with click-to-pin selection and auto-follow fallback.
- Present richer Discord Rich Presence (`Claude Code · <project>`, `<action> · <chat_title>`, and `<model> · <git_branch> · <N> chats` tooltip) while preserving privacy (no raw file paths, shell commands, or multi-line prompts) and Discord's 128-byte UTF-8 limits.
- Support `small_image` / `small_text` badge rendering in `PresenceCard` so the in-app preview matches Discord.

**Non-Goals:**
- Sending prompt text or code snippets to any external service.
- Watching `~/.claude/projects/` continuously with a filesystem watcher (metadata is refreshed on incoming hook events).
- Using `discord_presence` party fields for chat counts (kept in `large_text` tooltip to avoid Discord "in a party" join UI noise).

## Decisions

### 1. Pure Domain Formatting vs. Adapter Metadata Reader
- **Choice**: Keep `ClaudeSessionMetadata`, `format_model_name`, `sanitize_chat_title`, `details_text`, `state_text_rich`, and `large_text` in `src-tauri/src/domain/claude_code.rs` (`std` + `serde` only), and place `parse_transcript_metadata`, `parse_git_head`, and `read_session_metadata` in `src-tauri/src/services/claude_code_metadata.rs`.
- **Rationale**: Preserves the Clean Architecture inward-only dependency rule and keeps domain formatting 100% unit-testable without disk I/O.

### 2. Bounded Head + Tail Transcript Reads via `spawn_blocking`
- **Choice**: In `read_session_metadata`, if `transcript_path` is `<= 192 KiB`, read it in full; otherwise read the first `64 KiB` + `\n` + last `128 KiB` via `std::io::Seek`, skipping partial boundary lines safely in `parse_transcript_metadata`. Run disk reads on `tokio::task::spawn_blocking` from the engine.
- **Rationale**: Long-running Claude Code sessions can produce multi-megabyte JSONL files. The first 64 KiB captures initial `user` prompts if needed, while the last 128 KiB captures the latest `custom-title`, `summary`, `last-prompt`, `gitBranch`, and `assistant` `message.model`. Running in `spawn_blocking` ensures the engine's `tokio::select!` loop never stalls.

### 3. Title Precedence (`has_explicit_title`)
- **Choice**: Track `has_explicit_title: bool` on `ClaudeSession`. Explicit titles (`custom-title`, `summary`, `agent-name`) set `has_explicit_title = true` and overwrite any earlier prompt-derived fallback. Prompt fallbacks (`UserPromptSubmit.prompt` or `last-prompt`) only populate `chat_title` when `!has_explicit_title` and `chat_title.is_none()`.
- **Rationale**: Users who rename a chat (`/rename`) or whose session receives a summary get the clean explicit title immediately without a subsequent prompt overwriting it.

### 4. Session Pinning with Auto-Unpin
- **Choice**: `ClaudeSessionTracker` stores `pinned_session_id: Option<String>`. When set and present in `sessions`, `active_session()` returns it instead of the most recently active session. `SessionEnd`, `prune_expired`, and `clear` automatically clear `pinned_session_id` if the pinned session is removed.
- **Rationale**: Users with multiple concurrent chats can lock Discord onto a primary chat without losing automatic restoration when that chat closes or times out.

## Risks / Trade-offs

- **Stale metadata between hooks**: Metadata is read when a hook event arrives, so a `/rename` done while idle appears on the next hook event. → Acceptable trade-off to avoid polling or extra filesystem watchers.
- **Worktree `.git` files**: Git worktrees use a `.git` file containing `gitdir: <path>` rather than a directory. → `read_session_metadata` resolves both `.git/HEAD` directories and `gitdir:` pointer files.
