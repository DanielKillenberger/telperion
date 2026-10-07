---
satisfies: [R1, R2, R3, R4]
---
# fn-222-internode-lengths-that-vary-node-to-node.1 Internode length variation

## Description
The spec's R1 to R4.

## Acceptance
- R1 to R4 as written; a walk test; both executors identical; the predictor unchanged in expectation; every passed species byte-identical; gates green.

## Done summary
form.internode_spread: each internode scaled by 1 + s(2u - 1) from its node's keyed draw (lineage::INTERNODE), mean one, neutral 0, at most 1. Lay only; the reach and landing bounds take the longest internode. The GPU mirrors it (contract.wgsl internode_draw, probe selector 16, table slot 22). The walk stays within 30; the GPU settles within the screen at spread 0.6; predictor counts and expected lengths are bit-identical; all 16 species hashes match the base; the gate and npm test are green. The catalogue identity was re-pinned (Debug text only). See .flow/evidence/fn-222-*/RESULT.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8034829c, d12d1e9c, ecb31901
- Tests: cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci -p telperion-jev --test replay, npm test
- PRs: