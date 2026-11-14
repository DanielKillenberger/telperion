# The wgpu fork

The workspace takes the whole wgpu family (wgpu, wgpu-core, wgpu-hal,
wgpu-types, naga, wgpu-naga-bridge, and the crates they pull in) from one
commit of a fork, through `[patch.crates-io]` in the root `Cargo.toml`
(fn-311).

- Fork: https://github.com/DanielKillenberger/wgpu, branch `30.0.1-async`.
- Rev: `165ebd16f930baefecca51c22e753a48f911a5fa`, pinned as the full sha.
- Base: tag `v30.0.1`, `40f4a34eb`.
- Upstream: gfx-rs/wgpu#10438, head `9d3215c1`, for issue gfx-rs/wgpu#3794.

**Why.** Chrome makes a synchronous `createComputePipeline` on its GPU
process's main thread. The chain grower's `grow` takes 13 to 17 s to
compile cold natively, and on a cold Chrome profile the GPU watchdog ended
the GPU process 19 s into the first build (fn-243 R1,
`.flow/evidence/fn-243-the-browser-compiles-the-gpu-programs/R1.md`).
`createComputePipelineAsync` compiles off that thread; wgpu 30.0.1 has no
public way to call it.

**What the branch carries.** #10438's browser half, verbatim:
`Device::create_render_pipeline_async` and `create_compute_pipeline_async`,
returning `impl Future<Output = Result<_, wgpu::Error>>`, with the
browser's `GPUPipelineError` as the error. Natively the branch does not
take the PR's half: the methods call the synchronous creation and return a
ready future, so native errors stay on the error scope, as before.
wgpu-core is unchanged from v30.0.1. Telperion's one caller is
`crates/telperion-render/src/programs.rs`.

**When it goes.** The fork is dropped when a wgpu release carries #10438:
delete the patch table and move to that release.

**Re-pinning.** Push the change to the fork's branch, set every `rev` in
the patch table to the new full sha, run `cargo update --workspace`, and
run `npx vitest run scripts/wgpu-family.test.mjs`: it fails unless
`Cargo.lock` takes exactly one of each family crate, all from the fork at
that rev.
