---
satisfies: [R4, R7]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.5 The blend, the sweep, and the transition on the headless target

## Description
Add the family blend and the every-pair sweep test in the core, and teach the headless example to render a transition between two presets as a numbered frame sequence with a best-effort video (R4 mechanism, R7 no-switch-frame proof).

## Acceptance
- Blend at 0 and 1 returns the inputs exactly; a blend of two valid families validates
- Every preset pair in ten steps generates with finite positions; leaf counts in band
- Headless `--to`/`--frames` writes a numbered PNG sequence and record; video when an encoder exists

## Done summary
`blend::families(&a, &b, t)` interpolates every numeric parameter (angles along the shorter arc, lobes rounded down, sections up). `tests/sweep.rs`: five tests, 6.2 s, every pair in ten steps under an 8,000-node cap, and `the_oak_to_spruce_walk_has_no_switch_frame`. Headless gained `--to` and `--frames` (240 default, 24 fps); both encoder branches proved by running.

## Evidence
- Commits: 288d640
- Tests: worker baseline: cargo test --release --workspace green in 213 s before any edit, worker: cargo test --release --workspace - 30 binaries green in 234 s, ...
- PRs:
