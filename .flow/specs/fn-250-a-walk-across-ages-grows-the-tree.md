# A walk across ages grows the tree

## Goal & Context

Found by the teaser session (2026-10-09) on F4's `headless --walk`: a walk between two ages of one species (Oregon white oak 5 → 80, seed 7) does not grow the tree. `blend::sized` (fn-230's size factor) shrinks every interior frame toward the ends' weighted geometric-mean size, so the frame near t = 0.95 (about age 76) was a modest crown and t = 1 jumped to the full oak at 80. The size factor treats age as one more row: its target is the weighted geometric mean of the two ends' kept counts, each at its own age, which for an age walk lies far below the tree actually grown at the frame's age. Host design (2026-10-09): the target compares like with like: the ends are re-evaluated at the frame's own age (the blended age), and the target is the weighted geometric mean of those. For two species at one age nothing changes; for one species across ages the target equals the frame's own tree, so the factor is 1 and the walk is growth; mixed walks fall between by degree. No switch on "same species".

## Requirements

- **R1:** the size target built from the ends evaluated at the frame's blended age, on the existing size solve; the cached ends (fn-238) keyed by that age.
- **R2 (proved):** an age walk of one species equals that species grown at each frame's age (kept and grown counts, wood length within the solve's tolerance; `space_hash` equal where the age is whole); every cross-species walk at one age unchanged (walk digest and fn-230 strips identical); fn-231's end-continuity tests hold, and a new test holds the age walk continuous at t → 1; solve time measured.
- **R3:** gate and npm test green.

## Boundaries

The blend's size target only. Release map item (the harness's walks).

## Closed (host, 2026-10-13)

Not reproduced: fn-251 R1b measured the age walk (oak 5 -> 80) equal to the oak grown at each frame's age in every variant, factor 1. Nothing to fix.
