/* The renderer's module, initialised once per page for every renderer
 * (./render) on it. wasm-bindgen's init finalises its instance after an
 * await, so two renderers each starting it would instantiate twice and
 * leave the first one's handles in a replaced instance (fn-297 R1). */
import init from "./render/telperion_render.js";
import { wasmSource } from "../wasm-source";

let loading: Promise<void> | undefined;

/** Starts the module once, from `telperion-render.wasm` beside this file;
 * a later call joins that instance. A refused start is forgotten, so the
 * next call tries again. */
export function loadModule(): Promise<void> {
  // The literal names the file for a consumer's bundler; the library build
  // leaves it as it is (vite.config.ts) and ships the file beside this one.
  loading ??= init({ module_or_path: wasmSource(new URL("./telperion-render.wasm", import.meta.url)) })
    .then(() => undefined, error => { loading = undefined; throw error; });
  return loading;
}
