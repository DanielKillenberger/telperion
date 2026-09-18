# fn-76 Actions runs, observed on GitHub 2026-09-18, PR #37

| Run | What | Total | receipts | rust | rust-receipts | node |
|---|---|---|---|---|---|---|
| 35385460955 | first run on the runner, cold caches, four count partitions of the whole workspace (commit c3458189) | 6 min 23 s | 11 s | (1) 4 min 17 s, (2) 3 min 52 s, (3) 5 min 50 s, (4) 5 min 06 s | 8 s | 3 min 55 s |

## What the numbers say

- **R1 across machines:** green, every one of the 49 committed digests
  matched on the runner. The desk generated them; the runner's first run was
  the cross-machine observation, and it holds.
- **R3:** the four partitions ran as one nextest pool each; the receipts were
  saved by rust-receipts after all four were green.
- **R5, every suite executes:** 6 min 23 s cold against the three-minute
  bound. Missed as the shards stood; the bound stays as written. Inside each
  shard the test phase was 93 s, 95 s, 178 s and 137 s (the spruce and the
  beech fixed-seed tests alone, each thirteen or twelve seeds one after
  another on one core), after about two minutes of cold build; the
  per-package sums were core 989 s, wasm 17 s, jev 10 s, render 8 s.

The commit after this run answers with named shards per crate and the
species seeds on threads; its run is the next row.
