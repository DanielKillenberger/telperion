---
satisfies: [R3]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.5 The curve's sweep and clusters on the device

## Description
The contract's probe, second part: F3's curve points, runs, clusters, bounds, radius extrema and error ladders for fine roots built on the device, compact rings read from the curve as task 2 made them.

## Acceptance
- Device curve equal to the CPU reference's within the screen, run order as D5.
- Two-part gate green.

## Done summary
The fine roots' F3 curve is built on the device (sweep.wgsl): leaders as paths.rs picks them, ties broken in the conversion's node order; runs from every seam; a bitonic sort widest first (D5); samples, transported frames, clusters and ladders. Beech and spruce at 40 and 80, seeds 1 and 7: run counts equal the reference's fine runs, at most one unmatched (D1''s residual); centres within 49 um (RMS 5.3 um); frames within 0.008 rad outside 4 to 40 runs a tree whose frame seed flips; 12 to 13 runs at the spruce at 80 have a point over 50 um. One beech codominance tie of f64 rounding is for the host. Every fine node drawn from wood above the cut is now a seam (D4 completed); no curve moved at 80. Spruce at 80: 61 to 85 ms on the device. Gate: CPU half 988 passed, render half 159 passed.

## Evidence
- Commits: 7d14154a, fca85e39
- Tests: cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render swept_at_40_and_80 -- --ignored --test-threads=1
- PRs: