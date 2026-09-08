---
satisfies: [R3, R4]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.3 Foliage pipeline, submission contract and conformance tests

## Description
Add the instanced foliage draw, the `submit` contract with the pure fit check and the counts report, the three views (whole, bare, single leaf) the harness's QA modes need, and the two conformance tests: counts and bounds equal the core's for oak and spruce (R3), and every preset plus twenty random parameter sets render through one path (R4).

**Size:** M
**Files:** `crates/telperion-render/src/foliage.rs`, `src/submit.rs`, `src/view.rs`, `src/shaders/foliage.wgsl`, `src/lib.rs`, `crates/telperion-render/examples/headless.rs`, `crates/telperion-render/tests/submit.rs`, `crates/telperion-render/tests/conformance.rs`
**Touches:** [crates/telperion-render/**]

### Approach
- `foliage.rs`: element positions converted f64→f32 once in Rust at upload, element indices as is, per-instance vertex buffer of `[f32; 16]` as four `Float32x4` attributes with `VertexStepMode::Instance`, cast straight from `Instances.matrices`; one `draw_indexed` over all instances; cull mode none with the normal flipped toward the eye so both leaf faces light; the same hemisphere term as wood; headroom and used range like wood.
- `submit.rs`: `fits(limits: &Limits, mesh: &TreeMesh) -> Result<(), RenderError>` checks wood positions, normals, indices and foliage instance bytes against `max_buffer_size`, naming the buffer and both sizes; `Renderer::submit(&TreeMesh, Transform) -> Result<Submitted>` calls it first, uploads, and returns counts and bounds read from the mesh (`Submitted { wood_vertices, wood_triangles, foliage_instances, bounds }`). Frame stats add instances.
- `view.rs`: `View::{Whole, Bare, Leaf}` and `set_view`; `Whole` draws wood and foliage, `Bare` wood only, `Leaf` one element instance at generated scale with an identity placement at the origin, framed by `hero_pose` on the element's own bounds (the old `selectSpecimenView` at `harness/skeleton-view.ts:498-520` is the behaviour to match, not code to port). Frame stats reflect the view.
- `tests/submit.rs`: for oak and spruce, `Submitted` equals `TreeMesh` counts and bounds (skip without adapter); `fits` unit-tested against a synthetic small limit without a device, asserting the error names the buffer; each view renders non-blank and `Bare` reports zero instances.
- `tests/conformance.rs`: (1) all five presets at full detail render 128×128 offscreen and the still is not all background; (2) random sets: serialize each preset with `params::metadata`, walk numeric JSON leaves with a seeded generator, scale each by a factor in [0.8, 1.25] keeping integers integral, parse with `params::parse`, and render those that generate; sets start from a compact family (height about 4 m, attractors about 40, as `tests/browser/integration.mjs:5-17` does) so the loop stays near a minute; keep going until at least twenty rendered; (3) one deliberately invalid set (negative height) asserts the generator's message names the parameter; (4) a source check reads `crates/telperion-render/src/*.rs` and asserts no `presets` token.
- `examples/headless.rs` now submits foliage and takes `--view whole|bare|leaf` (default whole); an unknown view name exits non-zero.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/foliage.rs:42-70` — `Instances`, validation, bounds
- `crates/telperion-core/src/foliage/element.rs:64-72,111` — `Element` positions and indices
- `crates/telperion-core/src/params.rs` — `metadata` and `parse` for the random-set walk
- `harness/skeleton-view.ts:498-520` — the three views' meaning
- `tests/browser/integration.mjs:5-17` — the compact fixture shape

**Optional** (reference as needed):
- `crates/telperion-core/tests/species.rs:150-200` — per-preset chain in tests

### Key context
- Full-detail spruce is about 5.2 million instances, 330 MB of matrices, above wgpu's 256 MiB default; the device from task 2 already requests the adapter's `max_buffer_size`, and `fits` judges against the granted limit.
- JSON numbers that are integers must stay integers after scaling or `parse` rejects them.
## Acceptance
- [ ] `tests/submit.rs`: for oak and spruce, submitted wood vertex count, wood triangle count, foliage instance count and bounds equal the core's `TreeMesh`; `fits` rejects a synthetic oversize with an error naming the buffer and sizes; whole, bare and leaf views each render non-blank and bare reports zero instances; all skip cleanly without an adapter
- [ ] `tests/conformance.rs`: five presets plus at least twenty random valid sets render non-blank through the same `submit` and `frame` path; at least one rejected set surfaces the generator's message naming the parameter; the source check finds no `presets` token in the renderer library
- [ ] Headless still for spruce at seed 1 shows foliage; `--view leaf` shows one needle or leaf at generated scale; an unknown view exits non-zero
- [ ] `cargo test --release -p telperion-render` passes; clippy clean; every file under 400 lines
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
