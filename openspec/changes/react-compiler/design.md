# Design

## Context

The frontend is built with Vite 7.3 and `@vitejs/plugin-react` 4.7 (Babel for Fast Refresh, esbuild for JSX/TS). There is no ESLint setup, so nothing checks the Rules of React today. Hand-written memoization exists only in `pages/Apps.tsx` (`useMemo`) and `components/AppList.tsx` (`useCallback`).

Current upstream state (checked on npm): `react` 19.3.1, `babel-plugin-react-compiler` 1.0.0, `@vitejs/plugin-react` 5.2.0 (peer `vite` ^4–^8, still accepts `babel.plugins`) and 6.3.0 (peer `vite` ^8 only; Babel removed in favor of Oxc, compiler wired through `@rolldown/plugin-babel`).

## Goals / Non-Goals

**Goals:**
- Every component and custom hook in `src/` is compiled by the React Compiler, with zero bailouts.
- Production build and dev server (`tauri dev`) keep working with the same output behavior.

**Non-Goals:**
- Removing existing `useMemo` / `useCallback` calls (the compiler preserves them; removal is churn with no gain).
- Fixing data-flow issues unrelated to the compiler (e.g. `usePresence` being called from both `App` and `Dashboard`).

## Decisions

1. **`@vitejs/plugin-react` 5.2 + `babel.plugins` on Vite 7** instead of Vite 8 + plugin-react 6 + `@rolldown/plugin-babel`.
   The 5.x line keeps the Babel pipeline the compiler plugs into with one config line and needs no bundler migration. The Vite 8 path swaps Rollup/esbuild for Rolldown/Oxc and touches the Tauri build; it deserves its own change.

2. **Compiler runs first in the Babel plugin list, default options.**
   The compiler must see the original source before other transforms. React 19 is the default `target`, so no `react-compiler-runtime` package is needed. `compilationMode` stays `infer` (all components and hooks) rather than the opt-in `annotation` mode, because the code base is small and the goal is app-wide memoization.

3. **`react/compiler-runtime` is listed in `optimizeDeps.include`.**
   The compiler adds this import during the Babel transform, after Vite's esbuild dependency scan, so on a cold cache Vite discovers it on the first request, re-bundles React and forces a full reload (observed in testing; a render before the reload hits a second React copy and throws "Invalid hook call"). Pre-bundling it avoids that.

4. **Bailouts are guarded by a small `compiler:check` script, not ESLint.**
   With the default `panicThreshold` the compiler silently skips a function it cannot compile. `scripts/check-react-compiler.mjs` runs `@babel/core` with the compiler plugin and a `logger` over `src/**/*.{ts,tsx}` and exits non-zero on any `CompileError`. `@babel/core` becomes an explicit dev dependency instead of relying on the copy hoisted from `@vitejs/plugin-react`. Alternative: ESLint with `eslint-plugin-react-hooks` 7 (ships the compiler rules); rejected for now because the project has no ESLint setup.

5. **Rewrite unsupported syntax instead of opting components out.**
   Compiler 1.0 cannot lower `try { … } finally { … }` nor conditional/logical expressions inside a `try` block. The three affected spots (`ProcessPicker` effect, `useAppRules.fetchRules`, `useSettings.fetchSettings` / `updateSettings`) swallow errors in `catch`, so the `finally` body moves after the `try/catch`, and the condition is computed before the `try`. Alternative: `"use no memo"` on those functions; rejected because it gives up memoization for `ProcessPicker` and two hooks used by `App`.

## Risks / Trade-offs

- [Babel now transforms every TSX file in dev, so HMR gets slightly slower] → ~20 small files; the cost is negligible.
- [A future component uses unsupported syntax or breaks a Rule of React and is silently skipped, losing memoization] → `bun run compiler:check`; documented in `.agents/AGENTS.md`.
- [Auto-memoization changes when effects/re-renders happen in code that relied on re-render side effects] → manual check of the running app: presence card ticker, rule search, reorder, settings save.

## Migration Plan

Upgrade dependencies, enable the plugin, build, verify bailouts, smoke-test in `tauri dev`. Rollback: revert `package.json`, both lockfiles and `vite.config.ts`.
