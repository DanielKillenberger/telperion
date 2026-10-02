# fn-188 R3, round 3: space colonization from the resampled scaffold, 2026-10-02

## What was built

- **Code:** round 2's probe (`2dd97eb2`), changed, committed as `4c59e337` and reverted in `1b9d5e80`. The commit also carries R1's `FN188_NO_TWIGS` switch (`specimen.rs`), used here only to time the scaffold alone. Nothing is wired into the pipeline.
- **Buds along the scaffold:**
  - Each scaffold segment of the direct build is resampled into metamers of the probe's unit (height ÷ 128). The geometry is kept, and the radius runs from the segment's start radius to its end radius.
  - Every metamer bears one axillary bud, and every scaffold tip a terminal bud. The beech's 1,882 scaffold nodes become 4,930 metamers.
  - At the end, scaffold wood is never thinner than it started: max(pipe radius, the scaffold's own radius). The beech takes fn-182's G2 values; the other presets their shipped ones.
- **Space as the resource** (Pałubicki 2009, §4.1 space colonization with §4.2 BH allocation):
  - Markers are drawn uniformly over the envelope's box and kept where `Envelope::contains` holds, lobes included. There are none outside.
  - A bud perceives the markers in its cone (half-angle 45°) within the perception radius, and each marker goes to the closest bud that sees it.
  - Q is 1 for a bud with markers and 0 without, and its growth direction is the mean direction to them.
  - Markers within the occupancy radius of any wood die, the scaffold's included.
  - There is no wall check and no light term, unless the light row says otherwise.
- **Growing only what has space:**
  - A bud that sees no live marker at all is retired. Markers only die, so it never would again. This gives the same tree as without retirement (51,781 nodes either way) at 133 ms instead of 740 ms.
  - A shoot stops when its new tip sees no marker.
  - There is no shedding: grown equals kept in every space row.
- **Fixed:**
  - λ 0.5, α 2, ξ 0.3, η 0.15, at most 40 cycles. At 60 cycles the tree is identical, so growth has converged.
  - Density 0.05 markers per unit³, perception 5 units, occupancy 2 units (the paper gives 4 to 6 and 2 internodes).
- **Measures, tone, escape and timing** are as in round 2.
  - **Scaffold ms** is the same request's `skeleton_ms` with the twig layer skipped. On every preset but the date palm, that build is node-for-node the shipped scaffold ("!" below: the palm's no-twig build differs in node count).
  - **Probe ms** covers the marker fill and the growth, not the scaffold.

## Every preset, seeds 1 and 7

| preset s | shipped nodes / leaves | ship skel / scaffold ms | probe nodes (grown = kept) | markers | probe ms (cold) | strong probe (ship) | fine m/m probe (ship) | tone probe (ship) | tip r cm | escape max m |
|---|---|---|--:|--:|--:|---|---|---|--:|--:|
| ordinary 1 | 11,568 / 40,659 | 41 / 8 | 30,210 | 14,761 | 66 (67) | 4 (6) | 170 (183) | 0.016 (0.003) | 0.66 | 0.3 |
| ordinary 7 | 11,934 / 41,877 | 46 / 8 | 29,536 | 14,322 | 64 (67) | 6 (5) | 249 (186) | 0.013 (0.004) | 0.66 | 0.2 |
| oak 1 | 133,635 / 767,026 | 31 / 3 | 111,764 | 59,311 | 272 (268) | 4 (3) | 344 (1,262) | 0.014 (0.007) | 0.30 | 0.8 |
| oak 7 | 143,703 / 903,084 | 37 / 4 | 110,838 | 58,575 | 284 (291) | 3 (4) | 309 (3,600) | 0.013 (0.008) | 0.29 | 1.8 |
| spruce 1 | 96,174 / 7,414,510 | 31 / 15 | 31,269 | 10,255 | 53 (53) | 8 (5) | 407 (2,765) | 0.002 (0.005) | 0.23 | 0.0 |
| spruce 7 | 91,077 / 7,064,601 | 31 / 14 | 30,364 | 10,197 | 53 (55) | 9 (7) | 400 (2,628) | 0.002 (0.004) | 0.23 | 0.1 |
| birch 1 | 86,326 / 258,111 | 26 / 4 | 40,380 | 21,740 | 101 (102) | 6 (6) | 92 (990) | 0.011 (0.014) | 0.28 | 1.0 |
| birch 7 | 91,743 / 279,485 | 27 / 4 | 41,471 | 22,263 | 98 (99) | 8 (7) | 116 (1,132) | 0.009 (0.015) | 0.27 | 1.2 |
| date palm 1 | 552 / 6,554 | 2 / 2 ! | 249 | 74 | 1 (1) | 2 (0) | 0.9 (0) | 0.001 (0.003) | 4.7 | 0.0 |
| date palm 7 | 553 / 6,554 | 2 / 2 ! | 230 | 68 | 1 (1) | 1 (0) | 0.7 (0) | 0.001 (0.003) | 5.7 | 0.1 |
| telperion 1 | 77,324 / 550,030 | 357 / 6 | 12,534 | 6,323 | 28 (28) | 1 (4) | 51 (335) | 0.070 (0.022) | 20.4 | 1.9 |
| telperion 7 | 18,140 / 128,852 | 51 / 5 | 11,875 | 6,123 | 26 (27) | 3 (3) | 47 (74) | 0.084 (0.019) | 21.5 | 4.4 |
| laurelin 1 | 53,861 / 392,944 | 94 / 12 | 132,346 | 69,890 | 324 (332) | 6 (7) | 151 (67) | 0.099 (0.026) | 17.4 | 22.1 |
| laurelin 7 | 75,239 / 543,814 | 170 / 12 | 131,831 | 68,893 | 362 (335) | 6 (8) | 130 (89) | 0.123 (0.024) | 17.5 | 5.1 |
| beech G2 1 | 82,636 / 1,013,806 | 22 / 4 | 51,781 | 25,373 | 128 (132) | 14 (6) | 70 (185) | 0.0055 (0.008) | 2.13 | 1.0 |
| beech G2 7 | 92,606 / 1,035,559 | 23 / 3 | 49,846 | 24,519 | 122 (127) | 8 (3) | 69 (155) | 0.0056 (0.011) | 2.17 | 0.7 |

- **Escape:** p95 is 0.0 m on every preset except birch s7 (0.2 m). Up to 8% of grown nodes stand outside, all within a metamer's overshoot, except where noted below.
  - **Telperion and laurelin s7:** the maxima of 1.9 to 5.1 m are about 2 to 5 of their 1.0 to 1.2 m units.
  - **Laurelin s1:** its 22.1 m maximum sits beside a p95 of 0.0 m and an outside share under 0.5%. Which nodes reach it was not traced.
- **Date palm:** it has 68 to 74 markers in its envelope after the scaffold's occupancy, and grows 109 to 131 nodes beyond its scaffold.
- **Telperion and laurelin:**
  - Grown tips are 17 to 22 cm in radius: one unit of pipe demand under 7.4 m roots, raised by holding the scaffold's own radii.
  - Laurelin grows 132k nodes, and its tone (0.099 to 0.123) is 4 to 5x the shipped tree's.
- **Strong limbs:** at most 3 of the strong limbs are grown wood on any preset except the spruce (8 and 9, every one grown).

## Continuity, beech G2, seed 1

One parameter moved at a time.

| step | strong (grown) | o1 / o2 / o3 | mid / end | o3/o1 | tips | tip r cm | fine m/m | tone | nodes | ms |
|---|---|---|---|--:|--:|--:|--:|--:|--:|--:|
| λ 0.44 / 0.47 | 11 (0) / 11 (0) | 0.31 / 0.28 / 0.51 · 0.29 / 0.27 / 0.52 | 0.79 / 1.00 | 0.135, 0.137 | 9,405, 9,327 | 2.18 | 67, 67 | 0.0063 | 41,414, 40,562 | 111, 108 |
| λ 0.50 | 14 (3) | 0.25 / 0.31 / 0.55 | 0.79 / 1.00 | 0.137 | 10,102 | 2.13 | 70 | 0.0055 | 51,781 | 127 |
| λ 0.51 / 0.52 / 0.53 | 15 (4) / 15 (3) / 16 (5) | 0.23–0.25 / 0.28–0.31 / 0.52–0.53 | 0.79 / 1.00 | 0.135–0.137 | 9,378–9,561 | 2.17–2.19 | 77–80 | 0.0061–0.0066 | 39,298–39,404 | 104–106 |
| λ 0.56 | 14 (3) | 0.25 / 0.30 / 0.55 | 0.78 / 1.00 | 0.129 | 9,904 | 2.14 | 77 | 0.0059 | 40,969 | 109 |
| density 0.025 / 0.05 / 0.1 / 0.2 / 0.4 | 12 / 14 / 14 / 15 / 16 | o1 0.28 → 0.09 | 0.79 / 1.00 | 0.163 → 0.099 | 6,484 → 23,008 | 2.48 → 1.60 | 44 → 161 | 0.0053 → 0.0099 | 33.0k → 121.1k | 65 → 1,009 |
| perception 3 / 3.25 / 3.5 / 3.75 / 4 units | 9 / 11 / 11 / 14 / 17 | o3 0.78 → 0.68 | 0.68–0.79 / 1.00 | 0.261 → 0.148 | 1,069 / 1,866 / 4,073 / 7,141 / 8,642 | 4.62 → 2.25 | 7 / 12 / 22 / 47 / 52 | 0.0016 → 0.0061 | 6.8k / 9.8k / 19.3k / 32.8k / 40.1k | 23 → 84 |
| perception 5 / 6 / 8 units | 14 / 14 / 14 | o1 0.25 → 0.22 | 0.79 / 1.00 | 0.137 → 0.129 | 10,102 → 10,620 | 2.13 → 2.09 | 70 → 86 | 0.0055–0.0057 | 51.8k → 58.0k | 127 → 356 |
| occupancy 1 / 1.5 / 2 / 2.5 / 3 units | 16 / 16 / 14 / 13 / 14 | o1 0.18 → 0.31 | 0.79 / 1.00 | 0.112 → 0.164 | 21,477 → 6,043 | 1.64 → 2.54 | 125 → 39 | 0.0084 → 0.0043 | 93.5k → 32.1k | 280 → 74 |
| light × space | 13 (3) | 0.30 / 0.53 / 0.62 | 0.79 / 1.00 | 0.144 | 8,356 | 2.27 | 58 | 0.0068 | 30,599 | 268 |
| light × space, Takenaka shed 0.01 | 11 (0) | 0.37 / 0.56 / 0.78 | 0.70 / 1.00 | 0.243 | 667 | 2.17 | 10 | 0.0017 | 6,928 kept of 34,771 | 175 |

**Continuity:**

- **λ:** the strong-limb count across 0.44 to 0.56 is 11, 11, 14, 15, 15, 16, 14. The largest step is +3, between 0.47 and 0.50. Round 2 showed 21 → 32 at 0.51 → 0.52, and here that step is 15 → 15.
- **The leader no longer runs out:** its axis top stays at 25.8 m against an envelope of 32 m, because there are no markers above the envelope.
- **Density and occupancy** move tips, fine wood, tone and time monotonically. Limbs stay at 12 to 16.
- **Perception is steep below about 4 units:** from 3 to 4 units, nodes go 6.8k → 40.1k (×5.9) and fine wood 7 → 52 m/m.
  - The finer steps (3.25, 3.5, 3.75) show no discontinuity.
  - With occupancy 2 units, a bud's cone between the occupied sphere and the perception radius holds few markers until the radius is about twice the occupancy. Buds that see none retire.
  - Above 5 units the tree saturates: 51.8k to 58.0k nodes, at 2.8x the time.

**Light:**

- Multiplying space by shadow exposure gives 41% fewer nodes (30.6k), a higher o2 (0.53) and 2.1x the time.
- With Takenaka shedding on top, 80% of the wood grown is shed, as the paper predicts for binary space input.

## Speed

Beech G2 s1, by stage (ms):

| Stage | ms |
|---|--:|
| perception | 56.0 |
| extend, incl. occupancy and sight checks | 37.3 |
| marker fill | 13.0 |
| basipetal | 11.0 |
| shed pass (none shed) | 2.8 |
| setup and resampling | 2.7 |
| allocate | 2.4 |
| pipe | 2.1 |

- **Work done:** 7.3M node visits; no shadow updates.
- **Scaffold alone:** 3 to 15 ms on every preset, against shipped skeletons of 2 to 357 ms.
- **The probe beside the shipped skeleton:**
  - Beech 122 to 128 ms against 22 to 23 ms.
  - Oak 272 to 284 ms against 31 to 37 ms.
  - Spruce 53 against 31 ms.
  - Birch 98 to 101 against 26 to 27 ms.
  - Ordinary 64 to 66 against 41 to 46 ms.
  - Telperion 26 to 28 ms against 51 to 357 ms.
  - Laurelin 324 to 362 ms against 94 to 170 ms.
- **Time scales with nodes and perception:** density 0.4 costs 1,009 ms; perception 8 units costs 356 ms.

## Stills (two viewed)

- **`beech-probe3-bare.png`** (pinned pose):
  - Fine wood is spread evenly along the limbs, with no clumps at stations, in a dense mesh that fills the oval crown.
  - The edge is ragged twig ends inside the envelope, and the leader and limbs are visible through the mesh.
  - The tips are thicker than the shipped twig layer's: 2.1 cm radius.
- **`laurelin-probe3-s1-bare.png`** (hero pose):
  - A squat dome filled edge to edge with wavy branches of nearly one thickness (tips about 17 cm).
  - The scaffold is hidden inside, with no visible gradation from limb to twig, and the trunk is bare below the dome.
- **Not viewed:** `beech-probe3-whole.png` and `telperion-probe3-s7-bare.png`.

## Files

- `raw/probe3.jsonl`: every run, with the shipped and probe traced measures, stage times, markers, scaffold timing, escape and parameters.
- `raw/stills/`:
  - `beech-probe3-bare.png`
  - `beech-probe3-whole.png`
  - `telperion-probe3-s7-bare.png`
  - `laurelin-probe3-s1-bare.png`
