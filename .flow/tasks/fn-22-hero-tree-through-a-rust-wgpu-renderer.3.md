---
satisfies: [R3, R4]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.3 Foliage pipeline, submission contract and conformance tests

## Description
Add the foliage pipeline, the submission contract and conformance tests.

## Acceptance
- One instanced foliage draw from the core's matrices; submit reports counts and bounds and rejects oversize trees; every preset and twenty random sets render.

## Done summary
One instanced draw takes the core's placement matrices as they lie, two-sided leaf lighting, `submit` reports counts and bounds and refuses oversize trees naming buffer and sizes; three views restored; the conformance sweep covers every family and twenty generated sets.

## Evidence
- Commits: a80d6bb1ff92f2f6d9a4c3d096be2692390d30cb
- Tests: cargo test --release --workspace; cargo test --release -p telperion-render
