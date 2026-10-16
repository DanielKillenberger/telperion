/* The field entry point: a species id and a seed in, the field's bounds and a
 * batch occupancy query out. The tree grows on the device (WebGPU), through
 * the renderer's module (`telperion-render.wasm`), as the harness grows it,
 * and its field stays there: a query sends its cells and reads back only
 * their answers (fn-257). Without WebGPU every call rejects by name; there is
 * no CPU fallback. The same file runs in a browser page or worker; in Node,
 * which has no WebGPU, it rejects. A caller passes the module's bytes, a
 * Response or a compiled Module, or leaves the source to
 * `telperion-render.wasm` beside this file.
 *
 * Migrating from 0.1.4's slim entry: `query` resolves a promise now; the
 * `cut` option is gone, and nothing replaces it (every tree grows whole, on
 * the device); the source is the renderer's module, not
 * `telperion-field.wasm`. */
import init, { WebField } from "../browser/render/telperion_render.js";
import { wasmSource } from "../wasm-source";

export interface Bounds { min: [number, number, number]; max: [number, number, number] }
/** One batch query's answers, one entry per cell, as the field answers
 * them. `flags` bits: wood = 1, foliage = 2. `woodRadius` is the thickest
 * wood sweep reaching the cell, in metres, zero without wood. `leaves`
 * estimates the leaf stations in the cell before the crown cull. `limbs` is
 * the owning limb system, `NO_LIMB` where no foliage reaches. */
export interface FieldAnswer { flags: Uint8Array; woodRadius: Float32Array; leaves: Float32Array; limbs: Uint32Array }
export const NO_LIMB = 0xffffffff;
/** A grown tree's field. `query` takes packed cells, four numbers each:
 * centre x, y, z and the half extent of a closed cube; touching counts and
 * a zero half extent is a point. A batch with a length that is not a
 * multiple of four, a non-finite centre or a negative half extent is
 * refused whole. `release` frees the tree on the device; a released tree
 * refuses every query. */
export interface FieldTree {
  readonly species: string;
  readonly seed: number;
  readonly bounds: Bounds;
  query(cells: Float64Array): Promise<FieldAnswer>;
  release(): void;
}
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

/** Grows `species` at `seed` on the device and keeps its field there. An
 * unknown species id, or one the catalogue does not serve, is refused with
 * the core's error; a page or worker without WebGPU, by name. */
export async function growField(species: string, seed: number, options: GrowOptions = {}): Promise<FieldTree> {
  unsigned(seed, "seed");
  const order = options.limbOrder === undefined ? undefined : unsigned(options.limbOrder, "limbOrder", U32 - 1);
  device();
  await load(options.source);
  const field = (await WebField.grow(species, seed, order)) as WebField;
  return new Tree(field, species, seed);
}

class Tree implements FieldTree {
  readonly species: string;
  readonly seed: number;
  readonly bounds: Bounds;
  private field: WebField | undefined;
  constructor(field: WebField, species: string, seed: number) {
    this.species = species;
    this.seed = seed;
    this.field = field;
    const b = field.bounds();
    this.bounds = { min: [b[0], b[1], b[2]], max: [b[3], b[4], b[5]] };
  }
  async query(cells: Float64Array): Promise<FieldAnswer> {
    const field = this.field;
    if (!field) throw Error("Field tree is released");
    if (!(cells instanceof Float64Array) || cells.length % 4) throw Error("Field cells require packed Float64Array x,y,z,halfExtent");
    const words = (await field.query(new Float32Array(cells))) as Uint32Array;
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
  release(): void {
    this.field?.release();
    this.field?.free();
    this.field = undefined;
  }
}
