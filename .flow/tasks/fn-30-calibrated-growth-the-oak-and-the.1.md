---
satisfies: [R1, R2, R3, R4]
---
# fn-30-calibrated-growth-the-oak-and-the.1 Implement Calibrated growth: the oak and the spruce through time

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

## Blocker — NEEDS_HUMAN, 2026-09-14

R1 stops on its own rule. At the three fitted ages the oak and the spruce meet the composed open-grown height references within 15 percent (oak -5.3, +3.8, -5.8 percent at 26.7, 56.1 and 112 years; spruce -4.0, +5.0, -14.0 percent at 14.1, 26.6 and 36.9 years), and the trunk diameters miss (oak +317, +418, +235 percent; spruce +15, +48, +30 percent) because fn-11's thickening rule keeps the trunk a fixed fraction of the live envelope height, so the model's height-to-diameter ratio stays near 28 (oak) and 35 to 39 (spruce) while the references fall from 125 to 44 and 47 to 35. The spec says a miss outside the tolerance stops the spec unless the owner's recorded judgment accepts it, so `flowctl done` was not run. Decisions the owner must record, in `.flow/evidence/fn30/REPORT.md` under `## Owner verdict` (five slots: the oak curve, the spruce curve, the oak strip and mature still, the spruce strip and mature still, the report): whether the height-first calibration with the diameter deviation is accepted at each age or a thickening-by-age rule is a follow-up spec; whether the value-table change that sets `skeleton.habit.sheddingThreshold` to 0 on the ordinary preset and the Two Trees stands (the vigour proxy in `survival.rs` divides exposure by `1 + rate * node age` with no floor, so any nonzero threshold stripped every preset to about 30 nodes by maturity through the growth path, a defect for a follow-up spec); and whether the derived mature ages, oak 432 years and spruce 158 years, are acceptable as the production ages given the mature build cost through growth (oak 2,463 ms, spruce 867 ms, against fn-11's 1,273.5 and 932.9 ms and the half-second target). Open beside the verdict: four renderer tests (`bark_detail`, `bark_distance`, `bark_resolution`, `conformance`) fail after routing because their parameter-set fixtures now grow through `mesh::build` and two sets grow no wood or no placements; every other gate is green (core and wasm 224 tests, renderer 88 of 92, npm test 77 of 77 including native-to-wasm parity, typecheck, fmt, clippy). Checkpoint commit: 4117c8fb85959a5ae0a4ec18f79a81aabaa71bb6 on branch fn-30-calibrated-growth-the-oak-and-the.

## Owner verdict — 2026-09-14, R3 rejecting

The owner judged the age strips on the served page and rejected the young ages in the owner's words: "the 10yo one is way too low quality and i'm not convinced that the 26yo looks like that either? This calibration task seems to have missed the mark by not being able to touch the generator." The cause is the growth rule, which this spec's boundary forbids changing: laterals and foliage lag the stem by decades, the trunk is a fixed fraction of live envelope height, and the vigour proxy has no floor. All three go to a new growth-rule spec that may touch fn-11's machine, chained on this branch; the reference composition, the derived ages and the zero-threshold workaround are deferred to it. fn-30 stays open and in_progress until the strips re-rendered after that spec earn an accepting verdict. Verdicts are recorded in .flow/evidence/fn30/REPORT.md under Owner verdict.
