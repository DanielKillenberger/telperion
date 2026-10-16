---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-199-tree-space-relays-change-the-tree-by.1 Relays change the tree by degree

## Description
The spec's R1 to R6 on fn-198's work (taken over by the host, 2026-10-07). Each walk test red on the base first.

## Acceptance
- R1-R4 walk tests red then green.
- Crate tests and the workspace gate green.
- The beech re-rendered at its five seeds and viewed; change recorded.

## Done summary
R1 under decision (b) with A1, A3, B2 and C1; R2 by fn-206; R3 by Rule 1 (continuation = relay at blend 0) and Rule 2' (blend back by the last unit's extent); R4 girth and load over the span. Every rule mirrored on the GPU, census over seeds 1 to 50 clean, rows re-pinned once (beech, oak), F4's tip merged, gate green (render 163, rest 994). R6 beech sheets viewed.

## Evidence
- Commits: 56e431d8, 795a3555, eacb5181, 3afbe0fd, 704ad2d3, 442ab586, 0e7ddd65, a5445ba8, 43c30653, b2476a17, e314a066, 78cc3fbe, d3bb6617
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, cargo test --profile ci -p telperion-space --test relays census -- --ignored
- PRs:
