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
The crown reaches the pixels: one instanced draw takes the core's own placement matrices up as they lie, the element mesh goes with them once, and both faces of a leaf light so a crown seen from underneath is a crown rather than a field of holes. `submit` now reports the wood counts, the placement count and the bounds it was handed, and refuses a tree larger than the device granted by naming the buffer and both sizes instead of quietly truncating it. The three views the harness offers are back on the new renderer, and the conformance sweep holds every shipped family and twenty generated parameter sets to the same submit and the same frame.

Scope notes for the reviewer:

- **`submit` keeps task 2's signature; no `Transform` was added.** The task's approach sketch writes `submit(&TreeMesh, Transform)`, but nothing in this spec places a tree anywhere but the origin, and neither shader carries a model matrix. A parameter every caller passes as the identity, threaded through two pipelines to be multiplied by nothing, is dead API that lies about a capability. The acceptance asks for counts and bounds, and those it returns. A placement transform earns its keep in the forest spec, where there is a second tree to place.
- **`hero_pose` had two tree-scale constants that the leaf view exposed.** The eye was floored at the scale figure's 1.8 m and the near plane fixed at 0.1 m; against a 2 cm needle that put the camera eight metres of its own length away and clipped what was left. Both now scale with the subject: the eye-height floor applies only to a subject taller than a person, and the near plane is the smaller of a tenth of a metre and half the reach. The tree poses are bit-identical — every task 2 camera test passes untouched — and the new case was confirmed red against the old constants before the fix.
- **Leaf normals are computed, not shipped.** The core's `Element` carries positions and indices only. The renderer folds area-weighted vertex normals over the element once per tree, so cup and curl read under the same hemisphere as the wood, and `@builtin(front_facing)` flips the normal toward the eye rather than a second uniform.
- **The leaf view draws no room.** A 400 m ground disc and a 1.8 m figure behind a 2 cm needle are a wall, not a scale reference. Whole and bare keep both.
- **`Region` and the held buffers moved to `buffer.rs`.** Foliage needs exactly the wood's allocate-with-headroom and reuse-if-it-fits behaviour; extracting it beat a second copy. `wood.rs` fell from 256 lines to 114 and the reuse decision is still tested without a device.
- **`fits` judges the allocation, not the payload.** Buffers are taken with a quarter again of headroom, and the headroom is what the device has to grant; checking the payload alone would pass a mesh that then failed at allocation.
- **A non-blank still proves nothing on its own.** The room is always in frame, so `has_subject` is satisfied by the ground disc even when no tree was drawn. Every conformance assertion is against the frame statistics measured on the tree's own submitted counts, not against the picture not being flat.
- **The random sweep walks unsigned leaves too.** `canopy.maxInstances` is `usize::MAX`; read through `as_i64` it fell out as a float and the schema refused all sixty sets. The walk now takes `as_u64` first and lets the casts saturate. Yield is 20 rendered of 25 tried, the five refusals being ranges the jitter pushed out of bounds — which is the loop working, not failing.
- **Inherited red, not caused here:** `cargo clippy --workspace --all-targets -- -D warnings` fails in `crates/telperion-core/examples/geometry_benchmark/metrics.rs:238` (`assign_op_pattern`). That file is untouched by this task and outside its declared Touches, so it is recorded rather than fixed. Clippy is clean for `telperion-render` itself, and `cargo test --release --workspace` is green.
- `Cargo.lock` moved as the mechanical consequence of the dev-dependency added to `crates/telperion-render/Cargo.toml`; it is the only path in the diff outside the declared Touches.
- Every file is under 400 lines; the largest is `src/scene.rs` at 347.

stage: impl-review - skipped(config: REVIEW_MODE=none; parallel wave - conductor reviews after integration)
stage: gate-classify - ran (FULL; full workspace suite green, receipt a80d6bb1-unittest)

Conductor decisions: no Transform on submit accepted (nothing places a tree off the origin in this spec; the forest spec adds it); subject-scaled eye floor and near plane accepted; the inherited clippy red in the core geometry_benchmark example is out of scope and goes to Phase 4.

stage: plan-sync - skipped(config: planSync.enabled != true)
stage: wave-join - ran (commit a80d6bb already on target; no integration needed)
## Evidence
- Commits: a80d6bb1ff92f2f6d9a4c3d096be2692390d30cb
- Tests: cargo test --release --workspace: passed (green receipt a80d6bb1-unittest), cargo test --release -p telperion-render: passed (19 unit, 4 conformance, 4 submit, 2 headless), cargo fmt --all -- --check: passed, cargo clippy --release -p telperion-render --all-targets -- -D warnings: passed, cargo clippy --workspace --all-targets -- -D warnings: INHERITED RED in crates/telperion-core/examples/geometry_benchmark/metrics.rs:238 (assign_op_pattern), untouched by this task and outside its Touches, cargo run --release -p telperion-render --example headless -- --preset norway-spruce --seed 1 --size 512x512: 7911960 placements drawn, foliage visible, headless --view leaf on norway-spruce: one needle at generated scale filling the frame, headless --view sideways: exit 1, unknown view named
- PRs: