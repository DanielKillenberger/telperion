---
satisfies: [R1, R5]
---
# fn-23-fast-hero-the-oak-inside-the-frame.3 Selection on the GPU: one compute pass, one indirect draw per level, a bucket for the unseen

## Description
Selection on the GPU: one compute pass, one indirect draw per level, a bucket for the unseen.

## Acceptance
- Per-instance level selection on the GPU; oak hero within budget.

## Done summary
The ladder was thinned to useful levels and selection moved to one compute pass with indirect draws per level: oak 1.677 ms p50 native (from 18 ms), 2.196 ms in the browser.

## Evidence
- Commits: 397e13da95f10e9e69ea508cb25166faf0d4597e, 407fc921a0cab82a5be81080631cb4ee3527caad
- Tests: cargo test --release --workspace
