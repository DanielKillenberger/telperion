---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-101-a-slim-growth-and-field-package-for-the.1 Implement A slim growth-and-field package for the homepage

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

## Blocker (NEEDS_HUMAN, 2026-09-22)

R3 is missed on all six trees: warm medians of 349, 388, 406, 649, 289 and 273 ms against a 250 ms ceiling (`.flow/evidence/fn-101-a-slim-growth-and-field-package-for-the/RESULTS.md`, R3). The slim entry point runs at parity with the full binding measured in the same minute, so the owner's "no slower than today" holds; the time is `branching::generate` plus the plan query, which fn-100 measured above 250 ms on this same code, and the binding adds 1 to 17 ms. Making the query faster is a core change (a flags-only early exit would drop the leaf counts and limb ids the voxel rules read), which is the host's design call: accept parity as the criterion, or open a query-speed spec. Every other criterion (R1, R2, R4, R5, R6, R7) is met and the gate is green.
