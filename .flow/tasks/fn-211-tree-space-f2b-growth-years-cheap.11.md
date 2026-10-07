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
Part 1: stepped and grow give identical digests on all 20 trees. Stepped is faster on the lit oak (0.75 to 0.78 times grow) and level on the unlit oak (0.87 to 0.99). It is slower on the beech (1.11 to 1.50) and the spruce (1.53 to 1.90), whose grow is grow_fine, so the build was not moved. The gain comes from the cores: stepped spreads over the items the balance, allotment, rough layout and waking that the whole-tree grower runs on one thread. On one thread stepped is 1.09 times slower. Options for the host are in TASK11.md.

Part 2: the WGSL quotient is now Knuth's algorithm D in 16-bit digits. It matches the CPU to the bit on 4.2 M cases and on the probe. Layout kernels take 22 to 32 percent less time and D16 kernels 28 to 49 percent less. The device run takes 13.35 s and 30.74 s, against 6.86 s and 14.57 s on the CPU.

The gate is green: render 168 passed, the rest 1,013 passed.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 2e41e078, 62432ff6, 8a08be03
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render
- PRs: