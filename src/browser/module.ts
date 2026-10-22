/* The renderer's module, initialised once per page or worker for both
 * entries that load it: the renderer (./render) and the field
 * (../field). wasm-bindgen's init finalises its instance after an await,
 * so two entries each starting it would instantiate twice and leave the
 * first one's handles in a replaced instance (fn-297 R1). */
import init from "./render/telperion_render.js";
import { wasmSource } from "../wasm-source";

/** The module's bytes, a Response, or a compiled Module. */
export type ModuleSource = BufferSource | Response | WebAssembly.Module;

let loading: Promise<void> | undefined;

/** Starts the module once, from `source` or from `telperion-render.wasm`
 * beside this file. The call that starts it names the source; a later call
 * joins that instance whatever it names, since a page holds one module. A
 * refused start is forgotten, so the next call tries again. */
export function loadModule(source?: ModuleSource): Promise<void> {
  // The literal names the file for a consumer's bundler; the library build
  // leaves it as it is (vite.config.ts) and ships the file beside this one.
  loading ??= init({ module_or_path: source ?? wasmSource(new URL("./telperion-render.wasm", import.meta.url)) })
    .then(() => undefined, error => { loading = undefined; throw error; });
  return loading;
}
