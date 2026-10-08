---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-199-tree-space-relays-change-the-tree-by.1 Relays change the tree by degree

## Description
The spec's R1 to R6, on fn-198's work (taken over by the host, 2026-10-07: no other session works it). Each walk test red on the base first.

## Acceptance
- R1-R4 walk tests red then green.
- Crate tests and the workspace gate green.
- The beech re-rendered at its five seeds and viewed; change recorded.

## Done summary
Relays change the tree by degree:
- **R1:** failure under decision (b). The closed form takes the lived share at one point (A1) with relays not nesting within a cycle (A3); the shares below a tick are kept (B2); the lived part takes the unit's abortion draw (C1).
- **R2:** fn-206.
- **R3:** rule 1 (a continuation is a relay at blend 0) and rule 2' (a relay stands on the line's last unit, blended back by its extent).
- **R4:** girth and load over the span.

Every rule is mirrored on the GPU. The census over seeds 1 to 50 is clean, the rows are re-pinned once (beech, oak), F4's tip is merged, and the gate is green: render 163, the rest 994. R6's beech sheets were viewed.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 56e431d8, 795a3555, eacb5181, 3afbe0fd, 704ad2d3, 442ab586, 0e7ddd65, a5445ba8, 43c30653, b2476a17, e314a066, 78cc3fbe, d3bb6617
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, cargo test --profile ci -p telperion-space --test relays census -- --ignored
- PRs: