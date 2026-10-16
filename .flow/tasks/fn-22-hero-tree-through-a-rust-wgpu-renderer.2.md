---
satisfies: [R2]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.2 Renderer crate foundation and headless still

## Description
Found the renderer crate and render a headless still.

## Acceptance
- Native offscreen render to PNG from preset id, seed and view; every failure names its condition.

## Done summary
`crates/telperion-render` draws the hero tree: wgpu 30 device with named failures, clay room in Rust, hero framing from bounds, the core's wood arrays uploaded as they lie, and a headless example writing PNG.

## Evidence
- Commits: 9ddf53170b6d5787a5464b6877e78725261b70da
- Tests: cargo test --release --workspace; cargo test --release -p telperion-render
