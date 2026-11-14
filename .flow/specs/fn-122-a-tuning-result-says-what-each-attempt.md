# A tuning result says what each attempt moved and whether it stood

## Goal

A tuning result is the record a replay, reversal or cost review reads, but its attempts carried no moves (each dial's from and to) and no adoption outcome, so fn-114's replay "could not follow the dial along the path the run took" (fn-114 FRICTION, 2026-09-23). Owner (2026-09-23): "yes pls fix the frictions if they're obvious."

## Decisions

- Each attempt gains `moves`, `adopted`, `stood` (null when never adopted) and `rolled_back` reasons; older results read through serde defaults.

## Acceptance Criteria

- **R1:** Every new attempt carries `moves`, `adopted`, `stood`, and a rolled-back one names its reason — met.
- **R2:** A result written before the change still renders — met.
- **R3:** Gate green — met only with a four-line edit to a test helper in `conductor/gapcheck.rs`, a path forbidden to the worker; the host applied it.

## Result

Landed in `6a75db61` (972 passed with the helper edit); carried by the fn-149 rewrite (#121, `c2ac430b`). `docs/tuning-loop.md` describes the fields.

## Open

None.
