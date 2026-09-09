---
satisfies: [R1, R5]
---
# fn-23-fast-hero-the-oak-inside-the-frame.3 Selection on the GPU: one compute pass, one indirect draw per level, a bucket for the unseen

## Description
Replace the single instanced foliage draw with per-frame level selection (spec Architecture bullets two to five). A compute pass classifies every instance by projected deviation into a level or the none bucket and writes per-level index lists, counters and indirect arguments; the foliage pass issues one indexed indirect draw per level. Stateless, WebGPU core only, both targets. Timing of the pass is task 4.

**Size:** M
**Files:** `crates/telperion-render/src/select.rs` (new), `crates/telperion-render/src/shaders/select.wgsl` (new), `crates/telperion-render/src/shaders/foliage.wgsl`, `crates/telperion-render/src/foliage.rs`, `crates/telperion-render/src/submit.rs`, `crates/telperion-render/src/device.rs`, `crates/telperion-render/src/lib.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/examples/headless.rs`, `crates/telperion-render/tests/submit.rs`, `crates/telperion-render/tests/conformance.rs`
**Touches:** [crates/telperion-render/src/**, crates/telperion-render/examples/headless.rs, crates/telperion-render/tests/**]

### Approach
- Buffers, allocated through `buffer::Region`/`Held` (`crates/telperion-render/src/buffer.rs`): placements as a read-only storage buffer (the same 64-byte matrices, no longer a vertex buffer); one index list per level, each sized to the instance count; one counter per level; one indirect argument block per level (index count, instance count, first index, base vertex, first instance). Extend `submit::fits` (`submit.rs:23-59`) to name each of them.
- Compute shader, one thread per instance, workgroup of 256: read the placement, take the element's extent (uniform) through the placement's scale and the camera's projection to a pixel size, then choose the coarsest level whose deviation times pixels-per-metre at that depth is under 0.5; a placement whose bounding sphere is outside the frustum takes the none bucket. Accumulate a per-workgroup count per level in workgroup memory, reserve a range with one atomic add per level per workgroup, then write indices. Levels and deviations come in a uniform written at submit.
- Before dispatch each frame, zero the counters (`clear_buffer` on the encoder) and rewrite the indirect blocks' constant fields; after dispatch the argument blocks carry the instance counts. Dispatch zero workgroups for zero instances and skip the draws.
- Foliage vertex shader reads `placements[list[level][instance_index]]`; the level's list is bound per draw through a dynamic offset or a per-level bind group. Remove the per-instance vertex buffer layout at `foliage.rs:15-18`.
- `Foliage::draw` (`foliage.rs:180-205`) issues one `draw_indexed_indirect` per level over the level's index range. Bare view skips the dispatch and draws nothing; leaf view draws the finest level once at identity, no dispatch, as at `foliage.rs:186-188`.
- `Renderer::draw` (`lib.rs:144-190`) gains the compute pass before the vegetation pass; keep the room pass untouched. Statistics: `instances` stays the submitted count, `triangles` and `draw_calls` report what was issued (one indirect draw per level with a non-zero count is one draw call; triangles are the upper bound from the index ranges). No readback in this task.
- Device request (`device.rs:92-166`): confirm the default limits cover storage buffers in the vertex stage and the compute workgroup size; request nothing new unless a limit forces it, and name the limit in the error if it does.
- Replace the `--level quad` stand-in from task 1 with `--level <n>` that forces every classified instance to level n (still through the compute pass, so the pass is measured too); remove the quad code.
- Tests: a device test that submits oak and spruce and checks the sum of per-level counts plus the none bucket equals the instance count (read back once, test only); the fit check test names the new buffers; a zero-instance submit draws nothing without error; the conformance sweep (`tests/conformance.rs`) still renders every preset and twenty random sets with a non-background still.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/foliage.rs:74-216` — upload and the draw to replace
- `crates/telperion-render/src/lib.rs:144-226` — frame graph and the pass helper; `:230-276` the pipeline helper
- `crates/telperion-render/src/submit.rs:23-59` — the fit check to extend
- `crates/telperion-render/src/shaders/foliage.wgsl` — the vertex input to rework
- `crates/telperion-render/src/scene.rs:270-306` — the shared uniform bind group and its layout
- `crates/telperion-render/src/device.rs:92-166` — features and limits requested

**Optional** (reference as needed):
- `crates/telperion-render/src/buffer.rs` — allocate-with-headroom pattern
- `crates/telperion-render/tests/common/mod.rs` — the adapter-optional test pattern
- `crates/telperion-render/src/camera.rs:61-111` — hero pose and field of view, for the pixels-per-metre term

### Key context
- The renderer has no compute pipeline, storage buffer or indirect draw today; this task introduces all three. Keep them in `select.rs` and its shader so no existing file crosses about 400 lines.
- No hysteresis and no previous-frame state; the spec rejected it until observed.
- `wgpu` 30. Read-only storage buffers in the vertex stage and timestamp writes on compute passes are WebGPU core; multi-draw and indirect count are not, and are not used.
- The panel text parsers in `tests/browser/render.mjs:63-72` expect the same three numbers; keep `FrameStats` shape and semantics as stated above.

## Acceptance
- [ ] Whole view draws foliage through one indexed indirect draw per level, with the per-level counts plus the none bucket summing to the submitted instance count for oak and spruce at seed 7
- [ ] A leaf whose projected deviation at the coarsest level is under half a pixel takes the coarsest level; forcing `--level <n>` places every visible instance in level n
- [ ] Bare view dispatches no compute and draws no foliage; leaf view draws exactly one instance of the finest level with no dispatch
- [ ] Zero foliage instances submit, draw and frame without error
- [ ] `submit::fits` names every new buffer and refuses an oversize tree whole, tested without a device
- [ ] `FrameStats.instances` equals the submitted count; `draw_calls` and `triangles` reflect the issued indirect draws
- [ ] The `quad` stand-in from task 1 is gone; `--level <n>` out of range exits non-zero naming the range
- [ ] Every shipped preset and twenty random parameter sets render a non-background still through the same path; no anatomy or preset name appears in `select.rs`, `select.wgsl` or `foliage.rs`
- [ ] `cargo test --release -p telperion-render` passes or skips without an adapter; `npm run test:render` conformance passes under hardware WebGPU

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
