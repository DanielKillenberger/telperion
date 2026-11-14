---
satisfies: [R1, R2, R3]
---
# fn-320-a-tree-grown-twice-on-the-device-grows.1 Implement a tree grown twice on the device grows the same field

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The cause is named and fixed: the driver fuses a marked product into its add on some runs of one program and not others, so one invocation on restored words answered two ways. `cycle/kept.wgsl` keeps the marks apart in the six float modules. Every preset at seeds 1 and 5678 grew byte-identical fields and digests over 30 repeats in two processes, at no measured warm cost and about 4% on the cold compile. The gate gained its own driver cache. Evidence: FUSED.md, RESULT.md.

stage: impl-review - ran [cffdff890..510d44a24] (model: gpt-6.1-sol); e04c51ae5 skipped(policy: risk - a one-line gate fix with its test)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b67644849d71698c6df0d8bd99516d1b26536619, 9859c849814055214a98faec0ab163b2f3f61acf, 510d44a2421610ee3f2ddb385af1bd344c16b5de, e04c51ae5fca67666e8daf086083c33c0c617141
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py, every_preset_grows_the_same_field_on_thirty_repeats (ignored, two processes), every_preset_digests_the_same_on_thirty_repeats (ignored, two processes)
- PRs: