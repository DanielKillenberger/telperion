# Tree space D: the Norway spruce as a point

## Goal

Phase D of the tree-space programme (`docs/tree-space.md`; owner, 2026-10-03): the Norway spruce (*Picea abies*, Massart's model) as a point in the tree space after the beech, values only, from fn-190's MODEL-SPRUCE.md and its sources. Gate as in phase C (host confident, Astra agreeing, traits named); wrong-path stop as phase C, or the beech regresses.

## Decisions

- Values in `crates/telperion-space/src/spruce.rs`; sources in SOURCES.md (needle retention from Muukkonen and Lehtonen 2004). Drawn by `crates/telperion-render/examples/space_spruce.rs` through the shared still runner.
- Host answers to round 1 (2026-10-04): ground and droop built as fn-200 (`form.sag`, ground support); dormant buds later (built as fn-202); comb habit as S1 shows; colours moved into the norway-spruce preset; more seed variety, a thinner young trunk, a crown widened toward S1's 0.75.
- Engine work the spruce needed went into its own specs: fn-200 (sag, ground), fn-201 (foliage dispatch past 65,535), fn-202 (dormant buds), fn-203 (bent-lever sag), fn-205 (secondary girth, the spruce at `secondary: 1.0`).
- Up to about 9 s for an 80-year tree accepted; phase F owns cost (host, after round 3).
- Host and Astra iterated seven gates (rounds 4 to 10) without agreeing (Astra: insufficient foliage mass on the hanging branchlets), so the spruce went to the owner. Round 11 was tried and reverted; round 10 kept.
- R3 split to task 2, blocked by fn-206: the beech's and spruce's physiological-age chains differ, so they have no per-setting midpoint until both sit on one reference axis.

## Acceptance Criteria

- **R1:** The spruce's values from MODEL-SPRUCE.md and its sources, at several ages and mature, seeds 1 and 7, beside the references and today's spruce — met (rounds 1 to 11).
- **R2:** The gate as in phase C — met on the owner's verdict, round 10.
- **R3:** A beech-to-spruce walk at a fixed seed as still strips, no visible pop — met (task 2, on fn-206's shared chain, PR #154).
- **R4:** The beech's passed sheet re-rendered, unchanged in look — met (shown on fn-196's branch, `38c2de33`; only lighting and leaf placement differ).

## Result

Owner verdict (2026-10-05): "spruce looks good but not as lush and big as the reference. but definitely acceptable. Still much to be improved but i think it's structurally sound." Passed with round 10's values: the bough hook on fn-203's bent-lever sag, sleeping-bud curtains, dense sprays, a broad base. R3 walk (age 40, seed 1): by degree from the forked beech through open single-stemmed and tiered midpoints to the spruce spire, leader kept throughout, no pop. Gate 1044 passed; Codex SHIP (PR #149).

## Open

- The hanging curtains' volume: bounded by about 210M wood triangles and 16M needles per 80-year tree, where the GPU runs out of memory. Phase F levers: cheap fine-twig drawing, shedding before growth.
- Dormant-bud draperies are values-limited by the same budget (sleeping probability stops near 0.45).
- The 10-year sapling reads sparse.
- The spruce stills report height and width but not dbh (Codex P2, recorded).

## Friction

- `BelowGround` named only the axis index -> done in fn-200 (names PA, birth cycle, base height).
- wgpu panicked on a dispatch over 65,535 workgroups -> fixed by fn-201.
- GPU out of memory on 80-year trees, varying with other desktop apps -> proposed: renderer memory estimate refusing by name before allocating; a level of detail for sub-millimetre twigs (the limit is wood triangles, not needles).

## Evidence

- `.flow/evidence/fn-194-tree-space-d-the-norway-spruce-as-a/SOURCES.md` (values and sources; cited by `spruce.rs`)
