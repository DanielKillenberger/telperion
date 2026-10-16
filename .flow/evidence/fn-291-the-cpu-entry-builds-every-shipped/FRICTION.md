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

## 2026-10-16 - task 2: a hash run killed at its 8 GB cap
Doing: hashing every preset's dist outputs before and after R2a, all 8 presets in one Node process.
Slowed: `engine.dispose()` drops the instance but its Wasm memory is freed only by the garbage collector, so telperion and laurelin together passed the 8 GB cap and the run was killed twice before laurelin ran alone.
Cost: about 5 minutes.
Would remove it: one process per heavy preset (as probe-dist.mjs does), or a forced GC between builds in evidence tooling.


## 2026-10-16 - task 5: the core crate's suite outran a 10-minute foreground call
Doing: the focused run of `cargo test --profile ci -p telperion-core` after the chunked wood, under `telperion-heavy 12 --threads 4`.
Slowed: the crate's suite grows every shipped preset several times (species suites, the shipped-preset pipeline test, the attachment test over CATALOGUE and IN_WORK); four threads under the heavy cap took past the 600 s tool limit, so the run went to the background and the worker waited on it.
Cost: over 10 minutes of wall clock for one crate.
Would remove it: a named filter set for "the wood's tests" (surface, mesh, species digests) that a task touching the surface runs instead of the whole crate.

## 2026-10-16 - task 5: a one-test red run waited 31 minutes for admission
Doing: the red run of one new unit test (`chunks::tests::the_end_of_a_full_odd_chunk_is_the_next_chunks_start`) during the review's fix pass, under `telperion-heavy 12 --threads 4`.
Slowed: another job held the heavy room (4 GB free for a 12 GB request) from 11:07 to 11:39; the test itself ran in 0.00 s.
Cost: 31 minutes of wall clock.
Would remove it: a smaller admission request for a single-test run (the crate's test binary was already built; 12 GB is the whole-suite figure), or an admission class for incremental test runs.
