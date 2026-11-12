# Tasks

## 1. Dependencies

- [x] 1.1 Upgrade `react`, `react-dom` to ^19.3.1 and `@types/react`, `@types/react-dom` to ^19.3.0, and verify `node_modules/react/package.json` reports 19.3.x
- [x] 1.2 Upgrade `@vitejs/plugin-react` to ^5.2.0 and add `babel-plugin-react-compiler` ^1.0.0 as dev dependencies, and verify `bun install` reports no peer-dependency warnings for Vite 7
- [x] 1.3 Sync `package-lock.json` with `npm install --package-lock-only`, and verify both `bun.lock` and `package-lock.json` resolve `react@19.3.x` and `babel-plugin-react-compiler@1.0.0`

## 2. Compiler setup

- [x] 2.1 Enable `babel-plugin-react-compiler` as the first Babel plugin of `react()` in `vite.config.ts` and pre-bundle `react/compiler-runtime` via `optimizeDeps.include`, and verify `npm run build` passes, the bundle contains `react.memo_cache_sentinel`, and a cold-cache dev server logs no "optimized dependencies changed. reloading"
- [x] 2.2 Run the compiler with a logger over `src/**/*.{ts,tsx}` and verify every component and custom hook reports `CompileSuccess` with no `CompileError`; rewrite `try/finally` and in-`try` conditionals in `ProcessPicker`, `useAppRules`, `useSettings` and re-run until clean
- [x] 2.3 Add `scripts/check-react-compiler.mjs` + `compiler:check` npm script (with `@babel/core` as a dev dependency), and verify it passes on `src/` and exits 1 on a sample component using `try/finally`

## 3. Verification

- [x] 3.1 `npm run build` (tsc + vite build) passes with no new warnings
- [x] 3.2 Browser smoke test of the dev build with a mocked Tauri IPC, compiler on vs off: no runtime errors; presence timer ticks, backend events update the card, rule search filters, rule reorder / edit save and settings save send the expected commands, Advanced Settings opens; component renders over the same scripted interactions drop from 101 to 41
- [ ] 3.3 Check in the real app (`bun run tauri dev`) once the in-flight `linux-desktop-support` Rust work compiles: same flows as 3.2 against the real backend

## 4. Documentation

- [x] 4.1 Mention the React Compiler in the stack line of `openspec/config.yaml` context, and verify `openspec status --change react-compiler` still parses the config
- [x] 4.2 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise)
