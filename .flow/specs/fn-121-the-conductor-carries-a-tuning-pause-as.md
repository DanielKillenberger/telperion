# The conductor carries a tuning pause as its own

## Goal

A tuning revision pauses by design (pilot authority, priority approval, preflight), but the conductor treated every pause as a failure: it recorded no revision and never ran its gap check, so the host drove the palm's three revisions by hand beside it (fn-80 FRICTION, 2026-09-23). The conductor must carry the pause, pass the decision through and, when the revision ends, record it and check its gaps. Owner (2026-09-23): "yes pls fix the frictions if they're obvious."

## Decisions

- `tune` reads the tuning run's `run.json` after exit: a pause becomes a conductor pause with the tuning pause's id, reason, request and a handoff; `species-conductor resume --decision FILE` runs `tuning-loop run --resume FILE` in the same directory. A revision counts as ended only when its record holds neither a pause nor an attempt in flight.
- Superseded (2026-09-26, fn-149 R8): the conductor was removed by fn-149 (#121); the runner has no tuning pause.

## Acceptance Criteria

- **R1:** A tuning pause becomes a conductor pause with handoff, not a failed step — met.
- **R2:** `resume` runs `tuning-loop run --resume` in the same directory — met; a refused decision leaves the run paused.
- **R3:** An ended revision is recorded and every gap checked — met (reachable, covered, new via scripted transport).
- **R4:** Gate green — met (973 passed).

## Result

Landed in `47da598b`, `b5dd103d`; later superseded by the fn-149 runner.

## Open

None.

## Friction

- The gate's cold build took 521 s -> standing cost of a fresh worktree.
