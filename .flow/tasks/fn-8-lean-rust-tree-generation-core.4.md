---
satisfies: [R1, R3]
---
# fn-8-lean-rust-tree-generation-core.4 Port the final swept surface to independent Rust output

## Description
Port the final swept surface to an independent Rust output.

## Acceptance
- Caller-owned positions, normals, indices and bounds; no foliage or renderer dependency.

## Done summary
Surface ported with flat adjacency, reusable scratch, checked size math and fallible reservations; 11 tests and the reference integration pass; clippy clean.

## Evidence
- Commits: 8e9295f70e182fb85620fa56e94485ef8b094238
- Tests: cargo test (11 passed), cargo clippy -D warnings
- PRs:
