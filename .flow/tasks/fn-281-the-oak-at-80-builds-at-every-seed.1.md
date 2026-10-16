---
satisfies: [R1, R2]
---
# fn-281-the-oak-at-80-builds-at-every-seed.1 Repro, sweep, cause

## Description
R1 and R2; stop and report the cause to the host.

## Acceptance
R1 red test and sweep; R2 cause checked in code.

## Done summary
R1: the oak at 80, seed 7 red on "unsolved radii" in `radii_tests::the_oak_at_80_seed_7_solves_every_radius`; the device builds it (no `validate_solved`). Sweep (SWEEP.md): unsolved radii only in the oak at 60 (seed 25), 70 (25, 34), 80 (3, 7, 30, 39); engine-budget refusals at 80, 90, 100. R2: an f32-absorbed relay join gives `above` 0 and the conversion draws the head at radius 0.

## Evidence
- Commits: c6c2a14d5, e75a23b4f, 5d3447891
- Tests: target/ci/deps/telperion_core-* radii_tests --ignored (red, as R1 asks), .flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/sweep.sh, every_tree_builds_on_the_device with SWEEP_ONLY='oregon-white-oak 80 7' (builds)
- PRs: