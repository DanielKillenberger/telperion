---
satisfies: [R1, R7]
---
# fn-5-branch-until-the-tips-bear-leaves-one.2 Branching depth as a parameter, from the panel to both presets

## Description
Branching depth as one parameter from panel to both presets.

## Acceptance
- One step parameter scales kill distance and search radius together; non-finite falls back to the default; `maxNodes` fits the rail and capping is reported.
- Default reproduces both presets byte-identically (R7).

## Done summary
`SkeletonParams.step` (fraction of envelope height, default 0.022, rail 0.003..0.022) with kill at two steps and `maxNodes` scaled as 8000 x (0.022/step); `TreeStats.capped` reports ceiling hits. Both presets state `step: 0.022`. 12 pinned skeleton signatures byte-identical.

## Evidence
- Commits: a5f2a31996f7f91c7a17603715c5029e2f01bbce
- Tests: npx vitest run (280 passed), npx tsc --noEmit, npm run build; node hashes Telperion 808 / Laurelin 2442 unchanged
- PRs:
