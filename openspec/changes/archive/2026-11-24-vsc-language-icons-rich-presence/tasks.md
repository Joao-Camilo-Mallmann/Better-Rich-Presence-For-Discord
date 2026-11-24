# Tasks

## 1. Icons & Catalog

- [x] 1.1 Add 36 programming language icons and `vscodium` to `scripts/icon-sources.json` (sorted alphabetically) and run `npm run assets:generate -- --only ...` to generate 256×256 PNGs in `public/assets/icons/`.
- [x] 1.2 Add `vscodium` entry to `apps.json` and verify `npm run assets:generate` validation passes.

## 2. Pure Language Detection & Presence Helpers

- [x] 2.1 Create `src-tauri/src/domain/language.rs` (exported in `src-tauri/src/domain/mod.rs`) with `LanguageInfo`, `detect_language`, and `language_icon_url`; verify with unit tests.
- [x] 2.2 Add `resolve_small_image`, `ide_details`, and `ide_state` to `src-tauri/src/domain/presence.rs`; verify with unit tests.

## 3. IDE Parsing, Engine & Discord RPC

- [x] 3.1 Update `src-tauri/src/services/ide_registry.rs` (`VSCodium` catalog ID, `Visual Studio Code - Insiders` matching) and `src-tauri/src/services/title_parser.rs` (dirty markers, remote suffixes, 2-part workspace titles, Insiders); verify with unit tests.
- [x] 3.2 Update `src-tauri/src/services/activity_builder.rs` to set `ide_details`, `ide_state`, IDE `large_image`/`large_text`, and language `small_image`/`small_text`; verify with unit tests.
- [x] 3.3 Update `prepare_for_discord` in `src-tauri/src/services/engine.rs` to resolve `small_image`, update `src-tauri/src/services/discord.rs` to format `small_text` safely, and update `engine::tests::test_ide_presence_resolves_catalog_icon`.

## 4. Frontend Icon Fallback

- [x] 4.1 Update `src/components/AppIcon.tsx` to fall back to `/assets/icons/<file>.png` when a remote `/public/assets/icons/*.png` URL fails to load.

## 5. Verification

- [x] 5.1 Run `cargo test` in `src-tauri/` and verify all tests pass.
- [x] 5.2 Run `npm run compiler:check` and verify 0 skipped functions.
- [x] 5.3 Run `npm run build` and verify TypeScript and Vite production build succeed.

## 6. Documentation

- [x] 6.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise). — N/A.
