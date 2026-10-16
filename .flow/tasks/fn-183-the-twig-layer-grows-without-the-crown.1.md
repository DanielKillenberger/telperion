---
satisfies: [R1, R2]
---
# fn-183-the-twig-layer-grows-without-the-crown.1 Measure the ladder: queries by purpose, then each rung on five presets

## Description
Add crown-query counting by purpose to `growth_profile` (merged) and measure each rung of the ladder on five presets at seeds 1 and 7 (exploratory, not merged); stop for the host's R2 decision.

## Acceptance
- Queries by exclusive caller and purpose, counters compiled out for timing.
- R1-LADDER.md per rung and preset: queries, time, nodes, leaves, density, excursion; stills listed.

## Done summary
`growth_profile` counts crown queries by purpose behind the `query-count` feature (bd7a2c7d). The ladder was measured (c4950378, reverted in f68203c3); report in `R1-LADDER.md` (170b5981). The host first chose rung 3b (42f42e72); the owner then chose rung 1 from the stills.

## Evidence
- Commits: bd7a2c7d, c4950378, f68203c3, 170b5981
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs: