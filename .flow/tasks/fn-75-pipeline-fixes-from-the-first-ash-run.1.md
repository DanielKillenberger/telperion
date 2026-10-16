---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-75-pipeline-fixes-from-the-first-ash-run.1 Implement Pipeline fixes from the first ash run

## Description
Fix the six pipeline defects the first ash run exposed.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Discovery keys on the seed and the proposal binds to it (R1); a kinds table in `pipeline/consume.rs` names which stage consumes each option, refusing unconsumed options and replace-source without a url (R2); `AdmittedTable.block` and a two-sided coverage guard (R3); `pipeline/known.rs` seeds candidates from evidence manifests and specs' research URLs, plain-word queries (R4); ureq `native-certs` (R5, MoBot page still fails: server omits its intermediate); `pipeline/cost.rs` sums runs, Jev calls and credits per stage (R6). 140 jev tests pass.

## Evidence
- Commits: 78d6e3312ce6d0dc20d333098a2328d39b21a279
- Tests: cargo test --release -p telperion-jev, cargo clippy -p telperion-jev --all-targets -- -D warnings
- PRs:
