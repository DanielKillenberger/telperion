# A walk across ages grows the tree

## Goal

Found by the teaser session (2026-10-09) on F4's `headless --walk`: a walk between two ages of one species (Oregon white oak 5 → 80, seed 7) did not appear to grow the tree: `blend::sized` (fn-230's size factor) shrank interior frames toward the ends' weighted geometric-mean size, so t ≈ 0.95 was a modest crown and t = 1 jumped to the full oak. Release map item (the harness's walks); scope: the blend's size target only.

## Decisions

- Host design (2026-10-09): the target compares like with like: the ends are re-evaluated at the frame's own blended age, and the target is their weighted geometric mean. For two species at one age nothing changes; for one species across ages the factor is 1 and the walk is growth; mixed walks fall between by degree. No switch on "same species".
- Closed (host, 2026-10-13): not reproduced. fn-251 R1b measured the age walk (oak 5 -> 80) equal to the oak grown at each frame's age in every variant, factor 1. Nothing to fix.

## Acceptance Criteria

- R1: the size target built from the ends evaluated at the frame's blended age; cached ends keyed by that age — dropped (not reproduced).
- R2: an age walk equals the species grown at each frame's age; cross-species walks unchanged; continuity at t → 1 tested; solve time measured — dropped (fn-251 R1b measured it already holds).
- R3: gate and npm test green — dropped.

## Result

No change. fn-251 R1b showed the age walk already grows the tree.

## Open

None.
