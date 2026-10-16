---
satisfies: [R1, R3, R9]
---
# fn-5-branch-until-the-tips-bear-leaves-one.3 Local-rule branching below the crossover, down to leaf scale

## Description
Local-rule branching below the crossover down to leaf scale, appended into the same skeleton and shaped by the same bias field.

## Acceptance
- One structure, `parent < self`, level-capped, no chance or Map/Set in the candidate path.
- Every twig step passes through the colonization bias function (R9); sourced botanical defaults; leaf-to-wood ratio measured (R3).

## Done summary
`src/skeleton/twigs.ts` continues breadth-first from colonization's tips with defaults children 2, angle 45, divergence 137.508, internode 1 step, taper 0.6/order, plus a per-whorl sibling check. Zero orders byte-identical. At eight orders the leaf is 1.64x (Telperion) / 1.16x (Laurelin) the median terminal diameter, from 0.15/0.16. Area conservation alone did not reach the botanical ratio, so the fine-order taper law went to .4.

## Evidence
- Commits: 539359612a8e1e2fa5692e0beecb4c0f501b8b0f
- Tests: npx vitest run (302 passed), npx tsc --noEmit, npm run build
- PRs:
