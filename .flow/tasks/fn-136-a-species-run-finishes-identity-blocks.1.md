---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-136-a-species-run-finishes-identity-blocks.1 Implement fn-136-a-species-run-finishes-identity-blocks

## Description
Give a species run a finish line: identity capabilities block, improvements become known gaps.

## Acceptance
Spec R1-R7.

## Done summary
Assessment `classes` read by `capability_class.rs`; the gate blocks only on identity or unclassified gaps and records `capability.known_gaps`; reviewer findings carry `trait_id` (protocol v2) and known-gap blockers leave `ready()`; converged revisions (incl. bootstrap as `unqualified reviewer`) go to the packet with known gaps listed. Tested on the palm's recorded revision-3 state. Gate green (1038 passed).

## Evidence
- Commits: 88c77fe5a8d450063cc58e14a8f14c74d7938e5f, a3a0d97e6cca11742ca78c62bca0f69da380c350
- Tests: cargo test --profile ci --workspace --no-fail-fast (1038 passed); python3 scripts/test-reference-first-codex.py
- PRs:
