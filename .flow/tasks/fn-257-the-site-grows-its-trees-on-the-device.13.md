---
satisfies: [R8, R8a]
---
# fn-257-the-site-grows-its-trees-on-the-device.13 The field reads the drawn stage's output: one splat to the requested grid, the primitive format deleted

## Description
Build R8 as the probe settled it: the field is a consumer of the drawn stage's output through the one pipeline (`docs/pipeline.md`: stages private, no second chain), as the renderer is, with no intermediate stage or format between the drawn records and the grid. The CPU reference reads the CPU's drawn tree the same way.

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/field*`, `crates/telperion-render/src/generation/resident.rs`, `generation.rs`, `resident/build.rs`, `crates/telperion-core/src/pipeline/field*`, `docs/pipeline.md`
**Touches:** [crates/telperion-render/src/generation/**, crates/telperion-render/src/generation.rs, crates/telperion-render/src/shader_tests.rs, crates/telperion-core/src/**, crates/telperion-core/tests/**, .github/workflows/tests.yml, docs/pipeline.md]

### Approach
- Device: the probe's splat becomes the field's one pass; `Generator::grow_field` asks the pipeline for the drawn stage's output (the stages the field does not read are not requested, per the probe's list, without a second chain) and returns the grid. Delete `field.wgsl`'s primitive decoder, `field_build.wgsl`, `field_query.wgsl`, `field/index.rs`, `field/ask.rs`, the 16 B layout in `field/make.rs`, `query_field`, `Tools.fielder`'s query, and the field's ignored measurement tests that assume cubes (rewrite them on the grid).
- CPU (spec, "Both executors"): the CPU field reads the CPU's drawn tree as the device splat reads the device's (the same records, as the CPU's drawn stage emits them) to the spec's grid; it is R8a's reference. Delete the field's own forms: `Foliage::Planned`/`Foliage::Placed` (`pipeline/field.rs:52-55`), `Field::wood`'s capsule rebuild, `FieldSnapshot`/`IndexSnapshot` (`pipeline/contract.rs`), the field's index and cuboid, `planned.rs`'s route to the field, and the leaf plan (`foliage/plan.rs`, `plan/fronds.rs`) and `drawn.rs:25`'s `field && !supports` placing rule if nothing else reads them (grep; keep any other consumer). Keep `every_shipped_preset_builds_every_artifact_through_the_pipeline` green and adjust the `core-plan` CI job (`.github/workflows/tests.yml:148-166`) to what remains.
- The derivations the probe named (limb systems, leaf reach, frond leaflets) survive as passes over the drawn records, not as a stored format.
- Tests: `every_preset_s_device_field_answers_within_its_tolerance` over seeds 1 and 5678 at 24, 36, 48; same answers every time; lost device refused by name; refusals as the core refuses; the crowded-cell limb test on the grid.
- `docs/pipeline.md`'s field section rewritten: the field is a consumer of the drawn stage, like the renderer; fn-287's description of the primitive format goes.

### Investigation targets
**Required**:
- `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/PROBE-R8.md` — the probe's settled rule
- `crates/telperion-render/src/generation/resident/field.rs`, `field/make.rs`
- `crates/telperion-core/src/pipeline/field.rs`
- `docs/pipeline.md`
## Acceptance
- [ ] The field's device path is drawn stage → one splat → grid read-back; no primitive format, index or per-cube query remains (grep shows none of `field_build.wgsl`, `field_query.wgsl`, `field/index.rs`, `field/ask.rs`).
- [ ] The CPU field reads the CPU's drawn tree the same way; no `Foliage::Planned`/`Placed`, `Field::wood` capsule rebuild, `FieldSnapshot` or field route through `planned.rs` remains; every preset at 24, 36, 48, seeds 1 and 5678, within today's tolerances (a test).
- [ ] Same grid every time; a lost device and the core's refusals refused by name (tests).
- [ ] `docs/pipeline.md`'s field section describes the field as a consumer of the drawn stage.
- [ ] Focused crate tests green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
