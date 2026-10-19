---
satisfies: [R2, R3]
---
# fn-216-tree-space-flowers-on-the-engine-path.2 Flowering organs, option B

## Description
Build FLOWERS-DESIGN.md option B with the host's decisions 1-5: flowering share per PA, blossoms and racemes keyed by the phytomer key, a second foliage pass, season and petal count by degree.

## Acceptance
- Flowers at both executors where leaves are; nothing at zero.
- Oak catkins and beech blossom on stills beside photographs, viewed.
- A walk across flowering and petal count with no pop.
- npm test and the gate green.

## Done summary
Flowering share per PA (neutral 0); blooms drawn per node lineage (fn-220) with fractional last petal and a season; second foliage draw with its own colour block. One CPU step serves both executors (host decision (a), 2026-10-08); GPU bloom pass named in fn-125. Nothing at zero (`a_tree_that_flowers_nowhere_has_no_flowers`, catalogue identity). Stills accepted as an organ test (host, 2026-10-08); look follow-ups in fn-217. Walks tested with no pop. Gates: render 161, workspace without render 994, npm test 143 passed.

## Evidence
- Commits: e447f782, 458018b8, 455a88b8, 531fd9ab, a9fc8a96, a8e6a6c9
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs:
