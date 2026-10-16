---
satisfies: [R1, R2, R3, R4]
---
# fn-128-the-literature-step-retires-stale.1 Implement fn-128-the-literature-step-retires-stale

## Description
Retire stale literature decisions on rerun and cite the sentence each appearance value came from.

## Acceptance
Spec R1-R5.

## Done summary
`quality`, `select`, `verify` supersede decisions a rerun no longer files; appearance traits scored per sentence with `terms` and cite it; `verify` asks `appearance_supported`. Tests in `mature_sizes.rs` and `appearance_sentences.rs` from live palm data; 13-case support question. Gate green (975 passed).

## Evidence
- Commits: abbb19e746f0e43d82866830b0023d485cc36ed7, 88f85dabaf9ad7720a3ad3311171c0c3b585a0e9, ad5919738a11e7e274b0969bda16b9873b974945, 9e858aea317239eec6e70faecdd651f4c7a17c9a
- Tests: cargo test --profile ci --workspace --no-fail-fast (975 passed)
- PRs:
