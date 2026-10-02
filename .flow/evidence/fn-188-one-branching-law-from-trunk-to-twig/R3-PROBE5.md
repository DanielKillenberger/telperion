# fn-188 R3, round 5: leaf-pipe radii without a floor, moderate straightness, 2026-10-03

## Setup

- **Code:** round 4's probe (`fff4da72`), changed, committed as `e04db211` and reverted in `890a5202`.
- **Runs:** release on the Ryzen 9 5950X, 16 threads, with the exact speed optimisations.
- **Tree:** round 3's settings: G2 scaffold, exponent 2.9, root 0.512 m, space colonization (density 0.05, perception 5, occupancy 2 units).
- **Canopy:** the beech's shipped canopy rows (short shoots every 0.03 m on wood under 0.7 of the trunk, 8 leaves each).
- **Foliage:** the pipeline's own wood and foliage through `executor::expand(tree, family).mesh()`, as in round 4.
- **Rows:** `raw/probe5/`: `grid.jsonl`, `timing.jsonl`, the overlays and `photo_cov.py`.

## Radii: leaf pipes, no scaffold floor

**The pipe model (Shinozaki 1964, Pałubicki §4.5):** each node's demand is its own pipes plus its children's, and r = k · demand^(1/2.9). k is set so the root stays 0.512 m. Scaffold wood no longer keeps its own radius.

**The weights come from the beech's own rows:** one pipe per leaf the pipeline places on that wood.

- **Branch and scaffold wood:** length ÷ `shortShootSpacing` (0.03 m) short shoots, × `shortShootLeaves` (8). These are the short shoots the foliage stage stands on non-twig wood.
- **Twig wood** (a grown node on an unbranched terminal shoot, which the pipeline's foliage treats as twig): length ÷ the twig row's `internodeLength` (0.05 m) leaves.
- **None on the stem axis**, and at least one pipe at any tip.
- On presets without short shoots, only twig leaves count.

**Beech s1 at round 3's straightness (crookedness 10, ξ 0.3, η 0.15):**

| radii | one pipe k | short shoot (8 pipes) | grown tip, median | strong | o1 / o2 / o3 | mid / end | o3/o1 |
|---|--:|--:|--:|--:|---|---|--:|
| round 3: tip = 1 pipe, floor | 21.3 mm | – | 21.3 mm | 14 | 0.255 / 0.308 / 0.553 | 0.787 / 1.00 | 0.137 |
| tip = 1 pipe, no floor | 21.3 mm | – | 21.3 mm | 28 | 0.514 / 0.364 / 0.514 | 0.787 / 1.00 | 0.137 |
| leaf pipes, floor | 3.76 mm | 7.7 mm | 6.55 mm | 14 | 0.257 / 0.286 / 0.527 | 0.536 / 0.87 | 0.093 |
| **leaf pipes, no floor** | 3.76 mm | 7.7 mm | **6.55 mm** | **26** | 0.520 / 0.366 / 0.482 | 0.544 / 0.87 | 0.100 |

- **Grown tips land at 6.5 mm** (6.5 to 6.8 mm across the grid) with exponent 2.9: a 0.25 m terminal shoot carries about 5 leaf pipes.
- **2 to 4 mm at exponent 2.9 needs millions of pipes:** (0.512 ÷ r)^2.9 gives about 1.3M at 4 mm, 3.0M at 3 mm and 8.8M at 2 mm. The pipeline places 1.1M leaves on this tree.
- **Removing the floor doubles the strong limbs** (14 → 26 to 28). First-order laterals go to 0.51 to 0.52 of their parent: the scaffold laterals keep the pipe's ratio instead of their own thinner radii.
- **The scaffold limbs now taper into their fine wood.** The blunt stubs of round 4 are gone in the thumbnails.

## Straightness grid (bare thumbnails, pinned pose)

All rows use leaf pipes and no floor; crookedness is the scaffold's, ξ and η the probe's.

| name | tortuosity o0 / o1 / o2 / o3 | projected limbs | angle | strong | nodes | leaves | tree / leaf cover | width ÷ height |
|---|---|--:|---|--:|--:|--:|---|--:|
| c3-x0.2-e0 | 1.031 / 1.003 / 1.002 / 1.004 | 1.0027 | 41±4.0 | 25 | 49,480 | 1,075,979 | 0.831 / 0.722 | 0.827 |
| **c3-x0.2-e0.1** | 1.030 / 1.002 / 1.001 / 1.006 | 1.0027 | 40±4.3 | 25 | 52,383 | 1,129,864 | 0.831 / 0.731 | 0.834 |
| c3-x0.4-e0.1 | 1.030 / 1.003 / 1.003 / 1.010 | 1.0027 | 41±5.3 | 26 | 48,487 | 1,024,185 | 0.820 / 0.711 | 0.810 |
| c6-x0.2-e0 | 1.037 / 1.005 / 1.005 / 1.004 | 1.0048 | 41±3.8 | 28 | 50,070 | 1,099,419 | 0.835 / 0.733 | 0.829 |
| **c6-x0.2-e0.1** | 1.036 / 1.005 / 1.005 / 1.006 | 1.0048 | 40±4.1 | 27 | 52,550 | 1,167,304 | 0.836 / 0.739 | 0.837 |
| c6-x0.4-e0.1 | 1.037 / 1.007 / 1.006 / 1.010 | 1.0050 | 42±5.2 | 27 | 48,754 | 1,055,770 | 0.825 / 0.720 | 0.836 |
| c10-x0.3-e0.15 (round 3 straightness) | 1.047 / 1.014 / 1.010 / 1.010 | 1.0090 | 39±5.1 | 26 | 51,781 | 1,147,225 | 0.830 / 0.735 | 0.827 |

**How the best two were picked:**

- I viewed one contact sheet: the seven thumbnails cropped beside B-BARE's crown.
- At 280 px the seven read alike: a clear trunk, smoothly tapering upright limbs, a visible twig mesh, and an oval crown.
- **Judged on two points:**
  - gentle curvature in the main limbs without kinks: c6 shows it, c3 a little less;
  - fine wood that rises rather than wanders: ξ 0.2 with η 0.1 rises, while ξ 0.4 wanders more.
- **Picked:** c6-x0.2-e0.1 first, c3-x0.2-e0.1 second.
- No number decided it, and the differences at this size are small.

## Fine haze, leaves, coverage, crown shape

**The pipeline adds no wood finer than the probe's tips:**

- Its short shoots are placements on existing wood, with no node and no run (`foliage/short_shoots.rs`, header). It skips twig wood and wood thicker than the radius row.
- Its leaf element carries an 8 mm stalk (`connectorLength`).
- So the finest drawn wood is the probe's terminal shoots, at 6.5 mm.

**Coverage**, round 4's method:

- **Silhouette:** per row, the span between the outermost tree pixels, from the tree's top down to 0.8 of its height.
- **Render:** tree pixels differ from an empty-scene render and are not uniform ground shadow; leaf pixels differ from the bare render.
- **B-WHOLE:** sky by colour; the right tree spans x 590–1180, with its base at y 800.

| | leaves | tree cover | leaf cover | width ÷ height |
|---|--:|--:|--:|--:|
| B-WHOLE right / left tree | – | 0.876–0.918 / 0.849 | – | ≥ 0.739 / ≥ 0.655 |
| shipped beech preset (round 4) | 5,828,744 | 0.847 | 0.820 | – |
| c6-x0.2-e0.1 | 1,167,304 | 0.836 | 0.739 | 0.837 |
| c3-x0.2-e0.1 | 1,129,864 | 0.831 | 0.731 | 0.834 |

- **The photo ratios are lower bounds.** On both photo trees the widest row reaches the photograph's edge: 590 px of a 590 px crop, and 515 of 515. The right tree's top also touches the frame (top row 2).
- **The ratio is the widest span ÷ (base row − top row)**, computed the same way on render and photo. Perspective differs between them.
- **The render's crown outline follows the envelope** (spread 0.36), because markers fill only the envelope. The whole still is a dense oval.

## Timing, seed 1: probe plus pipeline foliage against the shipped build

- **Rows:** leaf pipes, no floor, ξ 0.2, η 0.1. The beech uses crookedness 6 on G2 with the shipped canopy rows; every other preset uses its shipped values.
- **Probe:** marker fill and growth, warm median of 3.
- **Pipeline mesh:** `executor::expand(..).mesh()` on the probe's tree (wood sweep and foliage), warm median of 2.
- **Shipped:** `mesh::build` of the same family, warm median of 3.
- **Not included:** the scaffold build, 2.5 to 14 ms per preset in R3-SPEED.

| preset | probe nodes | probe leaves | shipped leaves | probe ms (cold) | pipeline mesh ms (cold) | probe + mesh | shipped `mesh::build` | grown tip | strong |
|---|--:|--:|--:|---|---|--:|--:|--:|--:|
| ordinary | 30,320 | 125,148 | 40,659 | 34.2 (37.8) | 341.1 (339.0) | 375 | 101 | 3.85 mm | 4 |
| oak | 113,015 | 635,555 | 767,026 | 140.9 (142.4) | 275.8 (270.5) | 417 | 346 | 1.67 mm | 3 |
| spruce | 31,649 | 1,217,284 | 7,414,510 | 22.5 (24.0) | 812.7 (784.4) | 835 | 4,650 | 2.29 mm | 10 |
| birch | 41,131 | 53,231 | 258,111 | 45.9 (50.1) | 621.2 (600.5) | 667 | 2,692 | 1.59 mm | 6 |
| date palm | 258 | 6,554 | 6,554 | 2.4 (2.9) | 2.6 (2.6) | 5 | 4 | 32.6 mm | 8 |
| telperion | 12,629 | 330,076 | 550,030 | 16.9 (17.8) | 898.7 (894.0) | 916 | 881 | 122 mm | 1 |
| laurelin | 133,740 | 3,508,280 | 392,944 | 166.1 (173.6) | 6,956 (6,939) | 7,122 | 367 | 113 mm | 6 |
| **beech (c6-x0.2-e0.1)** | 52,550 | 1,167,304 | 2,519,119 | 56.0 (62.0) | 588.7 (585.0) | **645** | **1,341** | 6.5 mm | 27 |

- **The pipeline's wood sweep and foliage dominate** wherever leaves are many.
  - Laurelin's probe tree takes 3.5M leaves (9x shipped) and 7.0 s of meshing.
  - The beech's 1.17M leaves take 589 ms, against 1,341 ms for the shipped G2 build's 2.52M.
- **The probe's own growth is 2 to 166 ms.**
- **Twig and short-shoot rows give few pipes on telperion and laurelin against their 7.3 to 7.4 m roots,** so their tips stay at 11 to 12 cm.

## Stills

- **Pinned pose, best two:**
  - `beech-probe5-c6-x0.2-e0.1-bare.png`
  - `beech-probe5-c6-x0.2-e0.1-whole.png` (viewed)
  - `beech-probe5-c3-x0.2-e0.1-bare.png`
  - `beech-probe5-c3-x0.2-e0.1-whole.png`
- **Thumbnails (480 x 360):** `beech-probe5-thumb-<name>.png` for all seven rows.
- **Viewed:** the contact sheet of thumbnails beside B-BARE, and the c6 whole still.
- **The c6 whole still** shows a dense green oval with the wood visible through the leaves and the trunk below the crown. Its outline is the envelope's oval, not B-WHOLE's broad irregular masses.
