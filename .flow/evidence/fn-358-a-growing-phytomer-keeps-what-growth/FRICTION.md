# fn-358 friction

## 12 Nov 2026, 08:13 to 08:21, builder (task 1): a one-test pin run queued behind the scoreboard

- **Doing:** running the new pinned test once on the base, to read its pins before any product change.
- **Slowed by:** `telperion-heavy 8` waited with 0 to 1 GB of room while the scoreboard drew the oak at 200 in its own scope. Product edits in the checkout would have reached the queued build, so the edits were blocked too.
- **Cost:** 8 minutes of waiting, then a cold build of the base in a copy under the scratchpad with its own `target/`, also queued.
- **Would remove it:** a way to snapshot a commit for a queued run (build from `git archive` into its own target), or a small reserved room in the queue for single narrow tests.
