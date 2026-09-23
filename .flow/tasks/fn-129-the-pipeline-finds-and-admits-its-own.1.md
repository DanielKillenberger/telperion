---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-129-the-pipeline-finds-and-admits-its-own.1 Implement fn-129-the-pipeline-finds-and-admits-its-own

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The pipeline now admits its own sources. Discover admits a draft that only adds sources, when the ranking chose each one for its field and Jev classes its licence statements `open-licence` or `public-cite-only`; it writes the manifest with a `rights_class` on each new source and resolves the proposal `by: pipeline`. Any other draft goes to the owner with every reason listed. A `requirements-unmet` field gets two `search-again` rounds, each with a query aimed at the field's gap; after them the owner gets the decision with the sources tried.

- R1: `pipeline/admission.rs` (the verdict), `stages/discover.rs` (the admission); tests `a_draft_whose_new_source_is_open_is_admitted_by_the_pipeline_with_its_class` and `a_draft_with_a_restricted_or_unclassifiable_source_goes_to_the_owner` (tests/admission.rs), plus the unit test `a_restricted_or_unclassified_or_unranked_source_sends_the_draft_to_the_owner`.
- R2: `admission::structural`. Test `a_draft_that_changes_a_field_a_bar_an_appearance_trait_or_the_schema_goes_to_the_owner` covers a field's condition, a bar, an appearance trait, the schema version and an edited admitted source.
- R3: `pipeline/search.rs`, the `search-again` command, and `Next::SearchAgain` in the conductor, which comes before the owner pause and waits while a manifest proposal is open. Tests `an_unmet_requirement_searched_again_admits_a_passing_source_and_resolves_it` and `two_empty_rounds_hand_the_owner_the_decision_with_the_sources_tried` (tests/admission.rs), and `an_unmet_requirement_with_a_round_left_is_searched_again_before_the_owner_has_it` (tests/owner_first.rs). The owner handoff's `sources_tried` is asserted in the existing owner-first test.
- R4: `data/questions/rights.json` (three classes plus `none`) and 16 labelled cases in `data/cases/rights.json`: the palm's F1, A1, M1 and P4 to P8, the ash's J1, O1 and E1, and 5 that reject. They were built from real fetches: cached pages, plus 8 Firecrawl scrapes on 2026-09-23 kept in `raw/rights-fetches/`. The cases are scored in `jev cases` at the 0.9 bar. Test `the_rights_set_labels_the_palms_sources_and_offers_a_no_match_answer`; the runner test now covers "rights class".
- R5: gate `cargo test --profile ci --workspace --no-fail-fast` exit 0. The 134 `test result:` lines sum to 993 passed, 0 failed and 21 ignored.

Decisions taken inside the design:
- An appearance trait that `select` found unstated goes to the owner at once. Its source must be named on the trait, and R2 makes a trait change the owner's.
- A draft that lifts a v1 manifest to v2 goes to the owner, because it changes `schema_version`, which binds the bars.
- A round that an adapter or Jev error ended still counts, so the search cannot loop.
- The PMC open-access service now answers 404, so a PMC article's record comes from Europe PMC's REST search, whose `license` field carries the licence.
- `swap` accepts `search-again` as a runbook command.

Friction: 3 entries in FRICTION.md.

Follow-up, not built: letting the pipeline add a source to an appearance trait's list would need the owner's word on R2.

baseline: none (the project runs its gate once, at the end of a task)

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: session (host-settled design)
## Evidence
- Commits: 461a8c2ca4f29b4229e91bcc692591e740f461d0
- Tests: cargo test --profile ci --workspace --no-fail-fast (134 result lines: 993 passed, 0 failed, 21 ignored; log .flow/evidence/fn-129-the-pipeline-finds-and-admits-its-own/raw/gate.log), cargo test -p telperion-jev --lib --test admission --test owner_first --test sets (focused, during the loop), cargo clippy -p telperion-jev --all-targets (no warning in the changed files), baseline: none (the project gate runs once, at the end of a task)
- PRs: