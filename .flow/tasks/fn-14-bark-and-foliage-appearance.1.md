---
satisfies: [R5]
---
# fn-14-bark-and-foliage-appearance.1 Surface coordinates from the core, as one more buffer

## Description
Emit surface coordinates from the core: along and around each branch, along and across each blade, through every level.

## Acceptance
- Two floats per wood and leaf vertex in their own buffer; positions, normals, indices, skeleton and placement pins unchanged.

## Done summary
Wood and leaf vertices carry the two coordinates in a separate buffer through the core, the wasm slots and every level; existing pins hold.

## Evidence
- Commits: 0fd2edb33830319921133b4d4735923d936e9bff, 18859d1f523e6631ba0227e7f0edd474a6c5f6b5
- Tests: cargo test --release --workspace; cargo clippy --workspace --all-targets
