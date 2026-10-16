---
satisfies: [R5, R7]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.4 GPU timing session and native evidence

## Description
Add the GPU timing session and native evidence.

## Acceptance
- Conditioning, warmup and measured frames with a four-way validity verdict; oak and spruce native records and stills.

## Done summary
`timing.rs`: room and vegetation in two passes so timestamps measure the tree; 8 conditioning, 8 warmup, 120 measured frames into p50/p95 behind Valid/Unavailable/Disjoint/Contended (invalid sessions carry no number). `headless --timing` writes the record; oak and spruce stills and records in `.flow/evidence/fn22/`.

## Evidence
- Commits: 959d01d2e67045895e4c3ec5ebf55af61641e28d
- Tests: cargo test --release --workspace
