---
satisfies: [R9, R10, R11]
---
# fn-100-the-field-reads-the-plan-not-placed.2 Wood radius and a selectable limb order in the field answer

## Description
Add a per-cell wood radius and a selectable limb order to the field answer, and redraw the voxel sheet with them.

## Acceptance
Spec R9-R11; R8's verdict is taken on the R11 sheet.

## Done summary
`Occupancy.wood_radius` (binding slot 25, `FieldQuery.woodRadius`); `plan::plan` takes a limb order and `outputs.field` accepts `true` or `{"limbOrder": n}`, the boolean form byte-identical to the family order. The voxel prototype culls wood under a fifth of a cell and clumps by limb id; the sheet is drawn at order 3 (oak clump counts: base 67, order 2 129, order 3 281). Owner accepted R8 on it.

## Evidence
- Commits: 5d029fdba64e3001fea8e63b9c339a6fd6dd66e2
- Tests: cargo test --profile ci --workspace --no-fail-fast (887 ok); node scripts/test-wasm.mjs; cargo test --profile ci -p telperion-wasm
- PRs:
