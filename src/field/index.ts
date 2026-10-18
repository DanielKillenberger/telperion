/* The field entry point: a species id, a seed and a grid side in, the
 * tree's field read out to that grid (./voxelize `Grid`) out. The tree grows
 * on the device (WebGPU), through the renderer's module
 * (`telperion-render.wasm`), as the harness grows it, and its field is read
 * out there: only the grid crosses back, and nothing stays on the device
 * (fn-257). Without WebGPU every call rejects by name; there is no CPU
 * fallback. The same file runs in a browser page or worker; in Node, which
 * has no WebGPU, it rejects. A caller passes the module's bytes, a Response
 * or a compiled Module, or leaves the source to `telperion-render.wasm`
 * beside this file.
 *
 * Migrating from 0.1.4's slim entry: `growField` takes the grid's side and
 * resolves to the grid with its answers; the cube query (`query(cells)`),
 * `bounds` and `release()` are gone, and nothing replaces the query (a
 * coarser or finer reading is another side). The `cut` option is gone, and
 * nothing replaces it (every tree grows whole, on the device); the source is
 * the renderer's module, not `telperion-field.wasm`. */
import init, { WebField } from "../browser/render/telperion_render.js";
import { wasmSource } from "../wasm-source";
import { side, type Grid } from "./voxelize";

export type { Grid } from "./voxelize";
export interface Bounds { min: [number, number, number]; max: [number, number, number] }
/** One answer per cell, in the grid's cell order. `flags` bits: wood = 1,
 * foliage = 2. `woodRadius` is the thickest wood sweep reaching the cell, in
 * metres, zero without wood. `leaves` estimates the leaf stations in the cell
 * before the crown cull. `limbs` is the owning limb system, `NO_LIMB` where
 * no foliage reaches. */
export interface FieldAnswer { flags: Uint8Array; woodRadius: Float32Array; leaves: Float32Array; limbs: Uint32Array }
export const NO_LIMB = 0xffffffff;
/** A field read out: its grid and one answer per cell. */
export interface FieldGrid extends Grid { answer: FieldAnswer }
/** A grown tree's field, read out to the grid its call named over the tree's
 * own box. */
export interface FieldTree extends FieldGrid { readonly species: string; readonly seed: number }
export type FieldSource = BufferSource | Response | WebAssembly.Module;
export interface GrowOptions {
  /** The lateral order whose systems name limbs; the family's own when left
   * out. A higher order parts the crown into more and smaller systems. */
  limbOrder?: number;
  /** Where the renderer's module comes from. Left out, it is fetched from
   * `telperion-render.wasm` beside this module. */
  source?: FieldSource;
}

const U32 = 2 ** 32;

let loading: Promise<void> | undefined;
/** Loads the renderer's module once per page or worker. */
function load(source?: FieldSource): Promise<void> {
  // The literal names the file for a consumer's bundler; the library build
  // leaves it as it is (vite.config.ts) and ships the file beside this one.
  loading ??= init({ module_or_path: source ?? wasmSource(new URL("./telperion-render.wasm", import.meta.url)) })
    .then(() => undefined, error => { loading = undefined; throw error; });
  return loading;
}

/** Whether this page or worker has WebGPU: the field grows nowhere else. */
function device(): void {
  const gpu = (globalThis.navigator as { gpu?: unknown } | undefined)?.gpu;
  if (gpu === undefined) throw Error("WebGPU is not available: telperion/field grows its trees on the device");
}

/** Compiles the renderer's module once, for a caller that grows trees in
 * several workers and passes the Module to each. */
export async function compileField(source?: FieldSource): Promise<WebAssembly.Module> {
  if (source instanceof WebAssembly.Module) return source;
  const input = await (source ?? wasmSource(new URL("./telperion-render.wasm", import.meta.url)));
  if (input instanceof Response) {
    if (!input.ok) throw Error(`Field core load failed: HTTP ${input.status}`);
    return WebAssembly.compile(await input.arrayBuffer());
  }
  return WebAssembly.compile(input);
}

/** The device and its programs made ready before the first tree, so the
 * first `growField` does not wait on them: a page calls it as it loads. */
export async function prepareField(source?: FieldSource): Promise<void> {
  device();
  await load(source);
  await WebField.prepare();
}

function unsigned(value: number, what: string, limit = U32): number {
  if (!Number.isInteger(value) || value < 0 || value >= limit) throw RangeError(`${what} is an unsigned 32-bit integer`);
  return value;
}

/** Grows `species` at `seed` on the device and reads its field out to `n`
 * cells a side over the tree's box. A side outside 3 to 256 is refused by
 * name before any work, a grid past the device's limits by name before
 * anything is allocated; an unknown species id, or one the catalogue does
 * not serve, with the core's error; a page or worker without WebGPU, by
 * name. */
export async function growField(species: string, seed: number, n: number, options: GrowOptions = {}): Promise<FieldTree> {
  unsigned(seed, "seed");
  side(n);
  const order = options.limbOrder === undefined ? undefined : unsigned(options.limbOrder, "limbOrder", U32 - 1);
  device();
  await load(options.source);
  const field = (await WebField.grow(species, seed, n, order)) as WebField;
  try {
    const o = field.origin();
    return { species, seed, n: field.n(), cell: field.cell(), origin: [o[0], o[1], o[2]], answer: answers(field.answers()) };
  } finally {
    field.free();
  }
}

/** The binding's four words a cell, in cell order, as one answer per cell. */
function answers(words: Uint32Array): FieldAnswer {
  const n = words.length / 4;
  const answer = { flags: new Uint8Array(n), woodRadius: new Float32Array(n), leaves: new Float32Array(n), limbs: new Uint32Array(n) };
  const floats = new Float32Array(words.buffer, words.byteOffset, words.length);
  for (let i = 0; i < n; i++) {
    answer.flags[i] = words[4 * i];
    answer.woodRadius[i] = floats[4 * i + 1];
    answer.leaves[i] = floats[4 * i + 2];
    answer.limbs[i] = words[4 * i + 3];
  }
  return answer;
}
