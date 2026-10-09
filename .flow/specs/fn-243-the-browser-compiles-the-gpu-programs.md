# The browser compiles the GPU programs in the background

## Goal & Context

After fn-235, the owner's first harness load in a freshly restarted Chrome on 2026-10-09 still ended in "the GPU device was lost: A valid external Instance reference no longer exists" on the first build. fn-235 measured the `grow` pipeline at 12-17 s to compile cold natively and kept it synchronous; Chrome compiles a synchronous `createComputePipeline` on its GPU process's main thread, where its watchdog can end the process (likely cause, not yet confirmed: Astra notes the message names Dawn's instance shutdown, not the watchdog). WebGPU cannot ship precompiled GPU programs: the browser compiles WGSL for each machine's GPU and driver and caches the result, so the first load on a machine always compiles. Astra's review (2026-10-09, `ASTRA-ASYNC.md`): wgpu 30.0.1 exposes the browser device (`Device::as_webgpu`, api/device.rs:426) but no way to create a compute pipeline asynchronously or to import one; a small pinned patch to wgpu's WebGPU backend (descriptor mapping at backend/webgpu.rs:2410, the existing `create_compute_pipeline_async` binding at webgpu_sys/gen_GpuDevice.rs:238) can await the Promise and return an ordinary `ComputePipeline`. Host (2026-10-09): release blocker.

## Requirements

- **R1 (confirm the cause):** one Chrome run with GPU-process logging on (`--enable-logging=stderr --v=1`, a fresh profile), run by the owner or with the owner's go since a crash can disable WebGPU until restart: the GPU process's exit reason on the first build recorded. If it is not the watchdog during pipeline creation, stop and report before building.
- **R2 (async creation on the web route):** a minimal, pinned wgpu patch (a `[patch.crates-io]` to a vendored copy or fork at 30.0.1, the diff kept small and documented with its upstream issue) adding a browser-only async compute-pipeline creation that returns `wgpu::ComputePipeline`; the web route creates every pipeline through it, awaiting them, with Promise rejection and device loss propagated by name. Native creation unchanged.
- **R3 (compile at page load):** the harness starts creating the pipelines as soon as the device exists, before the first tree is requested, and shows "preparing GPU programs" until they resolve; the first build waits on them, never starts a second compile.
- **R4 (proved):** the pending and failure states tested with controlled failures and delayed promises (a browser-side test harness for the web route, e.g. a mocked GPUDevice whose async creation rejects, resolves late, or loses the device): a named pipeline rejection reaches the GROWER alert as a terminal error and no build starts after it; a device loss during preparation is reported by name; a build requested while preparation is pending waits on it, and each pipeline is created exactly once on the live device; native output unchanged (parity tests, `space_hash`); the wasm build and npm test green; one Chrome compile-only probe on a fresh profile (owner's go): every pipeline resolves without device loss, then one tree builds; Chrome's second load hits its cache (time recorded).
- **R5:** gate green.

## Boundaries

The web route's pipeline creation only; shader size work stays with fn-241/fn-242. Depends on fn-235. Release blocker for fn-237.
