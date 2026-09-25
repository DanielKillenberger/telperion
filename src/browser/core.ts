import type { LeafReference } from "./leaf";
import { specimenBinding, type SpecimenExports, type SpecimenHandle, type SpecimenSnapshot } from "./specimen";
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
  /** Wood and foliage occupancy from the solved tree and the leaf plan: no
   * leaf is placed for a family the plan describes. A family without a plan
   * places and culls its leaves for the field; no render buffers transfer
   * unless surface/foliage are requested separately. `true` names limbs at
   * the family's `clumpSystemOrder`; `{ limbOrder }` at that lateral order,
   * a higher one parting the crown into more and smaller systems. */
  field?: boolean | { limbOrder: number };
}
export interface Bounds { min: [number, number, number]; max: [number, number, number] }
export interface Timings { growthMs: number; surfaceMs: number; planMs: number; foliageMs: number; fieldMs: number; coreMs: number; transferMs: number; buildMs: number }
/** Which stages a build ran. `foliage` is the placement and the cull together;
 * `contacts` the wood's contact surface under placement; `plan` the leaf plan
 * a field request read. `fieldSource` names the field's foliage path. */
export interface Stages {
  surface: boolean; foliage: boolean; placement: boolean; cull: boolean; contacts: boolean; plan: boolean; field: boolean;
  fieldSource: "plan" | "placed" | null;
}
/** Half-open prototype ranges exclude connectors; indices are scalar offsets. */
export interface FoliageAnatomy {
  unit: "leaf" | "needle"; vertices: [number, number]; indices: [number, number]; sections: [number, number][];
}
export interface Diagnostics {
  biologicalUnits: number | null; foliageAnatomy: FoliageAnatomy | null;
  nodes: number; crossover: number; shed: number; capped: boolean; levelCapped: boolean;
  attractionCapped: boolean; complete: boolean; handoffs: number; generationCounts: number[];
  levelCappedHandoffs: number; twigs: number; leavesPlaced: number; instances: number;
  /** Stations the leaf plan counts before any cull; zero without a plan. */
  leavesPlanned: number;
  surfaceBounds: Bounds | null; foliageBounds: Bounds | null;
  /** The box every packed leaf position is quantised against. */
  foliageReference: LeafReference | null;
  fieldBounds: Bounds | null;
  fieldBytes: number; revision: number; timings: Timings;
  stages: Stages;
}
/** One batch query's answers, one entry per cell. `flags` bits: wood=1,
 * foliage=2. `woodRadius` is the thickest wood sweep reaching the cell in
 * metres, zero without wood. `leaves` estimates the stations in the cell
 * before the crown cull (the retained leaves overlapping it on the placed
 * path). `limbs` is the owning limb system, UINT32_MAX where no foliage
 * reaches. */
export interface FieldQuery { flags: Uint8Array; woodRadius: Float32Array; leaves: Float32Array; limbs: Uint32Array }
/** Owned canonical f64 CPU BVH data for generation experiments; survives release,
 * rebuild and disposal. Mutating these arrays cannot alter the native field.
 * Bounds: six f64 min/max xyz per node then item. Topology: four u32 per
 * node [start,end,left,right], then primitive IDs; UINT32_MAX children = leaf. */
export interface FieldIndexSnapshot { bounds: Float64Array; topology: Uint32Array; nodeCount: number }
export interface FieldSnapshot {
  schema: 2; revision: number; bounds: Bounds | null;
  /** Eight f64: a xyz, b xyz, proximal and distal radius. */
  wood: Float64Array;
  woodIndex: FieldIndexSnapshot;
  /** Placed-leaf boxes; empty on a planned field. */
  leaves: FieldIndexSnapshot;
  /** The leaf plan's sweeps: seven f64 (a xyz, b xyz, reach) and two u32
   * (station count, limb system) a sweep, with their index. Empty on a
   * placed field. Where the plan holds a ribbon, `sides` carries six f64 a
   * sweep, its half-width vectors at its two ends (zero for a capsule), and
   * its reach is its thickness: the trapezoid the segment sweeps between
   * them, pushed out by the thickness along its normal. Empty otherwise. */
  plan: { segments: Float64Array; stations: Uint32Array; sides: Float64Array; index: FieldIndexSnapshot };
  timings: { extractionMs: number; copyMs: number; totalMs: number };
}
export interface TreeOutput {
  surface?: { positions: Float32Array; normals: Float32Array; indices: Uint32Array; bounds: Bounds | null };
  /** Three u32 words a leaf, decoded against `reference` by `./leaf`. The
   * sixteen-float form is never built here: twelve bytes are the storage. */
  foliage?: { positions: Float32Array; indices: Uint32Array; leaves: Uint32Array; reference: LeafReference | null; anatomy: FoliageAnatomy | null; bounds: Bounds | null };
  /** Six f64 values per node: xyz, distal radius, proximal radius, base radius.
   * Three u32 values per node: parent (UINT32_MAX for root), branch, kind (0/1/2). */
  structure?: { values: Float64Array; topology: Uint32Array };
  /** Query packed x,y,z,halfExtent cells. Invalid after this engine's next
   * build or release; copied results remain owned. `bounds` encloses the
   * field's wood and foliage, as the slim entry's `FieldTree.bounds` does;
   * null for an empty field. */
  field?: { bounds: Bounds | null; query(cells: Float64Array): FieldQuery; snapshot(): FieldSnapshot };
  diagnostics: Diagnostics;
}
interface Exports extends WebAssembly.Exports, SpecimenExports {
  memory: WebAssembly.Memory;
  request_alloc(n: number): number; request_ptr(): number;
  metadata_ptr(): number; metadata_len(): number; build(): number; release(): void;
  buffer_ptr(slot: number): number; buffer_len(slot: number): number;
  field_snapshot(revision: number): number; field_snapshot_release(): void;
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
    return JSON.parse(new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.metadata_ptr(), e.metadata_len())));
  }
  private check(code: number): void {
    if (code) throw Error(`Tree core: ${(this.metadata() as { error: string }).error}`);
  }
  /** Releases native outputs and invalidates fields; JS copies remain usable.
   * Wasm linear memory retains its high-water capacity for allocator reuse. */
  release(): void { this.e.release(); }
  private specimens = specimenBinding(() => this.e, code => this.check(code), () => this.metadata());
  buildSpecimen(family: Family | string, historyCap = 10_000): SpecimenHandle { return this.specimens.build(family, historyCap); }
  importSpecimen(snapshot: SpecimenSnapshot): SpecimenHandle { return this.specimens.import(snapshot); }
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
    if (outputs.foliage) result.foliage = { positions: f32(3), indices: u32(4), leaves: u32(5), reference: diagnostics.foliageReference, anatomy: diagnostics.foliageAnatomy, bounds: diagnostics.foliageBounds };
    if (outputs.structure) result.structure = { values: new Float64Array(e.memory.buffer, e.buffer_ptr(6), e.buffer_len(6)).slice(), topology: u32(7) };
    if (outputs.field) result.field = { bounds: structuredClone(diagnostics.fieldBounds), query: cells => this.query(diagnostics.revision, cells), snapshot: () => this.snapshot(diagnostics) };
    diagnostics.timings.transferMs = performance.now() - transfer;
    diagnostics.timings.buildMs = performance.now() - started;
    return result;
  }
  private snapshot(diagnostics: Diagnostics): FieldSnapshot {
    const start = performance.now(), e = this.e;
    let result: FieldSnapshot;
    try {
      this.check(e.field_snapshot(diagnostics.revision));
      const meta = this.metadata() as { woodNodes: number; leafNodes: number; planNodes: number; extractionMs: number };
      const copy = performance.now();
      const f64 = (slot: number) => new Float64Array(e.memory.buffer, e.buffer_ptr(slot), e.buffer_len(slot)).slice();
      const u32 = (slot: number) => new Uint32Array(e.memory.buffer, e.buffer_ptr(slot), e.buffer_len(slot)).slice();
      result = {
        schema: 2, revision: diagnostics.revision, bounds: structuredClone(diagnostics.fieldBounds),
        wood: f64(9),
        woodIndex: { bounds: f64(10), topology: u32(11), nodeCount: meta.woodNodes },
        leaves: { bounds: f64(12), topology: u32(13), nodeCount: meta.leafNodes },
        plan: { segments: f64(21), stations: u32(22), sides: f64(26), index: { bounds: f64(23), topology: u32(24), nodeCount: meta.planNodes } },
        timings: { extractionMs: meta.extractionMs, copyMs: performance.now() - copy, totalMs: 0 },
      };
    } finally { e.field_snapshot_release(); }
    result.timings.totalMs = performance.now() - start;
    return result;
  }
  private query(revision: number, cells: Float64Array): FieldQuery {
    if (!(cells instanceof Float64Array) || cells.length % 4) throw Error("Field cells require packed Float64Array x,y,z,halfExtent");
    const e = this.e;
    this.check(e.query_alloc(cells.length / 4));
    new Float64Array(e.memory.buffer, e.query_ptr(), cells.length).set(cells);
    this.check(e.query(revision));
    return {
      flags: new Uint8Array(e.memory.buffer, e.buffer_ptr(8), e.buffer_len(8)).slice(),
      woodRadius: new Float32Array(e.memory.buffer, e.buffer_ptr(25), e.buffer_len(25)).slice(),
      leaves: new Float32Array(e.memory.buffer, e.buffer_ptr(19), e.buffer_len(19)).slice(),
      limbs: new Uint32Array(e.memory.buffer, e.buffer_ptr(20), e.buffer_len(20)).slice(),
    };
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
