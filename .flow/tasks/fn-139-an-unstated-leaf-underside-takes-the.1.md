---
satisfies: [R1, R2, R3, R4]
---
# fn-139-an-unstated-leaf-underside-takes-the.1 Implement fn-139-an-unstated-leaf-underside-takes-the

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
An unstated `leaf_back_colour` now defaults to `leaf_front_colour`'s sourced level (ranges mapped onto the back's material fields), recorded as a default citing the front's source, instead of filing `requirements-unmet`; bark colour and every other colour trait, and a back with an unstated front too, are unaffected. R4 (host addition): the same default applies when the back's value was instead dropped by a claim resolution (`replace-source`/`drop-value`) and no replacement source was found on the rerun, as long as the front is still sourced - a dropped underside is an unstated one.

Tier: value (small, fully designed fix)

Change: `crates/telperion-jev/src/pipeline/stages/appearance.rs` - `run()` gathers every appearance trait's chosen sentence into a map first (order-independent), then a new `default_to_front` records `leaf_back_colour` at the front's level/ranges/reason when the back is unstated and the front is sourced (R1-R3); `drop_flagged` now runs in two passes too - first removing every flagged value, then, for a dropped `leaf_back_colour`, checking the front's post-drop sidecar state (`front_sourced_level_in`) and applying the same `default_to_front` when it is still sourced, falling through to the existing requirements-unmet path otherwise (R4). Unchanged for every other trait.

Tests: `crates/telperion-jev/tests/judged_select.rs` - `an_unstated_back_with_a_sourced_front_defaults_to_the_fronts_level` (R1), `both_faces_unstated_file_requirements_unmet_and_bark_never_defaults` (R2), the pre-existing `a_level_is_the_most_probable_one_never_a_zero_probability_average` updated to the new correct back-colour outcome in that same fixture, and `a_back_a_resolution_dropped_with_no_replacement_defaults_to_the_sourced_front` (R4: back first sourced wrongly off F1's frond sentence, then dropped via a constructed `claim-unsupported`/`drop-value` resolution matching that exact source+span, rerun leaves it defaulted to the still-sourced front). Every new test confirmed red against the pre-fix source first (R1/R2 via a scoped, tagged `git stash push/apply/drop`, R4 against the R1-only source before the drop_flagged change).

stage: impl-review - skipped(config: REVIEW_MODE=none)

Gate: `cargo test --profile ci --workspace --no-fail-fast` - 144 suites, 1052 passed, 0 failed, 21 ignored. Full log at `.flow/evidence/fn-139-an-unstated-leaf-underside-takes-the/raw/gate.log`.

No friction to report - both rounds were contained additions to `appearance.rs`'s existing default pattern, and the existing test fixtures (`judged` helpers, `verify::unsupported`'s payload shape) already covered everything R4 needed without new fixture machinery.
## Evidence
- Commits: 03e1cc9a16b962b0a867abf0822d12ba4e02a21c, <R4 commit sha>
- Tests: cargo test -p telperion-jev --profile ci --test judged_select, cargo test --profile ci --workspace --no-fail-fast
- PRs: