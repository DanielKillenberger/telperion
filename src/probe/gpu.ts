/* The slice of WebGPU the probe touches, typed here so the probe needs no
 * new dependency and puts nothing in the global scope. Names follow the
 * WebGPU IDL; only what the probe calls is declared. */

export interface GpuAdapterInfo {
  readonly [field: string]: unknown;
}

export interface GpuAdapter {
  readonly info: GpuAdapterInfo;
  readonly limits: Readonly<Record<string, number>>;
  requestDevice(descriptor?: { label?: string; requiredLimits?: Record<string, number> }): Promise<GpuDevice>;
}

export interface Gpu {
  requestAdapter(options?: { powerPreference?: "high-performance" | "low-power" }): Promise<GpuAdapter | null>;
}

export interface GpuCompilationMessage {
  readonly type: string;
  readonly message: string;
  readonly lineNum: number;
  readonly linePos: number;
}

export interface GpuShaderModule {
  getCompilationInfo(): Promise<{ readonly messages: readonly GpuCompilationMessage[] }>;
}

export type GpuBindGroupLayout = { readonly __brand: "GPUBindGroupLayout" };
export type GpuPipelineLayout = { readonly __brand: "GPUPipelineLayout" };
export type GpuBindGroup = { readonly __brand: "GPUBindGroup" };
export type GpuCommandBuffer = { readonly __brand: "GPUCommandBuffer" };

export interface GpuComputePipeline {
  getBindGroupLayout(index: number): GpuBindGroupLayout;
}

export interface GpuBuffer {
  mapAsync(mode: number): Promise<void>;
  getMappedRange(): ArrayBuffer;
  unmap(): void;
  destroy(): void;
}

export interface GpuComputePassEncoder {
  setPipeline(pipeline: GpuComputePipeline): void;
  setBindGroup(index: number, group: GpuBindGroup, dynamicOffsets?: readonly number[]): void;
  dispatchWorkgroupsIndirect(buffer: GpuBuffer, offset: number): void;
  end(): void;
}

export interface GpuCommandEncoder {
  beginComputePass(): GpuComputePassEncoder;
  copyBufferToBuffer(source: GpuBuffer, sourceOffset: number, target: GpuBuffer, targetOffset: number, size: number): void;
  finish(): GpuCommandBuffer;
}

export interface GpuQueue {
  writeBuffer(buffer: GpuBuffer, offset: number, data: ArrayBufferView): void;
  submit(buffers: readonly GpuCommandBuffer[]): void;
  onSubmittedWorkDone(): Promise<void>;
}

export interface GpuLayoutEntry {
  binding: number;
  visibility: number;
  buffer: { type: "uniform" | "storage"; hasDynamicOffset?: boolean };
}

export interface GpuDevice {
  readonly queue: GpuQueue;
  readonly limits: Readonly<Record<string, number>>;
  readonly lost: Promise<{ readonly reason: string; readonly message: string }>;
  createShaderModule(descriptor: { label?: string; code: string }): GpuShaderModule;
  createBindGroupLayout(descriptor: { entries: readonly GpuLayoutEntry[] }): GpuBindGroupLayout;
  createPipelineLayout(descriptor: { bindGroupLayouts: readonly GpuBindGroupLayout[] }): GpuPipelineLayout;
  createComputePipelineAsync(descriptor: {
    label?: string;
    layout: GpuPipelineLayout | "auto";
    compute: { module: GpuShaderModule; entryPoint: string };
  }): Promise<GpuComputePipeline>;
  createBuffer(descriptor: { size: number; usage: number }): GpuBuffer;
  createBindGroup(descriptor: {
    layout: GpuBindGroupLayout;
    entries: readonly { binding: number; resource: { buffer: GpuBuffer; offset?: number; size?: number } }[];
  }): GpuBindGroup;
  createCommandEncoder(): GpuCommandEncoder;
  addEventListener(type: "uncapturederror", listener: (event: Event & { readonly error: { readonly message: string } }) => void): void;
}

/* GPUBufferUsage, GPUShaderStage and GPUMapMode, by value. */
export const USAGE = { MAP_READ: 0x01, COPY_DST: 0x08, COPY_SRC: 0x04, UNIFORM: 0x40, STORAGE: 0x80, INDIRECT: 0x100 } as const;
export const STAGE_COMPUTE = 0x4;
export const MAP_READ = 0x1;

export function navigatorGpu(): Gpu | undefined {
  return (navigator as Navigator & { readonly gpu?: Gpu }).gpu;
}
