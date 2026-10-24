import type { LeafReference } from "./leaf";
import { CATALOGUE, type Family } from "./presets.generated";
import { wasmSource } from "../wasm-source";
export type { Family } from "./presets.generated";

export interface TreePreset extends Family { id: string; name: string; note: string }
/** Explicit identity lookup; unknown identities never fall back to Ordinary. */
export function presetById(id: string): TreePreset {
  const entry = CATALOGUE.find(preset => preset.id === id);
  if (!entry) throw Error(`Unknown tree preset: ${id}`);
  return { ...structuredClone(entry.family), id: entry.id, name: entry.name, note: entry.note };
}
/** Plain parameter baseline, usable directly at the native request boundary. */
export const ORDINARY: Family = (({ id: _id, name: _name, note: _note, ...family }) => family)(presetById("ordinary"));
export const OREGON_WHITE_OAK = presetById("oregon-white-oak");
export const NORWAY_SPRUCE = presetById("norway-spruce");
export const SILVER_BIRCH = presetById("silver-birch");
export const TELPERION = presetById("telperion");
export const LAURELIN = presetById("laurelin");
export const TWO_TREES = [TELPERION, LAURELIN];
export const PRESETS = CATALOGUE.map(entry => presetById(entry.id));
export interface Outputs {
  surface?: boolean; foliage?: boolean; structure?: boolean;
  /** The drawn tree's field read out to a grid of `n` cells a side over its
   * box (`telperion/field/voxelize`'s `Grid`, a whole number from 3 to 256),
   * from the solved tree and the leaf plan: no leaf is placed for it, and a
   * family the plan cannot describe is refused by name. No render buffers
   * transfer unless surface/foliage are requested separately. Limbs are
   * named at the family's `clumpSystemOrder`, or at `limbOrder`, a higher
   * one parting the crown into more and smaller systems. */
  field?: { n: number; limbOrder?: number };
}
export interface Bounds { min: [number, number, number]; max: [number, number, number] }
export interface Timings { growthMs: number; surfaceMs: number; planMs: number; foliageMs: number; fieldMs: number; coreMs: number; transferMs: number; buildMs: number }
/** Which stages a build ran. `foliage` is the placement and the cull together;
 * `contacts` the wood's contact surface under placement; `plan` the leaf plan
 * a field request read. */
export interface Stages {
  surface: boolean; foliage: boolean; placement: boolean; cull: boolean; contacts: boolean; plan: boolean; field: boolean;
}
/** Half-open prototype ranges exclude connectors; indices are scalar offsets. */
export interface FoliageAnatomy {
  unit: "leaf" | "needle"; vertices: [number, number]; indices: [number, number]; sections: [number, number][];
}
export interface Diagnostics {
  biologicalUnits: number | null; foliageAnatomy: FoliageAnatomy | null;
  nodes: number; crossover: number; capped: boolean; levelCapped: boolean;
  attractionCapped: boolean; complete: boolean;
  /** Twig detail kept, in sixteenths of a generation, where the node budget reduced the tree. */
  twigDetail: number | null;
  leavesPlaced: number; instances: number;
  /** Stations the leaf plan counts before any cull; zero without a plan. */
  leavesPlanned: number;
  surfaceBounds: Bounds | null; foliageBounds: Bounds | null;
  /** Chunks the wood is written in; zero without a surface. */
  surfaceChunks: number;
  /** The box every packed leaf position is quantised against. */
  foliageReference: LeafReference | null;
  /** The box every packed floret position is quantised against; null where
   * the tree grew no flowers. */
  organReference: LeafReference | null;
  /** The grid a field request was read out to; null without one. */
  field: FieldFrame | null;
  revision: number; timings: Timings;
  stages: Stages;
}
/** The grid a field is read to: `n` cells a side of `cell` metres from
 * `origin`, cell `i + n * (j + n * k)` centred at `origin + (i + 0.5, j +
 * 0.5, k + 0.5) * cell`. */
export interface FieldFrame { n: number; cell: number; origin: [number, number, number] }
/** One answer per cell, in the grid's cell order. `flags` bits: wood=1,
 * foliage=2, flowers=4. `woodRadius` is the thickest wood sweep reaching the
 * cell in metres, zero without wood. `leaves` estimates the stations in the
 * cell before the crown cull. `limbs` is the owning limb system, UINT32_MAX
 * where no foliage reaches. `flowers` counts the florets whose seat lies in
 * the cell, at the flowering window's middle. */
export interface FieldQuery { flags: Uint8Array; woodRadius: Float32Array; leaves: Float32Array; limbs: Uint32Array; flowers: Float32Array }
/** One chunk of the wood: at most 65,536 vertices, its indices local to it.
 * `positions` holds three f32 a vertex; `normals` two snorm16 a vertex, the
 * unit normal folded onto the octahedron (`decodeNormals` in `./wood`). */
export interface SurfaceChunk { positions: Float32Array; normals: Int16Array; indices: Uint16Array }
export interface TreeOutput {
  /** The wood in chunks, each drawn on its own with 16-bit indices. */
  surface?: { chunks: SurfaceChunk[]; bounds: Bounds | null };
  /** Three u32 words a leaf, decoded against `reference` by `./leaf`. The
   * sixteen-float form is never built here: twelve bytes are the storage. */
  foliage?: { positions: Float32Array; indices: Uint32Array; leaves: Uint32Array; reference: LeafReference | null; anatomy: FoliageAnatomy | null; bounds: Bounds | null };
  /** The flowers, a second foliage beside the leaves: one flower mesh and
   * three Uint32 words a floret, decoded against `reference` by `./leaf`.
   * Present only where foliage was asked for and the tree flowers. */
  organ?: { positions: Float32Array; indices: Uint32Array; leaves: Uint32Array; reference: LeafReference };
  /** Six f64 values per node: xyz, distal radius, proximal radius, base radius.
   * Three u32 values per node: parent (UINT32_MAX for root), branch, kind (0/1/2). */
  structure?: { values: Float64Array; topology: Uint32Array };
  /** The field read out to the requested grid, as `telperion/field`'s
   * `growField` returns it: owned copies that outlive the build. */
  field?: FieldFrame & { answer: FieldQuery };
  diagnostics: Diagnostics;
}
// An address the core returns is an i32: one past 2 GiB of its memory
// reads negative, so each is read unsigned (`>>> 0`).
interface Exports extends WebAssembly.Exports {
  memory: WebAssembly.Memory;
  request_alloc(n: number): number; request_ptr(): number;
  metadata_ptr(): number; metadata_len(): number; build(): number; release(): void;
  buffer_ptr(slot: number): number; buffer_len(slot: number): number;
  chunk_ptr(chunk: number, field: number): number; chunk_len(chunk: number, field: number): number;
}
export class TreeEngine {
  private exports: Exports | undefined;
  private get e(): Exports {
    if (!this.exports) throw Error("Tree engine is disposed");
    return this.exports;
  }
  get disposed(): boolean { return this.exports === undefined; }
  private constructor(instance: WebAssembly.Instance) { this.exports = instance.exports as Exports; }
  /** Drop the instance, allowing its linear memory to be reclaimed by the host. */
  dispose(): void { this.exports?.release(); this.exports = undefined; }
  static async create(source?: BufferSource | Response): Promise<TreeEngine> {
    // The literal names the file for a consumer's bundler; the library build
    // leaves it as it is (vite.config.ts) and ships the file beside this one.
    const input = source ?? await wasmSource(new URL("./telperion.wasm", import.meta.url));
    if (input instanceof Response && !input.ok) throw Error(`Tree core load failed: HTTP ${input.status}`);
    const bytes = input instanceof Response ? await input.arrayBuffer() : input;
    const { instance } = await WebAssembly.instantiate(bytes, { env: { now: () => performance.now() } });
    return new TreeEngine(instance);
  }
  private metadata(): unknown {
    const e = this.e;
    return JSON.parse(new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.metadata_ptr() >>> 0, e.metadata_len())));
  }
  private check(code: number): void {
    if (code) throw Error(`Tree core: ${(this.metadata() as { error: string }).error}`);
  }
  /** Releases native outputs; JS copies, fields among them, remain usable.
   * Wasm linear memory retains its high-water capacity for allocator reuse. */
  release(): void { this.e.release(); }
  get memoryBytes(): number { return this.e.memory.buffer.byteLength; }
  build(family: Family | string, outputs: Outputs): TreeOutput {
    const started = performance.now();
    // JSON would otherwise silently replace nonfinite values with null.
    const request = new TextEncoder().encode(JSON.stringify({ family: typeof family !== "string" && "id" in family ? (({ id: _id, name: _name, note: _note, ...parameters }) => parameters)(family as TreePreset) : family, outputs }, (_key, value) => {
      if (typeof value === "number" && !Number.isFinite(value)) throw Error("Tree parameters must be finite");
      return value;
    }));
    const e = this.e;
    this.check(e.request_alloc(request.length));
    new Uint8Array(e.memory.buffer, e.request_ptr() >>> 0, request.length).set(request);
    this.check(e.build());
    const transfer = performance.now();
    const diagnostics = this.metadata() as Diagnostics;
    const f32 = (slot: number) => new Float32Array(e.memory.buffer, e.buffer_ptr(slot) >>> 0, e.buffer_len(slot)).slice();
    const u32 = (slot: number) => new Uint32Array(e.memory.buffer, e.buffer_ptr(slot) >>> 0, e.buffer_len(slot)).slice();
    const result: TreeOutput = { diagnostics };
    if (outputs.surface) result.surface = { chunks: this.chunks(diagnostics.surfaceChunks), bounds: diagnostics.surfaceBounds };
    if (outputs.foliage) result.foliage = { positions: f32(3), indices: u32(4), leaves: u32(5), reference: diagnostics.foliageReference, anatomy: diagnostics.foliageAnatomy, bounds: diagnostics.foliageBounds };
    if (outputs.foliage && diagnostics.organReference) result.organ = { positions: f32(27), indices: u32(28), leaves: u32(29), reference: diagnostics.organReference };
    if (outputs.structure) result.structure = { values: new Float64Array(e.memory.buffer, e.buffer_ptr(6) >>> 0, e.buffer_len(6)).slice(), topology: u32(7) };
    if (diagnostics.field) result.field = {
      ...structuredClone(diagnostics.field),
      answer: { flags: new Uint8Array(e.memory.buffer, e.buffer_ptr(8) >>> 0, e.buffer_len(8)).slice(), woodRadius: f32(25), leaves: f32(19), limbs: u32(20), flowers: f32(30) },
    };
    diagnostics.timings.transferMs = performance.now() - transfer;
    diagnostics.timings.buildMs = performance.now() - started;
    return result;
  }
  /** Each wood chunk copied out of the core, chunk by chunk. */
  private chunks(count: number): SurfaceChunk[] {
    const e = this.e;
    const view = <T>(type: new (b: ArrayBuffer, at: number, n: number) => T, chunk: number, field: number) =>
      new type(e.memory.buffer, e.chunk_ptr(chunk, field) >>> 0, e.chunk_len(chunk, field));
    return Array.from({ length: count }, (_, k) => ({
      positions: view(Float32Array, k, 0).slice(),
      normals: view(Int16Array, k, 1).slice(),
      indices: view(Uint16Array, k, 2).slice(),
    }));
  }
}
let shared: TreeEngine | undefined;
let loading: Promise<TreeEngine> | undefined;
/** Harness convenience; independent consumers can create their own TreeEngine. */
export async function initializeTreeCore(source?: BufferSource | Response): Promise<TreeEngine> {
  if (shared && !shared.disposed) return shared;
  if (shared?.disposed) { shared = undefined; loading = undefined; }
  loading ??= TreeEngine.create(source).then(engine => shared = engine).catch(error => { loading = undefined; throw error; });
  return loading;
}
export function treeCore(): TreeEngine {
  if (!shared) throw Error("Tree core is not initialized");
  return shared;
}
