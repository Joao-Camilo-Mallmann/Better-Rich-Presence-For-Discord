import babel from "@rolldown/plugin-babel";
import tailwindcss from "@tailwindcss/vite";
import react, { reactCompilerPreset } from "@vitejs/plugin-react";
import { defineConfig, type Plugin } from "vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// Fails `vite build` when no module imports `react/compiler-runtime`, i.e. the
// React Compiler was silently disconnected (plugin-react 6 ignores the old
// `babel` option, so a toolchain bump once turned it off without any error).
function assertReactCompiler(): Plugin {
  let compiled = false;
  return {
    name: "assert-react-compiler",
    apply: "build",
    enforce: "post",
    transform(code, id) {
      if (!compiled && !id.includes("/node_modules/") && code.includes("react/compiler-runtime")) {
        compiled = true;
      }
      return null;
    },
    buildEnd(error) {
      if (!error && !compiled) {
        this.error(
          "React Compiler did not run: no module imports react/compiler-runtime. Check the compiler setup in vite.config.ts.",
        );
      }
    },
  };
}

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    react(),
    // React Compiler (Babel): auto-memoizes components and hooks to skip
    // needless re-renders. plugin-react 6 has no Babel pipeline of its own.
    babel({ presets: [reactCompilerPreset()] }),
    assertReactCompiler(),
    tailwindcss(),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: "index.html",
        landing: "public/LandingPage.html",
      },
    },
  },
}));
