# Proposal

## Why

Every state change in the UI (presence events, the elapsed-time tickers, search input) re-renders the whole subtree below it, and only a few spots are memoized by hand (`useMemo` in `Apps`, `useCallback` in `AppList`). The React Compiler 1.0 is stable and memoizes components and hooks automatically at build time, so we can cut unnecessary re-renders without sprinkling `memo`/`useMemo`/`useCallback` across the code base.

## What Changes

- Upgrade `react` / `react-dom` to 19.3.x and `@types/react` / `@types/react-dom` to 19.3.x.
- Upgrade `@vitejs/plugin-react` from 4.7 to 5.2 (still supports Vite 7 and keeps its Babel pipeline).
- Add `babel-plugin-react-compiler` 1.0 as a dev dependency and enable it through the `babel.plugins` option of `@vitejs/plugin-react` in `vite.config.ts`.
- Pre-bundle `react/compiler-runtime` (`optimizeDeps.include`) so a cold `tauri dev` start does not re-optimize React mid-load.
- Make sure every component and hook in `src/` is compiled (no compiler bailouts); rewrite the code patterns the compiler cannot compile yet.
- Add `bun run compiler:check` (`scripts/check-react-compiler.mjs`, with `@babel/core` as an explicit dev dependency) that fails when the compiler skips any function.
- Keep `bun.lock` and `package-lock.json` in sync with the new versions.
- Out of scope: Vite 8 / `@vitejs/plugin-react` 6 / Rolldown migration, and adding ESLint.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. This is a build-tooling and dependency change; no user-visible behavior changes, so the change sets `skip_specs: true`.

## Impact

- `package.json`, `bun.lock`, `package-lock.json`: dependency versions.
- `vite.config.ts`: React plugin configuration and `optimizeDeps`.
- `scripts/check-react-compiler.mjs`, `compiler:check` npm script.
- `src/components/ProcessPicker.tsx`, `src/hooks/useAppRules.ts`, `src/hooks/useSettings.ts`: `try/finally` and in-`try` conditionals rewritten so the compiler can compile them (same behavior).
- No Rust (`src-tauri/`) changes; does not overlap with the in-flight `linux-desktop-support` change.
