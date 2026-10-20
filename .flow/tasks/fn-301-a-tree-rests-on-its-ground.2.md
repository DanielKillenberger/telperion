---
satisfies: [R2, R4]
---
# fn-301-a-tree-rests-on-its-ground.2 A frond rests on the ground

## Description
Put contact inside the frond curve, the CPU's `rachis` and the device's shared function. The rachis, its leaflets' stations, their run and the dead skirt all rest together. This is the defect the owner saw, and it is the early proof point.

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/foliage/leaflet.rs`, `foliage/rachis.rs`, `foliage/rosette.rs`, `foliage/plan/fronds.rs`, `crates/telperion-render/src/generation/rachis.wgsl`, plus the device callers that pass the radius
**Touches:** [crates/telperion-core/src/pipeline/foliage/leaflet.rs, crates/telperion-core/src/pipeline/foliage/rachis.rs, crates/telperion-core/src/pipeline/foliage/rosette.rs, crates/telperion-core/src/pipeline/foliage/plan/fronds.rs, crates/telperion-core/src/suite/**, crates/telperion-render/src/generation/rachis.wgsl, crates/telperion-render/src/generation/pack.wgsl, crates/telperion-render/src/generation/resident/fronds.wgsl, crates/telperion-render/src/generation/resident/field.wgsl, crates/telperion-render/src/generation/resident/*_tests.rs, crates/telperion-render/src/generation/resident/boots.rs]

### Approach
- The contract is the spec's R2 "An axis" and "Edge cases and constraints": the first contact `t`, flat at its radius `r0·(1−t)` (floored at the finest wood, `rachis.rs:62,73`) past it, the parabola's closed-form arc length preserved, and the horizontal direction of the contact tangent. A vertical tangent with no horizontal heading is a named error.
- The function returns the point and the tangent. `leaflets()` (`leaflet.rs:56-100`) reads the tangent for each leaflet's run and axis, in place of the analytic `heading + lift·2·arch·t`. The device's leaflet path (`pack.wgsl` `leaflet`) does the same.
- `r0` must reach the curve. On the CPU it lives in `rachises()`. Thread it as a parameter, or derive it where both callers can (the leaf plan, `plan/fronds.rs:79`, reaches the same function through `leaflets`). On the device, pack's frond table gains `r0` if it lacks it. Rows come from `Family::applied` (memory: rows-hoisted-before-the-expansion-read).
- Dormant gate: when the curve's bark never reaches the ground over [0, 1], the function evaluates exactly today's expression. Test it bit for bit.
- Unit tests in the crate suite (`crates/telperion-core/src/suite/`): descending arch, upward arch (first root), base below radius (`t = 0`), arc length preserved to 1e-9, the vertical-heading error, and a clear frond bit-identical to the base expression.

### Investigation targets
**Required:**
- `crates/telperion-core/src/pipeline/foliage/leaflet.rs`
- `crates/telperion-core/src/pipeline/foliage/rachis.rs:41-106`
- `crates/telperion-core/src/pipeline/foliage/rosette.rs:201-268, 369-411`
- `crates/telperion-render/src/generation/resident/agree_tests.rs:252-530`
**Optional:**
- `crates/telperion-core/src/pipeline/foliage/plan/fronds.rs:56-120`, `crates/telperion-render/src/generation/resident/rachis_tests.rs:113-170`

### Key context
The device works in f32, so its square roots and `asinh` differ from the CPU's f64. Agreement is to `agree_tests`' tolerances (R4), never bit equality. A test that pins the field's foliage share may move for the palm at 12; re-measure it against the device, never loosen it without a number.

## Acceptance
- [ ] The CPU and the device apply contact inside the one frond curve; the rachis, its leaflets and the skirt rest together.
- [ ] Unit tests cover the descending arch, the upward arch, the base below its radius, arc length kept, the named error, and a clear frond bit-identical to the base.
- [ ] The palm at 12: `agree_tests`, `rachis_tests` and `field_tests` pass (R4).
- [ ] `ground_reach` for the palm: no rachis wood below the ground at 6 to 20 (leaves may still cross until .4).

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
