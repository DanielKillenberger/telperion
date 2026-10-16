---
satisfies: [R1, R2, R3, R4]
---
# fn-229-flowers-placed-on-the-device.1 Implement flowers placed on the device

## Description
Place the fine roots' florets on the device following fn-212's leaf pattern (host decisions D1-D5), per the spec.

## Acceptance
Every R-ID in the parent spec's requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
The device places the florets of the fine roots it dresses: key and flowering share carried through settle, dress and sweep; counts in the stations' owners scan; `bloom.wgsl` mirrors `bloom.rs`; a second `place.wgsl` entry in the same submission; one organ buffer and mass grid. The `executor::apart` refusal is deleted. Parity equal on beech and spruce at 30, 40 and 80 with named residual classes (least, turned). Flowers cast no shadow (recorded). Non-flowering device output byte-identical. Gates green. Review: codex NEEDS_WORK, one fix pass, SHIP.

## Evidence
- Commits: b4a5dbf007decea5c004d4570fc9cf0dc00da336, a8a9fe02056bb15b67fe4c9cb25ce8c1fde79fe6, 895055a4b23b3bab9f92e5931a09a229000bf5b8, bcc91f136994b996b487782bfcc34dc860612a12, 7c40a35d9362ef4a96d3cd8e23333b3e130671a9, 35d17db82f3d00eb3992f430d9ca4fc54a8b823d, a0bc52f2a3e068ca7a3d1b116355f33672dd7188
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, npm test
- PRs: