/* The field entry point: a tree's parameters and a grid side in, the
 * tree's field read out to that grid (./types `Grid`) out. A preset is
 * grown by passing its parameters (`presetById(id)` from
 * `telperion/presets`); there is no id lookup here (fn-317). The tree grows
 * on the CPU, through the core's module (`telperion.wasm`), as the main
 * entry's `TreeEngine.build(family, { field: { n } })` grows it, and only
 * the grid comes back (fn-353). The same file runs in a browser page, a
 * worker or Node, with or without WebGPU. A caller passes the module's
 * bytes, a Response or a compiled Module, or leaves the source to
 * `telperion.wasm` beside this file.
 *
 * Migrating from 0.1.4's slim entry: `growField` takes the grid's side and
 * resolves to the grid with its answers; the cube query (`query(cells)`),
 * `bounds` and `release()` are gone, and nothing replaces the query (a
 * coarser or finer reading is another side). The `cut` option is gone, and
 * nothing replaces it (every tree grows whole); the source is the core's
 * module, not `telperion-field.wasm`. */
import { TreeEngine } from "../browser/core";
import type { Family } from "../browser/presets";
import { wasmSource } from "../wasm-source";
import { side } from "./voxelize";
import type { FieldGrid } from "./types";

export type { Bounds, FieldAnswer, FieldGrid, Grid } from "./types";
export type { Family } from "../browser/presets";
export const NO_LIMB = 0xffffffff;
/** A grown tree's field, read out to the grid its call named over the tree's
 * own box. */
export type FieldTree = FieldGrid;
/** The core's module: its bytes, a Response, or a compiled Module. */
export type FieldSource = BufferSource | Response | WebAssembly.Module;
export interface GrowOptions {
  /** The lateral order whose systems name limbs; the family's own when left
   * out. A higher order parts the crown into more and smaller systems. */
  limbOrder?: number;
  /** Where the core's module comes from. Left out, it is fetched from
   * `telperion.wasm` beside this module. A page or worker holds one core
   * for its fields: the first call to load it names the source, and a
   * later call joins that core whatever it names. */
  source?: FieldSource;
}

const U32 = 2 ** 32;

let bundled: Promise<WebAssembly.Module> | undefined;
/** Compiles the core's module, for a caller that grows trees in several
 * workers and passes the Module to each. The module beside this file is
 * compiled once, however many calls ask, and a failed compile is retried by
 * the next call; a Module is returned as given, and any other source is
 * compiled on every call. */
export function compileField(source?: FieldSource): Promise<WebAssembly.Module> {
  if (source instanceof WebAssembly.Module) return Promise.resolve(source);
  if (source !== undefined) return compile(source);
  // The literal names the file for a consumer's bundler.
  bundled ??= compile(wasmSource(new URL("./telperion.wasm", import.meta.url))).catch((error: unknown) => {
    bundled = undefined;
    throw error;
  });
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

let core: Promise<TreeEngine> | undefined;
/** The core every field of this page or worker grows on, started once from
 * the first call's source; a refused start is forgotten. */
function engine(source?: FieldSource): Promise<TreeEngine> {
  core ??= TreeEngine.create(source).catch((error: unknown) => {
    core = undefined;
    throw error;
  });
  return core;
}

function unsigned(value: number, what: string, limit = U32): number {
  if (!Number.isInteger(value) || value < 0 || value >= limit) throw RangeError(`${what} is an unsigned 32-bit integer`);
  return value;
}

/** Grows the tree `family` describes, at its own seed
 * (`family.skeleton.seed`), and reads its field out to `n` cells a side
 * over the tree's box. A preset's id, a side outside 3 to 256 and a
 * nonfinite parameter are refused by name before any work; parameters the
 * core refuses with the core's error, as `TreeEngine.build` refuses them. */
export async function growField(family: Family, n: number, options: GrowOptions = {}): Promise<FieldTree> {
  side(n);
  const limbOrder = options.limbOrder === undefined ? undefined : unsigned(options.limbOrder, "limbOrder", U32 - 1);
  const cpu = await engine(options.source);
  try {
    const { field } = cpu.build(family, { field: limbOrder === undefined ? { n } : { n, limbOrder } });
    if (field === undefined) throw Error("the core read out no field");
    return field;
  } finally {
    cpu.release();
  }
}
