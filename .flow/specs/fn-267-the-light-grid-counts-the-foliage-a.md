# The light grid counts the foliage a tree keeps

## Goal & Context

Found by fn-264 (2026-10-12): the lattice deposit counts only the year's foliage, while an evergreen keeps several years of it (Norway spruce about 5.5 years of needles), so an evergreen shades less than it should. The drawing already keeps leaves by `leaf_years` retention (fn-248), but it also bounds them by a slender girth read from the finished tree's radii (`foliage/plan.rs:484`), which the growth cycle does not have; and the oak, birch, spruce and palm carry unlimited `leaf_years`, so reading retention alone would deposit leaves on trunks.

## Requirements

- **R1 (checked against code):** where the drawing's leaf rule and the deposit differ, and what growth knows each cycle that could stand for the girth bound (axis order, wood age, the radius the cycle carries if any).
- **R2:** a sourced foliage lifespan for each species (spruce about 5.5 years; deciduous species one season), as a row; the deposit counts the foliage each node bears by that lifespan and a girth bound growth can evaluate, the same rule the drawing uses, by degree, on both executors.
- **R3:** stills of the spruce and one deciduous species beside the current ones, viewed; light error per bud before and after; cost; gate and npm test green; the owner's verdict.

## Boundaries

Foliage retention in the light deposit. After fn-264. Not on the release map until the owner says.
