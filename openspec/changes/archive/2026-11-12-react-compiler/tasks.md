# Tasks

## 1. Dependencies

- [x] 1.1 Upgrade `react`, `react-dom` to ^19.3.1 and `@types/react`, `@types/react-dom` to ^19.3.0, and verify `node_modules/react/package.json` reports 19.3.x
- [x] 1.2 Upgrade `@vitejs/plugin-react` to ^5.2.0 and add `babel-plugin-react-compiler` ^1.0.0 as dev dependencies, and verify `bun install` reports no peer-dependency warnings for Vite 7 (superseded by group 3 after the move to Vite 8)
- [x] 1.3 Sync `package-lock.json` with `npm install --package-lock-only`, and verify both `bun.lock` and `package-lock.json` resolve `react@19.3.x` and `babel-plugin-react-compiler@1.0.0`

## 2. Compiler setup (Vite 7)

- [x] 2.1 Enable `babel-plugin-react-compiler` as the first Babel plugin of `react()` in `vite.config.ts` and pre-bundle `react/compiler-runtime` via `optimizeDeps.include`, and verify `npm run build` passes, the bundle contains `react.memo_cache_sentinel`, and a cold-cache dev server logs no "optimized dependencies changed. reloading" (superseded by 3.2 after the move to Vite 8)
- [x] 2.2 Run the compiler with a logger over `src/**/*.{ts,tsx}` and verify every component and custom hook reports `CompileSuccess` with no `CompileError`; rewrite `try/finally` and in-`try` conditionals in `ProcessPicker`, `useAppRules`, `useSettings` and re-run until clean
- [x] 2.3 Add `scripts/check-react-compiler.mjs` + `compiler:check` npm script (with `@babel/core` as a dev dependency), and verify it passes on `src/` and exits 1 on a sample component using `try/finally`

## 3. Vite 8 / plugin-react 6 adaptation

- [x] 3.1 Add `@rolldown/plugin-babel` and `@types/babel__core`, pin `@babel/core` to ^7.29.0, and verify `bun install` reports no peer-dependency warnings and `bun run compiler:check` passes again (0 skipped)
- [x] 3.2 Replace the ignored `babel` option with `babel({ presets: [reactCompilerPreset()] })` in `vite.config.ts`, and verify the dev server serves `src/App.tsx` with a `react/compiler-runtime` import and a cold-cache start does not re-optimize mid-load (manual `optimizeDeps.include` dropped: the preset registers it)
- [x] 3.3 Add the build guard plugin to `vite.config.ts`, and verify `vite build` fails with the compiler preset removed and passes with it
- [x] 3.4 Sync `package-lock.json` (`npm install --package-lock-only`), and verify both lockfiles resolve `vite@8.3.x`, `@vitejs/plugin-react@6.3.x`, `@rolldown/plugin-babel` and `@babel/core@7.29.x`

## 4. Verification

- [x] 4.1 `npm run build` (tsc + vite build) passes with no new warnings and the bundle contains `react.memo_cache_sentinel`
- [x] 4.2 Browser smoke test of the dev build with a mocked Tauri IPC, compiler on vs off: no runtime errors; presence timer ticks, backend events update the card, rule search filters, rule reorder / edit save and settings save send the expected commands, Advanced Settings opens; component renders over the same scripted interactions drop clearly with the compiler on (Vite 7 run: 101 → 41; Vite 8 run: 107 → 41)
- [x] 4.3 Check in the real app (`bun run tauri dev`): same flows as 4.2 against the real backend (app starts on the new toolchain, its dev server serves compiled modules, and the user confirmed the flows in the window)

## 5. Documentation

- [x] 5.1 Mention the React Compiler in the stack line of `openspec/config.yaml` context, and verify `openspec status --change react-compiler` still parses the config
- [x] 5.2 Update the `openspec/config.yaml` stack line to the toolchain actually in use (Vite 8), and verify `openspec status --change react-compiler` still parses the config
- [x] 5.3 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise)
