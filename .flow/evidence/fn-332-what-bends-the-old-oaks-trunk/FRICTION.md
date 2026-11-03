# fn-332 friction

## 3 Nov 2026: a gate beside a worker's builds overran its budget

Doing: the gate on fn-332's one example file, while fn-333's worker built and tested in its own worktree.

Slowed by: the two shared the CPUs (load average 18); the test run took 1,945 s against the 1,560 s budget and the gate failed on time alone. About 32 minutes lost, and the gate must run again.

Removed by: running a gate only when no other build or test suite runs; the host waits for a worker's narrow runs to finish before starting one.

## 3 Nov 2026: the no-shade run outgrew the desk

Doing: R2's `TRUNK_SHADE=0` oak at 200.

Slowed by: it was killed at its 3, 6 and 12 GB claims in turn, each after minutes of growth and a wait for room; one batch at 8 GB was also killed by the session's memory pressure. About an hour over the afternoon, and the row stood as unmeasured.

Removed by: a machine with room. The owner's MacBook (36 GB) now takes heavy runs over SSH; the run grew there in 224 s at 17.2 GiB.

## 3 Nov 2026: git could not write its index

Doing: committing R1's seed 7 rows.

Slowed by: `/home`'s btrfs metadata was full (6.5 of 7.0 GiB, no unallocated space), so git failed with "unable to write new index file". The edits waited about two hours, until the owner ran a balance.

Removed by: the balance (`btrfs balance start -dusage=10 /home`), which the owner ran.

## 3 Nov 2026: the gate overran again under another project's load

Doing: the gate at 11f39ee0d (the join fix and the result), run alone from this run's side.

Slowed by: the desk's load average was 20 to 30 from other work (another repository's node test suites, a local search index); the test run took 1,939 s against 1,560 with every test passing. About 35 minutes lost, and the gate waits for a quiet desk.

Removed by: the gate waits on the desk's load, not only on its memory: `telperion-heavy` queues on room but not on CPU (fn-333's friction names the same gap). Local setup, reported to the owner.

## 3 Nov 2026: the gate on the MacBook, three blockers

Doing: the gate on the owner's MacBook, idle beside the loaded desk.

Slowed by: the render crate did not compile on macOS (`free_device_memory` asked wgpu for Vulkan unconditionally off Wasm); the checkout had no `master` for the gate's base; and two evidence files differing only by case (`fn-294-.../CODEX-RELEASE-REVIEW.md`, `codex-release-review.md`) leave a checkout on a case-insensitive disk always dirty, which the gate refuses. About 40 minutes, with Node and nextest installed.

Removed by: 01869223c (None on Apple platforms, as the function's doc said); pushing `master` with the branch; and, for the case clash, `assume-unchanged` on the Mac. Renaming one of the two files would remove the last for any Mac checkout; that file belongs to closed fn-294.
