---
satisfies: [R9, R10, R11]
---
# fn-100-the-field-reads-the-plan-not-placed.2 Wood radius and a selectable limb order in the field answer

## Description
TBD

## Acceptance
R9, R10 and R11 of the parent spec are satisfied; judge this task against the spec's criteria directly. R8's verdict is the owner's, taken on the R11 sheet.

## Done summary
R9, R10 and R11 of fn-100. `Occupancy` carries `wood_radius` (the thickest wood sweep reaching the cell, zero without wood), the binding exposes it in slot 25 and `FieldQuery.woodRadius` types it; `plan::plan` takes a limb order and `outputs.field` accepts `true` or `{"limbOrder": n}`, the boolean form answering byte for byte as the family's order (tests in `crates/telperion-wasm/src/generate/tests.rs`, `crates/telperion-core/tests/field_plan.rs`, `tests/browser/bindings.mjs`); the voxel prototype culls wood under a fifth of a cell by the radius and coins clumps by the limb id at `LIMB_ORDER`, and the order-3 sheet sits beside the base sheet under the spec's `raw/`, described in RESULTS.md. R8's verdict awaits the owner on that sheet.

Surprise for the host: by clump count the family's order 2 (129 clumps) is the oak's nearest to the base coin (67), not a finer one; order 3 and deeper give 281 (the oak's laterals stop at 3), order 1 gives 21. The count is inflated by small systems while the mass sits in a few large ones, so the sheet is drawn at order 3 as the finer order R11 names, with the default-order sheet kept beside it; the table is in RESULTS.md.

Tier: session (host judgment, continuation of task 1) - actual_model: claude-fable-5-1
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 5d029fdba64e3001fea8e63b9c339a6fd6dd66e2
- Tests: cargo test --profile ci --workspace --no-fail-fast (suite_rc=0, 887 ok; log raw/task2-gate.log), node scripts/test-wasm.mjs (browser binding suite, rc=0), cargo test --profile ci -p telperion-core --test field_plan --lib -- plan wood_radius limb_order two_limbs, cargo test --profile ci -p telperion-wasm, FIELD=1 LIMB_ORDER=3 node experiments/voxel-field/voxels.mjs 64 (sheet raw/sheet-64-candidate-field-order3.png), baseline: none (the spec lists no Quick commands; the gate runs once at the end per CLAUDE.md)
- PRs: