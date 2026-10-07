---
satisfies: [R2]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.8 The predictor merged, and the GPU's growth made fast

## Description
Owner, 2026-10-06: "I want to wake up to super high performing generator"; "if you see a path for perf improvements try them". After task 6 the GPU executor equals the CPU to the bit but is no faster: spruce 80 1.1 s on both the GPU and 32 cores; beech 80 0.38-0.53 s on the GPU against 0.17-0.28 s. Phase 1 (chain growth) is most of it (spruce 80: 735-790 ms), each presence costing four 128-bit log2 calls (REALIZED.md, "The GPU mirror built").

1. First merge branch fn-209-predictor (task 7, the count-only predictor) into this branch: it conflicts with task 6's `settle` driver in crates/telperion-space/src/fine.rs and in two evidence files. Keep both: task 6's driver with the predictor feeding its cut; every tree identical (lineage digest, `measures hash 80`), the GPU tests green.
2. Then make the GPU's growth phase fast without moving a bit: REALIZED.md's options (a table of the threshold's logit for species-constant bounds; one logit_draw per two-sided lead; a workgroup per heavy chain for the beech), and whatever the profile names next (per-kernel GPU timestamps). Every change keeps CPU and GPU identical by lineage and the contract's probe green.
3. Measure the whole fine stage and `grow` end to end (GPU against 1 and 32 CPU cores, warm and cold) for beech and spruce at 40 and 80.

Bar: the GPU at least 3x faster than 32 CPU cores on spruce 80 and faster than 32 cores on beech 80; if the profile shows that is out of reach, report why with the numbers.

## Acceptance
- fn-209-predictor merged; every tree identical; GPU tests green. - CPU and GPU identical by lineage after every speed change; the contract probe green. - GPU at least 3x faster than 32 CPU cores on spruce 80 and faster than 32 cores on beech 80, or the reason with numbers. - Workspace gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
