---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-117-a-species-run-carries-no-spending-cap.1 Implement A species run carries no spending cap

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Every conductor cap (max_tokens, attempt_max_tokens, max_dispatches, max_tuning_revisions) and every tuning cap (max_evaluations, max_images, max_tokens, max_rounds, max_visual_passes) is now optional: an absent cap is no cap, and a set cap still pauses at it. The conductor config needs no budget block. Resume and the experimental_pilot authority compare only the caps a config sets, and a cap the config leaves out is dropped from the run (new `tuning/caps.rs`, moved out of `command.rs`). Spend is recorded as before. A new runaway guard (`tuning/runaway.rs`) pauses a run after `runaway_rounds` rounds in a row with no kept adoption (config, default 5). The pause names the rounds and their spend, and the owner's resume of that pause starts the count again. Both docs are updated.

Tests: R1 `a_run_without_a_budget_block_runs_where_a_set_cap_pauses_and_records_its_spend` (tests/conductor.rs) and `only_a_set_cap_is_a_hard_limit` (conductor/dependency.rs). R2 `an_uncapped_budget_counts_every_spend_and_refuses_none` (tests/tuning.rs), `only_the_caps_a_config_sets_are_compared_on_resume` and `a_pilot_authority_restates_only_the_caps_the_run_carries` (tuning/caps.rs). R3 is the existing capped tests unchanged apart from `Some(..)`, plus the Some(0) row of the R1 test. R4 `an_uncapped_run_pauses_as_a_runaway_after_rounds_that_keep_nothing` (tests/tuning_engine.rs), which is red without the guard. R5: the gate is green.

Decisions the host should check. (1) The continuation contract asks for `next_tokens` only when a token cap is set. With no attempt bound and no usage yet, the conductor records its estimate as unknown instead of inventing a number. (2) A fresh tuning run opens its visual counter at 0 when the config names none; otherwise an uncapped config would stop at "visual usage requires reconciliation". (3) Resuming an old capped run with an uncapped config drops its caps, per "compare only the caps a config sets". A config that sets a cap the run lacks is still refused. Follow-up, not built: a shared budget builder in tests/fixture (see FRICTION.md).

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: session (host-settled design)
## Evidence
- Commits: e0e157f3b2c73e7ca3996592522dec5408862879
- Tests: cargo test --profile ci --workspace --no-fail-fast (658 passed, 0 failed; baseline: none run pre-edit, gate run once at the end per CLAUDE.md), cargo test -p telperion-jev --profile ci --test tuning_engine runaway (red with the guard disabled, green with it)
- PRs: