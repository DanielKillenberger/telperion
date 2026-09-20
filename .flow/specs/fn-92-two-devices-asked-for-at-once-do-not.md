# Two devices asked for at once do not crash the process

## Goal & Context

`cargo test --profile ci -p telperion-render --test bark_plates` dies with SIGSEGV on the owner's desk, 4 runs of 4, and passes with `-- --test-threads=1`. The core (`coredumpctl info 54060`, 2026-09-20) shows why. One test thread is in `wgpu::Instance::new` -> `vkEnumerateInstanceExtensionProperties` -> `libvulkan.so.1` -> a call through a null function pointer (`SEGV_MAPERR` at `0x0`). At the same instant a second test thread is inside `vkCreateInstance` -> `vk_icdNegotiateLoaderICDInterfaceVersion` in `libGLX_nvidia.so.0`, still loading the driver. A third waits on the loader's mutex. [user]

Every device-backed test reaches `Gpu::request(None)`, which builds a fresh `wgpu::Instance`: through `crates/telperion-render/tests/common/mod.rs:7` in 30-odd integration test files, and directly in `src/shadow/kernel_test.rs:41`, `src/select/tests.rs:29` and `src/wood/calibration.rs:117,231`. libtest runs a binary's tests as threads of one process, so any binary with two device-backed tests races the loader. The helper is unchanged since `bf16220b` (#6, 2026-09-09). `coredumpctl list` holds 20 telperion cores since 2026-09-13 across `telperion_render`, `headless`, `conformance`, `look`, `shadow`, `submit`, `timing`, `leaf_detail`, `blade_colour`, `bark_resolution` and `bark_plates`; only the 2026-09-20 `bark_plates` core was opened, the rest are inferred to share the mechanism. #47 gave `bark_plates` four device-backed tests that start together, which made an occasional crash a certain one. [user]

`vulkan-icd-loader 1.4.357.0` and `nvidia-utils 610.57.04` are unchanged since 2026-08-31, so no update introduced it. CI is green because `cargo nextest` gives each test its own process. The cost today: `cargo test --profile ci --workspace`, the local gate PR #48 named, cannot go green on an NVIDIA desk, and it stops at the first crashed binary so every suite after it goes unrun. [user]

Whether the fault is the loader's or the NVIDIA ICD's is unknown and out of reach; the frames inside `libvulkan.so.1` carry no symbols. The repository's part is that it asks for instances concurrently, which it has no need to do. [inferred]

## Architecture & Data Models

Instance creation is serialised inside `Gpu::request` on native targets: one process-wide `static` `Mutex<()>` in `crates/telperion-render/src/device.rs`, held around `wgpu::Instance::new` alone and released before any `.await`. Both frames in the core, `vkEnumerateInstanceExtensionProperties` and `vkCreateInstance`, sit inside that one call, and a `std::sync::Mutex` guard held across `request_adapter().await` would make the future `!Send` for no gain (host, 2026-09-20, amending the first draft's wider span). The lock lives in `Gpu::request`, not in `tests/common`, because four call sites bypass the helper and an embedder that asks for two devices would meet the same crash. The `wasm32` build is untouched: the browser has no loader and no threads here. A poisoned lock is recovered with `into_inner`, since a panicking test must not fail every test after it. [inferred]

Tests keep a device each. Sharing one `Gpu` per binary behind a `OnceLock` was considered and set aside: it would couple tests through shared device state and error scopes, and it would not cover the in-crate callers. If the lock alone does not hold the crash off, that is the fallback, and it goes up to the host as a design question before it is built. [inferred]

## Acceptance Criteria

- **R1** The red proof is `bark_plates` itself, run as the built `ci` test binary with libtest's default threading on the owner's desk: it died by signal 5 runs of 12 on the base commit `5a6b9aca` and 0 of 24 with the lock, both counts recorded in `.flow/evidence/<spec>/REPRO.md` with the core's two frames. A bare concurrent `Gpu::request` does not reproduce here (0 of 17 behind a barrier, 0 of 36 staggered, 0 of 12 as eight plain tests), so what `bark_plates` adds is unknown and is written as unknown. `crates/telperion-render/tests/concurrent_request.rs` stays as a smoke test that eight threads asking at once all get a device or the same skip reason; its doc comment says it has never been seen red (host, 2026-09-20, amending the first draft, which asked it to be red on the base).
- **R2** `cargo test --profile ci -p telperion-render` with libtest's default threading passes 5 runs of 5 on the owner's desk, where `bark_plates` alone failed 4 of 4 before.
- **R3** `cargo test --profile ci --workspace` runs to the end with no target failed.
- **R4** The lock exists only on non-`wasm32` targets, and the `wasm and jev` CI job stays green.
- **R5** The render crate's wall time under `cargo test --profile ci -p telperion-render` is recorded before (with `-- --test-threads=1` as the only green baseline available) and after in `.flow/evidence/<spec>/TIMING.md`.

## Boundaries

- No change to what any test asserts, to rendering, to presets or to the generator.
- No change to CI; nextest already isolates each test.
- No GPU captures and no image inspection; the evidence is exit codes and wall times.
- Not a driver or loader bug report. If the owner wants one filed upstream it is a separate act.
- The render test that rewrites tracked `.flow/evidence/fn71/resolution/smooth_bark.json` on every run is a separate defect and stays out.
