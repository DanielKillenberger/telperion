---
satisfies: [R2, R3, R4, R5]
---
# fn-19-comparative-botanical-geometry-benchmark.3 Add matched anatomy captures and projected crown-gap diagnostics

## Description
Add an isolated capture adapter and projected crown-gap diagnostics for the frozen anatomical views.

## Acceptance
- Analytic mask tests distinguish holes, exterior openings and outside-crown background and reject crop/identity mismatch.
- A small oak/spruce smoke capture inspects framing and connected anatomy; unresolvable targets are rejected.

## Done summary
Adapter prepares fixed anatomical cameras and crown ROIs, captures at native/64/128 samples and reports projected holes/openings with explicit identity, clipping and convergence failures. Nine analytic controls and fourteen small views passed collection; occluded attachment traits stay unassessed.

## Evidence
- Commits: cc20df5d1accf73d7db994a094f869949cc901fc
- Tests: node --test tests/browser/geometry-benchmark.test.mjs; npm run typecheck; cargo test --release -p telperion-core --test species_metrics
