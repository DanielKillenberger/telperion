---
satisfies: [R4]
---
# fn-211-tree-space-f2b-growth-years-cheap.5 The oak stepped on the GPU

## Description
Probe (b) and the oak on the GPU (DESIGN-INTERLEAVE.md, "After Astra's review", items 2 and 5). Starts after fn-209 task 6 (realized mechanics on fn-209's GPU executor) is merged into this branch.

- fn-209's GPU executor steps fine roots one cycle at a time under PHASES.md's four phases, with persistent device buffers; only per-root records cross (Astra change 3), round trips counted and timed apart from useful work.
- Canonical arithmetic through the light feedback on the device: the lattice deposits mirror fixed/sum.rs; the sweep, sampling, shade survival and balance hazard follow a specified procedure both executors run (extend FINE-CONTRACT.md); the CPU reference adopts it and its change on today's trees is measured apart from CPU/GPU agreement.
- The oak's refusal in fn-209's executors lifted when the oak at 40 and 80, seeds 1 and 7, is identical by lineage to the stepped CPU executor, geometry within the screen.
- Cost: the oak from parameters to a laid tree, warm and cold, against today's 3.65 s (80, seed 1, all cores).

## Acceptance
- The oak identical by lineage to the stepped CPU executor on the GPU (geometry within 50 µm max, 10 µm RMS) at 40 and 80, seeds 1 and 7. - The canonical light arithmetic specified, mirrored and measured on today's trees. - Round trips counted; oak timing warm and cold against today. - Workspace gate green.

## Done summary

Blocked on an owner-level design decision (host, 2026-10-07; TASK5-GPU.md). The oak's rough layout reaches topology through light (positions, wood, sizes, balance sums, re-lay girth and sag), so CPU/GPU identity by construction needs option A: the whole rough layout and its light feedback in one specified fixed-point arithmetic on both executors (about 5-7 sessions, moves today's oak at rounding level). Option D (f32 with a measured flip rate) is declined: it breaks the CPU reference as the definition of correct (fn-125) and repeats the shape declined for D17. Interim host decision: option C, light-reading roots stay on the exact CPU stepped executor (no GPU speed-up for the oak yet). Astra reviews A against alternatives; the owner decides in the morning. Merge f929dbeb and probe (b) (113-282 ms of synchronisation at oak 80) are done and kept.

## Evidence
- Commits:
- Tests:
- PRs:
