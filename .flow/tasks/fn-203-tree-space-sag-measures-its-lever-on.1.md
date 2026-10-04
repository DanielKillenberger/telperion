---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-203-tree-space-sag-measures-its-lever-on.1 Sag measures its lever on the bent branch: one march, tests, spruce round-9 re-render

## Description
Build the spec's Design: each phytomer's moment from the load beyond it rotated rigidly by the upstream bends; R1 to R5.

## Acceptance
R1 to R5 of the spec.

## Done summary
Sag measures its lever on the bent branch, built to the Design and host decisions 1 to 4.

- **The lever:** each phytomer's load lever, kept on the unbent tree, is turned rigidly by the bends before it. It is capped by the unbent lever, so it only ever shrinks and trunks do not buckle. Each phytomer's turn is integrated exactly.
- **The ground:** it carries the wood beyond any point where it rests, measured along the turned chord from the phytomer's base through its continuations.
- **One landing bound:** 0.8 rad.
- **Tests:**
  - R1 (i): free-air convergence.
  - R1 (ii): the low bough does not swing round. Red on fn-200 at 1.61 rad.
  - R2: within 1 percent of fn-200's bends.
  - The carried-load sweep.
  - The landing walk up to the spruce's sag.
- **Neutral:** the beech is byte-identical.
- **R4:** spruce seeds 1 and 4 with no loops and nothing toppled.
- **Codex:** SHIP in round 2.
- **Gate:** 1,038 passed. One telperion-core memory-ceiling test failed under parallel load and passes alone (friction).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 3b877b13, 625a16c8, 31ec3638, affb8d4f, 588fc406
- Tests: cargo test --profile ci -p telperion-space --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs: