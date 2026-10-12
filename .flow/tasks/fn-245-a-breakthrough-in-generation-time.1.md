# fn-245-a-breakthrough-in-generation-time.1 Spike: is the tree coupled only through a smooth field?

## Description
Owner, 2026-10-12: test whether growth can couple only through a coarse light-and-vigour field, so subtrees grow independently in a few passes. CPU reference cycle only, no GPU. For Telperion 82 (seed 42) and the oak 80 (seed 1): (1) per bud per cycle, the exact light read and incoming vigour against the same quantities from the field smoothed onto 32^3 and 64^3 grids over the crown (and vigour as a per-limb share); report error distributions; (2) regrow three subtrees of each tree (a big limb, a mid branch, a shaded interior one) independently against the smoothed field held from the exact run, and compare against their exact selves: kept phytomers, shed decisions, tip positions, leaf area, and stills; (3) the same with the field iterated 2-3 passes from a first guess instead of taken from the exact run. Evidence FIELD.md; stop and report.

## Acceptance
FIELD.md with the error distributions, the regrown-subtree comparisons with viewed stills, and the iterated-pass results.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
