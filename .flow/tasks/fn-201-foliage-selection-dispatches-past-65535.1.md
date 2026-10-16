---
satisfies: [R1, R2, R3, R4]
---
# fn-201-foliage-selection-dispatches-past-65535.1 Split the per-instance select dispatches over x and y

## Description
Per-instance select passes (select, scatter) split their workgroups over x and y as `generation/io.rs` does; `select.wgsl` rebuilds the flat index and skips past the end. The per-level prefix pass is unchanged.

## Acceptance
- A GPU test selects a set needing more than 65,535 workgroups without a panic, red on the base.
- Below the limit the selection is byte-identical to the flat dispatch.
- Headless, catalogue and workspace gate green.

## Done summary
`grid` in `select.rs` folds groups into y; `select.wgsl` rebuilds the flat index, and a group past the end returns whole. Test `a_crown_past_one_dispatch_dimension_is_selected_in_order` panicked on the base; `the_grid_is_flat_until_one_dimension_runs_out` pins the grid; the 65,539-leaf compaction test stays green. Gate 1,020 passed, 0 failed. Codex SHIP in round 2.

## Evidence
- Commits: 0c34e200, 247e09e5
- Tests: cargo test --profile ci -p telperion-render --lib select::, cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs:
