---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-223-leaves-cluster-at-the-engines-short.1 Implement leaves cluster at the engine's short shoots

## Description
Carry plan stated before code (R1, checked at 891c3bd0): per-PA rows `cluster_leaves`/`cluster_spread` in `PaState` and the device geometry table (37 -> 39 words); the CPU conversion and the device dress mark clustered nodes and the axis's last drawn node as its tip; clustered nodes bear no stations and each tip is its own run of `leaves` stations, counted by the instance budget; leaves keyed by `cluster_key(tip key)`; the canopy's `shortShoot*` rows deleted (they drew nothing).
## Acceptance
Every R-ID in the parent spec's requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
Engine axes whose PA sets `cluster_leaves` > 0 bear their leaves as one cluster at their last drawn node, on both executors, keyed by the `CLUSTER` registry stream. The canopy's `shortShoot*` rows and mechanism deleted (beech unchanged). Neutral rows byte-identical (artifacts and stills); device vs CPU 323 clusters matched, tips within 9.9e-7 m; beech at 40 builds in 260 ms vs 303. `cluster_leaves` mixes on a `Whole` scale (codex finding). Review: NEEDS_WORK, one fix, SHIP.

## Evidence
- Commits: 3e5d85d2cfae3caea2f485490f8142539fdc54c6, 6ca50c4eb3c6f20460c18612251d0842f5c3464d, 60efe6c580f2a2428bbd2c343130dd966968230a, 45dd1ef4c186bc06b472063f821bfdb047caddbc, 717699bb3dfd2dc3c7df96db0d00b78244975874, 4db2343744e6e316f9b2ee26e67ab6fec33ca320, d1e5361b0fd27214480099a2fd876e76f8dd40cb
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (under GPU lock): green, cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast (6 failures fixed in 4db23437, rerun green), npm test
- PRs: