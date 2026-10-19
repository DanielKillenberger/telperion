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

- **Doing:** the codex review's second finding on task .1 (all three lenses agreed on the first).
- **Slowed by:** the spec's architecture says the axis's node kind (wood, `stem: false`) keeps foliage off it; that claim was not checked. `plan::bearing_of` bears cauline leaves on any slender wood, and the axes taper to FINEST. Measured on the palm at 40 (`raw/cauline-probe.log`): cauline 1 and shootRadius 0.1 place 6,554 leaves at width 0 and 77,518 at width 0.25; cauline 0.5, 37,629; shootRadius 1, 802,300 (9,944 at width 0, the retained bases already bearing). The walk to the palm blends cauline above zero with a width above zero, so R7's walk meets it.
- **Cost:** one review round and a stop for the host; how the clothing's wood is marked unbearing in both executors is a design call.
- **Would remove it:** checking an architecture claim against `bearing_of` before the spec is marked ready (AGENTS.md, "Gates and checked claims").

- **Doing:** waiting on the task's one workspace gate after review SHIP.
- **Slowed by:** the 12 GB gate queued from 16:08 to 23:37 and was never admitted (room 1 to 8 GB); it ended by SIGTERM from outside this worker (gate_rc=143), with no test run. The task cannot close on the gate.
- **Cost:** about 7.5 hours of a finished task held open.
- **Would remove it:** a gate admission the desk can meet (a smaller declared size, or a gate run when the desk is idle), or the conductor running the gates of finished tasks in one queued batch.
