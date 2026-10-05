---
satisfies: [R1]
---
# fn-197-tree-space-e-light-and-the-carbon.2 The rough layout grown with the tree, at neutral: its gap from the final lay, timed by stage

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The rough layout grown with the tree (src/grow/sketch.rs) on the final lay's stepper (geometry/lay.rs); light swept per cycle, read per living apex, used by none yet. Neutral byte-identical (80-year hashes, tests/light.rs); stage timings in the measures example; gap measured on oak and beech (STEPS-1-2.md). Gate: 1048 passed, 1 inherited failure from fn-195 (FRICTION.md).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: cc7d2606
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo run --release -p telperion-space --example measures -- hash 80 1,7
- PRs: