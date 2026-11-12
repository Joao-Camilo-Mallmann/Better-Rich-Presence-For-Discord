# Design

## Context

When this change started, the frontend used Vite 7.3 and `@vitejs/plugin-react` 4.7, and the first implementation enabled the compiler through plugin-react 5.2's `babel.plugins` option. A separate, later dependency bump moved the working tree to Vite 8.3 (Rolldown), `@vitejs/plugin-react` 6.3, `@babel/core` 8.1 and TypeScript 7.0. plugin-react 6 has no Babel pipeline and silently ignores the `babel` option, so after that bump the compiler was off in both `tauri dev` and production builds while every build still passed. The user chose to adapt to Vite 8 instead of reverting the bump.

There is no ESLint setup, so nothing checks the Rules of React today. Hand-written memoization exists only in `pages/Apps.tsx` (`useMemo`) and `components/AppList.tsx` (`useCallback`).

## Goals / Non-Goals

**Goals:**
- Every component and custom hook in `src/` is compiled by the React Compiler, with zero bailouts.
- A build cannot succeed with the compiler silently disabled.
- Production build and dev server (`tauri dev`) keep working with the same output behavior.

**Non-Goals:**
- Removing existing `useMemo` / `useCallback` calls (the compiler preserves them; removal is churn with no gain).
- Fixing data-flow issues unrelated to the compiler (e.g. `usePresence` being called from both `App` and `Dashboard`).
- Owning the rest of the dependency bump (TypeScript 7, Tauri packages, etc.).

## Decisions

1. **Babel React Compiler through `@rolldown/plugin-babel` + `reactCompilerPreset()` on Vite 8 / plugin-react 6.**
   This is the Babel path plugin-react 6 documents; `reactCompilerPreset()` supplies the compiler plugin with the right file filter and ordering. Alternatives: the native `react({ compiler: true })` path via `oxc-transform-react` (rejected: documented as experimental, and the user asked for the Babel compiler); reverting to Vite 7 + plugin-react 5.2's `babel.plugins` (the original implementation; rejected by the user in favor of keeping the bump).

2. **Default compiler options.**
   React 19 is the default `target`, so no `react-compiler-runtime` package is needed. `compilationMode` stays `infer` (all components and hooks) rather than the opt-in `annotation` mode, because the code base is small and the goal is app-wide memoization.

3. **`@babel/core` pinned to 7.x.**
   Under Babel 8, compiler 1.0 rejects destructuring defaults in parameters (`{ size = 24 }`: "Expected object property value to be an LVal, got: AssignmentPattern") and skips `AppIcon`, `PresenceCard` and a `Settings` sub-component. `@rolldown/plugin-babel` accepts `@babel/core` ^7.29, and no newer stable compiler exists yet. `@types/babel__core` is added because Babel 7 ships no types. Revisit when a compiler release supports Babel 8.

4. **`react/compiler-runtime` must be pre-bundled; `reactCompilerPreset()` does it.**
   The compiler adds this import during the transform, after Vite's dependency scan, so on a cold cache Vite would discover it on the first request, re-bundle React and force a full reload (observed on Vite 7; a render before the reload hits a second React copy and throws "Invalid hook call"). The Vite 7 setup needed a manual `optimizeDeps.include`. On Vite 8, `reactCompilerPreset()` registers `optimizeDeps.include: ["react/compiler-runtime"]` itself (verified: a cold start pre-bundles it with no reload), so the manual entry was removed.

5. **Build guard in `vite.config.ts`.**
   A small inline Vite plugin records, during `vite build`, whether any module imports `react/compiler-runtime`, and fails the build at the end if none did. This catches the exact failure that happened here (a plugin option silently ignored after a major bump). `compiler:check` cannot catch it, because it runs Babel directly instead of the Vite pipeline.

6. **Bailouts are guarded by a small `compiler:check` script, not ESLint.**
   With the default `panicThreshold` the compiler silently skips a function it cannot compile. `scripts/check-react-compiler.mjs` runs `@babel/core` with the compiler plugin and a `logger` over `src/**/*.{ts,tsx}` and exits non-zero on any `CompileError`. Alternative: ESLint with `eslint-plugin-react-hooks` 7 (ships the compiler rules); rejected for now because the project has no ESLint setup.

7. **Rewrite unsupported syntax instead of opting components out.**
   Compiler 1.0 cannot lower `try { … } finally { … }` nor conditional/logical expressions inside a `try` block. The three affected spots (`ProcessPicker` effect, `useAppRules.fetchRules`, `useSettings.fetchSettings` / `updateSettings`) swallow errors in `catch`, so the `finally` body moves after the `try/catch`, and the condition is computed before the `try`. Alternative: `"use no memo"` on those functions; rejected because it gives up memoization for `ProcessPicker` and two hooks used by `App`.

## Risks / Trade-offs

- [Babel now transforms every TSX file in dev, so HMR gets slightly slower] → ~20 small files; the cost is negligible.
- [A future component uses unsupported syntax or breaks a Rule of React and is silently skipped, losing memoization] → `bun run compiler:check`; documented in `.agents/AGENTS.md`.
- [A future toolchain bump disconnects the compiler again] → the build guard fails `vite build` / `tauri build`.
- [Someone bumps `@babel/core` to 8 again] → `compiler:check` fails on the destructuring-default components; the pin is explained in `.agents/AGENTS.md`.
- [Auto-memoization changes when effects/re-renders happen in code that relied on re-render side effects] → smoke test of the main flows, then the real-app check.

## Migration Plan

Add the Babel plugin and pin, rewire `vite.config.ts`, add the guard, build, verify bailouts and the dev transform, re-run the compiler on/off comparison, smoke-test in `tauri dev`. Rollback: revert `package.json`, both lockfiles and `vite.config.ts`.
