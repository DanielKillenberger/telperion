---
satisfies: [R1, R2, R3, R4]
---
# fn-277-a-walk-between-leaf-forms-changes-the.1 Repro, cause, fix, sweep

## Description
In order: R1, R2 (stop and report the cause to the host before fixing), R3, R4.

## Acceptance
R1-R5 per the spec.

## Done summary
A walk between leaf forms changes the leaves by degree. Six organ counts and a new cauline share (`/canopy/cauline`, 1 on dicots and conifers, 0 on the palm) walk unrounded, each bearer drawing the whole counts either side by one keyed 32-bit draw shared by CPU and device. Causes: `rosetteFronds` at zero, `leafletCount` at one and `cluster_leaves` at zero switched the leaves. `walk_law`: steepest step 0.0128 of a form's range against a bound of 0.02; `walk_leaves` draws within 4 sigma; `organ_tests` agree on device. Gate and npm not admitted in 30 minutes (host runs them on the merged branch). Codex SHIP after host rulings 5 and 6.

## Evidence
- Commits: 31d64595775f947bdda78a04644226d902b82d58, 10d7f6ed40db8d33d153d4332aeb81afe52c8033, deef617599866f7e2e8b111d8c25dc3de3ad487b, ec00428fbf527ddebe92ae5e8c89551919037d06, c094b4c10e0f0060f7c01e0d00979b7afee0a39f, fafc59dbe9ebed829dc32937b3da8c491264c6f6, 013fe787aabc7284ff85029d835dfb7403813cac, 83168acf880f03a2d192f79755b9dab1dd9eb4c1, 0324d5efe775c1e189c7dea64ebfb6a6e9d20925, 5fe234948bc8882b93be75f85c6d671e47cf4137, 1355838f7637e016068c5f8793fd4d4398fd9ce8, a4d1be3efb837875aee00cd24f28591f60053844, d2ab1347cecb2ba025f8325931aa598cea325a17, 97db363778533ea56356a936502fdd75ffe6c7be, 3a1ae810bfbf25a61ab2afefbe2c7a9b7012e27d, fe0582e811706d96955387cca29d3d8bea40e6d8, 379a8753115baea23efe5951d534034b1c7136b9
- Tests: cargo test --profile ci -p telperion-core --test walk_leaves, cargo test --profile ci -p telperion-core --test walk_law, cargo test --profile ci -p telperion-render --lib (GPU, organ_tests included), cargo test --profile ci -p telperion-core --test walk_sizes, GATE_SKIPPED:gate:not admitted by telperion-heavy in 30 minutes, GATE_SKIPPED:npm:not admitted by telperion-heavy in 30 minutes
- PRs: