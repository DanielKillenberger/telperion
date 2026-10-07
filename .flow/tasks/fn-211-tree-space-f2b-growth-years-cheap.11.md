---
satisfies: [R4]
---
# fn-211-tree-space-f2b-growth-years-cheap.11 The stepped executor as the build, and the device's division

## Description
Host decisions (2026-10-07) after task 10 (TASK10-GPU.md). First: task 10 measured the stepped executor on the CPU at 6.67 s against today's grow at 8.73 s for the lit oak at 80, seed 1, with the same tree to the bit. Measure stepped against grow for the beech, spruce, oak, lit oak and palm at 40 and 80 (seeds 1 and 7), digests and time; if stepped is identical and never slower, make it the build's executor (one grower, not two: delete the path it replaces, per docs/principles.md). Second, option (a): a faster fixed-point division in WGSL, identical bits (the probe proves it), then re-measure the device run on the lit oak at 80 (both seeds) beside the CPU. Options (b) resident state and (c) device framing are the next task, decided on these numbers.

## Acceptance
- Stepped vs grow measured on 20 trees; identical digests; the build moved to stepped and the replaced path deleted, or the reason it isn't.
- Device division faster with identical bits; device time re-measured.
- Gate green (two parts, render first).

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
