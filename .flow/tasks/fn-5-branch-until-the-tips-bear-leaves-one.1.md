---
satisfies: [R4, R7]
---
# fn-5-branch-until-the-tips-bear-leaves-one.1 The search radius learns how far apart the attractors are

## Description
Derive the attractor search radius from attractor spacing so finer steps cannot starve growth, while today's trees stay byte-identical.

## Acceptance
- The 0.44 m / 1,600-attractor collapse grows a whole tree (R4).
- Every existing fixture and both presets byte-identical at today's step (R7).

## Done summary
`influenceRadiusFor` takes the wider of nine steps and a spacing floor (cube root of crown volume per attractor). Telperion at 0.44 m / 1,600 attractors now grows ~14,000 nodes / ~1,200 tips (was 169 / 3). At 1.2 spacings 3 of 108 seed-by-cell draws still starved (colonize ends the tree when no tip sees an attractor); the spec settled on 2.0 spacings.

## Evidence
- Commits: 24aa80d8b2307df4b702f6e43e598859a18bffe7
- Tests: npx vitest run (272 passed), npx tsc --noEmit, npm run build
- PRs:
