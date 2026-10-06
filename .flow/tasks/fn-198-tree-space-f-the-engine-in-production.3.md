---
satisfies: [R1]
---
# fn-198-tree-space-f-the-engine-in-production.3 The harness and the browser through the engine

## Description
Overnight scope step 3, by INTEGRATION.md host decisions 5, 6 and 10 (owner: "integrated into the harness").

- The harness (`npm run dev`) opens beech, spruce, oak and palm through the engine via `setTreeGpu` and the Wasm binding, at default age 40, with an age slider to 80 that rebuilds through the same path (the age slider the owner asked for).
- A species and age whose CPU preparation cannot fit the 4 GB Wasm ceiling is refused with a named error before the build starts (the spruce at 80 measured 5.3 GB natively), never a trap; measure each species' ceiling age in the browser build.
- The Wasm size ceilings: measure the engine's added size per shipped module; raise the ceilings on this branch to the measured sizes and list each change for the owner (decision 6).
- The refused presets (silver birch, laurelin, telperion, ordinary) show their named error in the harness instead of a tree.
- Drive the harness with the existing QA/browser tooling (headless, `npm run dev`), never a full-forest capture (AGENTS.md budget): screenshots of the four species at 40 and the beech at 80 in the harness, viewed.
- Time each species from a parameter change in the harness to the first drawn frame, at 40 and 80 where it fits.

## Acceptance
- The four species open in npm run dev through the engine; the age slider rebuilds; screenshots viewed. - Over-ceiling species and ages refused by name before building. - Size ceilings measured and raised on the branch, listed. - Refused presets show their error. - Times from a parameter change to the first frame. - npm test and the workspace gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
