# fn-298 friction

## 2026-10-20 13:45, task .1: the first device run queues on the desk's memory

Doing: the R1 base run of the fn-276 device tests (`telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- cpu_builds_tests`), the first build in this worktree's cold `target/`.
Slowed by: `telperion-heavy` admits a job when available RAM minus its 2 GB reserve covers the 8 GB asked; the desk had 6 to 8 GB available (browsers hold most of the 31 GB) and no other heavy job was running, so the room stood at 4 to 6 GB and the job waited from 13:34.
Cost: the wait, in minutes, with no work possible on R1 or R2 (both need the device).
Would remove it: a smaller ask for a single-crate test build at 2 threads (its real peak, measured once, in place of 8 GB), or free memory on the desk before a dispatch.

## 2026-10-20 14:03, task .1: the wait passed 30 minutes; the task returns NEEDS_HUMAN

The same job never started: from 13:34:18 to 14:02:49 the room stood at 3 to 7 GB for the 8 asked (59 polls), with no other heavy job claiming memory. Per the dispatch the queued job was stopped and the task returned `NEEDS_HUMAN` with nothing measured. Cost: 29 minutes of wall time and the R1 and R2 runs not begun. Prepared for the resumed run, both under `raw/`: `r1-new-test.patch` (the t = 0.2160 row as a test, `fn298_...`, not yet run) and `r2-scratch.patch` (the R2 switch, `R2_FULL=all|rounds,passes,...`; unset, it builds as the base).
