---
satisfies: [R2, R3]
---
# fn-5-branch-until-the-tips-bear-leaves-one.4 Crossover continuity, and the test that cannot pass vacuously

## Description
A taper law for the fine orders, and a crossover-continuity test that cannot pass vacuously.

## Acceptance
- Radius, direction and taper continuous across the crossover on both presets, asserted on the drawn junction ring; a parameter set with no crossover reports untested.

## Done summary
`branchTwigs` returns the crossover index; below it the radius solve runs top-down from each parent's actual radius (`twigTaper`, optional, railed), so the seam has no ratio of its own and trunk-to-limb radii stay byte-identical. Tests were red first against the old solve. The 90th-percentile taper assertion stands; the tail is fn-4's blunt-tip defect.

## Evidence
- Commits: 562cb70, 7118168
- Tests: npx vitest run (325 passed), npx tsc --noEmit, npm run build
- PRs:
