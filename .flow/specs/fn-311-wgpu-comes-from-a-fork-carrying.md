# wgpu comes from a fork carrying upstream's async pipelines, not a vendored copy

## Goal & Context

Owner (2026-10-24), on PR #159's size ("it's still 250k loc added which is a bit crazy. can we do another cleanup?"): fork wgpu, carry the upstream pull request, and drop the vendored copy. Their words: "ok do all of that".

What exists, checked 2026-10-24:
- `vendor/wgpu-30.0.1-async/` is wgpu 30.0.1 from crates.io: 42,420 lines, about 100 of them our patch. The patch is a WebGPU-only `Device::create_compute_pipeline_async`, which returns `Option<impl Future<Output = Result<ComputePipeline, String>>>` (`vendor/README.md`). It reaches the workspace through `[patch.crates-io]`, and its one caller is `crates/telperion-render/src/programs.rs`.
- Why we patch (fn-243 R1): Chrome compiles a synchronous pipeline on its GPU process's main thread. The `grow` pipeline takes 13 to 17 s cold. On a cold profile the GPU watchdog killed the GPU process 19 s into the first build.
- Upstream: issue gfx-rs/wgpu#3794. Pull request gfx-rs/wgpu#10438 is open, by abdymazhit, with head `kazmaps-gl/wgpu:async-pipeline-creation` at `9d3215c105e02aa6ab97360f82a94dbeb7d45630`. It adds `create_render_pipeline_async` and `create_compute_pipeline_async` on every backend, returning `impl Future<Output = Result<_, Error>>`. Its comments say another engine carries it on 30.0.1 through `[patch.crates-io]` (maxcelar/wgpu@57e052f). The latest wgpu release is 30.0.1 (2026-08-22).
- The owner's fork: `DanielKillenberger/wgpu`, created 2026-10-24. The host has commented on #10438 with our case.

## Unknown, checked 2026-10-24

Resolved by the conductor's trial cherry-pick: #10438 at `9d3215c1` touches `wgpu` and `wgpu-core`. Its browser half (`wgpu/src/api/device.rs`, `wgpu/src/backend/webgpu.rs`, the `GpuPipelineError*` bindings, `dispatch.rs`) applies cleanly to v30.0.1. Its native half conflicts in `wgpu-core/src/device/resource.rs` and `wgpu/src/backend/wgpu_core.rs`, because it is written against trunk's `Arc<Device>` wgpu-core API, which v30.0.1 does not have. maxcelar/wgpu@57e052f ports the PR's older revision `714c674` and rewrites the native error path; it is not used. The host ruled option C, made complete (2026-10-24); R1 and R2 carry the ruling.

The original questions:

- Which wgpu crates #10438 touches (wgpu, wgpu-core, wgpu-types, ...). Every touched crate must come from the fork at one commit, or the build mixes versions.
- Whether #10438's revision applies to the v30.0.1 tag, or needs a backport the way maxcelar's does. Read maxcelar/wgpu@57e052f as a reference; copy nothing from it without understanding it.

## Requirements

- **R1:** branch `30.0.1-async` on `DanielKillenberger/wgpu` is the v30.0.1 tag plus three things (host ruling, 2026-10-24):
  1. #10438's browser half at `9d3215c1`, verbatim: `wgpu/src/api/device.rs`, `wgpu/src/backend/webgpu.rs`, the `GpuPipelineError*` bindings, and the `dispatch.rs` interface changes, with the registration lines those need (`webgpu_sys/mod.rs`, `wgpu/Cargo.toml`, `xtask/src/vendor_web_sys.rs`).
  2. The native backend's side of those dispatch methods, done the way wgpu's maintainers named in #3794: call the existing synchronous `create_*_pipeline` and return an already-ready future holding `Ok(pipeline)`. Errors stay on today's synchronous error path. `wgpu-core` is untouched: no change to native error handling, and nothing from maxcelar.
  3. A branch README: the branch carries #10438's browser half, with a minimal native implementation in place of the PR's native half; it names the upstream PR and its head `9d3215c1`, and says the branch is dropped when a wgpu release carries #10438.
  Push it. The owner authorised pushing to their fork.
- **R2:** the workspace's `[patch.crates-io]` takes every crate the build resolves from the wgpu family from that fork at one pinned `rev`: wgpu, wgpu-core, wgpu-hal, wgpu-types, naga, and wgpu-naga-bridge (the last for the dev-dependencies). A check shows `cargo tree` resolves exactly one of each, not just one wgpu. `vendor/wgpu-30.0.1-async/` is deleted. `vendor/README.md` is replaced by a short section in `docs/` naming the fork, the rev, the upstream PR and when to drop it.
- **R3:** `programs.rs` calls #10438's `create_compute_pipeline_async`, adapting to its `Result<_, Error>`. Off the web backend the native path still compiles pipelines as today.
- **R4 (proof):**
  - The gate is green, and `npm test` is green.
  - The Wasm render module builds, and its size against the budget is stated.
  - fn-243's test or probe that the browser compiles off the GPU process's main thread passes, on the test it already uses.
  - `cargo tree` shows exactly one of each wgpu-family crate R2 names.
  - A codex review per task, dispatched inside telperion-heavy.

## Boundaries

- No other wgpu upgrade, and no behaviour change.
- No comment or pull request upstream beyond the host's comment.
- Joins the release (fn-237).
