# fn-291 friction

## 2026-10-16 - conductor: a wait loop that matched itself
Doing: waiting on the background `npm run build` before the R1 probe.
Slowed: `until ! pgrep -f "npm run build"` matched its own command line, so it never exited; the build had finished long before.
Cost: about 10 minutes of wall clock and two killed waiters.
Would remove it: wait on the background task's own completion (its output file's exit line), never on `pgrep -f` with a pattern the waiter's command line contains.

