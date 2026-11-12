# Proposal

## Why

Every state change in the UI (presence events, the elapsed-time tickers, search input) re-renders the whole subtree below it, and only a few spots are memoized by hand (`useMemo` in `Apps`, `useCallback` in `AppList`). The React Compiler 1.0 is stable and memoizes components and hooks automatically at build time, so we can cut unnecessary re-renders without sprinkling `memo`/`useMemo`/`useCallback` across the code base.

## What Changes

- Upgrade `react` / `react-dom` to 19.3.x and `@types/react` / `@types/react-dom` to 19.3.x.
- Add `babel-plugin-react-compiler` 1.0 and enable it on the project's current toolchain: Vite 8 + `@vitejs/plugin-react` 6 (moved there by a separate dependency bump). plugin-react 6 has no Babel pipeline, so the compiler runs through `@rolldown/plugin-babel` with plugin-react's `reactCompilerPreset()`.
- Pin `@babel/core` to 7.x (plus `@types/babel__core`): compiler 1.0 skips components with destructuring defaults under Babel 8.
- Pre-bundle `react/compiler-runtime` (`optimizeDeps.include`) so a cold `tauri dev` start does not re-optimize React mid-load.
- Add a build guard that fails `vite build` when no module imports `react/compiler-runtime`, so a toolchain change can never silently turn the compiler off again.
- Make sure every component and hook in `src/` is compiled (no compiler bailouts); rewrite the code patterns the compiler cannot compile yet.
- Add `bun run compiler:check` (`scripts/check-react-compiler.mjs`) that fails when the compiler skips any function.
- Keep `bun.lock` and `package-lock.json` in sync.
- Out of scope: the rest of the separate dependency bump (TypeScript 7, Tauri packages, lucide-react, etc.), the experimental native compiler (`compiler: true` / `oxc-transform-react`), and adding ESLint.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. This is a build-tooling and dependency change; no user-visible behavior changes, so the change sets `skip_specs: true`.

## Impact

- `package.json`, `bun.lock`, `package-lock.json`: dependency versions.
- `vite.config.ts`: compiler wiring, build guard and `optimizeDeps`.
- `scripts/check-react-compiler.mjs`, `compiler:check` npm script.
- `src/components/ProcessPicker.tsx`, `src/hooks/useAppRules.ts`, `src/hooks/useSettings.ts`: `try/finally` and in-`try` conditionals rewritten so the compiler can compile them (same behavior).
- No Rust (`src-tauri/`) changes.
