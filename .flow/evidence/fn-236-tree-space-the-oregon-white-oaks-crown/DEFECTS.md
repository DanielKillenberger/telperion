# fn-236: the owner's oak defects of 2026-10-14, reproduced and traced

The preset at `5a7665aa5`, age 40, seeds 1 (the harness's default), 7 and 42. Measures come from `oak_defects.rs` (run by `defects.sh`) and fn-248's `twig_layer` example. Raw output and stills are in the ignored `raw/defects*/`. No value has moved.

## (a) Leaves along the older limbs

| Seed | Leaves | On wood of age 0 | 1 to 3 | 4 and more | On wood thicker than 5 mm radius |
|---|---|---|---|---|---|
| 1 | 342,878 | 61,916 (18 %) | 168,428 | 112,319 (33 %) | 88,765 |
| 7 | 523,164 | 92,774 (18 %) | 251,193 | 178,739 (34 %) | 157,936 |
| 42 | 350,269 | 62,172 (18 %) | 167,465 | 120,413 (34 %) | 94,181 |

At seed 1, leaves sit on wood up to 32 mm in radius. The bearing limit is `shootRadius` 0.1 of the root's radius, which comes to 29 mm. Along that wood the canopy places one leaf every 6.9 mm, and at `/canopy/divergence` 180 they stand two-ranked, which reads as a comb.

**Cause, checked against code.** The oak states no `leafYears`, so `oak.rs:91` leaves `leaf_years: f64::INFINITY`. `tree::retention` then returns `u32::MAX`, `space.rs:343` turns `leafing_on` off, and every station on slender wood of any age keeps its leaf. `plan.rs` `bearing_of` bears on any node whose girth is at most `stem_radius × shoot_radius`. The beech states `leafYears = 1` on every age (fn-248, `0a28bbc58`) and `shootRadius = 0.018`.

**Counterfactual.** With `leafYears = 1` on all ten ages, the comb is gone from the limb still (`raw/defects-ly1/oak-40-1-limb-whole.png` against `raw/defects-r0/`). Leaves then stand at the shoot tips, and the crown opens.

## (b) Too many leaves

The leaves' one-sided area over the ground hull of their seats is 60 / 67 / 50 at seeds 1 / 7 / 42. That is 3,284 / 5,009 / 3,351 m² of leaf, at a mean of 96 cm² a leaf.

Sources, each checked against the text saved in `raw/lit/`:

- **Q. garryana**, Phillips et al. 2003 (*Funct. Ecol.* 17: 832), Table 1, Corvallis oak groves: "Oak leaf area index (m2 m−2) 3·3 ± 0·10 / 3·2 ± 0·24" for 250-year and 40-year plots. The two plots also give DBH "0·56 / 0·23" m, density "140 / 1200" n ha−1, and "Oak leaf area: sapwood area (m2 cm−2) 0·37 / 0·23". Computed from these, that is about 236 m² and 27 m² of leaf per tree. Both plots are stand-grown, not open-grown.
- **Q. douglasii**, Karlik & McKay 2002 (PSW-GTR-184), 14 harvested savanna trees aged 70 to 180: "The LA for these trees ranged from 11.1-177 m2" and "The mean per-tree value for LAI was 4.4 m2 m-2". The per-tree LAI ranges from 2.5 to 7.7.
- No whole-tree leaf count or leaf area exists in these sources for an individual open-grown Q. garryana.

| Rows changed (probe only) | Leaves, seeds 1 / 7 / 42 | Leaf over hull |
|---|---|---|
| today | 342,878 / 523,164 / 350,269 | 60 / 67 / 50 |
| `leafYears` 1 | 61,927 / 92,809 / 62,181 | 11 / 14 / 9 |
| and `shootRadius` 0.018 | 61,044 / 89,841 / 61,068 | 11 / 13 / 9 |
| and twig `internodeLength` 0.025 | 22,043 / 32,168 / 22,079 | 3.9 / 4.8 / 3.3 |

The row changes do not touch the grown structure.

## (c) Limbs through each other, and the collar ring

**The ring.** At seed 1 the fork stands at 4.70 m. The trunk's radius is 0.199 m there, and two codominant limbs leave it with start radii 0.129 and 0.112 (weights 0.83 and 0.54), 25° off the leader.

The drawn girth of the stem's own run has no step at seed 1. `samples.rs` `girths` eases it over twice the trunk's radius. The steps at seeds 7 and 42 are 8 to 11 % a node.

The ring is the rim of each limb's shell where it crosses the trunk's shell (`raw/defects-side/oak-40-1-side-bare-270.png`). It is drawn from four sources:

- each run is an independent closed shell (`surface.rs`);
- the limb is sunk 0.0995 m into a socket at radius 0.114;
- the limb swells by `forkSwell` 1.35, to about 0.165 m at its first node, computed from `branch_run`;
- nothing joins the limb's shell to the trunk's.

With `/surface/forkSwell` at 1.0 the lip shrinks but the seam stays (`raw/defects-swell1/`). The device's `sweep.wgsl` mirrors `samples.rs` (`girth`, `swelling`, lines 337 to 359), and the owner's harness shot shows the same ring, so the CPU and the device agree.

**Crossings.** These are pairs of structural segments (radius at least 5 % of the root's) on different runs that overlap away from their common node:

| Seed | Crossing pairs | Thinner axis inside the thicker tube | Median depth over the thinner radius |
|---|---|---|---|
| 1 | 1,276 | 490 | 0.60 |
| 7 | 3,816 | 834 | 0.44 |
| 42 | 1,194 | 174 | 0.40 |

Setting `forkSwell` to 1.0 leaves these counts unchanged.

The engine sets each axis's direction from insertion, tropism, wander, straightening and sag (`telperion-space/src/geometry.rs`). Light changes how much an axis grows, not which way it points. No term reads where the other wood is, so the engine has no limb avoidance.

The stills `raw/defects-r0/oak-40-*-fork-bare-*.png` and `raw/defects-side/` show straight sprigs and boughs passing through limbs and the trunk, and limb shells overlapping for about half a metre above the fork.
