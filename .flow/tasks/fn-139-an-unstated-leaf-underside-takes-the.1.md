---
satisfies: [R1, R2, R3]
---
# fn-139-an-unstated-leaf-underside-takes-the.1 Implement fn-139-an-unstated-leaf-underside-takes-the

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
An unstated `leaf_back_colour` now defaults to `leaf_front_colour`'s sourced level (ranges mapped onto the back's material fields), recorded as a default citing the front's source, instead of filing `requirements-unmet`; bark colour and every other colour trait, and a back with an unstated front too, are unaffected.

Tier: value (small, fully designed fix)

Change: `crates/telperion-jev/src/pipeline/stages/appearance.rs` - `run()` now gathers every appearance trait's chosen sentence into a map first (order-independent), then a new `default_to_front` records `leaf_back_colour` at the front's level/ranges/reason when the back is unstated and the front is sourced; falls through to the existing `requirements-unmet` path otherwise, unchanged for every other trait.

Tests: `crates/telperion-jev/tests/judged_select.rs` - `an_unstated_back_with_a_sourced_front_defaults_to_the_fronts_level` (R1, red-first confirmed against the pre-fix source via a scoped `git stash`, then green after the fix), `both_faces_unstated_file_requirements_unmet_and_bark_never_defaults` (R2), and the pre-existing `a_level_is_the_most_probable_one_never_a_zero_probability_average` updated to the new correct back-colour outcome in that same fixture (front sourced to `grey_green`, so the back now defaults to `grey_green` instead of staying `unstated`).

stage: impl-review - skipped(config: REVIEW_MODE=none)

Gate: `cargo test --profile ci --workspace --no-fail-fast` - 144 suites, 1048 passed, 0 failed, 21 ignored. Full log at `.flow/evidence/fn-139-an-unstated-leaf-underside-takes-the/raw/gate.log`.

No friction to report - the fix was a contained addition to `appearance.rs`'s existing zero-width-default pattern, and the palm's own live fixture (front `grey_green` from F1, back unstated) was already the exact scenario an existing test exercised, so the red-first check and both new tests reused it directly.
## Evidence
- Commits: 03e1cc9a16b962b0a867abf0822d12ba4e02a21c
- Tests: cargo test -p telperion-jev --profile ci --test judged_select, cargo test --profile ci --workspace --no-fail-fast
- PRs: