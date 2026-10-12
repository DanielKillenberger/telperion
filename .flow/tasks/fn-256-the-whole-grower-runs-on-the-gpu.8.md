# fn-256-the-whole-grower-runs-on-the-gpu.8 Two suspected races, reproduced and fixed

## Description
COUNCIL round 5 step 8: fault_out (layout.wgsl:182) sets a fault bit with plain read-write; node_laterals (grow.wgsl:504-520) writes in the counted pass. Focused tests red first (e.g. a two-fault dispatch), then fix (atomic OR; counted branch counts only).

## Acceptance
Each confirmed race has a red-then-green test; an unconfirmed one is recorded with the test that failed to reproduce it.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
