# fn-284-the-harnesss-controls-change-the-tree.2 The engine's rows reach the browser table and the panel

## Description
R2. Engine keys enter the browser table with bounds, units, meanings and scale from species/scale.rs and validate (one source, no copy). One control per key moves every age the preset states on the key's scale, clamped to each age's bounds; it opens to one box per age.

## Acceptance
R2

## Done summary
One table per engine setting in telperion-space (species/scale/table.rs): name, scale, bounds, unit, meaning. validate reads its single-range checks from it, its messages and order unchanged; edges.rs proves the bounds equal the checks as written before and that refusals fire one step past each edge. The catalogue generator emits SPACE_SETTINGS; the harness moves a setting at every stated age on its scale (log multiplies, linear offsets, kept on its share), held to bounds, opening to one box per age and zone; rows go back through /space, the engine refusing by name. Bud tables (lateral, dormant) are not offered. Engine probe: palm 17 live/39 dormant, ordinary 49/9, oak 51/7, beech 49/9, birch 52/6; refused by name: zone.nodes.min everywhere (min above max), elongation and expansion on the palm (sum over the year).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 25faf8bf4
- Tests: cargo test --profile ci --workspace --no-fail-fast (1409 passed, 0 failed, 101 ignored), npm test (18 files, 179 tests)
- PRs: