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
The harness opens the engine's beech at 40 (`OPENING_PARAMS`) with an age dial from 1 to 80 that rebuilds through `setTreeGpu`; links take `?age=`. Beech, oak, spruce and palm open in the dev harness through the engine at 40, and the beech at 80 by the dial. Screenshots were taken headless on the desk GPU and viewed (BROWSER.md, `raw/task3/`).

On 32-bit Wasm, a grown tree that keeps more than 4,000,000 phytomers is refused by name before its preparation is allocated (`pipeline/space.rs`, `WASM_KEPT`). The threshold is measured in the harness: the spruce at 73 keeps 4.27M and builds; at 75 it keeps 4.48M and traps. The rule refuses the spruce from 71, and the walk 40, 70, 80 (refused), 60 runs in one page without a trap. The refusal follows growth, not precedes it (up to 8 s for the spruce at 80); refusing before growth is left to the host.

Silver birch, laurelin, telperion and ordinary show "no engine species", and a failed build hides the canvas's tree.

The engine adds 309 to 357 KB to each Wasm module against fn-198's base. The field module's ceiling is raised from 400,000 to 632,580 and `telperion.js`'s from 150,000 to 153,565, on the branch only and for the owner. Times from a parameter change to the first frame are in BROWSER.md: beech 0.55 s at 40 and 5.7 s to 80; oak 1.6 s and 17.5 s; spruce 4.8 s at 40 and 15.6 s to 70; palm under 0.4 s.

Open: a second spruce build in one page took 66 s against 9.6 s fresh, and was not rerun because the GPU lock was held.

## Evidence
- Commits: cfe65d4d, 95eaf7e7, e1b0423c, 47e2df19, 2b32a3b5
- Tests: npm test (143 passed), cargo test --profile ci --workspace --no-fail-fast (exit 0)
- PRs: