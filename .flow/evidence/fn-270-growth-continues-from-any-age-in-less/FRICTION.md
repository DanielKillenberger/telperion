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
