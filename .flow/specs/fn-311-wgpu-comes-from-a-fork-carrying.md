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
  Push it. The owner authorised pushing to their fork. Errors: a path of the browser half that no longer applies verbatim, or a fork that fails `cargo check` natively or for `wasm32-unknown-unknown`, stops before the push; a push is never forced and never goes to gfx-rs.
- **R2:** the workspace's `[patch.crates-io]` takes every crate the build resolves from the wgpu family from that fork at one pinned `rev`: wgpu, wgpu-core, wgpu-hal, wgpu-types, naga, and wgpu-naga-bridge (the last for the dev-dependencies). A check shows `cargo tree` resolves exactly one of each, not just one wgpu. `vendor/wgpu-30.0.1-async/` is deleted. `vendor/README.md` is replaced by a short section in `docs/` naming the fork, the rev, the upstream PR and when to drop it. The `rev` is the full commit sha, never a branch. "Every crate the build resolves" includes the family crates the six pull in (`naga-types`, `wgpu-core-deps-*`) whenever the lock would still take one from crates.io. Errors: the check fails when any wgpu-family crate in `Cargo.lock` comes from crates.io, appears twice, or comes from another rev. A cargo "patch was not used" warning on any entry is reported to the host, not silenced.
- **R3:** `programs.rs` calls #10438's `create_compute_pipeline_async`, adapting to its `Result<_, Error>`. Off the web backend the native path still compiles pipelines as today. Errors: a refused program still settles as `RenderError::ProgramRefused` naming the program, with the reason in the old `<reason>: <message>` shape (`validation: …`, `internal: …`), so the wording a user reads does not change.
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
- The fork's `create_render_pipeline_async` gets no telperion caller; only the compute call site moves.
- The owner's cold-profile Chrome run (fn-243 R4) is the owner's; this spec reruns the automated tests fn-243 uses and lists the cold-profile run as open for the owner.

## Early proof point

Task fn-311-wgpu-comes-from-a-fork-carrying.1 validates the core approach (the browser half applies verbatim and the minimal native side compiles on both targets). If it fails, re-evaluate the host's option C before continuing with fn-311-wgpu-comes-from-a-fork-carrying.2.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration**: the renderer's one wgpu comes from a pinned, reviewable fork instead of a 42,000-line vendored copy, shrinking the release PR (fn-237).

## Decision Context

- Host ruling 2026-10-24, option C made complete: the PR's native half is not backported, because it rewrites wgpu-core's error handling against an API v30.0.1 lacks; the native side is the ready-future form #3794's maintainers named.
- The one-of-each check reads `Cargo.lock`, which records every target's resolution at once, so a wasm-only duplicate cannot hide; a `cargo tree -d` run per target is kept as evidence.

## Memory findings

| Track | Category | Entry | Why relevant |
|---|---|---|---|
| knowledge | workflow | One heavy run at a time across workers; 8 GB free first | every cargo, review and gate run here goes through telperion-heavy, one at a time |

## Quick commands

```bash
~/.local/bin/telperion-heavy 5 --threads 2 -- cargo tree -p telperion-render -d
~/.local/bin/telperion-heavy 5 --threads 2 -- cargo check --profile ci -p telperion-render --target wasm32-unknown-unknown
~/.local/bin/telperion-heavy 5 --threads 2 -- npx vitest run scripts/wgpu-family.test.mjs
```

## Requirement coverage

| Req | Task(s) | Note |
| --- | --- | --- |
| R1 | .1 | |
| R2 | .2 | |
| R3 | .2 | |
| R4 | .2 | The per-task codex reviews are the work stage's |
