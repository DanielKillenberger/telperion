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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
