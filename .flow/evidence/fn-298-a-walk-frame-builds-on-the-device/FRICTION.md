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

## 2026-10-20 17:50, task .2: the one-minute step-1 trace waited 40 minutes for room

Doing: the revised ruling's step 1, a single `telperion-heavy 8 --threads 2` run of a scratch trace that grows four presets twice, stamped. The test itself ran in 10 s.
Slowed by: from 17:09 to 17:49 the room stood at 2 to 4 GB for the 8 asked, while other agents' jobs held memory.
Cost: 40 minutes of wall time with no device work possible; the step's real work was under 2 minutes.
Would remove it: as the 13:45 entry says, a measured smaller ask for a single-crate test build, or a reserved slot for short measurement runs.

## 2026-10-20 18:10, task .2: the fixed frame met the next stepped limit only once the first was lifted

Doing: option (d), the layout's closing pass, against the acceptance "the t = 0.2160 test is green".
Slowed by: with the layout no longer refusing, the same frame is refused by `HierGens` at cycle 10 (`rounds` 3 projected from `gens` 1; 4 reached). R1 could only see the first refusal a frame meets, so the acceptance assumed `laygens` was the frame's only limit.
Cost: small in time (two short admissions, about 11 minutes of device time); the task stops `NEEDS_HUMAN` with its acceptance unmet.
Would remove it: an R1 repro that records every stepped limit a frame passes, by running it with each other limit at its bound (the `R2_FULL` scratch switch), before a fix is scoped to one.

## 2026-10-20 18:40, task .2: the survey found a refusal no earlier run could reach

Doing: the host's survey of every R3 frame with the kept limits at their bounds (one admission, no wait; 819 s for 1,533 frames).
Slowed by: 42 oak-to-palm frames (i 106 to 119, every seed) are refused before their build by `FRAMES` = 11, a table no stepped limit touches. Neither R1 (the six FILM.md rows) nor fn-276's sweep (`T` = 0, 0.01, 0.1, 0.5, 0.9, 1 per walk) builds a frame between t 0.9 and 1, so this surfaced only now, after two fixes were scoped.
Cost: no time lost to the run itself; the task's acceptance (every frame builds) is further than it looked, and the fix is a host call.
Would remove it: run R3's own frame list (this survey's segment table) as the first repro of a spec like this one, before any fix is scoped.

## 2026-10-20 18:50, task .2: the ruled `rounds` floor rested on which census a cycle has read

Doing: the host's `rounds` ruling, which floors a cycle's `rounds` by "the latest read census's laid depth (a cycle fresher than `gens`)".
Slowed by: the claim it was conditioned on holds exactly (k = 0), but a cycle's limits read the census two cycles back, not one (`build_grow.rs:381`, `428`), so the floor covers 1 of the 9 frames. The task stops `NEEDS_HUMAN` a second time with nothing built.
Cost: about 10 minutes and one 81 s admission; another host round trip.
Would remove it: a ruling that names a census states its cycle offset, checked against `Growing::cycle` before the acceptance is written around it (AGENTS.md, "Gates and checked claims").
