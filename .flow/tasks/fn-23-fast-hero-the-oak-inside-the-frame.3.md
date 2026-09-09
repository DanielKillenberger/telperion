---
satisfies: [R1, R5]
---
# fn-23-fast-hero-the-oak-inside-the-frame.3 Selection on the GPU: one compute pass, one indirect draw per level, a bucket for the unseen

## Description
Replace the single instanced foliage draw with per-frame level selection (spec Architecture bullets two to five). A compute pass classifies every instance by projected deviation into a level or the none bucket and writes per-level index lists, counters and indirect arguments; the foliage pass issues one indexed indirect draw per level. Stateless, WebGPU core only, both targets. Timing of the pass is task 4.

**Size:** M
**Files:** `crates/telperion-render/src/select.rs` (new), `crates/telperion-render/src/shaders/select.wgsl` (new), `crates/telperion-render/src/shaders/foliage.wgsl`, `crates/telperion-render/src/foliage.rs`, `crates/telperion-render/src/submit.rs`, `crates/telperion-render/src/device.rs`, `crates/telperion-render/src/lib.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/examples/headless.rs`, `crates/telperion-render/tests/submit.rs`, `crates/telperion-render/tests/conformance.rs`
**Touches:** [crates/telperion-render/src/**, crates/telperion-render/examples/headless.rs, crates/telperion-render/tests/**, crates/telperion-core/src/foliage/levels.rs, crates/telperion-core/tests/foliage.rs]

### Approach
- First, thin the ladder task 2 built (conductor amendment, 2026-09-09). The halving loop in `crates/telperion-core/src/foliage/levels.rs` (`build`, the `while tolerance` loop) emits a level at every tolerance step, which gives the oak 13 levels from 4 to 268 triangles with a fine tail that buys almost nothing (level 11 is 250 triangles against 268). Emit a coarse level only when it carries at least twice the triangles of the previously emitted level; the finest level stays the element's own indices byte for byte and the pins in `tests/identity.rs` must not move. Expect about seven oak levels. Update the level tests in `crates/telperion-core/tests/foliage.rs` for the new counts; nothing else in the core changes. Commit this on its own before the renderer work.
- The core's `telperion_core::foliage::Level` now shares a name with the renderer's task-1 `Level::{Full, Quad}`; this task deletes the renderer one, so the clash resolves itself. Coarse triangles live in the element's `level_indices` buffer with the finest level as the last range; today's `indices` are untouched, so the leaf view and the normals computation can keep reading them.
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
The oak hero's vegetation pass, including selection, now runs at a p50 of
1.677 ms and a p95 of 1.843 ms at 1600 by 1000 on the RTX 3080 through the
native headless target, from fn-22's 18 ms. In the browser on the same
machine it reads 2.196 ms p50 over 120 frames. Task 4 owns the R1 verdict;
these are the numbers the path it will measure produced on the way past.

Two commits.

The first thins the ladder task 2 built. The halving loop emitted a level at
every tolerance step, which gave the oak 13 levels from 4 to 268 triangles
with a tail that bought nothing (level 11 was 250 triangles against 268), and
each of those levels costs an index list, a counter and a draw call every
frame. A level is now emitted only when it carries at least twice the
triangles of the last one emitted. The oak gets 6 levels (4, 12, 28, 56, 128,
268), the spruce 3 (14, 30, 56), the generic grid 3 (4, 12, 16). The finest
level is still the element's own index list byte for byte and the identity
pins in `tests/identity.rs` did not move.

The second replaces the single instanced foliage draw. A compute pass
(`select.rs`, `select/frame.rs`, `select/bind.rs`, `shaders/select.wgsl`)
takes one thread per placement, projects the element's deviation at that
leaf's depth, and takes the coarsest level under half a pixel; a placement
whose bounding sphere falls outside the frustum takes the unseen bucket.
Each workgroup of 256 tallies its own threads in workgroup memory and takes
one atomic range per level, so 555 thousand leaves cost about 2,170 atomic
reservations rather than 555 thousand. The foliage pass then issues one
indexed indirect draw per level. The placements moved from a per-instance
vertex buffer to a read-only storage buffer that the vertex shader reads
through the level's own list, bound at that level's dynamic offset.

Six decisions worth naming.

`Renderer::draw` now takes the viewport in pixels where it took an aspect.
Half a pixel is the whole rule, and the pixel height is the only thing that
turns a deviation in metres into pixels; the aspect is derived from the pair.
That change reaches `measure`, `Session::sample`, `headless::render`, `web.rs`
and the timing test.

The device now asks for the adapter's own `max_storage_buffer_binding_size`
beside its `max_buffer_size`. The spruce's 7.9 million placements are 505 MB
of storage buffer and the WebGPU default caps a storage binding at 128 MiB, so
the spruce would have been refused whole. This is the "unless a limit forces
it" case the task named; the refusal path already prints the limit that failed.

`RenderError::TooManyLevels` is new. The per-workgroup tally is sized when the
shader compiles, at 16 levels plus the bucket, so `fits` refuses a longer
ladder by name rather than letting the pass write past it. The doubling rule
puts 16 levels at an element of 32 thousand triangles, so nothing the core
builds today comes near it.

`FrameStats` keeps its three fields and its parsers. `instances` stays the
submitted count. `draw_calls` counts the indirect draws issued, one per level,
because no count comes back from the device in this task. `triangles` is the
upper bound the issued draws can reach, the crown at its finest, which is the
same 149 million the panel showed before; it is an upper bound and not a
measurement until task 4 reads the counters back.

`select.rs` came to 680 lines, so it is three files: the pass, the frame it
decides against (frustum planes, eye, pixels per metre) and the bind groups.
Every renderer file is now under 400 lines except `timing.rs` at 413, which
the spec already parks for its own split.

`buffer.rs` gained `reserve`, which allocates without uploading, for the four
buffers the GPU is the one that fills, and `read`, which brings a buffer's
live range back for `Renderer::level_counts`.

For task 4: `Renderer::level_counts()` returns the per-level counts with the
unseen bucket last, which is what the timing record wants; the compute pass
takes `timestamp_writes: None` today and that second pair is task 4's. The
browser rig writes its timing records into `.flow/evidence/fn22` unless
`RENDER_EVIDENCE` names somewhere else, so running `npm run test:render` here
overwrote fn-22's browser numbers; the overwrite is out of the tree in
`git stash@{0}` and fn-22's evidence is as it was committed. Task 5 should set
`RENDER_EVIDENCE=.flow/evidence/fn23`. Worth recording for the parked spruce
question: the browser spruce read 29.428 ms p50 in this run against fn-22's
63 ms, on the same machine and the same tree.

Tests: the level ladder's doubling and its length are asserted per species in
the core; `fits` names each new buffer and refuses each one on its own limit
without a device; a device test sums the per-level counts and the bucket to
the submitted crown for oak and spruce at seed 7, holds every leaf at the
coarsest level at 256 pixels, watches the oak outgrow it at 2,048, and forces
one level; the bare and leaf views are shown to leave the counters untouched,
which is how "no dispatch" is observed; a crown of no leaves submits, draws
and frames. `npm run test:render` passes on hardware WebGPU with 5 presets,
the dial, the three views and 2 timing sessions.

Not done here, by scope: no timestamps on the compute pass (task 4), no orbit
session (task 4), no readback in a frame, no hysteresis.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 397e13da95f10e9e69ea508cb25166faf0d4597e, 407fc921a0cab82a5be81080631cb4ee3527caad
- Tests: baseline: green via handoff (verified at d556b54 by fn-23-fast-hero-the-oak-inside-the-frame.2), cargo test --release --workspace (119 tests, suite_rc=0; green receipt .flow/tmp/green-receipts/407fc921-unittest.json), npm run test:render (browser conformance PASS on hardware WebGPU: 5 presets, height dial, three views, 2 timing sessions), cargo clippy --release --workspace --all-targets (clean), cargo check --release --target wasm32-unknown-unknown -p telperion-render, cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --timing (vegetation p50 1.677 ms, p95 1.843 ms, valid), cargo run --release -p telperion-render --example headless -- --preset ordinary --seed 7 --level 99 (exit 1, names the range 0 to 2)
- PRs:
stage: plan-sync - skipped(config: planSync.enabled != true)
