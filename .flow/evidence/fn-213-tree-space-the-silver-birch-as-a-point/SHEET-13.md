# fn-213 sheet 13: round 13, more short shoots, and the crown's colour at distance (worker, 2026-10-08)

The host's reading of round 12: close up, the clusters work; from a distance the crown is still see-through and reads grey-olive against S2-Oslo and the old birch. Round 13 changes values only, on top of round 12 (`65b43cb0`).

## What moved

| Row | 12 | 13 |
|---|--:|--:|
| `BRANCH` (hanging strand) medial lateral `SHORT`, `/space/ages/9/zones/1/lateral/14` | 0.5 | 0.845 |
| `LIMB` medial lateral `SHORT`, `/space/ages/6/zones/1/lateral/14` | 0.35 | 0.58 |
| `BOUGH` medial lateral `SHORT`, `/space/ages/7/zones/1/lateral/14` | 0.35 | 0.45 |
| `SPRIG` medial lateral `SHORT`, `/space/ages/8/zones/1/lateral/14` | 0.4 | 0.45 |

- **Rule:** each of these medial nodes now bears a shoot with probability 0.85. The long laterals (`BOUGH`, `BRANCH`) keep their shares, so the short shoot takes the rest. The bare remainder is 0.15.
- **Sources:** SU03 on *B. maximowicziana*: each long shoot bears "three to seven leaves with axillary buds", and ">80 % of all current-year shoots within each first-order branch were short shoots" (`raw/lit/pmc4257506.md`). JH87 on *B. pendula*: long shoots "bear many daughter buds". The 0.85 is **estimated** within that range; no source gives the share of nodes that bear a shoot.
- **Unchanged:** three leaves a cluster, the 0.6 rad spread, every long-lateral probability, every form row, the stem ages, and every material row.
- **Records:** MODEL-BIRCH.md, "Round 13", and the comment above `branch` in `birch.rs`.

## Measures (`birch_measure.rs`, seed 1 unless stated; raw in `raw/measure-r13.txt` beside `raw/measure-r12.txt`)

| Measure | 12 @40 | 13 @40 | 12 @80 | 13 @80 |
|---|--:|--:|--:|--:|
| Nodes | 282,041 | 393,323 | 1,862,185 | 2,567,034 |
| Leaves | 116,328 | 158,624 | 820,004 | 1,081,117 |
| Clusters (short-shoot tips) | 21,448 | 35,820 | 122,746 | 207,038 |
| Leaves in clusters | 55% | 68% | 45% | 57% |
| Leaves per short run | 3.27 | 3.28 | 3.31 | 3.32 |
| Leaves per metre of hanging strand | 14.0 | 13.0 | 14.7 | 13.7 |
| Hanging strands counted (fine runs, base radius < 5 mm) | 1,874 | 1,476 | 12,517 | 9,994 |
| Wood thicker than 30 mm radius, m | 39.1 | 56.6 | 176.1 | 303.3 |
| Height, m | 16.3 | 16.4 | 24.9 | 25.0 |

**Clusters rise** by 67% at 40 and 69% at 80. **Leaves rise** by 36% and 32%, because a cluster replaces a short shoot's leaves rather than adding to the long shoot's.

**The wood grew thicker; its topology did not change.** Every added short shoot is a pipe the engine adds to the girth below it:

- **Strands:** at 80 the 2 to 5 mm band loses about 1,900 m of wood and the 5 to 10 mm band gains about 3,000 m. That takes a fifth of the strands over the measure's 5 mm "fine" cut, so the strand count falls. The total number of runs rises by about the number of added clusters (84,785 against 84,292 at 80). No long axis was added or lost.
- **Stems and limbs:** wood over 30 mm of radius grows by 45% at 40 and 72% at 80.
- **Consequence:** this girth moved the old bark up the trunk. See "Gate" and "For the host".

## Cost (native, `--profile ci`, limits 4; `raw/sheet13/render.log`)

| Seed | Nodes @40 | Leaves @40 | Nodes @80 | Leaves @80 | Grown + dressed @80, ms |
|---|--:|--:|--:|--:|--:|
| 1 | 393,323 | 158,624 | 2,567,034 | 1,081,117 | 2,249 + 1,773 |
| 7 | 738,585 | 311,218 | 2,983,808 | 1,206,153 | 2,589 + 2,148 |
| 42 | 467,889 | 195,142 | 2,925,910 | 1,221,416 | 2,292 + 1,980 |

- **Node cap:** the largest tree is 2.98 M nodes (seed 7 at 80), 75% of the browser's 4 M cap. Round 12's was 2.17 M.
- **Leaf instances:** 1.08 to 1.22 M at 80, 30 to 35% over round 12.
- **Timings:** they ran at load 37 of 32 and are noisy, as round 12's were (SHEET-12.md, "Cost").

## Colour at distance (`crown_colour.py`; `raw/colour13/colour.txt`, `fit.txt`, masks in `raw/colour13/`)

**The method.** The rendered crown is the hero still's tree pixels: those that depart from the row's background, sampled from the frame's margins, by more than 24. The sample runs from the top of the tree down 0.7 of the way to its lowest pixel, which keeps it clear of the ground shadow. S2-Oslo's crown is round 11's box with the sky taken out. Both are converted sRGB to linear. I checked the masks by eye (`raw/colour13/masks13.png`).

| Crown, hero view | Linear mean | R:G:B | Luminance p10 / p50 / p90 | Gaps inside the outline |
|---|---|---|---|--:|
| S2-Oslo | 0.155/0.166/0.077 | 0.94:1:0.47 | 0.013 / 0.073 / 0.430 | 7% |
| Old birch, s1 / s7 / s42 | 0.163/0.204/0.123 (s1) | 0.78 to 0.80:1:0.58 to 0.62 | 0.052 / 0.157 / 0.416 (s1) | 31 to 38% |
| Round 12 @80, s1 / s7 / s42 | 0.172/0.184/0.126 (s1) | 0.91 to 0.93:1:0.68 to 0.70 | 0.034 / 0.146 / 0.410 (s1) | 31 to 38% |
| Round 13 @80, s1 / s7 / s42 | 0.168/0.181/0.126 (s1) | 0.90 to 0.93:1:0.69 to 0.71 | 0.032 / 0.141 / 0.407 (s1) | 28 to 35% |
| Round 13 @40, s1 / s7 / s42 | 0.191/0.202/0.147 (s1) | 0.91 to 0.94:1:0.69 to 0.73 | 0.027 / 0.154 / 0.434 (s1) | 46 to 50% |

**What differs from S2-Oslo.**

- **Blue:** the rendered crown's blue is 0.69 to 0.73 of its green, against Oslo's 0.47. That excess blue makes the green grey.
- **Hue:** red to green matches Oslo (0.93 against 0.94). The old birch is greener, at 0.80.
- **Shade:** the render's median luminance is about twice Oslo's (0.14 against 0.073), and its darkest tenth is 2.5 times lighter. The photographed crown is half in deep shade, and the rendered one is not.
- **Gaps:** background shows through 28 to 35% of the rendered crown's outline at 80, against 7% in Oslo. Round 13 closes about 3 points of that.
- **Exposure:** where the sunlit stem can be read (at 40), the render sits on an exposure 1.07 to 1.18 times the bark rows' and Oslo on 0.90 to 0.97. Both are within about 20% of each other, and no difference above depends on exposure.

**The leaf rows are not the cause.** I tested this by code. A probe rendered the hero stills with the leaf front, back and transmission rows halved (`raw/probe13-half/`, not committed). Per channel, the fit is rendered = gain × row + offset:

| @80, s1 / s7 / s42 | Gain R/G/B | Offset R/G/B (light with no leaf row) | Row that would match Oslo |
|---|---|---|---|
| s1 | 0.381/0.379/0.530 | 0.081/0.090/0.067 | 0.194/0.200/0.020 |
| s7 | 0.399/0.399/0.557 | 0.077/0.086/0.063 | 0.196/0.200/0.026 |
| s42 | 0.376/0.385/0.541 | 0.076/0.087/0.067 | 0.210/0.206/0.019 |

At 40 the matching blue row falls to between −0.07 and 0.01.

- **The light on the leaves is blue:** the blue gain is 1.4 times the red and the green. The renderer lights the leaves with more blue than the white sun alone would give, from sky fill, wrap or transmission. The fit cannot tell which.
- **About half the crown's colour comes from no leaf row:** the offset is a near-neutral 0.08/0.09/0.07, which is wood, sky mixed into partly covered pixels, and sheen. On its own it gives 53% of the rendered blue at 80.
- **Matching Oslo by the rows would need an implausible leaf:** a blue row of 0.02, a leaf of 1:1:0.1, as yellow as April's young leaves (S5-short-shoots, B:G 0.08). Round 11 read the summer row from Oslo's sunlit leaves as 0.227/0.241/0.111 (B:G 0.46).

So, as the brief asked, the leaf rows were not changed. The cause sits in three places:

- **The lit fraction:** the crown carries too little deep shade. The candidate rows are the canopy's `canopyNormal` 0.8, `lightWrap` 0.5, `diffuseTransmission` 1.0 and `crownShade` 0.2, or the renderer's fill.
- **The light:** the light on the leaves carries extra blue.
- **The gaps:** they let in the blue-grey sky and wood.

## The stills

| Sheet | Path | Rows and contents |
|---|---|---|
| Compare at 40 | `raw/compare/compare13-40.png` | Per seed (1, 7, 42): the old birch, round 12, round 13. Columns: bare, in leaf, crown, limb (`compare_sheet.sh 12 13`) |
| Compare at 80 | `raw/compare/compare13-80.png` | As at 40 |
| Leaf close-up | `raw/compare/leaf13.png` | S5 spray and S2-Oslo, then round 12 beside round 13 at 40 and 80, seed 1 |
| Spray and base | `raw/compare/rest13.png` | Round 12 beside round 13, every seed and age |
| Round 13's own stills | `raw/sheet13/` | |

I viewed every sheet above, the hero still of round 12 seed 1 at 80, S2-Oslo, and the mask montage.

**My reading.**

- **Close-ups:** the strands at 80 are leafed more densely along their whole length, with clusters at most nodes. At 40 the strands carry visibly more clusters than round 12, though they remain lighter than at 80. Leaf colour is unchanged: pale sage beside S5's green.
- **In-leaf and crown views:** the crown is a little denser and has more shade inside it, clearest in seed 42's crown view at both ages. From a distance it still reads see-through and grey-olive beside S2-Oslo's saturated, deeply shaded mass and the old birch's green. The values do not close the colour gap, and the measures above say why.
- **Bare views:** the fine wood reads darker and fuller, because of the added short shoots and thicker strands.
- **Base views (new, and not asked for):** round 13's thicker trunks carry the old, black, furrowed bark much higher.
  - **At 80:** it covers the whole stem in the base view for seeds 1, 7 and 42. Round 12 showed a white stem above a dark flare.
  - **At 40:** a black band now shows at the flare of seeds 1, 7 and 42.
  - **Against the earlier target:** round 5 and round 7 aimed for "the 40-year stem white above its flare, the 80-year base black to about a metre". Round 13 overshoots both.
- **Limb views:** unchanged apart from the fuller strands.

## Gate (round 13)

- **Render half first, under the GPU lock:** `cargo test --profile ci -p telperion-render --no-fail-fast` exits 101. 192 tests passed and 1 failed, in 43 binaries (`raw/gate13-render.log`).
  - **The failure:** `smooth_bark::the_smooth_barks_hold_the_resolution_and_redraw_contracts`. The birch's bark close-up aliases across resolution at a mean of 3.25/255, against a bound of 3.0, and a p95 of 12.75/255, against 12.0. The beech's reads 0.73 and 2.75.
  - **Repeat:** I reran the test alone and got identical numbers.
  - **Cause:** round 12 passed this test. The close-up (`targetHeight` 0.06, at 1.5 m) now frames the black, furrowed old bark that round 13's thicker stem brings to the 40-year flare. The cause is consistent with the base views, though I did not isolate it by a separate render.
  - **Not done:** I did not weaken the gate and did not touch the bark rows.
- **Then the workspace:** `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0, with 1,038 passed in 92 binaries (`raw/gate13-core.log`).
- **Generated files:** I ran `npm run wasm:build` with `TELPERION_WRITE_REFERENCE=1`, which regenerated `presets.generated.ts`. `npm test` passes 143 of 143 (`raw/npm13.log`).
- **Pins:** the three catalogue digests are re-pinned in `catalogue_identity.rs`.
- **Limits:** RAYON_NUM_THREADS=4, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=4.

## For the host and the owner

- **The bark (gate red).** Raising the short shoots thickened every axis below them under the pipe model, and the old bark followed the girth up the trunk. These are the options; the choice is the host's:
  - **Re-derive `/material/matureRadius` (0.30 m) and `matureSpan` (0.07 m)** against the new girth, to the round 5 and round 7 target. This is the cheapest. These are owner-reviewed rows outside this brief.
  - **Lower the short-shoot shares** until the girth and the bark come back. This gives up leaf density.
  - **Ask whether the pipe model should count a short shoot's pipe at full weight.** This is a design question.
- **The colour.** It is not in the leaf rows, by the fit above. The candidates are the canopy shading rows (`canopyNormal`, `lightWrap`, `diffuseTransmission`, `crownShade`), the renderer's fill light, and the remaining gaps. Which to move is the host's call. A probe of each is cheap: one row per render, about 30 s under the lock.
- **See-through.** Round 13's density closes the gaps by about 3 points at 80. That leaves 28 to 35% against Oslo's 7%, so the short shoots alone will not close the crown at the 4 M cap. The cap leaves about 1 M nodes of room at seed 7.
- **The 0.85 share** is estimated. SU03 gives a share of shoots, not a share of nodes.
