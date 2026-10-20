# fn-298 friction

## 2026-10-20 13:45, task .1: the first device run queues on the desk's memory

Doing: the R1 base run of the fn-276 device tests (`telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- cpu_builds_tests`), the first build in this worktree's cold `target/`.
Slowed by: `telperion-heavy` admits a job when available RAM minus its 2 GB reserve covers the 8 GB asked; the desk had 6 to 8 GB available (browsers hold most of the 31 GB) and no other heavy job was running, so the room stood at 4 to 6 GB and the job waited from 13:34.
Cost: the wait, in minutes, with no work possible on R1 or R2 (both need the device).
Would remove it: a smaller ask for a single-crate test build at 2 threads (its real peak, measured once, in place of 8 GB), or free memory on the desk before a dispatch.

## 2026-10-20 14:03, task .1: the wait passed 30 minutes; the task returns NEEDS_HUMAN

The same job never started: from 13:34:18 to 14:02:49 the room stood at 3 to 7 GB for the 8 asked (59 polls), with no other heavy job claiming memory. Per the dispatch the queued job was stopped and the task returned `NEEDS_HUMAN` with nothing measured. Cost: 29 minutes of wall time and the R1 and R2 runs not begun. Prepared for the resumed run, both under `raw/`: `r1-new-test.patch` (the t = 0.2160 row as a test, `fn298_...`, not yet run) and `r2-scratch.patch` (the R2 switch, `R2_FULL=all|rounds,passes,...`; unset, it builds as the base).

## 2026-10-20 15:00, task .1 (resumed): the batched R1/R2 run queues behind another agent's workspace gate

Doing: one `telperion-heavy 8 --threads 2` admission holding the whole task's device work (build, the R1 tests under `CYCLE_TRACE=1`, and 21 `build_time` runs: stepped, all at the bound, and each stepped limit alone, three each), batched so the queue is paid once.
Slowed by: from 14:49 the room stood at 5 to 6 GB for 8 while another agent's `cargo test --profile ci --workspace` held its own 8 GB claim (`telperion-heavy-444462`).
Cost: 41 minutes (14:49 to 15:30 admission), with R1 and R2 both blocked on the device.
Would remove it: the same as the 13:45 entry, a measured smaller ask for a single-crate test build.

## 2026-10-20 15:37, task .1: the per-limit runs drifted against controls taken earlier

Doing: R2's per-limit A/B. The first batch interleaved stepped and all-at-bound runs, then ran each limit alone afterwards with no control beside it.
Slowed by: the stepped total moved from 4,298 ms (15:30 batch) to 4,002 ms (15:37 batch), a 7% drift between batches, larger than the per-limit effects. The first batch's per-limit numbers (all 4 to 5% faster than its stepped runs) are unusable, and a second batch interleaving a stepped control with each limit was needed.
Cost: one more heavy admission (30 s wait) and 3 minutes of device time; small, but a wrong decision would have followed from the first table.
Would remove it: `build_time` A/Bs always interleave their arms within one batch; a scratch switch read per build (not per process) would let one run alternate arms per preset.
