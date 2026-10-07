---
satisfies: [R3]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.9 The stations' bases pass as a parallel scan

## Description
Host decision (2026-10-07) after task 7's profile: the stations' `bases` pass numbers about a million fine runs serially in one invocation (`@workgroup_size(1)`), 248 to 258 ms of the stations' 284 ms on the spruce at 80. Replace it with a parallel exclusive scan (per-block scan, block totals, add), the same first station and segment per run as the serial numbering, and retime seed 1 on the real path the same way as task 7: five builds a side, alternating, one a process, at low load.

## Acceptance
- The bases pass is a parallel scan; each run's first station and segment equal the serial numbering's (a test holds it).
- The spruce at 80 seed 1 retimed on the real path, five builds a side alternately at low load, with stage medians.
- Gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
