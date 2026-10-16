---
satisfies: [R3]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.6 Stations and contact rings on the device

## Description
The contract's probe, third part: leaf stations and contact rings for fine roots on the device, counts from record data (D2), seated leaves against the CPU reference.

## Acceptance
- Seated leaves within fn-125's bound of the CPU reference; counts equal.
- Two-part gate green.

## Done summary
Stations and contact rings for the fine roots built on the device, and their seated leaves placed there. Both are matched to the CPU reference run by run and leaf by leaf.

- **Counts** come from grown length, summed in whole micrometres on both executors (host decision (a)). The CPU's `bearing_runs` now starts a run at every seam; before, it dropped seams that continue their bearer's branch.
- **Placement:** the device writes the fine block's ring words, the positions pass's own `emit` lays them, and one placement pass (`Generator::place_leaves`) serves the resident path and the fine block.
- **Result:** every leaf of eight trees (beech and spruce at 40 and 80, seeds 1 and 7) is within fn-125's bound as the host set it (0.005 + 50 µm/chord), or in a named class:
  - code step;
  - face turned about a matching axis;
  - `SHORTEST`, which explains the spruce 40 s1 frame divergence;
  - half-micrometre rounding;
  - contact grazing.
- **Counts are equal** on every tree except for the rounding class on spruce 80 s1.
- **Gate:** render 161 passed, CPU 988 passed.

## Evidence
- Commits: 023d3d19, bfc5ee20, e17ed84d, 809fdc47
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render
- PRs: