---
satisfies: [R1, R2, R3, R4]
---
# fn-316-the-field-answers-a-trees-flowers.1 Implement the field answers a tree's flowers

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
The field answers a tree's flowers on both executors (R1-R3):
- core: `field::seats`, `seat_bounds`, `florets_into`; the field stage places the florets at the window's middle against the box both executors pack them in, its box holds every seat, and a build asking for leaves too takes those florets for its organ output (placed once).
- device: the field's stations count florets with the organ rows, the device places them, their words are read back and counted by the same core function.
- bindings: `answer.flowers` and flags bit 4 (CPU slot 30; device five words a cell); voxelize `Cubes.flowers`, exclusive of foliage; README, field-package.md and the flowers migration note.

R4 measured (RESULT.md): device against CPU, totals equal but Telperion at 5678 (+10 of 157,417), at most 4.0e-4 of florets in another cell; tolerances recorded in the spec (host). The other answers are byte-identical on every preset (24 field_answers hashes). Field time +0.2 % (Telperion) and +1.0 % (Laurelin), under 5 %.

OPEN_ITEM: the owner looks at a voxelised Telperion with its flowers (the site's port, or the harness) and the words are recorded in the spec.

stage: impl-review - ran (codex gpt-6.1-sol, three lenses then one): NEEDS_WORK (3 findings, all fixed) then SHIP

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: c7e0422ef, 596620438, e6a0aa28c, 344360beb
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py (1504/1504), npm test (191/191), cargo test --profile ci -p telperion-render --lib -- field_tests:: (8 passed), field_time_tests::field_answers on base and branch (identical)
- PRs: