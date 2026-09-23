---
satisfies: [R1, R2, R3, R4]
---
# fn-128-the-literature-step-retires-stale.1 Implement fn-128-the-literature-step-retires-stale

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
`quality`, `select` and `verify` now retire their own open decisions that a rerun did not file again. They mark each one resolved with the option `superseded`, and `hold_unmet` leaves a superseded decision resolved. An appearance trait is now scored one sentence at a time, and its value cites the sentence its level came from. `verify` asks an appearance value `appearance_supported` in place of `measurement_not_invention`, and files `claim-unsupported` on the value's pointer when the sentence does not describe the level.

Tier: session (host-settled design)
stage: impl-review - skipped(config: REVIEW_MODE=none)

- R1: `tests/mature_sizes.rs::a_rerun_supersedes_the_unmet_decisions_it_no_longer_files_and_select_fills_them`. Red on base, and red again with only the `hold_unmet` exemption removed.
- R2: `tests/appearance_sentences.rs::bark_colour_is_read_from_a1s_trunk_sentence_and_cites_it`, built from A1's live sentences.
- R3: `tests/appearance_sentences.rs::verify_holds_a_supported_appearance_value_and_files_a_claim_on_one_that_is_not`. The support question has 13 labelled cases, 4 of them held out and 5 negative, and the runner scores them (`tests/sets.rs`).
- R4: the gate passed 975 tests with 0 failures and 21 ignored, across 132 binaries.
- R5 (host design, added 2026-09-23): `verify` retires its own open decisions that a rerun did not file again. Test: `tests/appearance_sentences.rs::a_verify_rerun_supersedes_the_live_decisions_it_no_longer_files`, built from the four live `obligation-unmet` decisions and `claim-unsupported/A1`, `/F1`.

For the host:
- `retire_unfiled` is ported from fn-80, where only `gate` calls it. `decision.rs` is byte-identical to fn-80's copy, so it merges cleanly; the `SUPERSEDED` constant `hold_unmet` reads lives in `consume.rs`. `gate` is untouched on this branch.
- `quality` and `select` pass their header inputs to the retire guard, not the decisions' own input map. A `select` decision records only the manifest checksum, so the fn-80 guard would never retire `bark_colour` while the manifest stays the same.
- Interpretation: "extracted candidate sentences" is read as every sentence of the cached text that carries the trait's words. Each appearance trait now has a `terms` list in `species-requirements.json`. `extract.json` holds only sentences with a unit, so it would drop colour sentences that carry no number.

## Evidence
- Commits: abbb19e746f0e43d82866830b0023d485cc36ed7, 88f85dabaf9ad7720a3ad3311171c0c3b585a0e9, ad5919738a11e7e274b0969bda16b9873b974945, 9e858aea317239eec6e70faecdd651f4c7a17c9a
- Tests: cargo test --profile ci --workspace --no-fail-fast (132 binaries: 975 passed, 0 failed, 21 ignored; log .flow/evidence/fn-128-the-literature-step-retires-stale/raw/gate.log), baseline: none (spec defines no Quick commands; gate run once at the end per CLAUDE.md)
- PRs: