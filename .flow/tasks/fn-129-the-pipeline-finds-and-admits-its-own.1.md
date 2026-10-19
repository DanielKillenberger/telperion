---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-129-the-pipeline-finds-and-admits-its-own.1 Implement fn-129-the-pipeline-finds-and-admits-its-own

## Description
Let the pipeline classify rights, admit sources-only drafts and search again for unmet requirements.

## Acceptance
Spec R1-R6.

## Done summary
Discover admits sources-only drafts whose sources are ranked for their field and classed open or public cite-only (`rights_class` recorded, `by: pipeline`); anything else goes to the owner with reasons. `search-again` gives unmet fields and appearance traits two rounds before the owner. 16 labelled rights cases. Tests in `tests/admission.rs`, `tests/owner_first.rs`. Gate green (994 passed).

## Evidence
- Commits: 461a8c2ca4f29b4229e91bcc692591e740f461d0, a9baf10db6201fe1740596de14853d2cfb59bcc0, 93169d74cdb939b51250089ff77685ec7ce856b4
- Tests: cargo test --profile ci --workspace --no-fail-fast (994 passed); cargo test -p telperion-jev --lib --test admission --test owner_first --test requirements --test sets
- PRs:
