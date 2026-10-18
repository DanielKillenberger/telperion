---
satisfies: [R8, R8a]
---
# fn-257-the-site-grows-its-trees-on-the-device.13 The field reads the drawn stage's output: one splat to the requested grid, the primitive format deleted

## Description
Build R8 as the probe settled it: the field is a consumer of the drawn stage's output through the one pipeline (`docs/pipeline.md`: stages private, no second chain), as the renderer is, with no intermediate stage or format between the drawn records and the grid. The CPU reference reads the CPU's drawn tree the same way.

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/field*`, `crates/telperion-render/src/generation/resident.rs`, `generation.rs`, `resident/build.rs`, `crates/telperion-core/src/pipeline/field*`, `docs/pipeline.md`
**Touches:** [crates/telperion-core/examples/**, crates/telperion-render/examples/**, crates/telperion-wasm/src/**, crates/telperion-render/src/web/field.rs, crates/telperion-render/src/generation/**, crates/telperion-render/src/generation.rs, crates/telperion-render/src/shader_tests.rs, crates/telperion-core/src/**, crates/telperion-core/tests/**, .github/workflows/tests.yml, docs/pipeline.md, docs/generation-limits-inventory.json]

### Approach
- **Worker (conductor, 2026-10-18), for the host to ratify:** `docs/generation-limits-inventory.json` joins the Touches: the field's limit sites moved to `field/splat.rs` and `Frame::over`, so `generation_limit_guard::generation_limits_are_classified` needs the inventory to follow them (four entries repointed, the two per-cube query entries dropped with the query, three new index clamps and the grid's refusal classified). Mechanical, no design change.
- **Host, after the worker's grep (2026-10-16):** the Rust consumers of the deleted field API move with this task so the workspace and the Wasm build never break between tasks: the examples (`core/examples/measure.rs`, `core/examples/artifact_digest.rs`, `render/examples/generation_digest.rs`) and the Rust half of R8b (the Wasm crate's field exports in `crates/telperion-wasm/src/{lib,generate}.rs` and their tests, and `WebField` in `web/field.rs`) take the grid here. The core `Request.field` names the grid's `n` beside the limb order, and `Outputs.field` is the grid.
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
- [ ] The workspace builds natively and for wasm32 (`telperion-wasm`, `web/field.rs`) at this task's end; the examples build.
## Done summary
R8 is built on both executors. The field reads the drawn stage's output and writes it into the requested grid: on the CPU, `pipeline/field.rs` plus `field/splat.rs` over the drawn tree's nodes and `Plan::descriptors`; on the device, the laid nodes and the station segments go through one splat (`field/splat.wgsl`), and only the grid is read back. The primitive format, its index, the per-cube query, `Foliage::Planned`/`Placed`, `FieldSnapshot`/`IndexSnapshot` and the planned route to the field are deleted. The examples, the Wasm crate's field exports and `WebField` take the grid.

baseline: inconclusive (the previous worker's core run was killed at the 8 GB cap; the edits were already checkpointed as d37801a21 when this run began)

Review fixes (codex round 1, NEEDS_WORK, then SHIP on re-review): the device's per-limb sums are now two words wide, so a cell holding more than 2^20 leaves of one limb (spruce, n = 3, limb order 0) reads out (test `a_cell_past_a_word_of_one_limb_s_leaves_reads_as_the_cpu_s`). The grid side is now refused before any device or kit work, natively and in `WebField::grow`.

Tests, measured on this machine (RTX 3080 for the GPU ones):
- `cargo check --workspace --all-targets`: green (all examples build).
- `cargo check --target wasm32-unknown-unknown -p telperion-wasm -p telperion-render`: green.
- `cargo test --profile ci -p telperion-render --lib -- field shader`: 14 passed. This includes every_preset_s_device_field_answers_within_its_tolerance (seeds 1 and 5678 at 24/36/48), same grid every time, the lost device refused by name, refusals as the core refuses, the crowded-cell limb test and the overflow regression.
- `cargo test --profile ci -p telperion-render --lib -- agree_tests the_device_path_draws_what_the_cpu_path_draws repeat_tests`: 7 passed. These cover the `readied`/`folded` and `Sweeper::laid` refactor.
- `cargo test --profile ci -p telperion-wasm`: green.
- The core-plan CI job run locally (core `--no-default-features` build plus lib tests): 132 passed.
- `cargo test --profile ci -p telperion-core`: every test passed except `generation_limit_guard`, which failed on the old inventory. The conductor added `docs/generation-limits-inventory.json` to the Touches (9e7d63e8a, labelled worker, for the host to ratify). The inventory now follows the moved sites (4069e2288), and `cargo test --profile ci -p telperion-core --test generation_limit_guard` passes. No core code changed after the full core run.

Notes (worker): `scale_tests` now checks that the field's thickest radius scales with the tree, where it used to check the deleted capsule list node by node. The two scaled grids differ in their box because the leaf reach does not scale, so their cells cannot be compared one to one. The Wasm request no longer accepts `field: true`; it needs `{"n": ...}`. The TypeScript side moves in task 14.

stage: impl-review - ran (codex: round 1 NEEDS_WORK with 2 findings, re-review SHIP; the inventory delta reviewed separately, SHIP)
Tier: (none given) (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8c6efae3f8837a7b499faef92b81fd5f9143643d, ada24e6c1b26f5c8a017cc71ae5edbf71ed9dea4, d37801a2175f30280dc062a8b97cd911656496f0, 42efb61a6bc0535f4ac0fd99528aba4b6d69e269, 14467459c6324d745b04af2408e85c405e5f398d, de6dd2f524dfe2a404940a1987141e35e38248f7, 9e7d63e8af2bae0ced50a47c6b437313fe27b489, 4069e2288af5868c245e586362fd1c514d039e13
- Tests: cargo check --workspace --all-targets, cargo check --target wasm32-unknown-unknown -p telperion-wasm -p telperion-render, cargo test --profile ci -p telperion-render --lib -- field shader, cargo test --profile ci -p telperion-render --lib -- agree_tests the_device_path_draws_what_the_cpu_path_draws repeat_tests, cargo test --profile ci -p telperion-wasm, cargo test --profile ci -p telperion-core --no-default-features --lib, cargo test --profile ci -p telperion-core, cargo test --profile ci -p telperion-core --test generation_limit_guard
- PRs: