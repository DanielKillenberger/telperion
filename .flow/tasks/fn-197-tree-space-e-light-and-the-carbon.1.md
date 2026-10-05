---
satisfies: [R1]
---
# fn-197-tree-space-e-light-and-the-carbon.1 The light lattice: Beer-Lambert from the sky, with its oracle tests

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The light lattice (src/light.rs): Beer-Lambert from nine sky directions on a world-anchored 0.5 m lattice, Light{extinction, sky} on Request (neutral at 0 extinction). Oracle tests: slab exact, sphere within 0.032 optical depth, GreenLab production 3.3%/1.5% converging, sky shares, neutral, by degree, refusals.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: eda1cb31
- Tests: cargo test --profile ci -p telperion-space --lib light
- PRs: