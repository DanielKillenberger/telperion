---
satisfies: [R1, R2, R3, R4]
---
# fn-277-a-walk-between-leaf-forms-changes-the.1 Repro, cause, fix, sweep

## Description
In order: R1, R2 (stop and report the cause to the host before fixing), R3, R4.

## Acceptance
R1-R5 per the spec.

## Done summary
A walk between leaf forms now changes the leaves by degree. Six counts of organs and a new cauline share walk unrounded, each bearer drawing the whole counts either side by one keyed 32-bit draw that the CPU and the device share. The device draws everything the CPU does, and the oracle tests the law and the draw apart.

- Causes (CAUSE.md, R2): `rosetteFronds` at zero, `leafletCount` at one and the engine's `cluster_leaves` at zero switched the leaves.
- Law (host rulings 1-3): the counts are `rosetteFronds`, `leafletCount` (log scale), `skirtFronds`, `leafBases`, `acanthophylls` and the engine's `cluster_leaves`. The new `/canopy/cauline` is 1 on dicots and conifers and 0 on the palm (FAO 156). A rosette no longer clears the wood.
- Device (ruling 6): organ draws in WGSL, crowns after the wood, and leaflets on wood stations. `organ_tests` agree on the birch with 12 fronds, the oak with 1.3 leaflets and oak to palm at t 0.01, 0.3 and 0.9. The refusal and the CPU route for fractional values are gone.
- Oracle (ruling 5 as rewritten): `walk_law` sweeps every film walk at steps of 0.001; the steepest step is 0.0128 of a form's range, against a bound of 0.02. `walk_leaves` holds draws within 4 sigma, dormant under 30 organs. Every film walk passes, except Laurelin to Telperion's draws, which were not admitted; the law shows nothing moving on it.
- Wood steps at oak to palm t 0.25-0.52 are recorded in fn-273.

Tests: walk_leaves (R1/R3 windows, 3), walk_law, suite::organs, organ_tests (GPU), the render lib (191 pass), walk_sizes. Not run: the gate and npm, unadmitted for 30 minutes each; the host runs them on the merged branch.
Follow-up: the window tests take about 300 s of suite time.

stage: impl-review - ran (codex; NEEDS_WORK, then SHIP after the host's rulings 5 and 6)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 31d64595775f947bdda78a04644226d902b82d58, 10d7f6ed40db8d33d153d4332aeb81afe52c8033, deef617599866f7e2e8b111d8c25dc3de3ad487b, ec00428fbf527ddebe92ae5e8c89551919037d06, c094b4c10e0f0060f7c01e0d00979b7afee0a39f, fafc59dbe9ebed829dc32937b3da8c491264c6f6, 013fe787aabc7284ff85029d835dfb7403813cac, 83168acf880f03a2d192f79755b9dab1dd9eb4c1, 0324d5efe775c1e189c7dea64ebfb6a6e9d20925, 5fe234948bc8882b93be75f85c6d671e47cf4137, 1355838f7637e016068c5f8793fd4d4398fd9ce8, a4d1be3efb837875aee00cd24f28591f60053844, d2ab1347cecb2ba025f8325931aa598cea325a17, 97db363778533ea56356a936502fdd75ffe6c7be, 3a1ae810bfbf25a61ab2afefbe2c7a9b7012e27d, fe0582e811706d96955387cca29d3d8bea40e6d8, 379a8753115baea23efe5951d534034b1c7136b9
- Tests: cargo test --profile ci -p telperion-core --test walk_leaves, cargo test --profile ci -p telperion-core --test walk_law, cargo test --profile ci -p telperion-render --lib (GPU, organ_tests included), cargo test --profile ci -p telperion-core --test walk_sizes, GATE_SKIPPED:gate:not admitted by telperion-heavy in 30 minutes, GATE_SKIPPED:npm:not admitted by telperion-heavy in 30 minutes
- PRs: