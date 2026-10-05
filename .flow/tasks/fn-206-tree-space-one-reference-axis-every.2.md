---
satisfies: [R3]
---
# fn-206-tree-space-one-reference-axis-every.2 Species walk strips: beech to spruce to oak to palm

## Description
Bare still strips walking every setting between the species at a fixed seed, one tree per process, viewed for a visible pop.

## Acceptance
- R3: strips rendered and viewed by worker and host; no visible pop.

## Done summary
Strips on the final fn-206 code, seed 1, one tree per process under the GPU lock, every frame viewed, no pops (raw/walk6/strips.png): beech to spruce at age 40 (host decision 16; passed by the host), spruce to oak at age 60, oak to palm at age 60 with decision 15's accepted early crown steepness recorded in RESULT.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: fd7d9ffe0519ef6473dc67c373477c4f9d684e5c
- Tests:
- PRs: