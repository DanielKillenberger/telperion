---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-116-a-trait-the-generator-cannot-draw-yet.1 Implement A trait the generator cannot draw yet never vetoes an adoption

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The tuning config now takes `unexpressed`: a list of `{trait, spec}` entries. `worsened` records a listed coverage trait that went backwards as the route note `trait <id> went from <a> to <b>; not a veto: the generator cannot draw it until <spec> lands`, where before it was a rollback reason. Required cells, `ready()` and the core-coverage gate are unchanged. `Config::verify` refuses an entry whose spec is blank, whose trait is missing from the pinned reference-first inventory, or that comes with no inventory at all.

Tests by R-ID: R1 `veto::a_listed_trait_going_backwards_is_recorded_and_never_a_reason` (unknown→fail and pass→fail) and `tuning_engine::a_trait_the_generator_cannot_draw_yet_never_rolls_an_adoption_back`, which checks that the adoption is kept and carries the note and was confirmed red without the wiring; R2 `veto::an_unlisted_trait_and_a_required_cell_still_roll_back` and the unlisted half of the engine test; R3 `reference_first::coverage_unknown_and_positive_finish_are_enforced`, where a listed failing core trait is kept by the veto and still not ready; R4 `tuning_command::an_unexpressed_trait_must_be_in_the_inventory_and_name_its_spec`; R5 the workspace gate, green.

Follow-up: none built. A config with no reference-first inventory that lists an unexpressed trait is refused too, because its trait cannot be checked.

Tier: session (host-settled design)
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: c1002d603c7b17e5aefed02339e883a97453d847
- Tests: cargo test --profile ci --workspace --no-fail-fast (suite_rc=0, 937 passed, 0 failed, 125 suites), baseline: none (spec defines no Quick commands; project gate runs once at task end)
- PRs: