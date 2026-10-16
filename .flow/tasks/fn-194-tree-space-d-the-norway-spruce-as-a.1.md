---
satisfies: [R1, R2, R4]
---
# fn-194-tree-space-d-the-norway-spruce-as-a.1 The Norway spruce as a point: values, stills, walk and the beech unchanged

## Description
The Norway spruce (Massart) as values only on the engine phase C left (`crates/telperion-space/src/spruce.rs`, sources in SOURCES.md), drawn by `examples/space_spruce.rs`; five-seed sheet at 80 years and 10/20/40 years beside the references. Engine gaps go to the host.

## Acceptance
- R1: values from MODEL-SPRUCE.md and sources, several ages, seeds 1 and 7, beside references and today's spruce.
- R2: the gate.
- R4: the beech unchanged in look.
- R3 is task 2.

## Done summary
Passed R2 on the owner's verdict (2026-10-05) with round 10's values: "spruce looks good but not as lush and big as the reference. but definitely acceptable. Still much to be improved but i think it's structurally sound." Engine work split into fn-200, fn-201, fn-202, fn-203, fn-205. R4: beech unchanged (fn-196's branch, 38c2de33). Known gaps: curtain volume (about 210M wood triangles, 16M needles per tree), values-limited draperies, sparse 10-year sapling. Gate 1044 passed. Codex SHIP with two P2s: walks refuse a missing PA index by name (fixed, 838b6e50); no dbh in spruce stills (recorded).

## Evidence
- Commits: e6f37885, 838b6e50, 1c3f4f67, 8daeef40
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs: #149
