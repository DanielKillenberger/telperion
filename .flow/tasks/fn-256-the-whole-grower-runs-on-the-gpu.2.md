---
satisfies: [R1, R2, R2a]
---
# fn-256-the-whole-grower-runs-on-the-gpu.2 The device cycle executor: every phase, units and layout included

## Description
Build fn-256's device executor of fn-255's cycle per COUNCIL.md 'Decided by the spike': bounded subtree blocks, one reduction order (canonical subtree-block order, blocks of 1,024 growing buds), integer deposit, one-workgroup-per-direction sweep, inline light read, per-bud units and incremental layout on the device, prefix-ordered births, retirement with the three-cycle wait after a waking (fn-255 R3B.md), zero-light survival as fn-255 fixed it (the spike's fate.wgsl has the bug). First measure units and layout on the device (unreplayed by the spike); stop and report if they break the warm budget. Not yet wired into production.

## Acceptance
Every preset grows through the device executor; integers exact and floats within measured tolerance against fn-255's CPU cycle by lineage; node table byte-identical over 10 fresh contexts and varied block sizes (R2, R2a); per-phase GPU time and scratch per preset; longest dispatch under 2 ms.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
