# Wood grows around the wood already there

## Goal & Context

The owner (2026-10-14): "branches clipping in each other". fn-236's DEFECTS.md counted pairs of thick segments on different branches overlapping away from their junction in the oak at 40: 1,276 / 3,816 / 1,194 at seeds 1, 7, 42, with 490 / 834 / 174 where the thinner branch runs wholly inside the thicker. Cause, checked in code: a branch's direction comes from insertion angle, tropism, wander, straightening and sag (`telperion-space/src/geometry.rs`); light changes how much a branch grows, not where it points; nothing reads where other wood is. Real crowns rarely interpenetrate: a shoot growing into wood is abraded, shaded and stops, and shoots grow toward open, lit space.

## Requirements

- **R1 (measure first):** the crossing count and depth for every shipped species at its age and at 80, seeds 1, 7, 42, and at which ages crossings first appear; how many are visible from outside the crown (stills viewed). What the growth law already has that could carry avoidance (the light lattice's occupancy, the ground rule of fn-252, space colonization's attraction points) checked against code.
- **R2 (design, the host's):** the host chooses the mechanism from R1 (e.g. a shoot's direction biased away from occupied lattice cells by degree, or a shoot whose apex enters existing wood dies there, through the existing abortion draws), one rule on both executors, by degree, dormant at 0, no species branch.
- **R3:** crossings fall to the measured target; growth's other measures (height, crown width, leaf area) within their tolerances; stills viewed; cost measured; gate, npm, review; the owner's verdict.

## Boundaries

Growth only; the drawn fork surface is fn-279. A model change for every species: release or later is the owner's call (2026-10-14).
