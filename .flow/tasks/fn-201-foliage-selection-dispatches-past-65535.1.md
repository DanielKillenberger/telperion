---
satisfies: [R1, R2, R3, R4]
---
# fn-201-foliage-selection-dispatches-past-65535.1 Split the per-instance select dispatches over x and y

## Description
Per-instance select passes (select, scatter) split their workgroups over x and y as generation/io.rs:133 does; select.wgsl rebuilds the flat index from global_invocation_id and num_workgroups and skips past the end. The per-level prefix pass is unchanged.

## Acceptance
- [ ] A GPU test selects a set needing more than 65,535 workgroups without a panic, red on the base.
- [ ] Below the limit the selection is byte-identical to the flat dispatch (existing compaction test stays green).
- [ ] Headless and catalogue tests stay green.
- [ ] Workspace gate green.

## Done summary
The per-leaf select and scatter passes now fold their workgroups into a second dimension once the first runs out (`grid` in `select.rs`, as `generation/io.rs:133` does), and `select.wgsl` rebuilds the flat leaf index from `global_invocation_id` and `num_workgroups`; a select group past the last one returns as a whole, so its barriers stay uniform and it writes no slot. The per-level prefix pass is unchanged. Below the limit the grid is `(groups, 1)`, the flat dispatch it replaced.

Test: `a_crown_past_one_dispatch_dimension_is_selected_in_order` selects 65,536 groups of 256 plus 3 leaves at two levels and checks every list in placement order; on the base it panicked with wgpu's "dispatch group size dimension ([65537, 1, 1])". It skips by name on a device that cannot bind the placements (Codex round 1, P2). `the_grid_is_flat_until_one_dimension_runs_out` pins the grid. The existing 65,539-leaf compaction test stays green.

Gate: `cargo test --profile ci --workspace --no-fail-fast` 1,020 passed, 0 failed, 21 ignored (run before the test-only skip guard; the select tests were rerun after it). Codex impl-review: SHIP in round 2, no open findings.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0c34e200, 247e09e5
- Tests: cargo test --profile ci -p telperion-render --lib select::, cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs: