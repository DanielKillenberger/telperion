---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-38-multi-stem-trees.1 Implement Multi-stem trees

## Description
Add `skeleton.habit.stems`, `stem_divergence` and `stem_lean`: stems are order-zero axes born at the root, each a trunk run, sharing the base by the pipe model (R1-R5).

## Acceptance
- [x] R1 three railed rows; presets byte-identical at one stem; birch re-pinned once
- [x] R2 stems spread and leaned, refused only when degenerate, each a trunk run
- [x] R3 pipe-model base, DBH of the largest stem with count, eligibility off the largest stem
- [x] R4 birch on two stems; round-6 pairs; verdict in fn-34
- [x] R5 tests

## Done summary
A table can declare a clump of stems; the birch stands on two. Round 6: S-BARE reads as two stems from one flared foot; S-WHOLE hides the base under the curtain. The symmetric V (both stems lean alike) was raised to the owner; a per-stem lean would be a new spec. The owner accepted the silver birch in fn-34; the beech moved to fn-62.

## Evidence
- Commits: 40e42bb5
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, ...
- PRs:
