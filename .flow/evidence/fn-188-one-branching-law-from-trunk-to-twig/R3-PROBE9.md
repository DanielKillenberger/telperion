# fn-188 R3, round 9: the bare scorecard and five levers, 2026-10-03

## Setup

- **Code:** round 8's probe with its bare measures (`7a61270d`), plus the scorecard and two dormant parameters. Committed as `01ba06a6`, reverted in `c868242b`.
- **Base:** round 8's chosen tree (base B, λ 0.55, two-ranked blend D 1, leaf pipes, no floor, unique identities, clumping 0) at seed 1, pinned pose. Its digest `5c13233733bb8a5b` is reproduced, as are base A (`4d2531e5…`) and base B (`fc438a66…`).
- **Runs:** release on the Ryzen 9 5950X, 16 threads. The levers ran four at a time; the timing ran alone.
- **Rows:** `raw/probe9/<tag>.json`, each with its `P_*` environment and overlay. The scorecard rows are in `raw/probe9/scorecard.jsonl`.

## The scorecard (Astra's seven targets)

- **Tree measures (`score.rs`)** work on any pipeline tree, shipped or probe. H is the top node's height.
  1. **Bole:** the lowest substantial lateral off the trunk axis, as a share of H. Substantial means child ÷ local parent radius ≥ 0.3 and an axis of 1 m or more. **Division:** the lowest height where 2 (and 3) crossing segments are at least 0.5 of the thickest crossing radius.
  2. **Major axes:** laterals born at ≥ 0.4 of the root radius ("root"), and laterals off the trunk at ≥ 0.4 of the local trunk ("local"). Both must be persistent (axis ≥ 0.2 H). **Headings:** the projected chord angle from vertical, median per height third, over the major paths plus a leader that passes mid-height.
  4. **Secondaries per major axis:** the ratio ≥ 0.3 and an axis ≥ 1 m. Also their attachment s/L, and child length ÷ the parent's length remaining.
  5. **Terminal wood:** terminal-run wood, in a star-shaped crown from its mid-height centre (8 × 16 direction bins, R = p98). The shell is d ≥ 0.75 R. Densities are per bin volume, shell over interior and upper over lower.
  6. **Junction ratio:** child ÷ parent radius by order, median, over axes of 1 m or more.
  7. **Fine-wood turning** (geometric): every non-structural axis projected, scaled so the tree is 370 px tall, and resampled every 7 px. The turning angle (p50) between chords, and the mean cos 2Δθ between chords of different axes within 7 px.
- **Image measures (`raw/probe9/score.py`, the same code for the photo):** a wood mask that excludes shadows. From it: W(h) in 5% bands, the widest-height fraction, and the low skirt (span area below 0.35 H ÷ all). The image clear bole is round 8's: the first row wider than 3× the trunk.
- **B-BARE could not be measured** (one view, `raw/probe9/mask-check.png`):
  - at 534 × 400 its twig haze masks solid, at 76% of the crown rows;
  - a neighbouring tree joins its right edge;
  - the ground behind the bole is dark below y 290;
  - top and left are clipped.
- **Targets are therefore Astra's bands**, not photo measures. Image line tracing at the photo's scale also saturates on the renders, so measure 7 uses the geometric version.

**Targets:** bole 0.20–0.23 visible, division 0.30–0.39; 4–6 major paths at 15–40°; widest 0.45–0.65 with a smaller skirt; 5–10 secondaries at 0.3–0.8 of the remaining length; shell/interior and upper/lower rising; major limbs at 0.4–0.6 of the trunk; turning and neighbour agreement falling. Diagnosis targets from the shipped beech: 1.57 laterals per m of fine wood, generations 1–3, runs 0.25 / 0.50 m.

| tree | bole tree / image | division 2 / 3 | major root / local | heading lo / mid / up ° | widest / skirt / W÷H | sec n, s/L, len÷rest | shell÷int, up÷lo | ratio o1 / o2 / o3 | turn °, cos2 | fine km, lat/m, gens p50/max, run p50/p90 m |
|---|---|---|---|---|---|---|---|---|---|---|
| shipped beech | 0.07 / 0.10 | 0.08 / 0.15 | 5 / 12 | 12 / 11 / 1 | 0.38 / 0.28 / 0.81 | 7, 0.59, 1.52 | 0.19, 1.09 | 0.16 / 0.15 / 0.14 | 1.1, 0.21 | 48.5, 1.57, 1/3, 0.25/0.50 |
| round 5 (base B) | 0.08 / 0.05 | 0.09 / 0.09 | 7 / 15 | 22 / 21 / 13 | 0.38 / 0.32 / 0.73 | 7, 0.63, 0.64 | 0.91, 1.00 | 0.52 / 0.36 / 0.48 | 6.2, 0.12 | 11.9, 0.80, 2/10, 0.50/1.25 |
| rounds 6–7 (base A) | 0.21 / 0.05 | 0.37 / 0.37 | 3 / 4 | 2 / 21 / 14 | 0.38 / 0.32 / 0.71 | **27**, 0.66, 0.53 | 1.05, 1.02 | 0.23 / 0.52 / 0.64 | 4.0, 0.28 | 12.3, 0.82, 4/13, 0.50/1.25 |
| round 8 chosen | 0.08 / 0.05 | 0.09 / 0.09 | 8 / 15 | 22 / 21 / 14 | 0.38 / 0.32 / 0.75 | 7, 0.61, 0.65 | 0.94, 1.03 | 0.47 / 0.32 / 0.46 | 4.6, 0.09 | 12.3, 0.77, 2/9, 0.53/1.22 |
| **(a) crown base 0.30** | 0.32 / 0.16 | 0.37 / 0.37 | 6 / 10 | – / 21 / 16 | 0.53 / 0.19 / 0.72 | 7, 0.67, 0.70 | 1.02, 1.08 | 0.43 / 0.45 / 0.51 | 4.6, 0.11 | 9.3, 0.76, 2/10, 0.53/1.21 |
| **(b) + ξ 0.1, η 0.05** | 0.32 / 0.15 | 0.37 / 0.37 | 6 / 9 | – / 21 / 16 | 0.57 / 0.19 / 0.71 | 9, 0.67, 0.70 | 1.01, 1.04 | 0.43 / 0.43 / 0.52 | 2.7, 0.10 | 9.7, 0.80, 2/10, 0.51/1.13 |
| **(d) + occupancy 1.5 (final)** | 0.32 / 0.16 | 0.37 / 0.37 | 6 / 9 | – / 21 / 16 | 0.53 / 0.19 / 0.72 | 8, 0.72, 0.70 | 1.04, 1.09 | 0.34 / 0.42 / 0.49 | 2.7, 0.11 | 13.9, 0.88, 3/11, 0.48/1.04 |
| (d′) occupancy 1.0 instead | 0.32 / 0.16 | 0.37 / 0.37 | 6 / 9 | – / 21 / 16 | 0.53 / 0.19 / 0.72 | 10, 0.71, 0.71 | 1.02, 1.04 | 0.34 / 0.38 / 0.47 | 2.7, 0.09 | 24.8, 1.05, 3/11, 0.45/0.90 |
| round 8, seed 7 | 0.08 / 0.04 | 0.09 / 0.09 | 4 / 14 | 20 / 12 / 10 | 0.33 / 0.34 / 0.72 | 7, 0.61, 0.64 | 0.92, 1.00 | 0.52 / 0.28 / 0.44 | 4.6, 0.12 | 12.0, 0.79, 2/13, 0.53/1.22 |
| final, seed 7 | 0.33 / 0.16 | 0.33 / 0.33 | 4 / 10 | – / 9 / 9 | 0.42 / 0.19 / 0.69 | 7, 0.67, 0.63 | 1.07, 1.07 | 0.34 / 0.33 / 0.47 | 2.8, 0.10 | 13.7, 0.94, 3/12, 0.48/1.03 |

"–" means only the clear trunk is in the lower third. Every lever row is seed 1 unless named.

## What each lever did

**(a) Clear bole and less skirt: the envelope's crown base** (`crownBase`, 0.06 shipped; markers fill only above it, and the scaffold's stations start there). Walked 0.12, 0.18, 0.24 and 0.30.

- The tree bole goes 0.08 → 0.16 / 0.24 / 0.24 / 0.32, and the image bole 0.05 → 0.10 / 0.13 / 0.14 / 0.16.
- The skirt falls 0.32 → 0.29 / 0.25 / 0.22 / 0.19, and nodes fall 41k → 32k.
- At 0.30 the division lands in band (0.37) and the widest height reaches 0.53.
- The image bole stays under the band: wood from 0.28 H projects downward at this camera. The tree bole overshoots it.
- **Kept, 0.30:** sheet 1 shows a clean trunk under the crown.

**(a) Light-driven self-pruning.** Implemented as a new continuous weight w on a space bud's Q, (1 − w) + w·exposure, with w 0 reproducing the tree byte for byte.

- **w 1 alone:** shell/interior 1.02 → 1.44 and upper/lower 1.08 → 1.27; fine wood −26%; probe 71 → 109 ms.
- **Sheet 1 rejected it:** it adds thick looping arcs at the crown edge.
- **Takenaka shedding cannot be tuned in space mode.** Thresholds 0.002 and 0.005 give the same tree: 6,926 nodes and 0.8 km of fine wood. Once a bud's markers are taken its Q is 0, so every branch falls under any positive threshold. Shedding by light alone, or on remembered Q, is a design choice left to the host.

**(b) Fine-wood straightness.** Redone on (a) after the light weight was rejected.

- **ξ alone does not straighten:** ξ 0.14 / 0.1 / 0.05 leave the turning at 4.7–5.3°.
- **η drives it:** η 0.05 gives 3.1°, η 0.02 gives 1.7–2.9°.
- **Kept ξ 0.1, η 0.05:** turning 4.6 → 2.7° and cos2 0.11 → 0.10, the only pair that lowers both. ξ 0.1 with η 0.02 reaches 1.7° but raises cos2 to 0.13.
- **Sheet 2:** less looping than (a).

**(c) Secondaries: the share of active axillary buds.** A new parameter a; a hash of the node index picks which buds stay dormant, and a 1 is neutral (byte for byte). Walked 0.7, 0.5 and 0.35.

- Secondaries per major stay 7–10 (already in band, as on base B; only base A's leaders were combs, at 27).
- Shell/interior rises to 1.83.
- **But laterals per m fall** (0.93 → 0.55) and runs lengthen (p50 0.40 → 0.63 m), against (d).
- **Sheet 1:** sparser and loopier. **Rejected.**

**(d) Short shallow shoots.**

- **Round 8's shoot form, s 1.5 / 1.9 / 2.5:** removes fine wood (7.0 → 1.3 km) and is rejected.
- **A finer unit, H/192 with the marker count held:** laterals per m 1.36–1.54 and runs 0.28–0.30 / 0.62–0.64 m, close to the shipped beech. But generations deepen (max 20–24) and shell/interior falls to 0.76–0.91. Sheet 2 shows coarse looping edges; rejected.
- **Occupancy 2 → 1.5 units: kept.** Fine wood 9.7 → 13.9 km, laterals per m 0.80 → 0.88, runs 0.51 / 1.13 → 0.48 / 1.04 m; shell 1.01 → 1.04 and upper/lower 1.04 → 1.09. o1 falls from 0.43 to 0.34, out of band. Sheet 2 shows an even, denser mesh with the limbs still visible.
- **Occupancy 1.0** goes further (24.8 km, 1.05 per m, runs 0.45 / 0.90 m, cos2 0.09). It hides the limbs in sheet 2 and costs 149 + 1,319 ms, so it was not chosen.
- **Generations 1–3 and 1.57 per m are not reached** by any lever here.

**(e) Major limbs: scaffold `lateralLengthRatio` 0.15 and `forkWays` 3** (5 is invalid; the maximum is 4).

- Root-count majors go 6 → 5–6, but the junction ratio o1 falls to 0.28–0.35 and the mid-third heading spread falls from 28 to 23–25°.
- No visible change in sheet 2. **Rejected.** Majors were already in band (5–6 root, 9 local).

## Judged on the sheets

- **Sheet 1:** `stills/beech-probe9-sheet1-bare.png`, B-BARE, baselines and first steps. **Sheet 2:** `stills/beech-probe9-sheet2-bare.png`, the redone steps.
- **Every probe crown still reads as a round ball** on a now-clear trunk. B-BARE is a tall, upright crown whose ascending leaders show through a dense, straight haze.
- **The shipped beech remains closest in its limbs:** long, straight, fanning.
- **The final tree** has the clearest bole and the most even mesh of the probe steps. Its outline is the envelope's dome, and no leader reaches the top.
- **Remaining differences,** none of them reached by these five levers:
  - crown outline and height ÷ width (the envelope's spread and fullness);
  - leaders that reach the top;
  - generations and laterals per metre.

## Final values

Base B (round 5's full G2 scaffold, crookedness 6, 40°, p 0), leaf pipes, no floor, unique identities, clumping 0, two-ranked D 1, λ 0.55, **crownBase 0.30, ξ 0.1, η 0.05, occupancy 1.5 units**. Shoot form, light weight and bud activation are dormant. The environment and overlay are in `raw/probe9/d2-rho1.5.json` and `ov-d2-rho1.5.json`.

**Whole, final** (`stills/beech-probe9-final-whole.png`, viewed): a dense green dome on a clear trunk. Tree cover is 0.918 and leaf cover 0.841; width ÷ height is 0.795; 1,842,570 leaves.

## Timing (alone, warm medians of 3 probe and 2 mesh runs; the scaffold build, about 3 ms, not included)

| seed | tree | nodes | leaves | probe ms warm (cold) | mesh ms warm (cold) | total warm | shipped `mesh::build` warm (cold) |
|---|---|--:|--:|---|---|--:|---|
| 1 | final | 45,305 | 1,842,570 | 74.2 (80.1) | 736 (750) | 810 | 3,451 (3,472) |
| 1 | round 8 | 41,345 | 1,635,512 | 69.5 (73.1) | 698 (676) | 768 | – |
| 7 | final | 42,510 | 1,781,578 | 78.0 (77.8) | 750 (746) | 828 | 3,418 (3,433) |
| 7 | round 8 | 39,250 | 1,586,209 | 66.7 (70.9) | 640 (639) | 707 | – |

- **The final tree costs 42 to 121 ms more than round 8** (+5 to +17%), for 1.6 more km of fine wood (13.9 against 12.3).
- **It builds in about a quarter of the shipped beech preset's time.**
- Occupancy 1.0 would cost about 1.47 s.

## Paths

- **Scorecard:** `raw/probe9/scorecard.jsonl`, all 46 tags.
- **Per-run rows and overlays:** `raw/probe9/<tag>.json` and `ov-<tag>.json`.
- **Timing:** `raw/probe9/time-*.json`.
- **Image scorer:** `raw/probe9/score.py`; also in the probe commit with the drivers (`py/`).
- **Stills:** `raw/stills/beech-probe9-<tag>-bare.png` and `-whole.png` for every tag. The finals are `beech-probe9-final-{bare,whole}.png` and `beech-probe9-final-s7-{bare,whole}.png`.
- **Contact sheets:** `beech-probe9-sheet1-bare.png` and `beech-probe9-sheet2-bare.png`.
- **Images viewed (4):** the mask check, the two sheets and the final whole still.
