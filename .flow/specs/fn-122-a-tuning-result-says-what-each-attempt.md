# A tuning result says what each attempt moved and whether it stood

## Conversation Evidence

> fn-114 FRICTION.md, 2026-09-23: the replay "could not follow the dial along the path the run took" because `result.json` keeps neither the values a bundle moved nor which attempt a round adopted.
> user (2026-09-23): "yes pls fix the frictions if they're obvious."

## Goal & Context
<!-- scope: business -->

A tuning result is the record a later replay, a reversal or a cost review reads. Today its attempts carry a dial, the round, feasibility and scores, but not the moves (each dial's from and to) nor whether the round adopted the attempt and whether the adoption stood. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23.** A gap's attempts in `result.json` have keys `action_ledger`, `dial`, `feasible`, `reason`, `review`, `round`, `score_after`, `score_before_round`, `visual_outcome`; no move values and no adoption flag. `bundle::Move` holds `dial`, `direction`, `from`, `to`. [checked]
- **Shape.** Each attempt gains `moves` (the bundle's `Move`s, or the single dial's from and to), `adopted` (true when the round adopted it) and `stood` (false when the closing review rolled it back, with the rollback reason). Older results without the fields still read. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every attempt in a new result carries `moves`, `adopted` and `stood`, and a rolled-back one names its reason. [inferred]
- **R2:** A result written before this change still renders. [inferred]
- **R3:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Decision Context

- Owner's standing rule of 2026-09-23: obvious friction fixes are the host's to spec. [user]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
