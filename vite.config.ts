import { defineConfig, type Plugin } from "vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";

/* The three entries name their Wasm as `new URL("./name.wasm",
 * import.meta.url)`, the one form Vite, webpack and Rollup read as an asset
 * reference when a consumer bundles the package, so the file is emitted into
 * the consumer's build beside the script. Vite's own library build inlines
 * every asset that form names as a base64 data URL, before assetsInlineLimit
 * is consulted, and its rewrite would carry a `@vite-ignore` that blinds the
 * consumer's Vite. So the literal is marked ignored before Vite's asset pass
 * and unmarked in the emitted chunk: dist holds the literal, and the build
 * scripts put each file beside its entry. */
const WASM_URL = /new URL\(("\.\/[\w-]+\.wasm"), import\.meta\.url\)/g;
const IGNORED = /new URL\(\s*\/\* @vite-ignore \*\/\s*("\.\/[\w-]+\.wasm"),\s*import\.meta\.url\s*\)/g;
function wasmBesideEntry(): Plugin {
  return {
    name: "telperion:wasm-beside-entry",
    apply: "build",
    enforce: "pre",
    transform: code => code.includes(".wasm\"") ? { code: code.replace(WASM_URL, "new URL(/* @vite-ignore */ $1, import.meta.url)"), map: null } : null,
    renderChunk: code => code.includes("@vite-ignore") ? { code: code.replace(IGNORED, "new URL($1, import.meta.url)"), map: null } : null,
  };
}

/* Two builds out of one config.
 *
 * `vite build` emits the LIBRARY: src/index.ts - the generator core, the
 * presets and the loader for the Rust renderer, its two wasm modules placed
 * beside it by the build scripts and fetched at run time, never inlined -
 * and src/field/index.ts, the slim growth-and-field
 * entry a consumer imports without the rest, with the example voxelizer
 * beside it as its own entry. The library has no runtime
 * dependencies, so nothing is left external.
 *
 * `vite` serves the HARNESS: the clay room the trees are judged in.
 * React is a dev dependency and appears nowhere in the library. */
export default defineConfig({
  plugins: [react(), wasmBesideEntry()],
  build: {
    lib: {
      entry: {
        telperion: fileURLToPath(new URL("./src/index.ts", import.meta.url)),
        field: fileURLToPath(new URL("./src/field/index.ts", import.meta.url)),
        voxelize: fileURLToPath(new URL("./src/field/voxelize.ts", import.meta.url)),
      },
      formats: ["es"],
      fileName: (_format, entry) => `${entry}.js`,
    },
    emptyOutDir: false,
    // The one module both entries share, the Wasm loader, under its own name.
    rollupOptions: { output: { chunkFileNames: "[name].js" } },
  },
});
