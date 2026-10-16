---
satisfies: [R4, R2]
---
# fn-6-branch-generations-below-the-crossover.5 Fill, shed, and the leaf on the twig

## Description
Measure fill against the tuft baseline, check the shell rule is load-bearing, and put the leaf on the twig.

## Acceptance
- Fill thresholds set tight-first; shell rule passes its two-sided guard; canopy places leaves on marked twig edges.

## Done summary
`buildCanopy` takes `TwigAnatomy` and places stations along marked twig edges. Occupancy rose 6.6x (Telperion, 20.2%) and 3.6x (Laurelin, 8.4%) over the tuft baseline; Telperion's clustering worsened to 64.3%, recorded.

## Evidence
- Commits: aa6bc4c
- Tests: npx vitest run (343 passed), npx tsc --noEmit
- PRs:
