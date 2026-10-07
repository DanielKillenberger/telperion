---
satisfies: [R4]
---
# fn-211-tree-space-f2b-growth-years-cheap.10 Contract A: the oak's fine roots on the device, against the CPU

## Description
Split from task 9 by the host (2026-10-07). Inside the stepped executor, the device runs the fine roots' deposits, sampling and D16 each cycle and the CPU runs the rest; complete phase state compared against an all-CPU run on a small lit specimen spanning a scheduled re-lay, across cuts, orders, chunkings and a forced retry. A full device grower for lit fine roots is a later step.

## Acceptance
- Complete phase state identical CPU against device across cuts, orders, chunkings and a retry.
- Gate green (two parts, 8-thread limits).

## Done summary
Inside the stepped executor the device runs the lit fine roots' rough layout (units sized, ranked and stepped), D16, deposits and sampling; the CPU frames each round and runs the rest. Every grower's whole state after every phase and every lattice equal the all-CPU run's on the lit oak at 24 (two re-lays) and on a neighbour that wakes D16's sheds, carried lifts and the landing: cuts 64, 256 and 1,024, three item orders and chunkings, three device deposit orders and chunkings, forced retries. Six device mutations are each caught. Timed at 80 (cut 1,024, warm): seed 1, today 8.73 s, stepped CPU 6.67 s, stepped device 13.52 s (GPU kernels 3.8 s); seed 7, 17.15 / 14.59 / 32.34 s. Identity at 80 holds by phase and tree digests. TASK10-GPU.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: ba848ae1, a963e86a, 11d3b703, 15c447f7
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render
- PRs: