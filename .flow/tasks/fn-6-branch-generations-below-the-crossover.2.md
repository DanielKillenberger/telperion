---
satisfies: [R1, R2, R3, R5, R8]
---
# fn-6-branch-generations-below-the-crossover.2 Branches from the tips: the pass grows under the law, reads the field, ends on radius

## Description
Rewrite the pass to grow branches from every handoff tip under the law, reading the radius field, ending on radius, refusing invalid input, with the solve below the crossover reading its records.

## Acceptance
- Twigs stay at the fixed anatomy across heights and trunk radii; invalid skeletons/fields refused by name; continuity at every handoff.

## Done summary
Tip branches grow under the law; the final solve reads recorded base radii; `twigTaper` retired. Task 3 was folded in after two worker attempts found the pass and the solve are one gate boundary.

## Evidence
- Commits: e7c73ad
- Tests: npx vitest run (338 passed), npx tsc --noEmit
- PRs:
