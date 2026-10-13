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
