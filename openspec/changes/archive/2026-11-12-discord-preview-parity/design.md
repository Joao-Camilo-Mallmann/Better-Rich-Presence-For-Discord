# Design

## Context

- The engine (`services/engine.rs`) builds a `PresenceData` from one of four sources: a matching rule, an IDE window title (`TitleParser` + `ActivityBuilder`), the catalog (`format_app_presence`), or the idle/default placeholders. `apply_presence` then sends it to the Discord RPC thread (`services/discord.rs`) and emits it to the frontend.
- All catalog apps use `DEFAULT_CLIENT_ID`. Its Discord application is named "Better Rich Presence" and has no art assets (verified with `GET /api/v9/oauth2/applications/{id}/assets` → `[]` and `/rpc` → `icon: null`).
- `discord-presence` 3.2 already exposes `status_display` (`DisplayType::{Name, State, Details}`) and `activity_type`. The project enables `unstable_name` only to send `name`.
- `rust-clean-architecture` (not started) moves these files into layers. `AGENTS.md` says new code goes into the layer it belongs to. Its planned `domain/presence.rs` owns presence formatting and `discord_asset_key`.
- `reorganize-main-window` (in progress in another session) owns `App.tsx`, `pages/Dashboard.tsx` and the status band. It does not touch `PresenceCard.tsx`, `types.ts` or the backend.

## Goals / Non-Goals

**Goals:**
- The same `PresenceData` drives Discord and the preview card, and the card draws it the way Discord does.
- Every image sent to Discord renders.

**Non-Goals:**
- Changing the bold title in Discord (needs one Discord application per app, which is a portal task, not code).
- Activity types other than Playing (the catalog's `media` category mixes music and video, so a mapping would be a guess).
- "Sharing off" / "not connected" notices (owned by `reorganize-main-window`).

## Decisions

### 1. New pure code goes into `domain/presence.rs`

Add `src-tauri/src/domain/mod.rs` and `domain/presence.rs` now. The file holds only `std` code: the logo URL and default activity-name constants, `discord_image_url` (moved from `engine::get_discord_asset_key`, without its `println!` debug lines), `resolve_large_image`, `activity_name` (the title Discord prints), `rule_details` (the prefix rule), and `ide_activity` (file/workspace sentence). `PresenceData` stays in `models/types.rs` for now: it lives next to `AppState`, which imports `tauri`, and the refactor will move it.

Alternative: put the functions in `services/`. Rejected because it adds code the refactor would then have to move, against the `AGENTS.md` rule.

### 2. One normalization point in `apply_presence`

`apply_presence` is the single path to Discord for every source, including `ManualProfile`. Before the diff check it:
1. resolves `large_image` through `resolve_large_image`, with a lookup closure over the catalog (id or executable, the same rule as React `AppIcon`) and the detected app's icon as the fallback for `""`, `auto` and `default`;
2. sets `activity_name`: `Better Rich Presence` for the default client, else the cached Discord application name, else `large_text`.

`discord.rs` then sends what it gets. It keeps the 2–128 byte limits, drops the URL munging and the `name` call, and adds `status_display(DisplayType::Details)`. The `unstable_name` feature is removed from `Cargo.toml`.

Alternative: resolve images inside each builder. Rejected because four builders plus `ManualProfile` would each need the catalog, and one would eventually be missed.

### 3. Text layout per source

| Source | `details` | `state` | `large_text` |
|---|---|---|---|
| Catalog app | catalog name | category activity (`Browsing GitHub`, `Working`, …) | catalog name |
| Unknown app | effective process name | window title, or `Active` | process name |
| IDE window | IDE display name | `Editing <file> in <workspace>` | IDE display name |
| Rule | rendered template, prefixed `<name> - ` on the shared client when it lacks the name; empty or legacy placeholder → name | rendered template | display name |
| Idle | idle message | `Idle` | `Idle` |
| Startup placeholder | `Browsing the PC` | `Just chilling...` | `Better Rich Presence` |

`format_app_presence` returns the activity line only, and the catalog and unknown-app builders pair it with the app name. The rule and fallback builders are free functions over `AppRegistry`, so they are unit-tested without a Tauri handle. The unknown-app name changes from the window title to the process name: with the window title already in `state`, putting it in `details` too would just repeat it.

For rules, `{file}` keeps its meaning for editors (the parsed file name). For other categories it now resolves to the new activity line. For the generic category, that is the window title instead of `Using <app>`.

### 4. IDE image through the catalog

`IdeDefinition.small_image_asset` becomes `catalog_id: Option<&'static str>` (`vscode`, `cursor`, `windsurf`, `antigravity`; VSCodium has no entry). `ActivityBuilder` puts that id in `large_image` and leaves the small image empty. Decision 2 resolves the id to a URL, or to the logo. `ExtensionMapping` is deleted.

### 5. Engine keeps working while sharing is off

In `apply_presence`, the `global_enabled` check moves after the state update. When off, it updates `current_data` and `current_client_id` and calls `sync_state` (which emits `presence-updated`). It does not send to Discord, does not sleep for the rate limit, and does not touch `last_update_time`. `AppState::update_settings` already sends `current_presence` when sharing turns on, so no change is needed there.

### 6. Diff check ignores only the timestamp

Compare the whole `PresenceData` with both timestamps set to zero, plus the client ID. This replaces the three-field comparison.

### 7. Preview card

`PresenceCard` reads `activity_name` (falling back to `Better Rich Presence`) for the bold title, `details` and `state` for the two lines, and `large_image` (now always a URL) for the icon, with `large_text` as its tooltip. A muted line under the username reads `Playing <details>`, mirroring the member list. The section header changes from "USING AN APPLICATION" to "PLAYING", which is what Discord prints for activity type 0. `types.ts` adds `activity_name?: string`.

## Risks / Trade-offs

- [Discord might start honoring `name` over local RPC] → No impact: the app no longer sends it, so the output stays deterministic and matches the preview.
- [`status_display_type` is new (2025). An older Discord client may ignore it] → It degrades to "Playing Better Rich Presence", today's behavior.
- [Users relying on `{file}` in generic-category rules] → Documented in decision 3. The value changes from `Using <app>` to the window title, which is what `{title}` gives too.
- [jsDelivr outage or a stale cache for the logo] → Same dependency the catalog icons already have.
- [Conflicts with `rust-clean-architecture`] → That change has not started. Its task 7.2 (idle text) becomes partly done, and `domain/presence.rs` already exists for it to extend.

## Migration Plan

No data migration. Saved rules and settings are unchanged. Old frontends ignore the extra `activity_name` field. Rollback is reverting the commit.
