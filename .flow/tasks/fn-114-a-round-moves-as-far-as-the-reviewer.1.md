---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-114-a-round-moves-as-far-as-the-reviewer.1 Implement A round moves as far as the reviewer says the tree is off

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
How far a bundle round moves now depends on how big a gap the owner or the reviewer describes. The new `tuning::stride` module offers three classes: near, clearly_off and far_off. They multiply the configured `bundle_strengths` ladder by 1, 2 and 4. The owner's class comes from the config's `magnitudes` and asks nobody. Otherwise each track asks one `gap_magnitude` question per round before the draw, and code maps the class Jev picks to the number.

The ladder stays as configured when Jev answers no_match, when its confidence is below the cut, when there is no finding yet, or when the set has not qualified and no experimental authority is scoped. The route note gives the class, its source (owner, jev with its confidence, or default with the reason) and the multiplier used.

A raised bundle that is rolled back, or a dial that turns round after a raised bundle stood, caps the track one class lower until a bundle on it is adopted (R4). The new calibration kind is `gap_magnitude`, and its labelled set is `data/cases/gap_magnitude.json`: 16 cases covering every class and no_match in both splits, including the palm's fn-80 sentences (R2). It is trusted only once a frozen calibration qualifies on the run's model and dial table.

R3 replays the fn-80 palm rounds (tests/stride_replay.rs, fixture copied from the fn-80 evidence) using the bundle code on the date palm's own settings. Each round is replayed at its top rung. On that assumption, the owner's far_off brings the frond length from 3.5 m to 7.0 m in 5 rounds, against 18 on the recorded ladder. The replay cannot say whether wider bundles would have survived the closing review: the recorded run kept none of its five adoptions.

Tests: R1 in tuning_engine.rs (an_owner_class_scales_the_ladder_and_asks_nobody, the_reviewer_s_words_choose_the_next_round_s_stride_or_the_ladder_stands, which covers every fallback). R2 in stride.rs. R3 in stride_replay.rs. R4 in tuning_engine.rs (an_overshoot_rolled_back_steps_the_stride_down_rather_than_oscillating, a_direction_that_turns_round_after_a_raised_bundle_caps_the_stride). All four engine tests went red with the multiplier forced to 1. R5: the gate is green, 933 passed.

Reading for the host to confirm: nothing in the code says which owner priority belongs to which track, because every track's sheet judges all the tuning priorities. The implementation takes a track's owner priority to be the first tuning-routed approved priority in the owner's order. "A finding flips direction" is read as a dial in this round's bundle turning against the one the current tree's raised bundle moved. The labelled set's 0.5 cut is provisional until a live calibration run chooses it. No live Jev call was made.

Follow-up (not built): fn-80's result.json does not record move values or which attempt was adopted, which limited the replay (see FRICTION.md).

Tier: session (host-settled design; jev route fork=host)
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: a12406fb10f5b6ce38bb8fb0ad535406a10dbc31
- Tests: cargo test --profile ci --workspace --no-fail-fast, baseline: none (the gate runs once, at the end of a task, per CLAUDE.md)
- PRs: