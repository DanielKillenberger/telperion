---
satisfies: [R1, R2, R6]
---
# fn-27-coarse-shadow-casters-and-a-filtered.2 The foliage stride with a scaled caster quad, and the filtered comparison with a normal offset

## Description
Stride the foliage depth draw over the placement buffer with each kept leaf's caster quad scaled to preserve coverage (R1 foliage half), and widen the shared shadow read to a kernel with a normal offset (R2's mechanism). The row values exist from task 1; this task reads them.

**Size:** M
**Files:** `crates/telperion-render/src/foliage.rs` (strided instance count), `crates/telperion-render/src/shadow.rs` (light uniform widened: stride and quad scale beside the matrix), `crates/telperion-render/src/shaders/shadow.wgsl` (index times stride, quad scale), `crates/telperion-render/src/shaders/common.wgsl` (kernel and offset in the shared read), `crates/telperion-render/src/shaders/wood.wgsl`, `foliage.wgsl`, `scene.wgsl` (pass the normal), `crates/telperion-render/src/scene.rs` (two uniform slots for filter texels and normal offset), `crates/telperion-render/tests/shadow.rs`, `tests/look.rs`
**Touches:** [crates/telperion-render/src/foliage.rs, crates/telperion-render/src/shadow.rs, crates/telperion-render/src/shaders/**, crates/telperion-render/src/scene.rs, crates/telperion-render/tests/shadow.rs, crates/telperion-render/tests/look.rs]

### Approach
- Stride: `foliage.rs:256-270` draws `0..select.instances()` at level 0; draw `instances.div_ceil(stride)` and let the vertex stage read `placements[instance * stride]` (`shadow.wgsl:15-23`). The light uniform is a bare `mat4` (`shadow.rs:116-121`, `shadow.wgsl:5`); widen it to the matrix plus stride and quad scale, both entry points and the layout together. Scale the caster quad about its centre by the square root of the stride in the same vertex stage; the peg is not scaled.
- Kernel: `common.wgsl:57-71` is the one read; take the receiver normal, move the world point along it by shadowNormalOffset times the texel world size, then average a square of hardware comparisons of radius shadowFilterTexels; a tap outside the map counts as lit. `key(n, world)` at `:86-89` already receives the normal, so the three callers (`wood.wgsl:33`, `foliage.wgsl:77`, `scene.wgsl:37`) change only if the signature does. The depth pipeline's constant bias (`pass.rs:215-219`) stays.
- Uniforms: two slots in the block at `common.wgsl:9-45` mirrored at `scene.rs:37-66` and filled at `:255-335`; `scene.rs` is at 390 lines, so if the fill grows past the rule, move the row-to-uniform fill into `scene/`.
- Clay path: the clay view never reads the shadow; keep it that way so `tests/look.rs:77` holds without a tolerance change.
- Tests: extend `tests/shadow.rs` with a claim for the stride (casterStride 1 covers at least as much as the default; the default covers a non-zero share) and for the kernel (a filtered read at radius 1 is never darker than the unfiltered read averaged over a 3 by 3 block, on the readback). `tests/look.rs` pins hold unchanged.
- Measure once: the oak still with the timing record at the default row; record the pass split beside task 1's numbers, and view one crop of the crown beside fn-14's hero for the dapple; that is the R2 aid for task 3.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/foliage.rs:120-127,256-270` — the shadow pipeline layouts and draw
- `crates/telperion-render/src/shaders/shadow.wgsl` and `common.wgsl:47-89` — the depth entry points and the shared read
- `crates/telperion-render/src/shadow.rs:60-125` — sampler, layouts, light uniform
- `crates/telperion-render/src/scene.rs:37-66,255-335` — the uniform block and its fill

**Optional** (reference as needed):
- `crates/telperion-render/tests/look.rs:77-130` — the clay pin that must not move
- `crates/telperion-render/src/shaders/foliage.wgsl:60-78` — the leaf fragment and its comment about dapple

### Key context
- The stride is fixed by buffer index, never by the per-frame selection; the shadow draw already ignores the level lists.
- A device without linear comparison filtering degrades each tap to a point sample; the kernel still averages, and the record's adapter line says which.
- Budget rules from CLAUDE.md bind: one timing run per change, at most four images viewed.

## Acceptance
- [ ] The foliage depth draw submits every k-th placement at the coarsest level with the caster quad scaled by the square root of k; stride 1 draws every placement
- [ ] The shared shadow read averages a square kernel of hardware comparisons of the row's radius with a normal offset of the row's texels; taps outside the map count as lit; both crown shaders and the ground disc read through it
- [ ] The light uniform carries the stride and quad scale beside the matrix on both the Rust and WGSL sides with one layout
- [ ] The clay, leaf-offset and lit-sky pins hold at their existing tolerances; the shadow test states the stride and kernel claims
- [ ] Oak still and timing record at the default row with the pass split recorded beside task 1's, and one crown crop beside fn-14's hero as the R2 aid
- [ ] `cargo test --release --workspace`, fmt and clippy pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
