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

## 2026-10-16 - task 5 (cap order fix): the flat-digest check was killed at the 8 GB cap
Doing: re-running `flatdigest.rs 1,42 --expect` after the split-run cap fix, under `telperion-heavy 8`.
Slowed: telperion seed 42 natively needs more than 8 GB; the wrapper killed the run after 13 of 16 lines, and it was re-run at 14 GB.
Cost: about 3 minutes and one extra build-and-run.
Would remove it: PROBE.md naming the cap the equivalence check needs (14 GB at seeds 1 and 42) next to the command.

## 2026-10-16 - task 6: a 590 s test call waited out its whole budget for admission
Doing: the core crate's integration tests after the encoded wood, under `telperion-heavy 14 --threads 8`.
Slowed: another worktree's workspace gate (devfield, `cargo test --workspace`) held the heavy room; the wrapper printed "room 4 GB for 14" every 30 s until the 590 s timeout, and no test ran. The crate-wide run before it had also hit 590 s, its lib suite alone taking 405 s.
Cost: about 20 minutes of wall clock, nothing measured.
Would remove it: an admission that fails fast with a reason when the room is held by another job's gate, and the named wood filter set the task 5 entry asks for.

## 2026-10-16 - task 6: the browser binding suite fails before reaching the wood
Doing: `npm run rust:test:wasm` (tests/browser/bindings.mjs in headless Chromium) for the chunk reads task 6 added.
Slowed: the suite stops at checks unrelated to the wood that fail on this base: "ordinary has local branches and a leaf plan" (nodes 152,178 = crossover), "bad height" (envelope height -1 builds; devfield's dist does the same), "caps explicitly partial" and "valid empty outputs" (growth.maxNodes is not honoured). The new chunk checks were replayed in Node against the built Wasm instead (`.flow/tmp/t6/abi.mjs`, all pass).
Cost: about 15 minutes and three headless runs.
Would remove it: bringing bindings.mjs up to the current generator (or running it in CI so it cannot go stale); out of this task's scope.

## 2026-10-16 - task 3: the redirect guard refused scratch paths held in a variable
Doing: writing measurement output to the session scratchpad through a shell variable (`> $S/x.log`).
Slowed: the dcg hook refuses any redirect whose target it cannot read literally. Three commands were refused and had to be rewritten as small scripts with literal paths.
Cost: about 5 minutes and three retries.
Would remove it: an evidence helper that takes the output path as an argument, or measurement commands written as scripts from the start, as `trace.sh` and `hash.sh` now are.

## 2026-10-16 - task 4: the stills waited for admission, and a pipe into node was refused
Doing: rendering the Telperion and Laurelin stills (`stills.sh` under `telperion-heavy 14`), and first piping the native base mesh driver into the still tool.
Slowed: the dcg hook refuses `driver | node still.mjs`, as an unverifiable pipeline consumer, so the tool now spawns the driver itself; then another job held the heavy room (room 4 to 11 GB for a 14 GB request) from 14:44 for more than 10 minutes before any still was drawn.
Cost: about 5 minutes on the pipe, and 22 minutes of admission (14:44 to 15:06) in which smaller jobs were admitted ahead of the 14 GB request and no still was drawn. Split into per-step requests (8 GB a dist still, 10 GB a native one), the same twelve renders were admitted at once and took 7 minutes (15:11 to 15:18).
Would remove it: done in `stills.sh`, which now admits each step at its own size. A general form: an admission that does not starve a large request behind a stream of smaller ones.
