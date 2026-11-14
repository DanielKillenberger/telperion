# The drawing draws the leaf the tree grew

Child of fn-351 (P15's leaf; Phases 4 and 5: "the drawn leaf ... rebuilt from the grower's counts", fn-354's LEAF-ACCOUNT.md). Host-written, 14 Nov 2026. Builds on `fn-364-a-tip-extends-by-what-its-balance-leaves`.

## Problem

Since fn-354 one definition of leaf serves the light and the wood: `girth::leaf(p, y) = leaf_area x scale x size x clamp(leaf_years - age, 0, 1)` while the phytomer's wood is open. The drawing does not read it. It counts stations from a bearing run's length and spacing and draws each blade at the canopy's size, so the drawn tree carries a different leaf from the one that shaded it and sized its trunk. Recorded on seed 1001 (`fn-354-…/LEAF-ACCOUNT.md`): the beech at 40 draws blades of 33.7 cm² where its leaf is 15.7 (2.14 times) and 1.43 times as many; the oak's blade is 1.44 times its leaf and its count 0.52 at 40 and 0.98 at 120; the birch's count is 3.03 times; the spruce draws 3.1 needles a centimetre, each about four times the grower's, where the grower has 24.5. Against measurements (`PHASE4-SURVEY.md`, 2a): the beech's drawn leaf over its own crown projection is 10.0 to 12.6 where Bartelink 1997's trees carry 3.7 to 4.0 (stand trees; no open-grown figure found), and its drawn 326 m² at 40 stands on a trunk whose sapwood row accounts for 128 m².

## The design (host)

One leaf, read three times: by the light, by the wood, by the drawing.

- **Area is conserved per bearing run.** For every run of wood the leaf plan walks, the blade area drawn on it equals the sum of `girth::leaf` over the phytomers of that run in the drawn year, to within one blade. The station count of a run is that sum over the area of one drawn blade, the remainder carried along the axis to the next run (a carried remainder, never a per-run rounding up), so the tree's total is exact to within one blade an axis. Where stations stand along the run follows the plan's existing spacing rule with that count; a short shoot's cluster and a frond's rosette take their count the same way from their own phytomers' leaf.
- **One blade is one leaf.** The area of a drawn broadleaf blade is the species' `leaf_area` (with the supernatural canopy's ratio where that group already multiplies the grower's leaf, fn-354), so a drawn leaf count is a leaf count. R1 says which rows scale the blade today (`/canopy/size`, the element's own area, any per-preset factor) and which of them become derived from `leaf_area`; a row that only restated the blade's size goes.
- **Needles keep a drawing grain, stated once.** A conifer's drawn element may stand for several needles (today each drawn needle is about four of the grower's): the element's area is what it is, the count follows from the area by the same rule, and the ratio is a drawing row with its reason (cost), not a species trait. R1 reports what sets it today.
- **Nothing else moves.** Where leaves may stand (the bearing bound, clustering, which wood bears), the leaf's levels of detail, colour and season, flowers, the field's reading of station counts (it reads the new counts), and the grower are untouched. fn-364's handing of a stretched span's leaf to the node it merges into carries the leaf's area, not a length: R1 says how that code changes or goes.
- The `leaf_years` gate the drawing applies today becomes the one inside `girth::leaf`; no second gate.

Expected on paper, from the recorded ratios (drawn area over the grower's): the beech's drawn leaf falls to about a third at 40 (1/(2.14 x 1.43)); the oak's falls by 0.75 at 40 and 0.71 at 120 in area while its count at 40 about doubles; the birch's falls to about 0.38 (its blade is already near its leaf); the spruce's drawn needle area moves to the grower's (ratio today: 3.1 x 4 / 24.5 = 0.51, so it about doubles in count at the same grain); the Two Trees follow the beech with their own blade. The beech's leaf over its own crown projection lands near 3.3 to 4.1. The builder returns if any tree's drawn area ends more than a tenth from its grower's leaf, which would mean a term was missed.

## Acceptance Criteria

- **R1.** The reading and the measurement first, on this base, no product change: for every shipped preset at its preset age on seeds 1001 and 1007 (the oak also at 120): the grower's settled leaf in m², the drawn leaf in m² (stations times the blade's area as drawn), leafed phytomers, stations, one blade's drawn area against `leaf_area`; and from the code, every row and constant that sets a blade's size and a station's count, the walk that counts stations, clusters and rosettes, what the field and the packed leaf read, and what fn-364's station handing does to each. Return with it if the ratios are far from the ones quoted above (the record is from 11 Nov, before fn-355 to fn-364).
- **R2.** Tests, red before and green after: on a hand-built axis the blade area drawn on each run equals its phytomers' leaf to within one blade and the tree's total to within one blade an axis; a run whose leaf is under one blade draws none and hands its remainder on; at `leaf_years` passed a phytomer draws nothing; the count moves by one blade at a time as a leaf's area moves (by degree, the walk's rule).
- **R3.** Every shipped preset draws within 2% of its grower's leaf, by a test that reads both, on its preset seed.
- **R4.** Pins and bounds by their own procedures, each listed with its tree; the leaf plan's determinism and identity tests (`cluster_tests`, `a_node_keeps_its_identity_across_ages`, the packed leaf's) by name. A bound is never loosened; one that held the old drawn leaf as a number is listed with old and new.
- **R5.** Measured before and after: leaf instances, drawn leaf m², the foliage stage's seconds and bytes, per preset.
- **R6.** One sheet per species for the host's verdict: base (`base-2026-11-11`), this base, this branch; whole and a mid view; every preset at its preset age on seeds 1001 and 1007, the oak also at 120 and 300. The beech's leaf over its crown projection beside Bartelink's 3.7 to 4.0 and the stand range 5.5 to 7.2 under each tile.
- **R7.** `docs/pipeline.md` states the one leaf; the parameter reference follows any row removed, by its generator.

## Stops

- A preset whose drawn area cannot be brought within a tenth of its grower's leaf by the rule as written.
- The rule needing a second leaf definition, or the grower changed.
- A blade-size row the owner set by a verdict (the Two Trees' canopy size, any preset comment that names an owner's verdict): list it and return before removing or deriving it.
- A tree that reads as bare on the first stills: return with the sheet; whether the grower's leaf is then too little is the host's question (Phase 4), not the drawing's.

## Boundaries

The leaf plan's count and the blade's size, their rows, their tests and documents. No change to the grower, the wood, the light, where leaves may stand, or the renderer beyond reading the blade's size.

## Rigor

Opus builds; narrow tests while building; heavy runs through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8`, asking for the memory a job needs, one GPU job at a time, in the background, checked by the clock. The host views the sheets and gives the verdict.
