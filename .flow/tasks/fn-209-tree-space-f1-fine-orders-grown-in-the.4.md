---
satisfies: [R1, R2]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.4 Fine geometry on the GPU executor

## Description
Task 2 built growth and shedding on the GPU (GPU-FINE.md); scale, girth and lay are not ported. Port them (about 1,800 lines of CPU code, in f32) so the executor ends at the botanical tree plus laid geometry in device buffers (host decision 5). Measure geometry against the CPU reference within fn-125's screen, and the whole fine stage's time.

## Acceptance
- Geometry of every fine root within 50 µm max, 10 µm RMS of the CPU reference, for beech and spruce at ages 40 and 80, seeds 1 and 7, in any item order and batch. - Topology still identical by lineage. - Whole fine stage (grow, shed, scale, girth, lay) timed warm and cold, worst item included, against the CPU executor on 1 and all cores. - A CPU-side WGSL parse (naga) in an ordinary test (FRICTION.md). - Workspace gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
