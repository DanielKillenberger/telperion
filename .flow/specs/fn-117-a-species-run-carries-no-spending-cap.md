# A species run carries no spending cap

## Conversation Evidence

> user (2026-09-23): "Can we stop with the budget pls? We're still building it. I think once we have run some loops to completion we can figure out what a usual loop would cost and start flagging ones that are completely out of line. But let's keep going for now, note when we're inefficient and try and fix that for later runs. Avoid runaway quotas but drop the cap."
> user (2026-09-23), on a CLAUDE.md rule saying not to ask: "but can't we just remove the thing that was asking for it instead of writing a NOT rule"

## Goal & Context
<!-- scope: business -->

On 2026-09-23 the date palm's run stopped for a cap three times (the conductor's 2 M, then 6 M, then 10 M) and the tuning loop twice more on its own token, evaluation and round caps, each needing a decision that restated the new numbers. The owner wants the caps gone while the loop is built, with only the guards against a runaway kept, and every run's spend still recorded so a usual loop's cost can be learned later. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.**
  - Conductor: `Budget` (`conductor/mod.rs:115`) holds `max_tokens`, `attempt_max_tokens`, `max_dispatches`, `max_tuning_revisions` as required `u64`; config validation refuses zero (`mod.rs:143`); `dependency.rs:93-98` pauses when dispatches or the next attempt's tokens would pass them. [checked]
  - Tuning: `state::Budget` (`tuning/state.rs:108`) holds `max_evaluations`, `max_images`, `max_tokens`, `max_rounds` (required) and `max_visual_passes`; `engine.rs:648` pauses "round preflight cannot fit"; `experimental_pilot` must repeat every cap exactly (`continuation.rs:334`, "authority mismatch"), and raising one needs a `*_cap_extension` in a resume decision. [checked]
  - The runaway guards that do not depend on a cap: the continuation trio refuses "equivalent failed attempts without new evidence" (`continuation.rs:38`), and a track whose every strength was tried stops (`bundle/round.rs`, `Turn::AllTried`). [checked]
- **The design (host, 2026-09-23).** [host design]
  - Every cap above becomes optional. Absent means no cap; a config that sets one keeps today's behaviour, so tests and replays that rely on a cap still can.
  - The shipped and example configs drop their caps. The conductor's config needs no `budget` block at all.
  - The pilot authority and the cap-extension decisions compare only the caps a config sets; with none set there is nothing to restate on resume.
  - Spend is recorded exactly as today (tokens, dispatches, revisions, evaluations, images, visual passes) in the run record, the result and the report.
  - The runaway guards stay as they are, and one is added in code: a tuning run that finishes `k` consecutive rounds (default 5, config) with no adoption kept pauses as a runaway, with the rounds' spend in the pause.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A conductor config with no budget block runs; nothing pauses for tokens, dispatches or revisions, and the run's spend is still recorded and reported. [inferred]
- **R2:** A tuning config with no caps runs rounds without a "cannot fit" pause; the pilot authority and a resume carry no cap numbers. [inferred]
- **R3:** A config that sets a cap still pauses at it, as today. [inferred]
- **R4:** A tuning run with `k` consecutive rounds keeping nothing pauses as a runaway, naming the rounds and their spend; the continuation guard against repeated failure is unchanged. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- Capture budgets in CLAUDE.md are unchanged. [paraphrase]
- No change to readiness, gates or verdicts. [inferred]

## Decision Context

- The owner chose on 2026-09-23 to remove the mechanism rather than write a rule against asking; PR #72, which wrote that rule, was closed. [user]

## Open Questions

- None.
