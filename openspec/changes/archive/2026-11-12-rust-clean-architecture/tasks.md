# Tasks

## 1. Preconditions

- [ ] 1.1 Confirm `linux-desktop-support` is archived and `main` is clean, and verify `cargo test` in `src-tauri/` passes before any move

## 2. Domain layer

- [ ] 2.1 Create `domain/` (`rule.rs`, `identity.rs`, `presence.rs`, `settings.rs`, `catalog.rs`, `ide/`, `error.rs`) by moving the pure types and functions listed in design decision 1, update imports, and verify `cargo test` passes with the moved tests
- [ ] 2.2 Make `AppCatalog` parse from a `&str` (the `include_str!` stays at the call site in infrastructure), and verify catalog lookup tests pass

## 3. Application layer

- [ ] 3.1 Add `application/ports.rs` (`ActivitySource`, `PresencePublisher`, `DiscordAppDirectory`, `RuleRepository`, `SettingsRepository`, `EventSink`) and the domain `AppEvent` enum, and verify `cargo check`
- [ ] 3.2 Split `AppState` into `application::state::SharedState` plus repositories, and verify rule and settings use-case tests with in-memory repositories (add, update, delete, reorder, reset, merge defaults)
- [ ] 3.3 Move `PresenceEngine` into `application/engine.rs`, generic over its ports, and verify fake-based tests for: rule match, IDE presence, idle switch at threshold, restore on activity, clear on empty window, settle delay, and rate limit
- [ ] 3.4 Move change detection and idle-minute reporting into `application/watcher.rs` over `ActivitySource`, and verify tests with a scripted fake source

## 4. Infrastructure layer

- [ ] 4.1 Move the Discord RPC thread and HTTP lookup into `infrastructure/discord/`, implementing `PresencePublisher` and `DiscordAppDirectory`, and verify `cargo check`
- [ ] 4.2 Move Win32 and Linux detection into `infrastructure/activity/` implementing `ActivitySource`, plus `processes.rs` and `detection_status.rs`, and verify the Linux tests still pass
- [ ] 4.3 Implement `store.rs` repositories on `tauri-plugin-store` and `TauriEventSink` with the existing event names and payloads, and verify `cargo check`

## 5. Interface layer and composition root

- [ ] 5.1 Move Tauri commands into `interface/commands/` as thin wrappers over use cases with unchanged names, arguments and return types, and verify `npm run build` and the running app's Dashboard, Apps and Settings pages
- [ ] 5.2 Move the tray menu into `interface/tray.rs` and reduce `lib.rs` to the composition root, and verify the app starts, the tray works, and `--minimized` still hides the window
- [ ] 5.3 Delete `models/`, `services/` and `commands/`, and verify no file references them (`cargo check`)

## 6. Layering check

- [ ] 6.1 Add `src-tauri/tests/architecture.rs` that fails when `src/domain/` or `src/application/` contain forbidden paths (design decision 6), and verify it passes now and fails when a forbidden import is added temporarily

## 7. English user-facing text

- [ ] 7.1 Change the tray labels to "Open" and "Quit", and verify in the running app
- [ ] 7.2 Change the idle presence state and large text to "Idle" and the default idle message to "Away", keep saved messages, and verify with an engine test and a new-install settings test
- [ ] 7.3 Keep "Usando o computador" recognized as a legacy placeholder next to "Using the computer", and verify with a test

## 8. Verification

- [ ] 8.1 Run `cargo test` in `src-tauri/` and verify all unit, use-case and architecture tests pass
- [ ] 8.2 Run `npm run build` and verify it succeeds
- [ ] 8.3 Manual check on Linux and Windows: presence for a rule app, IDE presence, idle switch and restore, priority mode, rule editing and reordering, settings save, autostart toggle

## 9. Documentation

- [ ] 9.1 Add a short "Backend architecture" section to the READMEs pointing to the layer table in `.agents/AGENTS.md`, and verify the paths it names exist
- [ ] 9.2 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise); remove the "Migration status" note once the migration is complete

## Workflow follow-up

- Archive the change with `/opsx:archive` once all tasks are done.
