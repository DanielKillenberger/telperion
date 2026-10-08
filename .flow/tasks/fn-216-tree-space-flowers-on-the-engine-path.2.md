---
satisfies: [R2, R3]
---
# fn-216-tree-space-flowers-on-the-engine-path.2 Flowering organs, option B

## Description
FLOWERS-DESIGN.md option B with the host's decisions 1-5 (the design's 'Host decisions' section): the flowering share per PA, the expansion drawing blossoms and pendant racemes keyed by the phytomer key, the second foliage pass with organ colours, the season by degree, the petal count with its fractional last petal. R2's proof on the oak's catkins and a blossom test on the beech, stills beside photographs, viewed. Every passed species byte-identical at zero flowering.

## Acceptance
- Flowers on the engine path at both executors where leaves are; nothing at zero.
- The oak's catkins and the beech blossom test on stills, viewed, beside photographs.
- A walk across flowering and petal count with no pop.
- npm test and the gate green.

## Done summary
Flowers on the engine path, option B: a flowering share per physiological age (neutral 0); the conversion keeps each drawn node of a flowering age as a bloom; the expansion draws each inflorescence keyed by the node's lineage (fn-220), with a petal count whose fraction sizes the last petal and a season that scales florets in and out; a second foliage draw through the leaf shader with its own colour block.

- Both executors: one CPU step, `Expansion::organ` (host decision (a), 2026-10-08); the GPU bloom pass and flower shadows are named in fn-125.
- Nothing at zero: a tree that flowers nowhere has no blooms and keeps its bytes (`a_tree_that_flowers_nowhere_has_no_flowers`, catalogue identity tests).
- Stills: the oak's catkins and the beech blossom beside photographs, viewed; accepted as an organ test (host, 2026-10-08). Bloom colours are linear albedos on the leaf scale. Follow-ups for the blossom's look are in fn-217 (STILLS.md).
- No pop: share, floret-count and petal-count walks tested; a six-frame walk still.
- Gates: render half 161 passed, the workspace without render 994 passed, npm test 143 passed.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e447f782, 458018b8, 455a88b8, 531fd9ab, a9fc8a96, a8e6a6c9
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: