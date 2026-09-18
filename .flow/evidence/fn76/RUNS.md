# fn-76 Actions runs, observed on GitHub 2026-09-18, PR #37

| Run | What | Total | receipts | rust | rust-receipts | node |
|---|---|---|---|---|---|---|
| 35385460955 | first run on the runner, cold caches, four count partitions of the whole workspace (commit c3458189) | 6 min 23 s | 11 s | (1) 4 min 17 s, (2) 3 min 52 s, (3) 5 min 50 s, (4) 5 min 06 s | 8 s | 3 min 55 s |
| 35387930637 | named shards per crate and the species seeds four at a time, caches warm from the run above (commit 6b11325a) | 2 min 55 s | 11 s | core 1/4 2 min 22 s, core 2/4 2 min 33 s, core 3/4 2 min 12 s, core 4/4 1 min 59 s, render 2 min 00 s, wasm and jev 2 min 32 s | 3 s | 2 min 22 s |

## What the numbers say

- **R1 across machines:** green, every one of the 49 committed digests
  matched on the runner. The desk generated them; the runner's first run was
  the cross-machine observation, and it holds.
- **R3:** the four partitions ran as one nextest pool each; the receipts were
  saved by rust-receipts after all four were green.
- **R5, every suite executes:** 2 min 55 s on the second run, against the
  three-minute bound. Met. The first run missed it at 6 min 23 s: inside each
  shard the test phase was 93 s, 95 s, 178 s and 137 s (the spruce and the
  beech fixed-seed tests alone, each thirteen or twelve seeds one after
  another on one core), after about two minutes of cold build; the
  per-package sums were core 989 s, wasm 17 s, jev 10 s, render 8 s. Running
  a species' seeds four at a time inside `fixed_species` is what closed the
  gap: no shard is bound by a sum of seeds any more, and the six jobs finish
  within 34 s of each other.
- **Shards, named per crate (owner, 2026-09-18):** `core 1/4` to `core 4/4`,
  `render`, `wasm and jev`, in place of the earlier `rust (1)` to `rust (4)`
  count partitions of the whole workspace. Each job names the crate it tests,
  and `rust-receipts` saves a crate's receipt when that crate's jobs are
  green rather than all-or-nothing.
- **The caches were warm for the second run and cold for the first.** A
  master run after the merge is cold again, because receipts and build caches
  are branch-scoped; its row belongs beside these.

## A receipt that was never written (found in host review, fixed)

Run 35388400735, the third on this PR, ran every Rust job again although
nothing a Rust suite reads had changed, while the Node suite skipped on its
receipt. The cause is in the `rust-receipts` job of the named-shard commit:
`actions/cache/restore` with `lookup-only` leaves `cache-hit` **empty** on a
miss, the job publishes that empty value, and `read -r suite key hit result`
collapses the empty field, so each crate's job result landed in `hit` and
`result` stayed empty. The guard `[ "$result" = success ]` was then never
true and no Rust receipt was ever saved. Node was unaffected because its
receipt is written by the suite composite action, which takes the value as a
named input instead of a whitespace-split field.

The fix publishes each hit as the literal `true` or `false`
(`cache-hit == 'true'`), so no field can be empty. The run after the fix is
the one that proves a Rust receipt is written, and the run after that is the
one that proves the skip.
