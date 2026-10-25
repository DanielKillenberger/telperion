/* The field entry point: a tree's parameters and a grid side in, the
 * tree's field read out to that grid (./voxelize `Grid`) out. A preset is
 * grown by passing its parameters (`presetById(id)` from
 * `telperion/presets`); there is no id lookup here (fn-317). The tree grows
 * on the device (WebGPU), through the renderer's module
 * (`telperion-render.wasm`), as the harness grows it, and its field is read
 * out there: only the grid crosses back, and nothing stays on the device
 * (fn-257). Without WebGPU every call rejects with a `FieldError` whose
 * `code` is "webgpu-unavailable", and on a lost device "device-lost";
 * there is no CPU fallback here. The same file runs in a browser page or
 * worker; in Node, which has no WebGPU, it rejects. The CPU's field of the
 * same tree, for Node and tooling, is `TreeEngine.build(family, { field:
 * { n } })` from the main entry. A caller passes the module's bytes, a Response
 * or a compiled Module, or leaves the source to `telperion-render.wasm`
 * beside this file.
 *
 * Migrating from 0.1.4's slim entry: `growField` takes the grid's side and
 * resolves to the grid with its answers; the cube query (`query(cells)`),
 * `bounds` and `release()` are gone, and nothing replaces the query (a
 * coarser or finer reading is another side). The `cut` option is gone, and
 * nothing replaces it (every tree grows whole, on the device); the source is
 * the renderer's module, not `telperion-field.wasm`. */
import { WebField } from "../browser/render/telperion_render.js";
import { loadModule, type ModuleSource } from "../browser/module";
import type { Family } from "../browser/presets";
import { wire } from "../browser/wire";
import { wasmSource } from "../wasm-source";
import { side, type Grid } from "./voxelize";

export type { Grid } from "./voxelize";
export type { Family } from "../browser/presets";
export interface Bounds { min: [number, number, number]; max: [number, number, number] }
/** One answer per cell, in the grid's cell order. `flags` bits: wood = 1,
 * foliage = 2, flowers = 4. `woodRadius` is the thickest wood sweep reaching
 * the cell, in metres, zero without wood. `leaves` estimates the leaf
 * stations in the cell before the crown cull. `limbs` is the owning limb
 * system, `NO_LIMB` where no foliage reaches. `flowers` counts the florets
 * whose seat lies in the cell, at the flowering window's middle: over the
 * grid they sum to the tree's florets, zero on a tree that flowers nowhere. */
export interface FieldAnswer { flags: Uint8Array; woodRadius: Float32Array; leaves: Float32Array; limbs: Uint32Array; flowers: Float32Array }
export const NO_LIMB = 0xffffffff;
/** A field read out: its grid and one answer per cell. */
export interface FieldGrid extends Grid { answer: FieldAnswer }
/** A grown tree's field, read out to the grid its call named over the tree's
 * own box. */
export type FieldTree = FieldGrid;
/** Why the device refused: no WebGPU in this page or worker, or a device
 * lost under a call. The message names the cause; the code is for code. */
export type FieldErrorCode = "webgpu-unavailable" | "device-lost";
export class FieldError extends Error {
  readonly code: FieldErrorCode;
  constructor(code: FieldErrorCode, message: string) {
    super(message);
    this.name = "FieldError";
    this.code = code;
  }
}
export type FieldSource = ModuleSource;
export interface GrowOptions {
  /** The lateral order whose systems name limbs; the family's own when left
   * out. A higher order parts the crown into more and smaller systems. */
  limbOrder?: number;
  /** Where the renderer's module comes from. Left out, it is fetched from
   * `telperion-render.wasm` beside this module. A page holds one module,
   * shared with `createRenderer`: the first call to load it names the
   * source, and a later call joins that module whatever it names. */
  source?: FieldSource;
}

const U32 = 2 ** 32;


interface Gpu { requestAdapter(): Promise<unknown> }
/** This page's or worker's WebGPU: the field grows nowhere else. */
function device(): Gpu {
  const gpu = (globalThis.navigator as { gpu?: Gpu } | undefined)?.gpu;
  if (gpu === undefined) throw new FieldError("webgpu-unavailable", "WebGPU is not available: telperion/field grows its trees on the device");
  return gpu;
}

/** `run` on the device, its coded refusals as `FieldError`s. Where the
 * module found no adapter but the browser, asked directly, has one, the
 * module's request was answered early (killenberger fn-46.3: headless
 * Chromium, a page whose first WebGPU use is the field, its first tree
 * refused and the next grown): `run` is tried once more, and only then. */
async function onDevice<T>(gpu: Gpu, run: () => Promise<T>): Promise<T> {
  try {
    return await run();
  } catch (error) {
    const refused = coded(error);
    if (refused?.code !== "webgpu-unavailable" || await gpu.requestAdapter() === null) throw refused ?? error;
  }
  try {
    return await run();
  } catch (error) {
    throw coded(error) ?? error;
  }
}
/** The module's refusal with a code, as a `FieldError`; none for another. */
function coded(error: unknown): FieldError | undefined {
  const code = (error as { code?: unknown } | null)?.code;
  if (code !== "webgpu-unavailable" && code !== "device-lost") return undefined;
  return new FieldError(code, (error as Error).message);
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
  const gpu = device();
  await loadModule(source);
  await onDevice(gpu, () => WebField.prepare());
}

function unsigned(value: number, what: string, limit = U32): number {
  if (!Number.isInteger(value) || value < 0 || value >= limit) throw RangeError(`${what} is an unsigned 32-bit integer`);
  return value;
}

/** Grows the tree `family` describes, at its own seed
 * (`family.skeleton.seed`), on the device and reads its field out to `n`
 * cells a side over the tree's box. A preset's id, a side outside 3 to 256
 * and a nonfinite parameter are refused by name before any work; a grid
 * past the device's limits by name before anything is allocated;
 * parameters the core refuses with the core's error, as
 * `TreeEngine.build` refuses them; a page or worker without WebGPU, or a
 * lost device, with a `FieldError`. */
export async function growField(family: Family, n: number, options: GrowOptions = {}): Promise<FieldTree> {
  const parameters = wire(family);
  side(n);
  const order = options.limbOrder === undefined ? undefined : unsigned(options.limbOrder, "limbOrder", U32 - 1);
  const gpu = device();
  await loadModule(options.source);
  const field = (await onDevice(gpu, () => WebField.grow(parameters, n, order))) as WebField;
  try {
    const o = field.origin();
    return { n: field.n(), cell: field.cell(), origin: [o[0], o[1], o[2]], answer: answers(field.answers()) };
  } finally {
    field.free();
  }
}

/** The binding's five words a cell, in cell order, as one answer per cell. */
function answers(words: Uint32Array): FieldAnswer {
  const n = words.length / 5;
  const answer = { flags: new Uint8Array(n), woodRadius: new Float32Array(n), leaves: new Float32Array(n), limbs: new Uint32Array(n), flowers: new Float32Array(n) };
  const floats = new Float32Array(words.buffer, words.byteOffset, words.length);
  for (let i = 0; i < n; i++) {
    answer.flags[i] = words[5 * i];
    answer.woodRadius[i] = floats[5 * i + 1];
    answer.leaves[i] = floats[5 * i + 2];
    answer.limbs[i] = words[5 * i + 3];
    answer.flowers[i] = floats[5 * i + 4];
  }
  return answer;
}
