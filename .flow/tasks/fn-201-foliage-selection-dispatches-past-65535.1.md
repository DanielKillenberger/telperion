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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
