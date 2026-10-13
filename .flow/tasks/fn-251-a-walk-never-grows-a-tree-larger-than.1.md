---
satisfies: [R1, R1b, R2, R3]
---
# fn-251-a-walk-never-grows-a-tree-larger-than.1 Repro, natural-scale rows, the size factor questioned, the sweep

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Blocked:
Waits for fn-255 (one light-reading grower); host decisions in the spec, 2026-10-09. Measurement and red repro on branch walksize (d4cc14c4, 28b0200f).

Blocked:
NEEDS_HUMAN (host design call). Re-measured on F4 after fn-255 (958654d48, WALKS.md "Re-measured on F4"): the repro builds; 70 of 80 frames meet the oracle and none fails on the budget. The 10 that fail have three causes: the linear walk predicts below both ends (oak and spruce, 5 frames, out of reach for a shrink-only factor); one seed's kept count spreads by more than the 10 % widening (4 frames); whole phytomers at a seedling (1 frame). The fix needs a design decision first.
## Evidence
- Commits:
- Tests:
- PRs:
