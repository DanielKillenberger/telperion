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

## 2026-10-20 02:27, worker on task .2

- **Doing:** recording the base's device digests (every shipped preset, and the palm's young, fractional-year, settled and field lines) before the move, through the heavy queue at 6 GB.
- **Slowed by:** the queue shows 3 GB of room for the 6 GB focused run, so the base build waits on admission; no source can be edited until the base has compiled, since the queued run would compile the edits instead.
- **Cost:** 31 minutes queued (02:25 to 02:56), behind another worktree's 8 GB workspace gate.
- **Would remove it:** a base digest recorded by the previous task's gate run, or a cheaper way to snapshot the base's test binary before edits begin.

## 2026-10-20 04:00, worker on task .3

- **Doing:** the first type-check of the device axes (`telperion-heavy 6 --threads 4 -- cargo check`).
- **Slowed by:** the queue shows 2 GB of room for the 6 GB job, behind another worktree's 8 GB workspace gate (`telperion-heavy-3024530`); a `cargo check` cannot run outside the queue under the run rules.
- **Cost:** queued from 03:50; recorded once admitted.
- **Would remove it:** a smaller declared size for a type-check (it needs well under 6 GB), or a check lane that does not wait behind gates.
- **Admitted:** 04:28, after about 38 minutes queued.

## 2026-10-20 05:10, worker on task .3

- **Doing:** the agreement suites at the palm's 0.25 m width.
- **Slowed by:** two existing gates fail on a tolerance, not a fault. `every_preset_sweeps_its_wood_as_the_cpu_does`: the palm's 315 runs all match point for point, centre 1.01e-3 m against a 1e-3 bound. `every_preset_s_device_field_answers_within_its_tolerance`: the palm at seed 5678 and side 24 gives a wood Jaccard of 0.9976 against 0.999. `rachis_tests` measured the cause. The error grows linearly along each axis and is already 3e-4 m at the insertion. Both executors take the crown's axis from the apex and its parent. Positions that agree by lineage (about 5e-6 m) across a short apex internode leave the two axes about 1e-3 rad apart (inferred from the insertion error over a depth of 0.35 m or less). Every frond heading carries that angle, the leaflets' included, so it reaches about 1 mm at a 7 m rachis's tip.
- **Cost:** a stop for the host. The choice is between widening these gates for the frond axes and conditioning the crown's axis differently, which is a core change outside this task. That is a gate and design call.
- **Would remove it:** an agreement bound for long wood stated as a direction tolerance times the reach, as the leaves' bound is already relative to height, set when the spec is written.
