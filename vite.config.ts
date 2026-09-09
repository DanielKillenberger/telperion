import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";

/* Two builds out of one config.
 *
 * `vite build` emits the LIBRARY: src/index.ts only - the generator
 * core, the presets and the loader for the Rust renderer, with both
 * wasm modules as assets beside it. The library has no runtime
 * dependencies, so nothing is left external.
 *
 * `vite` serves the HARNESS: the clay room the trees are judged in.
 * React is a dev dependency and appears nowhere in the library. */
export default defineConfig({
  plugins: [react()],
  build: {
    lib: {
      entry: fileURLToPath(new URL("./src/index.ts", import.meta.url)),
      formats: ["es"],
      fileName: () => "telperion.js",
    },
    emptyOutDir: false,
  },
});
