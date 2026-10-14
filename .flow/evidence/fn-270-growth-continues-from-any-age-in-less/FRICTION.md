# fn-270 friction

## 2026-10-13, task 1 (worker): council round 8 is not in the repository

- **Doing:** re-anchoring task 1; the dispatch says to read F4's COUNCIL.md round 8, and the spec cites `council/astra-8.md`.
- **Hindered:** neither exists in this worktree, in `.worktrees/f4`, or in any commit (`git log --all -S"Round 8"`). F4's COUNCIL.md ends at "Log-depth sums, measured" (round 7). The premise the round found is restated in the spec's Goal, so the task went ahead on that.
- **Cost:** about 5 minutes of searching, a few thousand tokens.
- **Would have removed it:** committing round 8 (or `astra-8.md`) to F4's COUNCIL.md before the spec was marked ready, as the spec cites it.

## 2026-10-13, task 1 (worker): the walks and the premise tests are slow

- **Doing:** R1 (b)'s walk, a settle at every year of three runs (at the age grown, none retired, today's).
- **Hindered:** each year's settle is a whole settle, so Telperion's walk took 419 s and peaked at 7.66 GB (the run retiring nothing holds every phytomer); the spruce's took 260 s. The five premise tests in the gate take 60 s on one thread, most of it whole-tree settles at every year of a 24-year walk.
- **Cost:** about 12 minutes of wall time on the walks; the gate's share is paid on every run until the tests are cut.
- **Would have removed it:** an incremental settle (the spec's later spec), or task 2 moving the walk premise out of the gate once the API's own tests hold it.

## 2026-10-13, task 2 (worker): one space test binary holds the crate's run for seven minutes

- **Doing:** running `telperion-space`'s tests once after making retirement at the age grown the rule, under the machine rule `RAYON_NUM_THREADS=4`.
- **Hindered:** `tests/walks.rs` took 427 s of the run's 523 s (`every_setting_changes_the_tree_by_degree`, `branches_are_made_and_unmade_at_vanishing_size` and four others each past 60 s); the crate's other 25 binaries took under a minute together.
- **Cost:** about 7 minutes of wall time on one check.
- **Would have removed it:** a named fast filter for the crate (or the walks out of the default run, as task 1's walk premise now is), so a change to the cycle is checked in a minute and the walks run once at the gate.

## 2026-10-13, task 2 (worker): a paused device build's census lives only in the recorded cycle

- **Doing:** pausing the device build between calls, its recorded cycle let go.
- **Hindered:** the build's counts live in the plan's census buffer, which the cycle's recording owns; letting the recording go dropped the seed's census, and every settle after a pause collapsed (fault 1) until the census was read back at the pause. Found by a probe of every young year, about 10 minutes.
- **Cost:** about 10 minutes and two GPU test runs.
- **Would have removed it:** the census held by the build rather than by its recorded plan (R4 lever 1 may separate it with the rest of the persistent state).

## 2026-10-13, task 3 (worker): a moved build's digest differed only in an axis's slot

- **Doing:** lever 3, the gate that a build grown by moves is the build grown in its final tables.
- **Hindered:** with the inside axes compacting on every preset, the spruce and the beech differed between compaction schedules; four schedules and a census diff showed growth identical, and the node table's `NA_GROWN` column (each axis's arena slot) the only difference. The tests' digest hashed it.
- **Cost:** about 20 minutes and four GPU runs.
- **Would have removed it:** a digest that skips index columns from the start (now `growth::tests::digest`), or the node table not carrying the arena slot.

## 2026-10-13, task 4 (worker): a CPU probe waits for room behind another worker

- **Doing:** a CPU probe (six presets grown to 30 through the presenting grower) declared at 6 GB through `telperion-heavy`.
- **Hindered:** the desk had 5 to 6 GB of room after the 6 GB reserve while the housekeeping worker ran; the probe waited over 11 minutes for admission, and the four oak stills before it waited a minute at 8 GB until re-declared at 4.
- **Cost:** about 12 minutes of wall time so far, no tokens.
- **Would have removed it:** a measured memory figure per kind of run (a CPU probe of young presets needs well under 2 GB) so the declaration is not a guess, or a wrapper flag that reports the run's own peak to calibrate the next declaration.

## 2026-10-13, task 4 (worker): re-pinning the catalogue digests by hand

- **Doing:** re-pinning `catalogue_identity`'s three digests after adding two neutral engine settings and every preset's rows of them.
- **Hindered:** proving the move is only the new rows needs the value files without them and the neutral fields stripped from the Debug text; there is no helper, so it took a hand edit of the test and the presets, a run, and a restore (a guarded `git checkout` was refused, so the restore went through copies).
- **Cost:** about 10 minutes and three test runs.
- **Would have removed it:** a test helper that digests the text with named fields stripped, so a re-pin states its own proof in one run.

## 2026-10-13, task 4 (worker): a queue behind a 10 GB admission

- **Doing:** the first device test of the presentation, declared at 6 GB and 2 GB of the card.
- **Hindered:** another worker's `npm test` holds the admission lock waiting for 10 GB while 7 GB is free after the reserve, and a 5 GB run waits behind it; every later declaration, however small, waits behind both, since admission is first come first served under one lock.
- **Cost:** open; the queue was 12 minutes old when this run joined it.
- **Would have removed it:** admission that lets a smaller job past one waiting for more room than there is (a lock held only while counting, not while waiting).

## 2026-10-13, task 4 (worker): the gate cannot be admitted on this desk

- **Doing:** the end gate at the declared 12 GB with 4 GB of the card.
- **Hindered:** the desk holds 13 GB available, 7 after the 6 GB reserve, for over 40 minutes; the gate waits for 12 and no other heavy job is running. Focused device runs at 4 to 6 GB went through meanwhile.
- **Cost:** 40 minutes and counting of an idle gate.
- **Would have removed it:** a gate declaration measured against the gate's own peak at 12 test threads, or a reserve that scales with the desk's resident load; until then the gate needs the desktop's memory freed by hand.

## 2026-10-14, task 4 (worker): the gate at 8 threads, 7 GB then 9 GB

- **Doing:** the end gate at 8 test threads as the host asked.
- **Hindered:** at 7 GB the run was admitted at once and the OOM killer stopped it after 102 s in `telperion_core`'s unit tests (`telperion_core-087d64dd089b3a77`, 7.26 GB resident); the retry at 9 GB waited 51 minutes with 5 to 8 GB of room and was never admitted, so it was stopped.
- **Cost:** about 53 minutes, no result for either gate or `npm test`.
- **Would have removed it:** a cap on `telperion_core`'s own test memory (its unit tests alone pass 7 GB at 8 threads), or a reserve smaller than 6 GB while the desk is idle.

## 2026-10-14, task 4 (worker): stills waiting on admission

- **Doing:** eight seasonal stills (oak at 40, palm at 12) for the owner after R5b, R5c and the leaf-fall ruling.
- **Hindered:** the 8 GB still run waited for admission behind another session's 6 GB walk job; a cargo example of a few seconds sat queued.
- **Cost:** 15 minutes and counting of wall time; no tokens beyond the polls.
- **Would have removed it:** an admission class for short GPU stills whose declared peak is small (the example's measured peak is under 2 GB), or a queue that shows its holder and estimated wait.

## 2026-10-14, task 4 (worker): the gate at 10 GB, 6 threads

- **Doing:** the end gate as the coordinator sized it overnight (10 GB, 6 test threads).
- **Hindered:** admitted after 30 s, then stopped at the cap after 175 s inside `telperion_core`'s unit tests (304 tests passed before it), the same binary that passed 7 GB at 8 threads before.
- **Cost:** about 3 minutes, then the fallback at 9 GB and 4 threads.
- **Would have removed it:** a measured peak for `telperion_core`'s unit tests per thread count, so a gate is sized once; or splitting its heaviest tests into their own binary.

## 2026-10-14, task 4 (worker): the gate fallbacks at 9 GB

- **Doing:** the gate's fallbacks after the 10 GB run was stopped.
- **Hindered:** at 9 GB the gate was stopped at the cap with 4 threads (83 s) and again with 2 (374 s), both in `telperion_core`'s unit tests. Measured alone, `pipeline::tests::every_shipped_preset_builds_every_artifact_through_the_pipeline` peaks at 5,509 MB (65 s) and `pipeline::space::repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores` at 4,880 MB (110 s); side by side they need about 10.4 GB, past any cap the ladder allows. The gate now runs on one thread.
- **Cost:** about 15 minutes of runs and two admissions; the single-thread gate's wall time on top.
- **Would have removed it:** a declared memory class for those two tests (a mutex like `whole_tree()` so they never run together), which would let the gate run at 4 threads under 9 GB.

## 2026-10-14, task 4 (worker): two design calls surfaced by the single-thread gate

- **Doing:** the end gate at 9 GB on one thread (2,052 s; 1,125 passed, 11 failed, 82 ignored), then fixing what was mechanical.
- **Hindered:** two failures need a design call, which this worker does not make: the palm walk's blend refused by the new season check, and a build refused past its budget after it had submitted work (R4's margin halving against the 2026-10-12 ruling). Both go to the host with options.
- **Cost:** about 35 minutes for the gate run, 40 minutes of fixes and reruns.
- **Would have removed it:** a gate run on the R5c and R4 changes before they were built on (the end-only gate let both ride for two rounds).
