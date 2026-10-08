---
satisfies: [R1, R2, R3, R4]
---
# fn-224-leaving-angles-and-turning-rates-that.1 Insertion and tropism spread

## Description
The spec's R1 to R4, following fn-222's internode spread as the pattern (form.internode_spread, the INTERNODE lineage key, fixed::internode_draw, contract.wgsl).

## Acceptance
- R1 to R4 as written; walk tests; both executors identical; the closed form's treatment shown; every passed species byte-identical; npm test and the gate green.

## Done summary
form.insertion_spread (radians, at most pi/2) and form.tropism_spread (factor, at most 1): each axis's insertion offset and tropism factor drawn under its lineage (INSERTION, TROPISM), mean-preserving, neutral 0 with no draw. Both executors draw them identically (probe selector 17); the GPU walk lays them and settles within fn-125's screen, with both negative controls failing. The closed form reads E cos and E cos^2 over the draw, exact in expectation (400-seed check within 1.2 se; the constant reading is 10-21 se off); predictor counts, log lengths and reach bounds unchanged to the bit. All 16 clothed hashes identical; gate and npm test green. Evidence: .flow/evidence/fn-224-leaving-angles-and-turning-rates-that/RESULT.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 1097ad81, 098978d4, ded9c340
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: