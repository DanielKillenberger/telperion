---
satisfies: [R9]
---
# fn-14-bark-and-foliage-appearance.3 The sun and one shadow map, timed

## Description
Add the sun as a key light and one shadow map: a depth-only pass from the sun over the wood and every leaf at the element's coarsest level, a shadow bind group both shaders sample, uniforms for the light, and a third timestamp pair (R9). This is the early proof point: the shadow pass must fit inside the 1.5 ms the owner allowed.

**Size:** M
**Files:** `crates/telperion-render/src/lib.rs`, `crates/telperion-render/src/shadow.rs` (new), `crates/telperion-render/src/scene.rs` (uniforms; split under 400), `crates/telperion-render/src/shaders/shadow.wgsl` (new), `crates/telperion-render/src/shaders/wood.wgsl`, `crates/telperion-render/src/shaders/foliage.wgsl`, `crates/telperion-render/src/shaders/scene.wgsl`, `crates/telperion-render/src/timing.rs` (split under 400) and `timing/report.rs`, `crates/telperion-render/src/web/session.rs`, `crates/telperion-render/examples/headless/walk.rs`
**Touches:** [crates/telperion-render/src/lib.rs, crates/telperion-render/src/shadow.rs, crates/telperion-render/src/scene.rs, crates/telperion-render/src/scene/**, crates/telperion-render/src/shaders/**, crates/telperion-render/src/timing.rs, crates/telperion-render/src/timing/**, crates/telperion-render/src/web/session.rs, crates/telperion-render/examples/headless/**, crates/telperion-render/tests/**]

### Approach
- Uniforms at `scene.rs:39-54` gain the sun direction and colour and the light's view-projection; the scene row from task 2 feeds them each frame.
- Shadow pass: a depth-only pipeline through the shared helper at `lib.rs:320-366`, single-sample `Depth32Float` texture, frustum fitted to the crown bounds plus the ground shadow the sun's elevation throws; draws the wood mesh and one instanced draw of the element's coarsest level over the whole placement buffer, no selection. Slots before the room pass in `draw_with` (`lib.rs:222-274`).
- Sampling: a comparison sampler and the shadow texture in a new bind group; wood, foliage and the ground disc shaders sample it. The lit terms themselves land in task 4; here the shaders multiply their existing hemisphere by the shadow factor and add a plain sun term so the pass is visible and testable.
- Timing: a third pair around the shadow pass; query set to six (`timing.rs:128-172`), `draw_timed` and the web session updated together (`lib.rs:203-220`, `web/session.rs:73-99`); the report gains `shadow_p50_ms` and `shadow_p95_ms`.
- Views: bare draws wood shadows; leaf view skips the pass; clay skips the pass.
- Measure first: an oak still with the timing record on the RTX 3080, shadow pass p50 recorded; if the pass alone exceeds 1.5 ms, halve the map resolution before anything else and record both numbers.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/lib.rs:200-366` — draw_with, pass helper, pipeline helper
- `crates/telperion-render/src/timing.rs:120-260` — the query set and session sampling
- `crates/telperion-render/src/select.rs:80-120` and `select/bind.rs` — how a level's index range and the placement buffer are bound

**Optional** (reference as needed):
- `crates/telperion-render/src/scene.rs:180-220` — the bind group layout
- `.flow/evidence/fn24/oak-native-timing.json` — the record shape to extend

### Key context
- wgpu 30 API names: TexelCopy types, fallible mapped ranges; follow the in-repo patterns.
- The 400 m ground disc is never the shadow frustum.
- Budget rules: one timing re-run per change, no forest captures, at most four images viewed.

## Acceptance
- [ ] A shadow pass from the sun writes a single-sample depth map over the wood and every leaf at the coarsest level, with no second selection dispatch
- [ ] Wood, leaves and the ground disc sample it; the bare view has wood shadows, the leaf and clay views skip the pass
- [ ] Timing record carries a shadow p50 and p95 from a third timestamp pair; native and web sessions both resolve it
- [ ] Oak still on the RTX 3080 with the pass recorded, p50 of the shadow pass under 1.5 ms, or the resolution halved and both numbers recorded
- [ ] `cargo test --release --workspace`, fmt and clippy pass

## Done summary
The sun now writes one single-sample depth map from where the scene row puts
it, over the wood and every leaf at the element's coarsest level in one
instanced draw over the whole placement buffer, and the wood, leaf and room
shaders compare against it through a comparison sampler beside a sun term taken
from the row. A third timestamp pair times the pass, and the record's total is
now the three passes a frame costs.

stage: impl-review - skipped(config: REVIEW_MODE=none)

### What each acceptance line came to

- **The pass.** Depth-only, no fragment stage, fitted to the subject's bounds
  and the ground shadow the sun's elevation throws from them, never to the 400 m
  floor. The crown's draw reads `placements[instance]` directly, so no second
  selection runs; the pass is opened on every frame so a timed one always writes
  its pair. `tests/shadow.rs` reads the map back: the bare view covers some of
  it, the whole view covers more than the bare one, and the leaf view leaves it
  at the clear depth, which reads as open ground.
- **Sampling.** Wood, leaves and the room's ground disc take
  `sun * max(dot(n, towards the sun), 0) * sunlight(world)` beside the
  hemisphere they already had. A point outside the map stands open.
- **Timing.** Six queries, `Session::timed()` in place of the two pair
  accessors, `Report::with_passes` in place of `with_selection`; the record
  gains `shadow_p50_ms` and `shadow_p95_ms` and `total_*` is now all three
  passes added per frame and then ranked. Native and web sessions both resolve
  it (the web half is compiled for wasm32 as part of the gates).
- **The number, and it is over.** Oak at seed 7, 1024x1024, RTX 3080, valid
  session: at a 2,048 map the shadow pass was **2.30 ms p50** against the 1.5 ms
  allowed, so the map was halved as the task directs, and at 1,024 it is **1.81
  ms p50 / 1.99 ms p95**. Both numbers are recorded, in the task's terms and in
  `RESOLUTION`'s own doc comment. Halving bought 0.49 ms: the pass submits the
  same 8.25 M wood triangles the vegetation pass does (1.95 ms with shading on
  top), so the cost is geometry and not raster, and no map size reaches 1.5 ms.
  The lever is a coarser caster - the wood has no level ladder, only the crown
  does - which is the early proof point's re-evaluation and the owner's call,
  not this task's. Total p50 is 3.86 ms against R12's 3.8 ms, before task 5's
  multisampling.
- **Gates.** `cargo test --release --workspace` green (32 targets, no GPU test
  skipped), fmt and clippy clean, and the wasm target checks clean.

### Deviations, each with its reason

- **Files outside the task's Touches.** `wood.rs`, `foliage.rs` and `web.rs`
  were edited: the buffers a depth pass draws are private to the two subjects,
  and `web.rs` held the timestamp-pair type the third pair changed. All three
  edits are additive and small. No other task's files were touched.
- **The 400-line rule forced two splits**, both named in the commit: the
  pass-and-pipeline shapes out of `lib.rs` into `src/pass.rs` (which is where
  task 5's sample-count edit now belongs), and the sun's fit out of `shadow.rs`
  into `shadow/fit.rs`. Every file this task wrote or grew is under 400 lines;
  `camera.rs` at 504 is fn-24's inherited follow-up and was left alone.
- **The shadow multiplies the sun term, not the hemisphere.** The task's
  approach line says the hemisphere; multiplying ambient by a comparison result
  makes every shadow pitch black, which task 4's look work would only undo. The
  shadow reads strongly in the still as it is.
- **The still is overexposed.** A sun of radiance 3 with no tone map blows the
  lit ground and crown out; the tone map is task 4's, as planned. The still
  proves the pass, not the look: `/tmp/flow-handover-fn14/fn-14.3-oak.png`,
  with a dappled crown shadow lying away from the sun and no acne on the ground.

### Evidence on disk

- `/tmp/flow-handover-fn14/fn-14.3-oak.png` - the oak still, 1024x1024
- `/tmp/flow-handover-fn14/fn-14.3-oak-timing.json` - the shipped 1,024 map
- `/tmp/flow-handover-fn14/fn-14.3-oak-timing-2048.json` - the first measurement
- `/tmp/flow-handover-fn14/fn-14.3-tests.log` - the suite run
- `.git/flow-notes/.../fn-14.3-shadow.md` - what tasks 4, 5 and 6 inherit

stage: wave-join - ran (fast-forward 8eb9de0..426b31d, no collision; worker edited wood.rs, foliage.rs, web.rs outside declared Touches - no sibling in flight)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 426b31d191ba837802cbf25e5a8438ff2905f0dc
- Tests: baseline: green via handoff (green (verified at 8e001cd by fn-14-bark-and-foliage-appearance.2)); cargo fmt --all -- --check run pre-edit and green, cargo test --release --workspace (32 targets ok, suite_rc=0, no GPU test skipped), cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo check -p telperion-render --target wasm32-unknown-unknown (the web session is wasm-only), cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --out /tmp/flow-handover-fn14/fn-14.3-oak.png --timing /tmp/flow-handover-fn14/fn-14.3-oak-timing.json
- PRs: