# fn-284-the-harnesss-controls-change-the-tree.2 The engine's rows reach the browser table and the panel

## Description
R2. Engine keys enter the browser table with bounds, units, meanings and scale from species/scale.rs and validate (one source, no copy). One control per key moves every age the preset states on the key's scale, clamped to each age's bounds; it opens to one box per age.

## Acceptance
R2

## Done summary
One table per engine setting in telperion-space (`species/scale/table.rs`: name, scale, bounds, unit, meaning); `validate` reads its single-range checks from it, messages and order unchanged, and `edges.rs` proves the bounds equal the old checks. The catalogue generator emits `SPACE_SETTINGS`; the harness moves a setting at every stated age on its scale, held to bounds, opening to one box per age and zone. Bud tables not offered. Engine probe: palm 17 live/39 dormant, ordinary 49/9, oak 51/7, beech 49/9, birch 52/6.

## Evidence
- Commits: 25faf8bf4
- Tests: cargo test --profile ci --workspace --no-fail-fast (1409 passed, 0 failed, 101 ignored), npm test (18 files, 179 tests)
- PRs: