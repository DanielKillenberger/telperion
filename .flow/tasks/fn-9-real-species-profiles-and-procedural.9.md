---
satisfies: [R2, R3, R4, R5, R6]
---
# fn-9-real-species-profiles-and-procedural.9 Run cross-seed visual QA and document the species workflow

## Description
Cross-seed visual QA against the references, shared-rule corrections, and the species workflow docs.

## Acceptance
- Fixed/fresh protocol inspected in whole/bare/foliage views; failures retained; shared templates rechecked.

## Done summary
Retained species crowns replace post-growth occupancy transforms; oak infill and spruce shoots along descending secondaries; flat structural tips corrected. Production 0a8f343 passes 48 numeric cases and 68 native tests; 84 captures, 59 inspected, all required traits supported. Report: `.flow/evidence/fn9/REPORT.md`.

## Evidence
- Commits: fd4a87b, d8812e6, 17fd6e5, a2eddc5, ec75925, 9da2c5b, 9cd68be, 0a8f343, d388ca0
- Tests: cargo test --workspace, npm test, browser capture suite
- PRs:
