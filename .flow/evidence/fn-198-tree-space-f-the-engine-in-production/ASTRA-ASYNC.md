**Recommend a small, pinned wgpu patch adding async WebGPU pipeline creation and returning the resulting pipeline directly.** Cache warming cannot provide the guarantee you want. Shader reduction remains useful, but need not precede this fix.

The concrete findings in your installed **wgpu 30.0.1**:

| Option | Finding |
|---|---|
| Obtain the browser device | **Yes.** `Device::as_webgpu()` at [api/device.rs:426](/home/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-30.0.1/src/api/device.rs:426). It returns wgpu’s vendored `GpuDevice`; convert its clone to `web_sys::GpuDevice` with `JsCast::unchecked_into()`. See [lib.rs:336](/home/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-30.0.1/src/lib.rs:336). |
| Import an async-created pipeline | **No public API.** `ComputePipeline.inner` is private; there is no pipeline `from_custom()` or WebGPU import at [api/compute_pipeline.rs:10](/home/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-30.0.1/src/api/compute_pipeline.rs:10). |
| Use a custom backend wrapper | **Not as a drop-in bridge.** `Device::from_custom()` creates a custom dispatch device. The existing WebGPU compute pass requires a WebGPU pipeline variant at [backend/webgpu.rs:3472](/home/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-30.0.1/src/backend/webgpu.rs:3472). A custom pipeline cannot cross that boundary. |
| Pre-warm from JS | **Plausible optimization, unsuitable correctness dependency.** Dawn caches pipeline objects per device. Awaiting an identical explicit-layout pipeline on the **same device**, retaining it, then creating synchronously can hit that cache. Separate-device/per-origin disk warming adds further uncertainty. No WebGPU contract guarantees a hit. [Dawn cache implementation](https://dawn.googlesource.com/dawn/+/refs/heads/chromium/6936/src/dawn/native/Device.cpp). |

For the patch, reuse descriptor mapping at [backend/webgpu.rs:2410](/home/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-30.0.1/src/backend/webgpu.rs:2410), replace the creation call with `create_compute_pipeline_async`, await its Promise, then use the existing `WebComputePipeline { inner, ident }.into()` construction. The binding **already exists** at [gen_GpuDevice.rs:238](/home/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-30.0.1/src/backend/webgpu/webgpu_sys/gen_GpuDevice.rs:238). Expose a browser-only async method returning `Result<ComputePipeline, …>`; propagate Promise rejection and device loss.

Thread that await through [generation/io.rs:108](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/src/generation/io.rs:108) and [chains.rs:231](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/src/generation/fine/chains.rs:231). Merely putting today’s synchronous call inside an `async fn` or Web Worker does not change GPU-process scheduling.

Dawn’s Vulkan backend routes async creation to `InitializeAsync()`, which posts compilation to its async task manager. That is the mechanism you need. **“Never blocks” remains stronger than the guarantee**: validation, shader-module processing and driver bugs remain possible stalls. Check Chrome’s actual backend/version. [DeviceVk.cpp](https://raw.githubusercontent.com/google/dawn/main/src/dawn/native/vulkan/DeviceVk.cpp), [CreatePipelineAsyncEvent.cpp](https://raw.githubusercontent.com/google/dawn/main/src/dawn/native/CreatePipelineAsyncEvent.cpp).

WGSL has no portable `noinline` control. Shared helper functions can still be inlined; reducing reachable call sites/arithmetic is measurable work, not a compiler directive. Your existing evidence already demonstrates this.

Verify without repeated browser failures:

1. Test exact WGSL and descriptors in **standalone Dawn/Tint**, matching Chrome’s backend, with cold caches and a process timeout. Native wgpu/Naga timings are not Chrome timings.
2. Then run one isolated Chrome **compile-only async** probe, no tree dispatch. Trace compilation threads and a tiny independent GPU heartbeat; a JS heartbeat alone proves insufficient.
3. After resolution, exercise the returned pipeline and check parity. Keep watchdogs enabled.

Finally, the reported message denotes Dawn instance/event shutdown, **not proof of watchdog termination**. Confirm with GPU-process exit/watchdog logs. [Dawn loss handling](https://raw.githubusercontent.com/google/dawn/main/src/dawn/native/Device.cpp).

No files edited.