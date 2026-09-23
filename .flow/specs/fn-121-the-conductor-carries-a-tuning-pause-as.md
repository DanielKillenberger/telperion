# The conductor carries a tuning pause as its own

## Conversation Evidence

> fn-80 FRICTION.md, 2026-09-23: "the conductor cannot resume a tuning run it started"; the host resumed three tuning revisions by hand beside the conductor, so its gap check never ran on them.
> user (2026-09-23): "yes pls fix the frictions if they're obvious."

## Goal & Context
<!-- scope: business -->

A tuning revision pauses by design: for its pilot authority, for the owner's priority approval, at a preflight. The conductor treats every such pause as a failure, records no revision and never runs its gap check (`gapcheck`: reachable back to tuning, covered as a dependency, new escalated), so the host drove the palm's three revisions by hand and answered their gap lists itself. The conductor must carry the pause, pass the decision through and, when the revision ends, record it and check its gaps. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.** `conductor/step.rs` `tune` runs `tuning-loop run --config --out` and returns an error on any non-zero exit; `tuning-loop` exits 1 on a pause (`bin/tuning_loop.rs:11`, `tuning/command.rs:477` "paused; see run.json"); a revision is pushed (`step.rs:197`) and gaps are checked only on success. `species-conductor resume` resumes the conductor's own pauses. [checked]
- **Shape.** [inferred]
  - `tune` reads the tuning run's `run.json` after it exits: a `pause` becomes a conductor pause carrying the tuning pause id, reason and decision request, with a handoff written as for any other pause; a result is recorded as today.
  - `species-conductor resume --decision FILE` on a tuning pause passes the file to `tuning-loop run --resume FILE` in the same run directory, then continues the step.
  - A revision that ends (stopped, not paused) is pushed and its gaps are checked, as the design already says.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A tuning pause becomes a conductor pause with the tuning pause's id and request and a handoff; nothing is recorded as a failed step. [inferred]
- **R2:** `resume` on it runs `tuning-loop run --resume` in the same directory with the given decision. [inferred]
- **R3:** A tuning revision that ends is recorded and every gap in its result is checked (fixture transport). [inferred]
- **R4:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No change to the tuning loop's own pauses or decisions. [inferred]

## Decision Context

- Owner's standing rule of 2026-09-23: obvious friction fixes are the host's to spec. [user]

## Open Questions

- None.
