---
satisfies: [R1, R2]
---
# fn-140-generate-writes-three-holdout-seeds.1 Implement fn-140-generate-writes-three-holdout-seeds

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
generate's write_packet now draws three holdout seeds deterministically from the species id (distinct from the fixed seeds and from each other, stable across reruns) and writes them into species.json's holdout_seeds and specimens.json's holdout cases, so the seeds gate resolves with 3 fixed and 3 holdout instead of failing every species. write_packet moved from taking &Context to &Paths (it only ever used ctx.paths), enabling a focused unit test; the existing packet-shape integration test was updated for the six total cases. Red-then-green: the new unit test was run against the unfixed body first (0 holdout seeds, failed for the intended reason), then against the fix (green).

Gate: cargo test --profile ci --workspace --no-fail-fast, gate_rc=0, 1054 passed / 0 failed summed across every `test result:` line in .flow/evidence/fn-140-generate-writes-three-holdout-seeds/raw/gate.log. No failing tests to name.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 1bd7dd12426453955adc7cf97a4f1187cb83b4ff
- Tests: cargo test -p telperion-jev --lib pipeline::stages::generate::packet::tests, cargo test -p telperion-jev --test stages_downstream, cargo test -p telperion-jev --lib, cargo fmt -p telperion-jev -- --check, cargo clippy -p telperion-jev --lib --tests, cargo test --profile ci --workspace --no-fail-fast (gate_rc=0, 1054 passed, 0 failed, see .flow/evidence/fn-140-generate-writes-three-holdout-seeds/raw/gate.log)
- PRs: