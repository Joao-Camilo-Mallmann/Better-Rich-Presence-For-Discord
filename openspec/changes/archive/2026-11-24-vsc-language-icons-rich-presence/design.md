# Design: VS Code & Derived IDEs Rich Presence + Language Icons

## Context

See `proposal.md` for motivation. `TitleParser`, `IdeRegistry`, and `ActivityBuilder` map VS Code, Cursor, VSCodium, Windsurf, and Antigravity window titles to `PresenceData`. Currently `small_image` and `small_text` are left empty and `prepare_for_discord` only resolves `large_image`.

## Goals / Non-Goals

**Goals:**
- Surface project/workspace name in `details` (`<IDE> · <workspace>`) and file + detected language in `state` (`Editing <file> (<Language>)`).
- Show the IDE icon (e.g. Visual Studio Code) as `large_image` with the IDE name as `large_text`, and the detected language icon as `small_image` with the language name as `small_text`; omit the small image when no language is detected.
- Clean common VS Code title decorations (dirty markers, remote suffixes, Insiders edition, 2-part workspace-only titles).
- Keep language detection pure (`std` only) in `domain/language.rs`.

**Non-Goals:**
- Reading workspace `.git` or LSP state from disk for VS Code (window title parsing remains zero-I/O).

## Decisions

### 1. Language Icons via Existing `theSVG` Pipeline
- **Choice**: Add verified `theSVG` slugs to `scripts/icon-sources.json` and generate 256×256 PNGs with `npm run assets:generate -- --only ...`, served from `cdn.jsdelivr.net` via `language_icon_url`.
- **Rationale**: Reuses the project's single-source dark-background-optimized icon pipeline with zero runtime dependencies.

### 2. Small Image Resolution & Preview Fallback
- **Choice**: Add `resolve_small_image` in `domain/presence.rs` (returning `""` instead of the Better RPC logo when empty or unknown on the shared client) and call it in `engine::prepare_for_discord`. In `AppIcon.tsx`, fall back to `/assets/icons/<file>.png` if a GitHub Raw `/public/assets/icons/*.png` URL fails to load.
- **Rationale**: Prevents an unwanted Better RPC logo badge when no small icon is intended, and lets newly generated local icons render immediately in the in-app preview before being pushed to `main`.

### 3. IDE as the big icon, language as the badge
- **Choice**: `large_image` is always the IDE (the app the user is in); the language is the small badge in the corner.
- **Rationale**: Requested by the owner: the presence should read as "Visual Studio Code" first, with the file's language as a detail. It also keeps the large image stable while switching files, so only the badge changes.

## Risks / Trade-offs

- **Discord CDN propagation**: New PNGs in `public/assets/icons/` are fetched by Discord via `cdn.jsdelivr.net/.../@main/...`, so Discord will render them once pushed to `main`. → `AppIcon.tsx` local fallback ensures the in-app preview works immediately during local development.
