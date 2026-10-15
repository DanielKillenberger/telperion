---
satisfies: [R6]
---
# fn-257-the-site-grows-its-trees-on-the-device.4 Fewer dispatches: the tiny passes batched, the saving measured

## Description
TBD

## Acceptance
- Dispatches that run in a pass of their own, and dispatches under 20 µs, reduced where they can share a pass; the warm build time of spruce, birch and oak at 40 before and after.
- Every shipped preset's tree unchanged (the existing agreement tests).

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
