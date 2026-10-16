# fn-5-branch-until-the-tips-bear-leaves-one.6 The docs that still say one algorithm, and fn-1's numbers that no longer hold

## Description
Retire the docs that say the skeleton is colonization alone, export the depth surface, and re-measure fn-1's superseded numbers.

## Acceptance
- README, index header, package.json and colonize.ts header describe the two-pass structure; depth exports in the barrel; fn-1's figures re-measured.

## Done summary
Docs updated; thirteen depth exports added to `src/index.ts`; README quickstart typechecked against the barrel. fn-1 gained a re-measured table: shedding removes 8-9% of twig nodes, the leaf cull then 0.4-3.3%, leaves 136,808 / 766,435 at eight orders.

## Evidence
- Commits: 831692a, 60805ae
- Tests: npx vitest run (325 passed), npx tsc --noEmit, npm run build
- PRs:
