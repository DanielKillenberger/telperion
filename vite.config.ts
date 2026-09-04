import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";

/* Two builds out of one config.
 *
 * `vite build` emits the LIBRARY: src/index.ts only, three left
 * external because it is a peer dependency and bundling it would give
 * a consumer two copies of three.js and a very confusing afternoon.
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
    rollupOptions: {
      external: [/^three($|\/)/],
    },
    emptyOutDir: false,
  },
});
