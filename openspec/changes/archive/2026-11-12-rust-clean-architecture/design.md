# Design

## Context

See `proposal.md` for motivation; the layer rules are in `.agents/AGENTS.md` ("Rust Backend Architecture"). The only behavior change is in `specs/user-facing-text`.

Current coupling that drives the design:

- `models/types.rs` defines pure entities (`AppRule`, `PresenceData`, `Settings`) next to `AppState`, which holds a `tauri::AppHandle`, the `DiscordHandle`, and writes to `tauri-plugin-store`.
- `services/engine.rs` (`PresenceEngine`) reads `AppState`, emits Tauri events, sends `EngineCommand`s to the Discord thread and calls `reqwest` to look up Discord apps. Its pure helpers (name matching, presence formatting, asset keys) live in the same file.
- `services/watcher.rs` mixes the polling loop and change detection with Win32 calls; Linux detection lives in `services/linux/` (from `linux-desktop-support`).
- `services/discord.rs` owns the RPC thread and also emits Tauri events.
- Commands in `commands/` call `AppState` directly; `commands/system.rs` contains OS-specific process listing.

## Goals / Non-Goals

**Goals:**
- Domain and application code compile and test without Tauri, Discord, a desktop session, or the network.
- Each adapter can be replaced or faked through a port.
- Incremental migration: every step compiles and passes `cargo test`.

**Non-Goals:**
- Changing the frontend contract (command names, arguments, payloads, events).
- Changing persisted data formats.
- Introducing a dependency-injection framework or new crates.

## Decisions

### 1. Target layout

```text
src-tauri/src/
├── domain/
│   ├── rule.rs            AppRule, normalize_process_name, rule_matches
│   ├── identity.rs        resolve_effective_name (rule → catalog → primary)
│   ├── presence.rs        PresenceData, PresenceState, ConnectionInfo, PriorityInfo, DEFAULT_CLIENT_ID,
│   │                      format_presence_for_default_client, discord_asset_key
│   ├── settings.rs        Settings (+ English defaults)
│   ├── catalog.rs         AppEntry, AppCatalog::from_json(&str), format_app_presence
│   ├── ide/               Ide, IdeContext, IdeRegistry, TitleParser, ExtensionMapping, ActivityBuilder
│   └── error.rs           AppError
├── application/
│   ├── ports.rs           ActivitySource, PresencePublisher, DiscordAppDirectory,
│   │                      RuleRepository, SettingsRepository, EventSink
│   ├── state.rs           SharedState (rules, settings, presence, connection) — no Tauri
│   ├── engine.rs          PresenceEngine<P, D, E> (event loop, settle timer, idle, rate limit)
│   ├── watcher.rs         ActivityWatcher<A> (poll, change detection, idle-minute reporting)
│   ├── rules.rs           rule use cases (add, update, delete, reorder, reset, merge defaults)
│   └── settings.rs        settings use cases
├── infrastructure/
│   ├── discord/           RPC thread (implements PresencePublisher), HTTP app lookup (DiscordAppDirectory)
│   ├── activity/          windows.rs (Win32), linux/ (moved as is), mod.rs (platform factory)
│   ├── store.rs           tauri-plugin-store RuleRepository + SettingsRepository
│   ├── events.rs          TauriEventSink (AppHandle::emit)
│   ├── processes.rs       running-process listing per OS
│   └── detection_status.rs
├── interface/
│   ├── commands/          presence.rs, rules.rs, settings.rs, system.rs (thin)
│   └── tray.rs            tray menu (English labels)
└── lib.rs                 composition root: logger, stores, adapters, use cases, spawn tasks
```

`models/presets.rs` (`default_app_rules`) moves to `domain/rule.rs`.

### 2. Ports as traits, generics over trait objects

Ports are plain traits; async methods use `async fn` in traits where needed (stable since Rust 1.75), and synchronous ones stay synchronous (`PresencePublisher::set_activity` only enqueues a command for the RPC thread). Use cases are generic over their ports (`PresenceEngine<P: PresencePublisher, …>`), so tests pass in-memory fakes without boxing.

Alternative considered: `Box<dyn Trait>` everywhere. Rejected because `async fn` in traits is not object-safe without extra crates, and generics keep the hot path static.

### 3. Events through an `EventSink` port

Every `app_handle.emit(...)` call (engine, Discord thread, logger) goes through `EventSink::emit(AppEvent)`, where `AppEvent` is a domain enum (`PresenceUpdated`, `StateChanged`, `ConnectionChanged`, `PriorityInfo`, `DiscordUser`, `Log`). `TauriEventSink` maps each variant to the existing event name and payload, so the frontend contract is unchanged and is defined in one place.

### 4. Shared state without Tauri

`AppState` splits into `application::state::SharedState` (an `Arc<RwLock<…>>` of rules, settings, presence and connection) and the repositories that persist rules and settings. Tauri commands receive use cases through `tauri::State`, not the raw state.

### 5. Activity detection behind one port

`ActivitySource` exposes `poll(&mut self, rules, priority_mode) -> ActivitySnapshot { window: Option<WindowSnapshot>, idle_ms }`. The Win32 code and `LinuxDetector` become its two implementations; change detection and idle-minute reporting move into `application::watcher`, so they are unit-tested once for both OSes.

### 6. Enforced layering check

A test in `src-tauri/tests/architecture.rs` walks `src/domain/` and `src/application/` and fails when a file contains a forbidden path (`tauri::`, `windows::`, `reqwest`, `discord_presence`, `std::process`, `std::fs`, `crate::infrastructure`, `crate::interface`). It runs with `cargo test`, so violations fail locally and in CI.

Alternative considered: separate workspace crates per layer, which the compiler would enforce. Rejected for now: it complicates the Tauri build (`crate-type`, `generate_context!` paths) for little gain at this size; it can follow later.

### 7. Migration order

Each step lands with `cargo check` and `cargo test` green:

1. `domain/` (pure moves; old paths updated in place).
2. `application/ports.rs` and `SharedState`; adapters implement the ports in their current files.
3. Move the engine and watcher into `application/`, generic over the ports, with fake-based tests.
4. Move adapters into `infrastructure/`.
5. Move commands and tray into `interface/`; `lib.rs` becomes the composition root.
6. Delete `models/`, `services/`, `commands/`; add the layering test.
7. Translate user-facing text (spec `user-facing-text`).

## Risks / Trade-offs

- [Large diff conflicts with in-flight work] → Start only after `linux-desktop-support` is archived; land one migration step per commit.
- [Silent frontend contract drift] → `TauriEventSink` and the command signatures are the only contract surface; the verification group exercises every page of the running app and `npm run build`.
- [Generic use cases make type signatures noisy] → Type aliases for the production wiring in `lib.rs`.
- [The text-based layering test can miss indirect imports] → It catches the common case; switching to per-layer crates is the stronger follow-up if needed.

## Migration Plan

No data migration. Rollback is reverting the change's commits; each migration step is independently revertible.
