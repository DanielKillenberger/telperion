---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-200-tree-space-branches-sag-under-their.1 Branches sag under their load and rest on the ground

## Description
Build the spec's design as written: form.sag per PA (geometry.rs, after presences), beam curvature = presence-weighted carried moment / r^4, tip keeps its tropism, ground support for bent wood, base-below-ground still an error. Tests in crates/telperion-space/tests (form.rs, walk/mod.rs settings). R5 strip from the fn-194 branch rebased onto this one.

## Acceptance
R1 to R5 of the spec.

## Done summary
Branches sag under their load and rest on the ground, as the host designed it (crates/telperion-space/src/sag.rs, geometry.rs).

- R1: `form.sag` per PA, neutral 0. With every sag at 0 the second lay never runs, so the beech's stills are byte-identical (evidence raw/beech2). Walked on every PA in radians of bend (tests/walk/bend.rs); worst slopes 0.30, 0.10, 0.07. The twig range stops at 3e-4 because a steep but converging region near straight down sits above it (RESULT.md).
- R2: each phytomer bends by the moment at its far end, over r^4, carried into the bent frame, so an unloaded tip keeps its tropism (tests/sag.rs).
- R3: lateral wood rests on the ground and runs along it. The trunk reaching the ground is still BelowGround, which now names the axis's PA, birth and base.
- R4: oracle, walks and crate tests pass; workspace gate 1025 passed, 0 failed, 21 ignored.
- R5: the spruce walking its branch sag from 0 to 2e-4 (raw/r5b/strip.png), viewed: droop by degree, a skirt on the ground, swept-down bases with upturned tips.

Review: Codex NEEDS_WORK (2 P1, both fixed in 1976400b), then SHIP.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 75ca8ac6, 1976400b
- Tests: cargo test --profile ci -p telperion-space --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs: