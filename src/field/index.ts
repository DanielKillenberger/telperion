/* The slim entry point: a species id and a seed in, the field's bounds and a
 * batch occupancy query out. It loads the slim Wasm (crates/telperion-field),
 * which carries the direct build's growth, the leaf plan and the field, and
 * none of the wood surface, the leaf placement, the materials, the specimen
 * API or the JSON request path. The same file runs in a browser worker and in
 * Node: a caller passes the module's bytes, a Response or a compiled Module,
 * or leaves the source to `telperion-field.wasm` beside this file, fetched
 * in a browser and read from disk in Node. */
import { wasmSource } from "../wasm-source";

export interface Bounds { min: [number, number, number]; max: [number, number, number] }
/** One batch query's answers, one entry per cell, as the core's field answers
 * them. `flags` bits: wood = 1, foliage = 2. `woodRadius` is the thickest
 * wood sweep reaching the cell, in metres, zero without wood. `leaves`
 * estimates the leaf stations in the cell before the crown cull. `limbs` is
 * the owning limb system, `NO_LIMB` where no foliage reaches. */
export interface FieldAnswer { flags: Uint8Array; woodRadius: Float32Array; leaves: Float32Array; limbs: Uint32Array }
export const NO_LIMB = 0xffffffff;
/** A grown tree's field. `query` takes packed cells, four f64 each: centre
 * x, y, z and the half extent of a closed cube; touching counts and a zero
 * half extent is a point. A batch with a length that is not a multiple of
 * four, a non-finite centre or a negative half extent is refused whole.
 * `release` drops the tree; a released tree refuses every query. */
export interface FieldTree {
  readonly species: string;
  readonly seed: number;
  readonly bounds: Bounds;
  query(cells: Float64Array): FieldAnswer;
  release(): void;
}
export type FieldSource = BufferSource | Response | WebAssembly.Module;
export interface GrowOptions {
  /** The lateral order whose systems name limbs; the family's own when left
   * out. A higher order parts the crown into more and smaller systems. */
  limbOrder?: number;
  /** Where the slim Wasm comes from. Left out, it is fetched from
   * `telperion-field.wasm` beside this module. */
  source?: FieldSource;
}
interface Exports extends WebAssembly.Exports {
  memory: WebAssembly.Memory;
  species_alloc(len: number): number; species_ptr(): number;
  error_ptr(): number; error_len(): number;
  grow(seed: number, limbOrder: number): number; revision(): number;
  bounds_ptr(): number; bounds_len(): number; release(): void;
  query_alloc(count: number): number; query_ptr(): number; query(revision: number): number;
  answer_ptr(slot: number): number; answer_len(slot: number): number;
}
const FAMILY_ORDER = 0xffffffff;
const U32 = 2 ** 32;

let bundled: Promise<WebAssembly.Module> | undefined;
/** Compiles the slim Wasm once; a source given to every `growField` call is
 * compiled each time, so a caller that grows many trees passes a Module. */
export function compileField(source?: FieldSource): Promise<WebAssembly.Module> {
  if (source instanceof WebAssembly.Module) return Promise.resolve(source);
  if (source !== undefined) return compile(source);
  // The literal names the file for a consumer's bundler; the library build
  // leaves it as it is (vite.config.ts) and ships the file beside this one.
  bundled ??= compile(wasmSource(new URL("./telperion-field.wasm", import.meta.url))).catch(error => { bundled = undefined; throw error; });
  return bundled;
}
async function compile(source: BufferSource | Response | Promise<BufferSource | Response>): Promise<WebAssembly.Module> {
  const input = await source;
  if (input instanceof Response) {
    if (!input.ok) throw Error(`Field core load failed: HTTP ${input.status}`);
    return WebAssembly.compile(await input.arrayBuffer());
  }
  return WebAssembly.compile(input);
}
function unsigned(value: number, what: string, limit = U32): number {
  if (!Number.isInteger(value) || value < 0 || value >= limit) throw RangeError(`${what} is an unsigned 32-bit integer`);
  return value;
}
/** Grows `species` at `seed` and answers its field. An unknown species id,
 * or one the catalogue does not serve, is refused with the core's error. */
export async function growField(species: string, seed: number, options: GrowOptions = {}): Promise<FieldTree> {
  unsigned(seed, "seed");
  const order = options.limbOrder === undefined ? FAMILY_ORDER : unsigned(options.limbOrder, "limbOrder", U32 - 1);
  const id = new TextEncoder().encode(species);
  const module = await compileField(options.source);
  const { exports } = await WebAssembly.instantiate(module, {});
  return new Tree(exports as Exports, species, seed, id, order);
}
class Tree implements FieldTree {
  readonly species: string;
  readonly seed: number;
  readonly bounds: Bounds;
  private exports: Exports | undefined;
  private readonly revision: number;
  constructor(e: Exports, species: string, seed: number, id: Uint8Array, order: number) {
    this.species = species;
    this.seed = seed;
    this.exports = e;
    check(e, e.species_alloc(id.length));
    new Uint8Array(e.memory.buffer, e.species_ptr(), id.length).set(id);
    check(e, e.grow(seed, order));
    this.revision = e.revision();
    const bounds = new Float64Array(e.memory.buffer, e.bounds_ptr(), e.bounds_len());
    if (bounds.length !== 6) throw Error("Field core: the field is empty");
    this.bounds = { min: [bounds[0], bounds[1], bounds[2]], max: [bounds[3], bounds[4], bounds[5]] };
  }
  query(cells: Float64Array): FieldAnswer {
    const e = this.exports;
    if (!e) throw Error("Field tree is released");
    if (!(cells instanceof Float64Array) || cells.length % 4) throw Error("Field cells require packed Float64Array x,y,z,halfExtent");
    check(e, e.query_alloc(cells.length / 4));
    new Float64Array(e.memory.buffer, e.query_ptr(), cells.length).set(cells);
    check(e, e.query(this.revision));
    const view = (slot: number) => e.memory.buffer.slice(e.answer_ptr(slot), e.answer_ptr(slot) + e.answer_len(slot) * WIDTH[slot]);
    return { flags: new Uint8Array(view(0)), woodRadius: new Float32Array(view(1)), leaves: new Float32Array(view(2)), limbs: new Uint32Array(view(3)) };
  }
  release(): void {
    this.exports?.release();
    this.exports = undefined;
  }
}
const WIDTH = [1, 4, 4, 4];
function check(e: Exports, code: number): void {
  if (code) throw Error(`Field core: ${new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.error_ptr(), e.error_len()))}`);
}
