---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-42-bark-depth-level-and-a-fair-capture.1 Implement bark depth, level and a fair capture

## Description
Implement fn-42 against R1-R5 under the owner's qualitative-reference decision of 2026-09-19: calibrated bark captures, corrected parallax, tuned oak/spruce rows, unchanged filtering bounds, and recorded views for the owner.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Calibrated oak/spruce bark captures, corrected parallax, continuous shallow chipped scale profiles with the authored `plateEdgeShape` blend, and restrained warm recess colours; other species unchanged. Material, shader, profile, distance, resolution, redraw, browser round-trip and catalogue checks pass at unchanged bounds. Native whole-tree p50 8.9231 -> 14.5405 ms, cost disclosed per owner decision. The historical hero reconstruction remains a disclosed inherited failure under the owner's exception. Owner accepted and authorised a squash merge (PR #47, `a63802a2`). Overlapping flakes went to fn-90.

## Evidence
- Commits: 361dedc1c93781edc29f512fb228a559b7d22b2f
- Tests: cargo test --profile ci -p telperion-core --test material_detail, cargo test --profile ci -p telperion-render --test bark_plates --test bark_relief_range --test material_shaders -- --test-threads=1, ...
- PRs:
