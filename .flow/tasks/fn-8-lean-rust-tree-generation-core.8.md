---
satisfies: [R3, R7]
---
# fn-8-lean-rust-tree-generation-core.8 Expose mesh-free wood and foliage fields

## Description
Expose mesh-free wood and foliage fields.

## Acceptance
- Separate occupancy flags, caller-resolution cell queries, no surface dependency.

## Done summary
Owned wood and retained-foliage fields with median BVHs for spatial pruning; three tests cover empty/invalid queries, tapered wood, leaf overlap and malformed input.

## Evidence
- Commits: d1c4419bb3d0a01a8470cd8cd3f2b7a7e72ba38b
- Tests: cargo test (3 passed)
- PRs:
