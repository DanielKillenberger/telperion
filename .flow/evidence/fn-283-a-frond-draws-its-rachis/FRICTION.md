# fn-283 friction

## 2026-10-19, task .1 (worker)

- **Doing:** recording every shipped preset's CPU mesh hash on the base before the axes land (AC2).
- **Slowed by:** `telperion-heavy 8` waited 10 minutes for room (6 to 7 GB free beside the desktop); the base had to be recorded before any edit, so the edit waited with it.
- **Cost:** about 12 minutes idle.
- **Would remove it:** a byte-identity check that builds the base from a revision (fn-224's `space_hash` taking a revision), so the before half does not block the edit.

- **Doing:** the task's one workspace gate (`telperion-heavy 12 --threads 2`).
- **Slowed by:** the queue holds the gate until 12 GB are free beside a 4 GB reserve; at 16:08 the desk had 6 GB of room, so the finished task waits on memory, not on work.
- **Cost:** recorded in the done summary once the gate is admitted.
- **Would remove it:** a gate size measured against the desk's usual free memory, or a smaller declared size for a crate-only change.

## 2026-10-19 17:00, host of the run (conductor)

- **Doing:** waiting on task .1's workspace gate before tasks .2 to .4 can start (the chain is sequential).
- **Slowed by:** the heavy queue admits a 12 GB job only with 14 GB available; the desk shows 8 GB available (22 GB used, 6.5 GB of it the RAM-backed /tmp, 5.8 GB of that under /tmp/claude-1000). Another worktree's identical gate (f4-294) has waited 4 h 31 min unadmitted beside it, so two 12 GB gates now queue for room that has not appeared.
- **Cost:** the whole build stalls behind admission; open-ended.
- **Would remove it:** a gate declared at the memory it measures (its peak on this machine), or freed /tmp; the /tmp usage is a local setup matter for the owner, not a spec.
