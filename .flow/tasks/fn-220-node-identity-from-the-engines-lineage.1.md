---
satisfies: [R1, R2, R3, R4]
---
# fn-220-node-identity-from-the-engines-lineage.1 Node identity from lineage

## Description
The spec's R1 to R4.

## Acceptance
- R1's test red then green.
- R2's properties tested.
- R3's hashes recorded.
- Gates green.

## Done summary
The conversion sets each engine node's identity to its phytomer's lineage key (`pipeline/space.rs`), so draws keyed by identity are distinct per node and stable across ages and fine cuts.

- R1: `every_engine_node_has_its_own_identity` red on the base (5,224 nodes, one identity), green on 77b725fb.
- R2: distinctness, the age walk (14 to 15) and cuts of 0, 16 and 1,024 tested.
- R3: HASHES.md: every engine preset changes in identity alone (masked skeleton equal); beech, oak and spruce draw byte-identical; the date palm's fronds now draw from their apex's lineage (same count and leaf box), its pins moved in 531fd9ab, and the before/after still is viewed: one tree, jitter only.
- R4: render half (161 passed), workspace without render (994 passed), npm test (143 passed), all on 69c63dec's code.

## Evidence
- Commits: 77b725fb, 455a88b8, 531fd9ab, 69c63dec
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: