# fn-183 R1: the ladder measured, 2026-10-02

This report states facts only. The host picks the rung (R2).

## Method

- **Code.** The merged profiler is `bd7a2c7d`: `growth_profile` built with `--features query-count` counts each `Envelope::radius_at` against the innermost open purpose. The rungs ran on `c4950378` (`LADDER=0..4`, reverted in `f68203c3`), with `LADDER_STATS` printing excursion and density. The driver and aggregation scripts are in `ladder/` in that commit. Raw output is in `raw/` (ignored).
- **Rungs.** All rungs act on the direct build only (`!growing_envelope`). The growth path's check at `advance.rs:262` is untouched.
  - **R1, no outline question:** there is no stride check, no bisection and no terminal admission. The curtain keeps `clear` (its floor), its pendulous length and sag, and `below` on terminal twigs.
  - **R2, inherited allowance:** each first-order limb carries `A = reach·kept / share`, where `share` is the inner crown's radius over the full crown's (1 − `twigReach`). It travels in the `Bound` the orders already inherit. An axis or terminal twig gets `min(length, A − |start − limb station|)`. A stem-borne shoot has no limb and gets no cap.
  - **R3a, radial room:** `min(length, radius_toward(start) − radial(start))`. This is one query, taken through the mapped bound of a shortened system.
  - **R3b, directional probe:** the end of the authored length is tested first. If it is outside, the probe marches in quarters from the start, so an axis costs at most 4 queries. Length is `min(authored, room)`.
  - **In R2 and R3,** a capped axis's internodes scale as `ceil(n·capped/authored)`, at least `laterals + 1`. A curtain that drops is exempt from every cap and keeps its own controls.
- **Excursion.** Per node past the crossover, excursion is `max(radial − radius_toward, y − height, 0)`. For a shortened system the same is taken on `bound.map(p)` times the scale, and the larger of the two counts. It is a share of `max_radius()` (the smooth widest radius). An axis is a `branch` id, and a terminal twig joins its parent's axis. The axis value is the maximum over its nodes. Median, p95 and max are over the axes outside (> 1e-9). Nodes in a dropping curtain's band (`in_band`, tolerance 1e-9) count 0 and are counted as band nodes.
- **Density.** Density is per smooth crown volume, π∫r²dy with 256 slices. Leaves are `outputs.leaves.retained` from `pipeline::build` with `leaves: true`; placed equalled retained everywhere.
- **Timing.** The binary is built without `query-count` and timed on an AMD Ryzen 9 5950X. Per preset, seed and rung there are 3 rounds, interleaved across rungs, of 5 samples each. The cold first sample is dropped, which leaves the median of 12 warm samples. Warm ranges at R0 (ms): oak 45–54 / 60–70, beech 94–100 / 93–109, birch 180–189 / 185–194, spruce 35–42 / 32–45, telperion 376–398 / 51–55. R0 is the exploration binary at `LADDER=0`, which adds one `OnceLock` read per axis. Today's output was reproduced: the oak at seed 1 still hashes to `1e8bb52487db09f0`.
- **Counter cost.** With counters compiled in, the oak at seed 1 takes 63–66 ms against 47 ms without them.

## Today's queries by purpose (R0, seed 1; seed 7 within 5 points)

| Preset | Radius queries | Scaffold room | Scaffold containment and sampling | Twig stride | Twig bisection | Terminal admission | Curtain band | Shedding | Planned axes |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| Oak | 183,563 | 305 | 25,029 (14%) | 67,546 (37%) | 34,719 (19%) | 55,708 (30%) | 0 | 0 | 11,738 |
| Beech | 293,726 | 378 | 35,726 (12%) | 104,081 (35%) | 71,999 (25%) | 81,286 (28%) | 0 | 0 | 16,116 |
| Birch | 1,267,571 | 426 | 19,714 (2%) | 43,935 (3%) | 47,600 (4%) | 36,189 (3%) | 1,119,451 (88%) | 0 | 12,655 |
| Spruce | 259,439 | 541 | 168,078 (65%) | 36,339 (14%) | 4,920 (2%) | 49,305 (19%) | 0 | 0 | 16,292 |
| Telperion | 466,728 | 126 | 8,075 (2%) | 152,921 (33%) | 1,720 (0%) | 75,220 (16%) | 0 | 228,410 (49%) | 23,197 |

"Other" is 256 on every preset: `influence_radius`'s volume integral.

## Rungs: queries, time, nodes, leaves, density (seed 1 / seed 7)

| Preset | Rung | Radius queries | vs R0 | Twig-layer q per planned axis | Growth ms | vs R0 | Nodes | Leaves | Nodes per m³ | Leaves per m³ |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| Oak | R0 today | 184k / 198k | +0% / +0% | 13.5 / 12.0 | 47 / 62 | +0% / +0% | 125k / 139k | 715k / 869k | 16.1 / 17.9 | 92 / 112 |
| Oak | R1 none | 26k / 33k | -86% / -83% | 0.0 / 0.0 | 43 / 48 | -8% / -23% | 134k / 144k | 767k / 903k | 17.2 / 18.5 | 99 / 116 |
| Oak | R2 allowance | 26k / 33k | -86% / -83% | 0.0 / 0.0 | 34 / 48 | -27% / -23% | 130k / 141k | 746k / 882k | 16.7 / 18.2 | 96 / 113 |
| Oak | R3a radial | 96k / 117k | -48% / -41% | 5.9 / 6.1 | 41 / 56 | -12% / -10% | 129k / 143k | 729k / 885k | 16.6 / 18.4 | 94 / 114 |
| Oak | R3b probe | 99k / 119k | -46% / -40% | 6.3 / 6.3 | 41 / 56 | -13% / -11% | 127k / 142k | 733k / 885k | 16.4 / 18.2 | 94 / 114 |
| Beech | R0 today | 294k / 294k | +0% / +0% | 16.0 / 15.8 | 96 / 95 | +0% / +0% | 188k / 191k | 4929k / 4999k | 24.1 / 24.5 | 632 / 641 |
| Beech | R1 none | 36k / 37k | -88% / -87% | 0.0 / 0.0 | 70 / 68 | -27% / -29% | 213k / 215k | 5830k / 5856k | 27.3 / 27.6 | 748 / 751 |
| Beech | R2 allowance | 36k / 37k | -88% / -87% | 0.0 / 0.0 | 56 / 56 | -41% / -41% | 158k / 159k | 4109k / 4078k | 20.2 / 20.4 | 527 / 523 |
| Beech | R3a radial | 138k / 140k | -53% / -52% | 6.2 / 6.2 | 78 / 80 | -18% / -16% | 196k / 199k | 4942k / 5003k | 25.2 / 25.5 | 634 / 641 |
| Beech | R3b probe | 143k / 144k | -51% / -51% | 6.8 / 6.7 | 79 / 80 | -18% / -16% | 190k / 192k | 4916k / 4976k | 24.3 / 24.7 | 630 / 638 |
| Birch | R0 today | 1268k / 1291k | +0% / +0% | 98.6 / 94.5 | 183 / 187 | +0% / +0% | 80k / 87k | 232k / 261k | 65.7 / 71.9 | 191 / 215 |
| Birch | R1 none | 20k / 21k | -98% / -98% | 0.0 / 0.0 | 25 / 26 | -87% / -86% | 87k / 92k | 258k / 280k | 71.3 / 75.8 | 213 / 230 |
| Birch | R2 allowance | 20k / 21k | -98% / -98% | 0.0 / 0.0 | 27 / 28 | -85% / -85% | 94k / 102k | 280k / 309k | 77.7 / 84.2 | 231 / 255 |
| Birch | R3a radial | 48k / 45k | -96% / -97% | 2.1 / 1.7 | 31 / 31 | -83% / -83% | 101k / 101k | 294k / 303k | 83.3 / 83.1 | 242 / 250 |
| Birch | R3b probe | 52k / 47k | -96% / -96% | 2.5 / 1.9 | 29 / 29 | -84% / -84% | 91k / 95k | 268k / 287k | 75.2 / 78.3 | 221 / 236 |
| Spruce | R0 today | 259k / 245k | +0% / +0% | 5.6 / 5.5 | 38 / 39 | +0% / +0% | 95k / 90k | 7354k / 7012k | 292.5 / 277.4 | 22553 / 21506 |
| Spruce | R1 none | 169k / 160k | -35% / -35% | 0.0 / 0.0 | 39 / 36 | +2% / -7% | 96k / 91k | 7415k / 7065k | 294.9 / 279.3 | 22739 / 21666 |
| Spruce | R2 allowance | 169k / 160k | -35% / -35% | 0.0 / 0.0 | 41 / 39 | +7% / +1% | 96k / 91k | 7394k / 7046k | 295.6 / 280.0 | 22676 / 21609 |
| Spruce | R3a radial | 241k / 228k | -7% / -7% | 4.4 / 4.3 | 40 / 38 | +4% / -2% | 96k / 91k | 7371k / 7030k | 294.6 / 279.0 | 22605 / 21559 |
| Spruce | R3b probe | 242k / 229k | -7% / -7% | 4.5 / 4.4 | 36 / 37 | -6% / -4% | 96k / 91k | 7381k / 7036k | 294.5 / 279.0 | 22636 / 21577 |
| Telperion | R0 today | 467k / 73k | +0% / +0% | 9.9 / 11.7 | 381 / 52 | +0% / +0% | 76k / 17k | 535k / 118k | 0.4 / 0.1 | 3 / 1 |
| Telperion | R1 none | 239k / 41k | -49% / -44% | 0.0 / 0.0 | 363 / 50 | -5% / -4% | 77k / 18k | 550k / 129k | 0.4 / 0.1 | 3 / 1 |
| Telperion | R2 allowance | 118k / 15k | -75% / -79% | 0.0 / 0.0 | 174 / 17 | -54% / -68% | 26k / 1.4k | 181k / 6.7k | 0.1 / 0.0 | 1 / 0 |
| Telperion | R3a radial | 329k / 44k | -29% / -39% | 4.2 / 4.0 | 363 / 43 | -5% / -17% | 71k / 10k | 502k / 70k | 0.4 / 0.1 | 3 / 0 |
| Telperion | R3b probe | 335k / 52k | -28% / -29% | 4.2 / 4.6 | 368 / 50 | -3% / -3% | 75k / 16k | 535k / 107k | 0.4 / 0.1 | 3 / 1 |

"Twig-layer q per planned axis" divides stride, bisection, terminal, curtain-band and rung-room queries by the planned axes; terminal twigs are not planned axes. No preset hit the node budget at any rung (`twig_detail` was none everywhere).

## Whole-run excursion beyond the outline (share of widest radius; seed 1 / seed 7)

| Preset | Rung | Axes outside | Median | p95 | Max | Worst point above top / below base / beside | Band nodes |
|---|---|--:|--:|--:|--:|---|--:|
| Oak | R1 none | 7.08% / 3.85% | 0.065 / 0.040 | 0.394 / 0.281 | 0.780 / 0.677 | 4-572-3599 / 10-148-2519 | 0 / 0 |
| Oak | R2 allowance | 6.00% / 3.27% | 0.066 / 0.039 | 0.411 / 0.286 | 0.780 / 0.677 | 0-567-2906 / 0-148-2083 | 0 / 0 |
| Oak | R3a radial | 2.27% / 1.33% | 0.009 / 0.006 | 0.203 / 0.120 | 0.400 / 0.374 | 0-79-1229 / 0-32-887 | 0 / 0 |
| Oak | R3b probe | 0.09% / 0.05% | 0.004 / 0.003 | 0.016 / 0.015 | 0.032 / 0.020 | 0-0-54 / 0-0-35 | 0 / 0 |
| Beech | R1 none | 13.55% / 13.10% | 0.101 / 0.101 | 0.488 / 0.490 | 0.877 / 0.832 | 759-0-11615 / 875-0-11214 | 0 / 0 |
| Beech | R2 allowance | 4.05% / 4.40% | 0.044 / 0.045 | 0.280 / 0.273 | 0.500 / 0.514 | 0-0-2705 / 0-0-2952 | 0 / 0 |
| Beech | R3a radial | 1.98% / 1.79% | 0.001 / 0.002 | 0.027 / 0.035 | 0.329 / 0.268 | 14-0-1679 / 22-0-1526 | 0 / 0 |
| Beech | R3b probe | 0.01% / 0.01% | 0.000 / 0.000 | 0.001 / 0.001 | 0.001 / 0.001 | 0-0-10 / 0-0-7 | 0 / 0 |
| Birch | R1 none | 4.59% / 4.00% | 0.242 / 0.275 | 0.488 / 0.517 | 0.750 / 0.707 | 0-145-1624 / 0-188-1463 | 14,737 / 11,505 |
| Birch | R2 allowance | 3.91% / 3.28% | 0.243 / 0.284 | 0.489 / 0.521 | 0.750 / 0.707 | 0-144-1519 / 0-189-1333 | 15,168 / 12,067 |
| Birch | R3a radial | 2.78% / 3.11% | 0.275 / 0.287 | 0.502 / 0.526 | 0.750 / 0.707 | 0-139-1148 / 0-185-1244 | 12,653 / 10,511 |
| Birch | R3b probe | 2.63% / 2.97% | 0.302 / 0.309 | 0.519 / 0.531 | 0.750 / 0.707 | 0-139-953 / 0-185-1091 | 10,996 / 9,520 |
| Spruce | R1 none | 1.11% / 0.92% | 0.022 / 0.030 | 0.177 / 0.225 | 0.331 / 0.432 | 5-6-537 / 5-9-417 | 0 / 0 |
| Spruce | R2 allowance | 1.09% / 0.91% | 0.021 / 0.030 | 0.174 / 0.220 | 0.331 / 0.432 | 5-3-533 / 5-7-415 | 0 / 0 |
| Spruce | R3a radial | 0.61% / 0.54% | 0.017 / 0.022 | 0.099 / 0.147 | 0.220 / 0.314 | 0-0-303 / 0-2-251 | 0 / 0 |
| Spruce | R3b probe | 0 / 0 | – | – | 0 / 0 | – | 0 / 0 |
| Telperion | R1 none | 3.62% / 8.49% | 0.132 / 0.089 | 0.482 / 0.356 | 0.757 / 0.456 | 47-0-777 / 0-0-448 | 0 / 0 |
| Telperion | R2 allowance | 0 / 0 | – | – | 0 / 0 | – | 0 / 0 |
| Telperion | R3a radial | 0.20% / 0.14% | 0.004 / 0.004 | 0.061 / 0.005 | 0.081 / 0.005 | 0-0-41 / 0-0-4 | 0 / 0 |
| Telperion | R3b probe | 0.04% / 0 | 0.023 / – | 0.051 / – | 0.051 / 0 | 0-0-9 / – | 0 / 0 |

R0 has no axis outside on any preset. Its band holds 10,898 / 9,595 nodes on the birch and none elsewhere. Axes counted (R0): oak 55,590 / 67,329, beech 81,117 / 82,300, birch 36,064 / 39,489, spruce 49,162 / 46,429, telperion 22,201 / 4,868.

**The birch's excursion comes from its exempt curtain.** In one stats run (birch, seed 1, R3b) the dropping curtains were capped as well (`LADDER_CURTAIN=1`). Axes outside fell from 1,092 to 12 (max 0.150), and band nodes fell from 10,996 to 1,321. The excursion left at R1 to R3b on the birch is therefore curtain strands outside the band, not ordinary axes.

## Stills (seed 1, 960x720, headless renderer, exploration binary)

These are today (R0) and R3b, the one rung whose ordinary-axis excursion stayed under 0.035 of the widest radius on every preset. That choice keeps to four stills per preset; R1, R2 and R3a each need one `LADDER=n` command to render. All paths are under `raw/stills/`:
`oregon-white-oak-s1-rung{0,4}-{whole,bare}.png`, `european-beech-s1-rung{0,4}-{whole,bare}.png`, `silver-birch-s1-rung{0,4}-{whole,bare}.png`. Two were opened to check they are not blank (birch R3b whole, oak R3b bare). The renderer's leaf counts match the stats runs.

## Surprising

- **The birch's queries are the curtain's lower-surface search.** It made 1.12M of 1.27M queries, attributed exclusively. The bisection proper made 47.6k. QUERY-COUNTS.md credited 617k to bisection because it counted by the opening call site.
- **On the oak and beech, terminal admission (28–30%) costs nearly as much as the stride check** and more than the bisection.
- **Spruce:** scaffold containment and sampling is 65% of its queries. Removing the twig wall cuts 35% of queries and no measurable time (−7% to +7%, inside R0's 32–45 ms range).
- **Telperion:** shedding is 49% of its seed-1 queries, and that build spends 381 ms with 153k nodes shed. No rung except R2 moves its time beyond 5%.
- **R2 empties crowns.** Nodes fall 16% on the beech, 65% and 92% on Telperion, and the time saving follows. On the oak its node count sits between R0's and R1's. On the birch it has more nodes than R1 (94k against 87k at seed 1; the cause was not traced). On both, its excursion stays near R1's. All oak and birch axes are limb-borne. The beech has 203 stem-borne axes and the spruce 62–72, which R2 leaves uncapped.
- **Removing the wall adds wood.** R1 adds 7% (oak) to 13% (beech) nodes and 7–18% leaves. R3a adds up to 27% nodes on the birch at seed 1 (101k against 80k).
- **The worst point is often below the crown base,** on the oak (572 axes at seed 1) and the birch. The smooth outline has zero radius below the base, so drooping wood there counts its whole radial distance.

## Where the code differs from the spec

- **The spec's shares of queries that are the wall (56/60/74%) are per call site.** Attributed exclusively, the twig layer's stride, bisection and terminal queries are 86% (oak), 88% (beech) and 10% (birch) of the total. The curtain band is a further 88% on the birch.
- **`rejected` also refuses a point below `trunk_height`,** so removing admission (R1) removes that floor too. Nothing else stops a non-curtain twig dipping below the crown base.
- **The scaffold's `reach` is measured in the inner crown** (`planning`), so any allowance carried from it needs a conversion to the full crown. R2 used `1/share`.
- **Ways R2 could carry its allowance down.** I implemented the first; I did not choose between them:
  1. A sphere about the limb station (implemented).
  2. A path-length budget decremented along the hierarchy.
  3. Distance to the limb's planned tip point.
  4. The allowance scaled by `lateralLengthRatio` per order.
  5. For the value of `A`: re-probing reach against the full crown (about one probe per limb) instead of `/share`.
- **The task's line numbers held,** except `advance.rs:192/193`, now `advance.rs:200/205` (the curtain's `clear`, then admission).
