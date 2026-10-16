# fn-291 friction

## 2026-10-16 - conductor: a wait loop that matched itself
Doing: waiting on the background `npm run build` before the R1 probe.
Slowed: `until ! pgrep -f "npm run build"` matched its own command line, so it never exited; the build had finished long before.
Cost: about 10 minutes of wall clock and two killed waiters.
Would remove it: wait on the background task's own completion (its output file's exit line), never on `pgrep -f` with a pattern the waiter's command line contains.

## 2026-10-16 - task 1: the probe sweep and the native trace queued behind another heavy job
Doing: the R1 sweep (24 cases plus 12 split runs, one child each under `telperion-heavy 6`) and then a native trace of the failing wood at 10 GB.
Slowed: another worktree's 12 GB heavy scope held the admission room; the sweep took 20m42s wall for 6m20s of CPU, and the native trace waited for room.
Cost: about 15 minutes of waiting on admission.
Would remove it: nothing in this repository; admission is working as the owner set it. A probe that reads the wood's size without building it (a wasm32 extent prediction) would make the trace a millisecond question instead of a 10 GB build.

