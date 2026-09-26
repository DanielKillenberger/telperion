---
satisfies: [R1, R2, R3, R4]
---
# fn-119-tuning-aims-at-every-trait-the.1 Implement Tuning aims at every trait the generator can draw

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The priority pause now proposes an approval made of the owner's priorities, then every expressible core and secondary inventory trait (`tuning/objectives.rs`). Variation and unexpressed traits are recorded in `left_out`. A gap may name its track; each track's sheets carry only its own objectives, and its stride is judged on its own lead objective. A track with no objective is skipped with a route note. The caps rise to 32 objectives per approval and 16 per sheet or progress request. The host's decisions on the escalation are recorded in the spec.

R1: `tests/objectives.rs::the_palm_s_proposed_approval_is_the_owner_s_priorities_then_every_drawable_trait` (recorded palm inventory) and `tests/tuning_engine.rs::the_priority_pause_proposes_every_drawable_inventory_trait`. R2: `tests/objectives.rs::the_palm_s_materials_track_leads_with_its_own_objective` (recorded palm state) and `tests/tuning_engine.rs::each_track_s_stride_is_judged_on_its_own_lead_objective`, which went red on the old run-wide lead. R3: `tests/tuning_engine.rs::a_track_with_no_objective_is_skipped_and_an_undeclared_track_is_refused`. R4: the gate passed with rc 0; 127 `test result:` lines sum to 955 passed, 0 failed, 21 ignored.

For the host: `frond-colour-range` is a `variation` trait in the palm inventory, so R2's test assigns only the two proposed traits to materials (see FRICTION.md). Approved inventory traits become `owner-priority:` required cells through the existing `requirements()`, so the reviewer grades them and they count toward machine readiness. The readiness rule itself is unchanged.

Tier: session (host-settled design)
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 9819518815459f9d916001f11e24e28cd6853bb3, 0c4d8c864107609a77fd29f1baf4bf587d3fcf4e, 872fd73bf84871e40575f29cc7eff3036d6a962d
- Tests: baseline: none run pre-edit (the task opened with an investigation-only escalation); focused cargo test --profile ci -p telperion-jev green before the gate, cargo test --profile ci --workspace --no-fail-fast (rc 0; 127 test-result lines summed: 955 passed, 0 failed, 21 ignored)
- PRs: