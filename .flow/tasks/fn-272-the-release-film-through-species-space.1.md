---
satisfies: [R1, R1a, R1b, R2, R3]
---
# fn-272-the-release-film-through-species-space.1 The path, the map and the film tool

## Description
The path as a value file (R1, R1a's nine segments); the species-space map computed and tested (R1b); the headless film command on the device through the one pipeline and the shipped renderer, with the overlay composited flat (R2); fidelity in motion checked per segment (R3). Growth segments settle one GrowthState at fractional ages (fn-270); walks follow fn-251.

## Acceptance
R1-R3: a first cut rendered, per-segment cost reported, the map's projection tests pass, no pop in growth segments and fn-251's oracle on every walk frame, stills viewed.

## Done summary
Cuts 1 to 6 rendered through the film example (path, camera, map, overlay, cost, oracle). The cost and the oracle for each cut are in FILM.md. The owner's verdict on cut 6 is "looked good" (2026-10-23). The map's walk dot was re-ruled by the host on 2026-10-22 (`Map::along`). This task's gate never ran on its own; the release-branch gate after merging fn-272 is green: 3010ab022, 1,482 passed, 0 failed.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 3010ab022, aaa40d776
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py
- PRs: