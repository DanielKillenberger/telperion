---
satisfies: [R2]
---
# fn-211-tree-space-f2b-growth-years-cheap.3 A kept thread pool, scratch reuse and flat draw records

## Description
Host decisions on task 2's questions (2026-10-06): (1) a persistent pool for cores::spread, via rayon (the standard pool; a hand pool would need unsafe), respecting the process-wide core budget and working on one core; (2) no second index-order path for one core (docs/principles.md: no switch between ways of building); instead reuse scratch buffers across years so the depth-at-a-time passes cost no more than the loops they replaced on one core; (3) each axis's draw record and per-axis vectors become flat arrays (struct of arrays or one arena), removing the quarter second of freeing per big tree and cutting peak memory. Every tree identical to the bit.

## Acceptance
- The hash table identical at every commit (80 years, seeds 1 and 7, neutral and lit). - Per-pass timings (measures passes) before and after on 1 and all cores, ages 40 and 80, four species; one core no slower than before task 2. - Peak memory before and after. - The crate's unit tests compiled and run at every commit, not only the example. - Workspace gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
