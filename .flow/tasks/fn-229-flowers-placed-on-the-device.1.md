---
satisfies: [R1, R2, R3, R4]
---
# fn-229-flowers-placed-on-the-device.1 Implement flowers placed on the device

## Description
TBD

## Acceptance
Every R-ID in the parent spec's requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
The device now places the florets of the fine roots it dresses, following fn-212's pattern for leaves (host decisions D1 to D5). A flowering family comes apart like any other, and the interim refusal in `executor::apart` is deleted.

- **R1, keys and counts.** The phytomer's 64-bit key and its age's flowering share travel from the settle through the dress and the sweep. The per-node floret counts are scanned in the stations' existing owners scan.
- **R1, placement.** `bloom.wgsl` mirrors `bloom.rs`. A second `place.wgsl` entry point places the florets in the same submission as the device's leaves.
- **R1, delivery.** One organ buffer holds the CPU block's florets, repacked against the leaf box padded by a floret's reach, and then the device's. One mass grid covers both, and every summary comes back in one readback.
- **R2, parity.** Florets per tree equal the CPU whole tree's on the beech and the spruce at 30, 40 and 80. The residuals fall in two named classes: least (+1 to +5 device florets at the 1e-3 size threshold) and turned (the tangent at `perpendicular`'s switch). Details are in FLORETS.md.
- **R3, refusal and shadows.** The refusal is deleted, along with its limits-inventory entry. Flowers still cast no shadow, because the organ draw is not in the shadow pass; this is recorded.
- **R4, gate.** The render half (195 passed) and the rest of the workspace (1,040 passed) are green, and `npm test` passes. The device path's leaves and curve are byte-identical to the base for non-flowering trees, compared as sorted leaf words.
- **Decision, D3 reading.** The florets are counted per node in the per-node owners scan, not per run, because blooms also stand on non-bearing nodes.
- **Follow-up, not this change.** The device leaf buffer's order varies from run to run at the base (the `owned` atomicAdd rank).
- **Feature map.** No feature route changed.

Tier: (none given by the conductor)

stage: impl-review - ran (codex: NEEDS_WORK, then SHIP after one fix pass; contracts:3 declined, as R2 allows residuals by named class)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b4a5dbf007decea5c004d4570fc9cf0dc00da336, a8a9fe02056bb15b67fe4c9cb25ce8c1fde79fe6, 895055a4b23b3bab9f92e5931a09a229000bf5b8, bcc91f136994b996b487782bfcc34dc860612a12, 7c40a35d9362ef4a96d3cd8e23333b3e130671a9, 35d17db82f3d00eb3992f430d9ca4fc54a8b823d, a0bc52f2a3e068ca7a3d1b116355f33672dd7188
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, npm test, baseline: none
- PRs: