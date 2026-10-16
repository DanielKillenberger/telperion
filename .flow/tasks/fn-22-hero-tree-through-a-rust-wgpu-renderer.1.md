---
satisfies: [R3]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.1 Core mesh contract and parameter JSON in the core

## Description
Add the engine-neutral mesh contract and move the parameter JSON mirror into the core.

## Acceptance
- `mesh::build` returns wood, foliage element, instances, bounds and counts; JSON behind an optional core feature.

## Done summary
`mesh::build(&Family, Detail::Full) -> Result<TreeMesh>` runs the same chain as the C-ABI binding and returns wood surface, foliage element and instances, union bounds and counts. The JSON parameter mirror moved into the core behind a `json` feature; the default core stays serde-free.

## Evidence
- Commits: 6243121df520d0a9544c5c9b6dd03f77db37b0d7
- Tests: cargo test --release --workspace; cargo clippy --workspace --all-targets
