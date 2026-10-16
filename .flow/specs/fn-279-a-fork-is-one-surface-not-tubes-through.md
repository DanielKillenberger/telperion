# A fork is one surface, not tubes through each other

## Goal & Context

The owner (2026-10-14, the oak's fork in the harness, fn-236 `owner-2026-10-14-fork.jpg`): "branches clipping in each other", and a sharp collar ring where the trunk meets its limbs. fn-236's DEFECTS.md (2026-10-14) traced it: every run is drawn as its own closed tube; a limb's tube is sunk into its parent's (oak at 40, seed 1: 0.0995 m deep at radius 0.114, swollen by `forkSwell` 1.35 to about 0.165 m) and nothing joins it to the parent's surface, so the rim of each limb's tube shows as a ring where it cuts the trunk, and codominant limbs' tubes overlap for about half a metre above the fork. The CPU (`samples.rs`) and the device (`sweep.wgsl:337-359`) share the sample code. Every species with forks draws this way.

## Requirements

- **R1 (measure first):** the ring and the overlap measured at every shipped species' main forks (seeds 1, 7, 42): the lip (the step in surface radius across the seam) and the length of overlap above the fork; stills of each fork, viewed. Prior art in the codebase (any fork blend, `forkSwell`'s purpose, fn-66 "the ring at the birch's fork") read and stated.
- **R2 (one surface):** where a child leaves its parent, the drawn surface is continuous: no rim, no interpenetrating shells visible from outside, a smooth branch collar of the size the sources give (the swelling at a branch base is real: its size sourced per species or stated as a stand-in). By degree for every fork angle and radius ratio, one rule for codominant and lateral forks, no species branch; CPU and device agree.
- **R3:** stills of every species' forks before and after, viewed; cost measured (vertices, build time); the shipped trees' other geometry unchanged; gate, npm, review; the owner's verdict.

## Boundaries

Drawing only; where limbs grow is fn-280. Whether this is a release item is the owner's call (2026-10-14).
