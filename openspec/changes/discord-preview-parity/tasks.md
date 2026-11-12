# Tasks

## 1. Domain presence rules

- [x] 1.1 Add `src-tauri/src/domain/{mod.rs,presence.rs}` with the logo URL and default activity-name constants, `discord_image_url` (moved from `engine::get_discord_asset_key`, no `println!`), `resolve_large_image`, `activity_name`, `rule_details` and `ide_activity`, register `mod domain` in `lib.rs`, and verify with unit tests for every scenario of the "Only images Discord can render", "Rule presences keep the user's text" and IDE layout requirements
- [x] 1.2 Make `format_app_presence` return the activity line only and update its callers, and verify with tests for browser, productivity, editor, media and generic titles

## 2. Presence builders

- [x] 2.1 Rework `ActivityBuilder` and `IdeRegistry` (`catalog_id` replaces `small_image_asset`, no small image), delete `ExtensionMapping`, and verify the IDE tests assert `Visual Studio Code` / `Editing main.rs in my-project` and `large_image` `vscode`
- [x] 2.2 Rework the catalog and unknown-app fallback and the rule builder in `engine.rs` to use the app-first layout and `rule_details` (prefix only for rules on the shared client), and verify with engine unit tests for Firefox, an unknown process and the rule scenarios
- [x] 2.3 Add `activity_name` (serde default) to `PresenceData`, translate the startup placeholder and idle presence to English with the logo URL, and verify with a test on `PresenceData::default()` and the idle builder

## 3. Publishing

- [x] 3.1 In `apply_presence`, normalize `large_image` and `activity_name` for every source, compare all fields except the timestamp, and keep updating and emitting the presence while sharing is off without sending it, and verify with unit tests for the normalization and diff helpers plus `cargo check`
- [x] 3.2 In `discord.rs`, drop the `name` call and the URL munging, set `status_display(DisplayType::Details)`, and remove the `unstable_name` feature from `Cargo.toml`, and verify with `cargo check`

## 4. Preview card

- [x] 4.1 Add `activity_name?: string` to `PresenceData` in `src/types.ts` and update `PresenceCard.tsx` (bold title from `activity_name`, `details` and `state` lines, `large_image` URL with `large_text` tooltip, `Playing <details>` line, "PLAYING" header), and verify with `bun run build` and `bun run compiler:check`

## 5. Verification

- [x] 5.1 Run `cargo test` in `src-tauri/` and verify all tests pass
- [x] 5.2 Run `bun run build` and `bun run compiler:check` and verify both succeed
- [ ] 5.3 Manual check in the running app with Discord open: a catalog app, an unknown app, an IDE window, a rule and idle each show the same title, lines, image and timer in the preview and in the Discord profile popout, and the member list reads "Playing <app>"

## 6. Documentation

- [x] 6.1 Update `docs/Icons.md` where it says apps without `icon_url` use the `"default"` asset key, and verify no doc still names `rust-logo`, `file-icon` or `"default"` as a Discord asset
- [x] 6.2 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise): N/A, the new `domain/presence.rs` follows the existing layer rule and no workflow or reference doc changed

## Workflow follow-up

- Archive the change with `/opsx:archive` once all tasks are done, ideally before `rust-clean-architecture` starts.
