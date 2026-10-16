---
satisfies: [R5, R6]
---
# fn-5-branch-until-the-tips-bear-leaves-one.5 What it costs: the curve, the counts, and a build you can still drag

## Description
Measure what depth costs, shed interior twigs, reach the botanical leaf count, and keep the build draggable.

## Acceptance
- Leaf count measured after shedding on both presets with the reachable ceiling stated (R5).
- Cost curve recorded; `maxNodes` reported; dragging stays usable (R6).

## Done summary
A uniform grid over attractors made colonization 1.5-1.6x faster at the rail's bottom, byte-identical. `shedTwigs` applies the shell rule to twig nodes; node ceiling scales with orders under a 250,000 stop; builds dearer than 80 ms settle 250 ms after the last notch. Leaves after shedding: Telperion 2,904 / 14,310 / 40,523 / 136,686 / 519,001 at 0/4/6/8/10 orders; Laurelin 8,115 / 59,220 / 204,321 / 766,390 at 0/4/6/8. GPU frame cost was left to the conductor's sweep (in the spec's Result).

## Evidence
- Commits: 2421f9d8c7f4c8508eae207f8b74b711cfc4abfd, 776a011
- Tests: npx vitest run (325 passed), npx tsc --noEmit, npm run build
- PRs:
