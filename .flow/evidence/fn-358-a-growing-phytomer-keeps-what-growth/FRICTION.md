# fn-358 friction

## 12 Nov 2026, 08:13 to 08:21, builder (task 1): a one-test pin run queued behind the scoreboard

- **Doing:** running the new pinned test once on the base, to read its pins before any product change.
- **Slowed by:** `telperion-heavy 8` waited with 0 to 1 GB of room while the scoreboard drew the oak at 200 in its own scope. Product edits in the checkout would have reached the queued build, so the edits were blocked too.
- **Cost:** 8 minutes of waiting, then a cold build of the base in a copy under the scratchpad with its own `target/`, also queued.
- **Would remove it:** a way to snapshot a commit for a queued run (build from `git archive` into its own target), or a small reserved room in the queue for single narrow tests.

## 12 Nov 2026, 08:21 to 08:50, builder (task 1): the narrow suites and R3 cannot start beside the scoreboard

- **Doing:** type checks, the new tests, the mutants, and then R3's measures and the narrow suites.
- **Slowed by:** the queue's room stayed at 0 to 1 GB the whole time, because the scoreboard's scope claims 40 GB and uses about 5. Checks, the new test and the mutants were run in 1 GB scopes, `-j 1`/`-j 2`, and all fit. R3 (up to about 8 GB at the oak at 120) and the narrow suites (16 GB) cannot run until the scoreboard ends.
- **Cost:** 4 minutes of a 4 GB check that never started; R3 and the narrow suites wait an unknown time.
- **Would remove it:** a queue that charges a running job for what it uses plus a margin, not for its whole cap; or a scoreboard scope sized to its measured peak.
