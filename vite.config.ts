import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";

/* Two builds out of one config.
 *
 * `vite build` emits the LIBRARY: src/index.ts - the generator core, the
 * presets and the loader for the Rust renderer, with both wasm modules as
 * assets beside it - and src/field/index.ts, the slim growth-and-field
 * entry a consumer imports without the rest, with the example voxelizer
 * beside it as its own entry. The library has no runtime
 * dependencies, so nothing is left external.
 *
 * `vite` serves the HARNESS: the clay room the trees are judged in.
 * React is a dev dependency and appears nowhere in the library. */
export default defineConfig({
  plugins: [react()],
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
  },
});
