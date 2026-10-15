# fn-284-the-harnesss-controls-change-the-tree.1 Reach is what the engine reads: dead rows and adapters leave the panel

## Description
R1. Derive reach from what the engine reads (drop Grow from the dead rows' reads, or a reach value, whichever is smaller); remove the density and torsion adapters; fix rows.test.ts:44; replace the ignored dormancy test with the per-preset probe as an on-demand release check (4-9 min a preset, not the gate).

## Acceptance
R1

## Done summary
Reach is derived from the stages a row names; the scaffold rows the engine never reads (skeleton but seed, envelope, habit, bias, growth, twig rows the leaf box and stations do not read, radii but trunkRadius, shellDepth) name none, so 80 rows leave the panel (365 to 285). Density and torsion adapters removed. Conditional rows declare dormancy (incl. structural: a crown of fronds, a stem that never forks); trunkRadius and twig/diameter are relabelled as the leaf box. tests/dormancy.rs replaced by the on-demand probe tests/controls.rs.

Probe (frozen binary, 3 builds at once): palm 166 live/115 dormant/2 refused, ordinary 154/128/1, oak 154/126/3, beech 154/128/1, birch 155/126/2; one unexplained row on each, /canopy/clumpSystemOrder (read by placement thinning above limbClumping zero and by the planned field's limb systems; no artifact changed, field included). Spruce, Telperion, Laurelin pending (killed at the 6 GB cap; rerun at --threads 1 at the end).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 4aa72950a, 9a1f14ac0
- Tests: cargo test --profile ci -p telperion-core --lib catalogue, cargo test --profile ci -p telperion-core --test parameter_reference, controls probe (ignored, on demand)
- PRs: