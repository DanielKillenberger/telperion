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

Follow-up from the spruce's round 3 (host, 2026-10-04), 583a391b, 1f491cfb, dd5cc47d, 36e884df:
- Wood lands on the ground tangent: a contact zone of four radii plus six internodes, descent eased with the square root of height; the rest clamps height only.
- Sag turns about the gravity torque carried by the whole-frame rotation to the bent frame, levelled, limited to the room left to straight down less 0.15 rad. Wood past straight down is never lifted, and a vertical trunk bends towards its load.
- Tropism weakens to nothing within 0.05 rad of straight down (gravitropism's sine law), in place of the frame blend the host first designed. Accepted by the host as host decision 2: no continuous side field exists on the sphere (the hairy-ball limit).
- Spec amended, host decision 3: R1 now reads "form.sag at 0 adds no bending"; R4 now reads "the beech unchanged in look, pixel diff recorded" (43 of 2.8M pixels). Wood standing exactly vertical when it reaches the ground keeps a measure-zero side singularity, accepted and kept out of the walks.
- Workspace gate on 36e884df: 1030 passed, 0 failed, 21 ignored.

stage: impl-review - OVERRIDDEN (host, 2026-10-04) by host decisions 2 and 3. Codex follow-up review, base d27b6ef2:
- Round 1, NEEDS_WORK, three P1s. (1) A vertical trunk never sagged: fixed in dd5cc47d, with test a_vertical_trunk_bends_under_a_one_sided_crown. (3) Landing normalised a vanishing lean above the ground: fixed in dd5cc47d, with test a_lateral_swept_through_straight_down_lands_by_degree.
- Round 2: (1) confirmed fixed. (2) Zero-sag neutrality: resolved by the amended R1 and R4 (host decision 3). (3) at the ground, the vertical-wood singularity: accepted (host decision 3).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 75ca8ac6, 1976400b, 583a391b, 1f491cfb, dd5cc47d, 36e884df
- Tests: cargo test --profile ci -p telperion-space --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs: