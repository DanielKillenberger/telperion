---
satisfies: [R2, R3]
---
# fn-301-a-tree-rests-on-its-ground.3 Engine wood rests its girth and keeps its step; the sweep keeps bark off the ground

## Description
The lay walk's resting clamp becomes R2's rule on both executors. A non-trunk step whose bark would pass below the ground ends with its centreline at `radius` and keeps its length, along the horizontal part of its travel. This fixes the birch, the spruce at 40, the ordinary tree and the oak's wood. The radius is the girth (host ruling). The engine never gets surface rows. Second part: where the sweep emits ring vertices, on each executor, a vertex of non-trunk wood gets `y = max(y, 0)`, which keeps the flare and the lobes off the ground (spec, "Surface clearance").

**Size:** M
**Files:** `crates/telperion-space/src/geometry/lay.rs`, `crates/telperion-render/src/generation/cycle/lay_walk.wgsl`, `crates/telperion-space/tests/sag.rs`, and the settle agreement tests that move
**Touches:** [crates/telperion-core/src/pipeline/surface/**, crates/telperion-render/src/curve/**, crates/telperion-render/src/curve/**/*.wgsl, crates/telperion-core/tests/ground_reach.rs, crates/telperion-space/src/geometry/lay.rs, crates/telperion-space/src/invariants.rs, crates/telperion-space/tests/**, crates/telperion-space/src/grow/**, crates/telperion-render/src/generation/cycle/**]

### Approach
- The clamp is `lay.rs:373-380` (CPU) and `cycle/lay_walk.wgsl:219-223` (device). The landing ease (`lay.rs:357-364`, `lay_walk.wgsl:206-211`) and the trunk fault (`lay.rs:370`) stay unchanged.
- The test is `next.z < radius` (bark below the ground), not `next.z < 0`. The resting step keeps `length`: its horizontal part is rescaled. A vertical step takes the heading's horizontal part. With none, it is a named stop, matched on the device (no fallback).
- Radius is final at step time (`grow/settle.rs:123-175`; place runs twice under sag). The device reads `laid_radius(r)`. `cycle/layout.wgsl:171` stubs it to 0 for modules that do not lay: check that every module that lays reads the true radius, and that the CPU's matching pass (rough layout as well as the settle) does the same. Otherwise the two executors rest at different heights.
- Sag reads tip heights (`sag.rs:258`, `sagged.rs:38,71`). Resting at `radius` changes the levers. That is expected; the device must follow the same arithmetic.
- Update `tests/sag.rs:117` (`sagging_wood_rests_on_the_ground`) to the bark: lowest tip `z ≥ radius − ε`, some tip within ε of its radius. Add a unit test that the rested step keeps its length, and one for the vertical-step case.
- Surface clearance: on the CPU, find where the sweep writes ring vertices (`crates/telperion-core/src/pipeline/surface/build.rs` and the curve's `tessellate`; check both, since the curve is the renderer's wood). On the device, find the surfacing pass's `emit` (`crates/telperion-render/src/curve/`, the WGSL that writes ring vertices). Apply `y = max(y, 0)` to non-trunk runs only (`CurveRun::trunk`). Leave the normals as they are and say so in the commit. It is an identity on clear trees: test that a clear preset's mesh is bit-identical.
- `tests/ground_reach.rs`: read the wood from the drawn CPU vertices (the sweep's output), not from the curve's ring bound, leaving out the trunk's runs below the root. Then the probe sees the clearance.
- The settle and lay agreement suites (`cycle/settle_tests.rs`, `settle_det_tests.rs`, `settle_float_tests.rs`) must pass.

### Investigation targets
**Required:**
- `crates/telperion-space/src/geometry/lay.rs:228-440`
- `crates/telperion-render/src/generation/cycle/lay_walk.wgsl:180-240`
- `crates/telperion-render/src/generation/cycle/settle_place.wgsl:1-20`, `cycle/layout.wgsl:160-180`
- `crates/telperion-space/tests/sag.rs:100-240`
**Optional:**
- `crates/telperion-space/src/invariants.rs`, `crates/telperion-space/src/grow/settle.rs:123-175`

### Key context
This is the FINE-CONTRACT's lay decision. Every float the device takes must be the CPU's operation in f32, in the same order (`docs/pipeline.md`, "The one cycle"). A tree whose wood never rests takes exactly today's operations (R3).
## Acceptance
- [ ] On both executors a resting non-trunk step keeps its length and sets its centreline at its radius; trunks still fault.
- [ ] Unit tests: length kept, the vertical step, and the named stop for a step with no horizontal direction.
- [ ] `tests/sag.rs` holds the bark at the ground; the settle agreement suites pass.
- [ ] On both executors, the sweep's emitted vertices of non-trunk wood are never below the ground; the trunk's buried foot is unchanged.
- [ ] `ground_reach` reads the drawn wood vertices; no wood row is below the ground once .3 lands.
- [ ] A preset whose wood never rests (the beech at 40, seed 1) has an unchanged `artifact_digest`.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
