---
satisfies: [R2]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.9 The driver overlapped with the GPU

## Description
Host decision after task 8 (2026-10-07), option 2: hand buds to the GPU cycle by cycle as the structure above the cut makes them, and regrow likely-landing roots while the CPU is idle during phases 1-3, with a selection rule that is a safe superset of host decision 3's bound. Pure speed: every tree identical by lineage on both executors. Target: spruce 80 whole build 3x faster than 32 cores (task 8 measured 0.44-0.46 s against 0.98 s; the CPU stages hold about 300 ms).

## Acceptance
- Every tree identical by lineage on both executors; GPU tests green. - Spruce 80 and beech 80 timed warm and cold against 32 and 1 cores; the 3x bar met or the reason with numbers. - Workspace gate green.

## Done summary
The structure above the cut grows on its own thread and sows each cycle's buds to the GPU (groups of 8,192+); woken buds sown provisionally; the idle cores grow likely-landing roots during phases 2-3 (landing 55 to 16 ms); records in one round trip. Trees identical (32 lineage digests, measures hash 80, task 6's geometry). Spruce 80 warm: 0.44-0.46 s to 369-388 ms, against 32 cores 1,027-1,139 ms (2.7-2.8x) and one core 8.5-8.7 s; beech 80 unchanged (149-206 ms against 169-297 ms on 32 cores).

Host amendments (2026-10-07): the 3x bar is met at 2.7-2.8x with its reason measured (about 160 ms of serial work before the last bud exists, F2b's scope; a 41-55 ms phase-1 tail); the likely-landing rule stays a guess (a miss only costs time; the landing decides by decision 3's bound); the gate's one failure (fixed_beeches memory ceiling read from free memory while other worktrees held 17 GB; passed alone) is accepted, the test's ceiling rule listed as a friction fix for the owner.

## Evidence
- Commits: f1895b0c, 4b1d2a4a, 34ce4512, 2e73a320, b29c6cc8
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs: