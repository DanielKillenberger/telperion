---
satisfies: [R2, R3]
---
# fn-301-a-tree-rests-on-its-ground.3 Engine wood rests its girth and keeps its step; the sweep keeps bark off the ground

## Description
The lay walk's resting clamp becomes R2's rule on both executors. A non-trunk step whose bark would pass below the ground ends with its centreline at `radius` and keeps its length, along the horizontal part of its travel. This fixes the birch, the spruce at 40, the ordinary tree and the oak's wood. The radius is the girth (host ruling). The engine never gets surface rows. Second part: where the sweep emits ring vertices, on each executor, a vertex of non-trunk wood gets `y = max(y, 0)`, which keeps the flare and the lobes off the ground (spec, "Surface clearance").

**Size:** M
**Files:** `crates/telperion-space/src/geometry/lay.rs`, `crates/telperion-render/src/generation/cycle/lay_walk.wgsl`, `crates/telperion-space/tests/sag.rs`, and the settle agreement tests that move
**Touches:** [crates/telperion-render/src/generation/resident/rings.wgsl, crates/telperion-render/src/generation/resident/positions.wgsl, crates/telperion-render/src/generation/resident/rings.rs, crates/telperion-render/src/shaders/curve_emit.wgsl, crates/telperion-render/src/shaders/curve_walk.wgsl, crates/telperion-render/src/generation/resident/sweep.wgsl, crates/telperion-core/src/pipeline/foliage/leaflet.rs, crates/telperion-render/src/generation/rachis.wgsl, crates/telperion-render/src/generation/resident/rachis_tests.rs, docs/generation-limits-inventory.json, crates/telperion-core/src/pipeline/surface/**, crates/telperion-render/src/curve/**, crates/telperion-render/src/curve/**/*.wgsl, crates/telperion-core/tests/ground_reach.rs, crates/telperion-space/src/geometry/lay.rs, crates/telperion-space/src/invariants.rs, crates/telperion-space/tests/**, crates/telperion-space/src/grow/**, crates/telperion-render/src/generation/cycle/**]

### Approach
- The clamp is `lay.rs:373-380` (CPU) and `cycle/lay_walk.wgsl:219-223` (device). The landing ease (`lay.rs:357-364`, `lay_walk.wgsl:206-211`) and the trunk fault (`lay.rs:370`) stay unchanged.
- The test is `next.z < radius` (bark below the ground), not `next.z < 0`. The resting step keeps `length`: its horizontal part is rescaled. A vertical step takes the heading's horizontal part. A step heading straight down at the ground rests along the phytomer's `side`, which is horizontal for a vertical step (host ruling, 2026-10-21; never refused). The device matches it.
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
- [ ] Unit tests: length kept, the vertical step, and a straight-down step resting along `side`, with the CPU and the device equal on it.
- [ ] `tests/sag.rs` holds the bark at the ground; the settle agreement suites pass.
- [ ] On both executors, the sweep's emitted vertices of non-trunk wood are never below the ground; the trunk's buried foot is unchanged.
- [ ] `ground_reach` reads the drawn wood vertices; no wood row is below the ground once .3 lands.
- [ ] A preset whose wood never rests (the beech at 40, seed 1) has an unchanged `artifact_digest`.
## Done summary
Engine wood now rests on its girth, the frond curve has no taper refusal, and every producer of wood ring vertices keeps bark other than a trunk's off the ground, on both executors.

- **Lay (R2, ruling 1).** A non-trunk step whose bark would pass below the ground ends with its centreline one radius up. It keeps its length, along the horizontal part of its travel, or along the phytomer's side where it heads straight down. Its stored side turns with it (`carry`), so it stays square to the heading, and the rested endpoint is checked against the 4,096 m domain (`lay::rested`, `lay_walk.wgsl` `rested`). The landing ease is measured from the bark, `support(tip.z - radius)`, on both executors. sag.rs reads the bark, and its sharpest landing joint is 0.757 rad at sag 6e-4, under decision 4's 0.8.
- **Frond curve (ruling 2).** The taper refusal is deleted. Each resting step advances sqrt(max(0, ds² − dr²)), the same closed form on both executors, and goes straight down onto its own bark where the taper outruns it. `rachis`, `leaflets` and `rachises` are infallible again.
- **Surface clearance (ruling 3, review #3).** A vertex of non-trunk wood below the ground is set onto it (y = 0) wherever ring vertices are made: the CPU sweep (`build::emit_run`, `Curve::rings`), `Curve::tessellate`, the CPU seat rings (`AttachmentSurface::selected`), the device surfacing (`curve_emit.wgsl`) and the device seat vertices (`positions.wgsl`). The trunk's buried foot is unchanged, and normals are left as they are. The trunk bit uses spare bits: cluster flag 8 (bit 3 of word 4; CPU `packed_clusters`, device `resident/sweep.wgsl` `standing`), ring-record flag 64 (bit 6 of word 13, `curve_walk.wgsl` `TRUNK`), and ring word 15 = 1 (CPU `compact::read`, device `resident/rings.wgsl`, read from the run's first cluster).
- **ground_reach** reads the drawn mesh vertices, leaving out a trunk's vertices below the root. No wood row is below the ground on any of the 168 rows (raw/t3-ground-reach.log). Leaves still cross until task .4.

R3 (measured; base d600fa12c, `artifact_digest`, raw/t3-digest-*.txt): the beech, Telperion and Laurelin at their default ages, seeds 1, 7 and 42, are byte-identical. The palm at 40 changes one leaf-base vertex per seed (1.70, 1.89 and 2.36 mm, set onto the ground) and 9 to 10 encoded normals (at most 344/32767). The host accepted this into R3's changing rows (2b3985a03).

Tests: `geometry/lay_tests.rs` (length kept, near-vertical, straight down along side, square frame); `cycle/settle_tests.rs` `a_step_straight_down_rests_along_its_side_on_the_device`; `tests/sag.rs` (bark); `resident/rachis_tests.rs` `a_frond_as_thick_as_it_is_long_rests_on_the_device_as_on_the_cpu`; `tessellate/tests.rs` `wood_other_than_a_trunks_is_surfaced_off_the_ground`; `attachment/tests.rs` `a_seat_on_resting_wood_stands_on_the_drawn_bark`; `tests/curve.rs` `wood_resting_on_the_ground_surfaces_off_it_on_the_device`; `resident/agree_tests.rs` `seats_on_resting_wood_stand_on_the_ground_on_the_device` and the trunk-flag count in `every_preset_sweeps_its_wood_as_the_cpu_does`. Each clearance test was run red with its clamp switched off.

Decisions (host or conductor accepted):
- Edits outside the original Touches, all removals of .2's Result plumbing: `branching/leaf_bases.rs`, `foliage/plan/fronds.rs`, `foliage/rosette.rs`, `foliage/rachis.rs`, `suite/establish.rs`, `suite/rachis.rs`, `suite/frond_contact.rs`.
- Test files outside the original Touches: `telperion-render/tests/curve.rs`, `resident/agree_tests.rs`. Touches were widened for `resident/rings.wgsl`, `positions.wgsl`, `rings.rs`, the shaders and `resident/sweep.wgsl`.
- `docs/generation-limits-inventory.json`: the `speed < 0` entry is reworded as the closed form's zero. 0aa2a7a4f was in the review's scope.
- Review #2 (an edit to .2's done receipt) was declined, and the reviewer withdrew it.

Open: sag's ground support (`sag.rs:258`, `sagged.rs:71`) still reads the centreline; it is on the spec's Open list. The base worktree `.worktrees/f4-301-base` is kept until .5's digests.

stage: impl-review - ran (codex: round 1 three lenses NEEDS_WORK, fixed #1 #3 #4, declined #2; re-review SHIP)
Tier: not given by the conductor (model: claude-opus-5-5)

Gate: `telperion-heavy 8 --gpu 2 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` ran once at 510b551d5: 1,443 passed, 0 failed, 113 ignored, rc 0 (raw/t3-gate.log). Baseline: green (sag, settle suites).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 194df7d37d433ba9429b458fdf3ea979d96e0945, 3d82eafe0ffe36b6e82d794bdda874807469399e, 37c096a93e1719ca5ec4ca70a98d98a1c35e66a7, 2e321bf656769e5b9c46437eac42890674e1e943, d01c5f0d9a06f3d2e3942544fb3e5dfaf3938313, b8c5e22c16823612d4dc5f0c2a223f7bf33699ea, ecccbad441f5ac84a1b52866da96ec4e5b745539, 2b3985a03ce38c138ad3fe81c55938cd87609e29, acc0432b117bbe0fbc1cc48beaf93171c23f8a72, 041512ffa11870c5f077c2a54426ca3451fe1552, 610f2dd3eb645e1d947ea7766aa95dd840ced658, c9c02e88e2abad43afe6285125dc2fb519b3fe7c, 3cdf1001062b0cf0daa74ad142a49a9dcd101bfc, 510b551d5a066c03efb8df8ecbe48ad01102416d
- Tests: telperion-heavy 8 --gpu 2 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-core --test ground_reach -- --ignored (wood clear on all rows), artifact_digest d600fa12c vs change (beech, telperion, laurelin identical)
- PRs: