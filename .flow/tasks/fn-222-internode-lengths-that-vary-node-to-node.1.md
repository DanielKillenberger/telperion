---
satisfies: [R1, R2, R3, R4]
---
# fn-222-internode-lengths-that-vary-node-to-node.1 Internode length variation

## Description
The spec's R1 to R4.

## Acceptance
- R1 to R4 as written; a walk test; both executors identical; the predictor unchanged in expectation; every passed species byte-identical; gates green.

## Done summary
`form.internode_spread`: each internode scaled by 1 + s(2u - 1) from its node's keyed draw (lineage::INTERNODE), mean one, neutral 0, at most 1; lay only, reach and landing bounds take the longest internode. GPU mirrors it (probe selector 16, slot 22). Walk within 30; GPU within the screen at spread 0.6; predictor counts and expected lengths bit-identical; all 16 species hashes match the base; gate and npm test green. Catalogue identity re-pinned (Debug text only).

## Evidence
- Commits: 8034829c, d12d1e9c, ecb31901
- Tests: cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci -p telperion-jev --test replay, npm test
- PRs: