import wasmUrl from "./telperion.wasm?url";
import { CATALOGUE, type Family } from "./presets.generated";
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
export const TELPERION = presetById("telperion");
export const LAURELIN = presetById("laurelin");
export const TWO_TREES = [TELPERION, LAURELIN];
export const PRESETS = CATALOGUE.map(entry => presetById(entry.id));
export interface Outputs {
  surface?: boolean; foliage?: boolean; structure?: boolean;
  /** Wood and foliage occupancy. Places retained leaves internally, but transfers
   * no render buffers unless surface/foliage are requested separately. */
  field?: boolean;
}
export interface Bounds { min: [number, number, number]; max: [number, number, number] }
export interface Timings { growthMs: number; surfaceMs: number; foliageMs: number; fieldMs: number; coreMs: number; transferMs: number; buildMs: number }
/** Half-open prototype ranges exclude connectors; indices are scalar offsets. */
export interface FoliageAnatomy {
  unit: "leaf" | "needle"; vertices: [number, number]; indices: [number, number]; sections: [number, number][];
}
export interface Diagnostics {
  biologicalUnits: number | null; foliageAnatomy: FoliageAnatomy | null;
  nodes: number; crossover: number; shed: number; capped: boolean; levelCapped: boolean;
  attractionCapped: boolean; complete: boolean; handoffs: number; generationCounts: number[];
  levelCappedHandoffs: number; twigs: number; leavesPlaced: number; instances: number;
  surfaceBounds: Bounds | null; foliageBounds: Bounds | null; fieldBounds: Bounds | null;
  fieldBytes: number; revision: number; timings: Timings;
  stages: { surface: boolean; foliage: boolean; field: boolean };
}
export interface TreeOutput {
  surface?: { positions: Float32Array; normals: Float32Array; indices: Uint32Array; bounds: Bounds | null };
  foliage?: { positions: Float32Array; indices: Uint32Array; matrices: Float32Array; anatomy: FoliageAnatomy | null; bounds: Bounds | null };
  /** Six f64 values per node: xyz, distal radius, proximal radius, base radius.
   * Three u32 values per node: parent (UINT32_MAX for root), branch, kind (0/1/2). */
  structure?: { values: Float64Array; topology: Uint32Array };
  /** Query packed x,y,z,halfExtent cells. Bits: wood=1, foliage=2.
   * Invalid after this engine's next build or release; copied results remain owned. */
  field?: { query(cells: Float64Array): Uint8Array };
  diagnostics: Diagnostics;
}
interface Exports extends WebAssembly.Exports {
  memory: WebAssembly.Memory;
  request_alloc(n: number): number; request_ptr(): number;
  metadata_ptr(): number; metadata_len(): number; build(): number; release(): void;
  buffer_ptr(slot: number): number; buffer_len(slot: number): number;
  query_alloc(count: number): number; query_ptr(): number; query(revision: number): number;
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
    const input = source ?? await fetch(wasmUrl);
    if (input instanceof Response && !input.ok) throw Error(`Tree core load failed: HTTP ${input.status}`);
    const bytes = input instanceof Response ? await input.arrayBuffer() : input;
    const { instance } = await WebAssembly.instantiate(bytes, { env: { now: () => performance.now() } });
    return new TreeEngine(instance);
  }
  private metadata(): unknown {
    const e = this.e;
    return JSON.parse(new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.metadata_ptr(), e.metadata_len())));
  }
  private check(code: number): void {
    if (code) throw Error(`Tree core: ${(this.metadata() as { error: string }).error}`);
  }
  /** Releases native outputs and invalidates fields; JS copies remain usable.
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
    new Uint8Array(e.memory.buffer, e.request_ptr(), request.length).set(request);
    this.check(e.build());
    const transfer = performance.now();
    const diagnostics = this.metadata() as Diagnostics;
    const f32 = (slot: number) => new Float32Array(e.memory.buffer, e.buffer_ptr(slot), e.buffer_len(slot)).slice();
    const u32 = (slot: number) => new Uint32Array(e.memory.buffer, e.buffer_ptr(slot), e.buffer_len(slot)).slice();
    const result: TreeOutput = { diagnostics };
    if (outputs.surface) result.surface = { positions: f32(0), normals: f32(1), indices: u32(2), bounds: diagnostics.surfaceBounds };
    if (outputs.foliage) result.foliage = { positions: f32(3), indices: u32(4), matrices: f32(5), anatomy: diagnostics.foliageAnatomy, bounds: diagnostics.foliageBounds };
    if (outputs.structure) result.structure = { values: new Float64Array(e.memory.buffer, e.buffer_ptr(6), e.buffer_len(6)).slice(), topology: u32(7) };
    if (outputs.field) result.field = { query: cells => this.query(diagnostics.revision, cells) };
    diagnostics.timings.transferMs = performance.now() - transfer;
    diagnostics.timings.buildMs = performance.now() - started;
    return result;
  }
  private query(revision: number, cells: Float64Array): Uint8Array {
    if (!(cells instanceof Float64Array) || cells.length % 4) throw Error("Field cells require packed Float64Array x,y,z,halfExtent");
    this.check(this.e.query_alloc(cells.length / 4));
    new Float64Array(this.e.memory.buffer, this.e.query_ptr(), cells.length).set(cells);
    this.check(this.e.query(revision));
    return new Uint8Array(this.e.memory.buffer, this.e.buffer_ptr(8), this.e.buffer_len(8)).slice();
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
