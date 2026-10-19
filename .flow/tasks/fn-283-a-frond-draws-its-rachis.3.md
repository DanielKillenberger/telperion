---
satisfies: [R1, R5]
---
# fn-283-a-frond-draws-its-rachis.3 The device hangs the frond axes as the CPU hangs them

## Description
The axis records on the device: written from the frond table in the boots stage, in room the dress leaves past the bases', in the drawn-record layout the folds, sweep and field read, so the device draws, bounds, shadows and fields the axes through its existing path.

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/boots.wgsl`, `boots.rs`, `build.rs`, `fold.rs` (room), `crates/telperion-render/src/generation/cycle/dress.wgsl` if the room is reserved there, tests
**Touches:** [crates/telperion-render/src/generation/**]

### Approach
- Room: extend `boots::most` (`boots.rs:68-75`) with the most axis records a stem can hang: slots times (the most leaflets a frond draws + 1), from the rows' whole counts; refuse past the device's capacity by name.
- Records: mirror the CPU nodes of .1 (`hang` in `boots.wgsl` writes a base's two records; an axis writes one per node), with the same parent rule, radius and stations, reading the table .2 wrote.
- Agreement: extend `every_leaf_base_is_hung_as_the_cpu_hangs_it` (`boots_tests.rs:317`) or add its sibling for the axes, by lineage tolerance; the curve/mesh agreement tests already cover what the sweep makes of them.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/resident/boots_tests.rs`
- `crates/telperion-render/src/generation/resident/fold.rs` - `Dressed`, `record`, the room past the dress
- `docs/pipeline.md` (the one cycle and its executors) - the drawn-record buffer contract
**Optional:**
- `crates/telperion-render/src/generation/resident/field_tests.rs` - the device field against the CPU's

### Key context
- CPU = device agree by lineage, not to the bit (`telperion_space::agree`).
- The field reads the drawn records; `the_device_field_answers_as_the_cpu_field` must hold for the palm.

## Acceptance
- [ ] The device hangs every frond axis the CPU hangs: same count, nodes within lineage tolerance (palm at 40 and young).
- [ ] The device's drawn curve, mesh and field agree with the CPU's for every shipped preset (existing agreement tests green, palm included).
- [ ] A capacity overflow is a named error.
- [ ] The date palm's `/canopy/rachisWidth = 0.25` (FAO 156 ch. I citation in the values comment) lands here, with the generated presets, HASHES.md's palm line and every core, render and jev test green at the palm's real width (moved from .1, conductor decision 2026-10-20).

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
