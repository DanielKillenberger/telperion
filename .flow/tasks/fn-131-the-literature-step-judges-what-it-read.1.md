---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8, R9, R10]
---
# fn-131-the-literature-step-judges-what-it-read.1 Implement fn-131-the-literature-step-judges-what-it-read

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The literature step now judges what it read and files every gap. The screen has organ-size and cultivar classes. Each field reads only the rows its kinds and whole-word terms allow, and quality counts exactly the rows select reads. Select picks from spans keyed to their row and source, under a calibrated floor, with one number and unit grammar. A level is the most probable choice. Verify acts per pointer. A superseded decision stays retired, and a stage key carries the build identity.

Tier: session (host-settled design)

- R1: `tests/judged_select.rs` `leaflet_and_frond_sentences_reach_select_and_the_cultivar_fronds_count` (P4/P5/A1 leaflet and frond sentences in select's per-field documents, M1's cultivar fronds counted, "6–10m" filled as [6, 10]); `quantity.rs` and `pick.rs` unit tests.
- R2: `frond_length_is_credited_to_f1s_frond_sentence`; `a_pick_below_the_floor_is_not_filled_and_the_required_field_is_filed`. F1 states only "can grow to 20 feet", so the range is [6.096, 6.096]; I read "as a range" as the metric's range, credited to F1's sentence.
- R3: `a_level_is_the_most_probable_one_never_a_zero_probability_average` (the live palm probability maps), `a_sufficiency_level_is_the_most_probable_one`, and `tests/sets.rs` `chosen_level_is_the_most_probable_or_the_no_match`.
- R4: `tests/judged_verify.rs` `claims_are_per_pointer_and_the_appearance_claim_can_fail` and `a_dropped_value_leaves_the_packet_and_refiles_its_requirement`. Claim kinds are consumed by select, and `replace-source` and `drop-value` are routine in `conductor-policy.json`.
- R5: `a_superseded_decision_ignores_a_stale_resolution`.
- R6: `a_size_reached_at_a_stated_age_is_a_point` (ages [15, 20]); an `unstated` condition weighs 0.5.
- R7: `no_mature_size_fails_a_mature_field`, plus the floor test above for an unfilled required field.
- R8: `a_code_change_expires_a_stages_key` (BUILD_ID equals the digest of `src/` and `data/`).
- R9: `floors.rs` `each_floor_is_calibrated_on_its_labelled_cases`; `tests/screen.rs` `every_screen_class_has_a_labelled_case_and_a_no_match_remains`. The case sets are `data/cases/selection_floor.json` and `level_floor.json`, and there are 9 new screen cases.
- R10: the gate `cargo test --profile ci --workspace --no-fail-fast`: 137 result lines, 1018 passed, 0 failed, 21 ignored (`raw/gate.log`).

Open for the host:
- The labelled live picks do not separate right from wrong. The selection floor calibrates to 0.34, which catches none of the wrong picks (0.35, 0.66, 0.72), and the level floor to 0.56.
- The sufficiency levels take no floor, because no labelled set of live sufficiency answers exists.
- The labelled mature case `palm-frond-length-m1-cultivars` still expects `wrong_taxon`, while cultivar sizes now count toward the species.
- Height's terms gained "reaches", "reaching" and "grows to", so a "reaches N cm in diameter" sentence also counts for height.
- Existing fixtures now answer level scores with a probability map, as live Jev does, and span answers name a keyed candidate. The fn-127/128 requirements tests now expect select's `requirements-unmet` on unfilled fields.
- Friction is in `.flow/evidence/fn-131-the-literature-step-judges-what-it-read/FRICTION.md`: the thin labelled sets, and the gate at 500 s against the 600 s tool cap.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: adf16058e4c80ba2d7391290657c063017b294d8, 4379bfcc3d01156ddf91096ed34f84dea81d506d
- Tests: baseline: none (the spec defines no Quick commands; CLAUDE.md runs the gate once at the end), cargo test --profile ci -p telperion-jev --no-fail-fast (focused, rc=0), cargo test --profile ci --workspace --no-fail-fast (rc=0; 137 result lines, 1018 passed, 0 failed, 21 ignored)
- PRs: