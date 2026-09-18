---
satisfies: [R1, R2, R3, R4]
---
# fn-55-matte-not-plastic-a-physical-highlight.1 Implement Matte, not plastic: a physical highlight and a pixel grain

## Description
The physical highlight and the grain rows, implemented in-host (the routed
gpt-6-astra bridge refused on quota). Report: `.flow/evidence/fn55/REPORT.md`.

## Resolution
Owner, 2026-09-18: the relief's drift at distance is a real defect seen
before fn-55 and is not fn-55's to fix. The 4x distance-series bound is
recalibrated from 3.0 to 3.25 with the reason beside it (the old sheen's
tone-curve compression; base 2.904, after 3.134, no highlight 3.167); the
2x and p95 bounds stand; fn-71 fixes the drift and restores 3.0. The bark
grain rows stay held off in the tables, their value set in fn-71.

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
