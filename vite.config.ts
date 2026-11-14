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
 * beside it as its own entry, and src/browser/presets.ts, the presets as
 * plain data with no Wasm (fn-317). The library has no runtime
 * dependencies, so nothing is left external.
 *
 * `vite` serves the HARNESS: the clay room the trees are judged in.
 * React is a dev dependency and appears nowhere in the library. */
const RENDER = /\/src\/browser\/(module\.ts|render\/telperion_render\.js)$/;
const CORE = /\/src\/(browser\/(core|wire)\.ts|wasm-source\.ts)$/;
/* The presets compute their constants and touch nothing else; marked so,
 * an entry that never reads them (the field) does not import their chunk
 * for its side effects (fn-366, lever e). */
const PURE = /\/src\/browser\/presets(\.generated)?\.ts$/;
const chunk: [RegExp, string][] = [[RENDER, "module"], [CORE, "core"], [PURE, "presets"]];

export default defineConfig({
  plugins: [react(), wasmBesideEntry()],
  build: {
    lib: {
      entry: {
        telperion: fileURLToPath(new URL("./src/index.ts", import.meta.url)),
        presets: fileURLToPath(new URL("./src/browser/presets.ts", import.meta.url)),
        field: fileURLToPath(new URL("./src/field/index.ts", import.meta.url)),
        voxelize: fileURLToPath(new URL("./src/field/voxelize.ts", import.meta.url)),
      },
      formats: ["es"],
      fileName: (_format, entry) => `${entry}.js`,
    },
    emptyOutDir: false,
    // The renderer's loader and its glue, under the loader's name: only the
    // main entry imports it, so a field page never fetches the renderer's
    // glue (fn-366, lever e). What both entries share, the core's engine, the
    // parameters' serializer and the Wasm loader, is `core.js`; named, so the
    // loader is not drawn into the renderer's chunk with its importer. The
    // presets stay their own entry's, not drawn into `core.js` by its
    // re-export.
    rollupOptions: {
      treeshake: { moduleSideEffects: id => !PURE.test(id) },
      output: { chunkFileNames: "[name].js", manualChunks: id => chunk.find(([re]) => re.test(id))?.[1] },
    },
  },
});
