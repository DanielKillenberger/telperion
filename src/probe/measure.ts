/* The probe's measurements (fn-257, "Early test"): the device, every cycle
 * kernel's pipeline compiled cold and then again cached, and one empty
 * chain of 80 x 8 indirect dispatches. Every time is performance.now(). */
import { CHAIN, CYCLE, type Kit } from "./kernels";
import {
  MAP_READ,
  STAGE_COMPUTE,
  USAGE,
  type Gpu,
  type GpuAdapter,
  type GpuComputePipeline,
  type GpuDevice,
  type GpuLayoutEntry,
} from "./gpu";

export interface Acquired {
  readonly adapter: GpuAdapter;
  readonly device: GpuDevice;
  readonly adapterMs: number;
  readonly deviceMs: number;
  readonly requiredLimits: Record<string, number>;
}

export interface PipelineTime {
  readonly entry: string;
  readonly ms: number;
  readonly error?: string;
}

export interface KitTime {
  readonly kit: string;
  readonly layout: "explicit" | "auto";
  readonly ms: number;
  readonly pipelines: readonly PipelineTime[];
  readonly messages?: readonly string[];
}

export interface Round {
  readonly ms: number;
  readonly pipelines: number;
  readonly failed: number;
  readonly kits: readonly KitTime[];
}

export interface ChainRun {
  readonly encodeMs: number;
  readonly submitToDoneMs: number;
  readonly totalMs: number;
  readonly stepsDone: number;
}

/* The spike asks for at most 32 storage buffers a stage; a browser that
 * offers fewer gets what it offers, and a kit past it takes "auto". */
const STORAGE_LIMITS = ["maxStorageBuffersPerShaderStage", "maxStorageBuffersInComputeStage"];

export async function acquire(gpu: Gpu): Promise<Acquired> {
  const started = performance.now();
  const adapter = await gpu.requestAdapter({ powerPreference: "high-performance" });
  const adapted = performance.now();
  if (!adapter) throw new Error("requestAdapter returned null");
  const requiredLimits: Record<string, number> = {};
  for (const key of STORAGE_LIMITS) {
    if (key in adapter.limits) requiredLimits[key] = Math.min(adapter.limits[key], 32);
  }
  const device = await adapter.requestDevice({ label: "telperion probe", requiredLimits });
  const deviced = performance.now();
  return { adapter, device, adapterMs: adapted - started, deviceMs: deviced - adapted, requiredLimits };
}

function layoutFor(device: GpuDevice, buffers: number) {
  const entries: GpuLayoutEntry[] = [
    { binding: 0, visibility: STAGE_COMPUTE, buffer: { type: "uniform", hasDynamicOffset: true } },
  ];
  for (let b = 1; b <= buffers; b++) entries.push({ binding: b, visibility: STAGE_COMPUTE, buffer: { type: "storage" } });
  const group = device.createBindGroupLayout({ entries });
  return { group, pipeline: device.createPipelineLayout({ bindGroupLayouts: [group] }) };
}

/** One module and its pipelines, awaited one at a time as the spike
 * creates them, so each pipeline's time is its own. */
async function compileKit(device: GpuDevice, kit: Kit, pipelines: Map<string, GpuComputePipeline>): Promise<KitTime> {
  const storageLimit = device.limits.maxStorageBuffersPerShaderStage ?? 8;
  const started = performance.now();
  const module = device.createShaderModule({ label: kit.name, code: kit.source });
  const explicit = kit.buffers <= storageLimit;
  const layout = explicit ? layoutFor(device, kit.buffers).pipeline : "auto";
  const times: PipelineTime[] = [];
  for (const entry of kit.entries) {
    const begun = performance.now();
    try {
      const made = await device.createComputePipelineAsync({ label: entry, layout, compute: { module, entryPoint: entry } });
      pipelines.set(`${kit.name}.${entry}`, made);
      times.push({ entry, ms: performance.now() - begun });
    } catch (error) {
      times.push({ entry, ms: performance.now() - begun, error: String(error) });
    }
  }
  const ms = performance.now() - started;
  const result: KitTime = { kit: kit.name, layout: explicit ? "explicit" : "auto", ms, pipelines: times };
  if (!times.some(t => t.error)) return result;
  const info = await module.getCompilationInfo();
  const messages = info.messages
    .filter(m => m.type === "error")
    .slice(0, 5)
    .map(m => `${m.lineNum}:${m.linePos} ${m.message}`);
  return { ...result, messages };
}

export async function compileRound(
  device: GpuDevice,
  kits: readonly Kit[] = CYCLE,
  pipelines = new Map<string, GpuComputePipeline>(),
): Promise<Round> {
  const started = performance.now();
  const done: KitTime[] = [];
  for (const kit of kits) done.push(await compileKit(device, kit, pipelines));
  const all = done.flatMap(k => k.pipelines);
  return { ms: performance.now() - started, pipelines: all.length, failed: all.filter(p => p.error).length, kits: done };
}

const COUNT = 80 * 8;
const PARAMS_BYTES = 64;

/** The spike's chain, one pass: dispatch k reads its workgroup count from
 * args[k % 2] and writes dispatch k+1's into the other, so each waits on
 * the one before. Returns a runner that records, submits and awaits it. */
export async function chain(device: GpuDevice): Promise<{ compile: Round; run: () => Promise<ChainRun> }> {
  const pipelines = new Map<string, GpuComputePipeline>();
  const compile = await compileRound(device, [CHAIN], pipelines);
  const step = pipelines.get("chain.step");
  if (!step) throw new Error("the chain's step pipeline did not compile");
  const align = Math.max(device.limits.minUniformBufferOffsetAlignment ?? 256, PARAMS_BYTES);
  const uniform = device.createBuffer({ size: (COUNT + 1) * align, usage: USAGE.UNIFORM | USAGE.COPY_DST });
  const params = new Uint32Array(((COUNT + 1) * align) / 4);
  for (let k = 0; k < COUNT; k++) params[(k * align) / 4] = k;
  device.queue.writeBuffer(uniform, 0, params);
  const argBytes = 12 * (COUNT + 2);
  const args = [0, 1].map(() => device.createBuffer({ size: argBytes, usage: USAGE.STORAGE | USAGE.INDIRECT | USAGE.COPY_DST }));
  const state = device.createBuffer({ size: 16, usage: USAGE.STORAGE | USAGE.COPY_SRC | USAGE.COPY_DST });
  const readback = device.createBuffer({ size: 16, usage: USAGE.MAP_READ | USAGE.COPY_DST });
  const layout = step.getBindGroupLayout(0);
  const binds = [1, 0].map(to =>
    device.createBindGroup({
      layout,
      entries: [
        { binding: 0, resource: { buffer: uniform, size: PARAMS_BYTES } },
        { binding: 1, resource: { buffer: args[to] } },
        { binding: 2, resource: { buffer: state } },
      ],
    }),
  );
  const run = async (): Promise<ChainRun> => {
    for (const a of args) device.queue.writeBuffer(a, 0, new Uint32Array(argBytes / 4));
    device.queue.writeBuffer(args[0], 0, new Uint32Array([1, 1, 1]));
    device.queue.writeBuffer(state, 0, new Uint32Array(4));
    const started = performance.now();
    const encoder = device.createCommandEncoder();
    const pass = encoder.beginComputePass();
    pass.setPipeline(step);
    for (let k = 0; k < COUNT; k++) {
      pass.setBindGroup(0, binds[k % 2], [k * align]);
      pass.dispatchWorkgroupsIndirect(args[k % 2], 12 * k);
    }
    pass.end();
    encoder.copyBufferToBuffer(state, 0, readback, 0, 16);
    const commands = encoder.finish();
    const encoded = performance.now();
    device.queue.submit([commands]);
    await device.queue.onSubmittedWorkDone();
    const done = performance.now();
    await readback.mapAsync(MAP_READ);
    const stepsDone = new Uint32Array(readback.getMappedRange())[0];
    readback.unmap();
    return { encodeMs: encoded - started, submitToDoneMs: done - encoded, totalMs: done - started, stepsDone };
  };
  return { compile, run };
}

export const CHAIN_DISPATCHES = COUNT;
