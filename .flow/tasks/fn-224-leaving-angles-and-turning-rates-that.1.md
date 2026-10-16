---
satisfies: [R1, R2, R3, R4]
---
# fn-224-leaving-angles-and-turning-rates-that.1 Insertion and tropism spread

## Description
The spec's R1 to R4, following fn-222's internode spread as the pattern (form.internode_spread, the INTERNODE lineage key, fixed::internode_draw, contract.wgsl).

## Acceptance
- R1 to R4 as written; walk tests; both executors identical; the closed form's treatment shown; every passed species byte-identical; npm test and the gate green.

## Done summary
form.insertion_spread (radians, <= pi/2) and form.tropism_spread (factor, <= 1), drawn per axis under INSERTION and TROPISM, mean-preserving, neutral 0 with no draw. Both executors identical (probe selector 17); GPU within fn-125's screen, negative controls fail. Closed form reads E cos and E cos^2, exact in expectation (400 seeds within 1.2 se); predictor unchanged to the bit. 16/16 hashes identical; gate and npm test green.

## Evidence
- Commits: 1097ad81, 098978d4, ded9c340
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: