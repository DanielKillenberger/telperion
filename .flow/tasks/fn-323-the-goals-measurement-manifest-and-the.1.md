---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-323-the-goals-measurement-manifest-and-the.1 Implement the goal's measurement manifest and the coarse route's floor

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The manifest and the route's stop were frozen before any measurement. Limb count and shed share are measured, and the floor's field, cost and reach are recorded. Verdict by the frozen rule: stop, since no candidate passes the field limit at both ages, and the only one to pass anywhere costs seconds against 50 ms. Evidence: MANIFEST.md, RESULT.md. Measuring code only; no tree changes.

stage: impl-review - ran [7e9c0404e..fcfae803e] (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 66efb244fa17dd6f26e9bfea0ba63e2411e96009, 24fc50577d98e114214ebfd9d0bae219c3de03f9, 00c10f8dcfbf236b82eb69e1a78b2f5d0cedb580, 608f307e01a79b211b81d930b15b2775a13ef275, 0c731ed53d1ca1836a929e3075e527914f958127, fcfae803ec805cedc8343a356618e575d25f43f8
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py, floor_field, floor_cost, floor_reach, the_device_reach_at_an_age (ignored measurements)
- PRs: