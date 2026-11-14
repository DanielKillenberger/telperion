# fn-364 R9 probe: turning by the length grown (builder, Opus, 14 Nov 2026; not built into the product)

**The probe falls short of the host's bar, so the product is not built.** Tortuosity over the outer 5 m falls by about 30% on the worst tip at 300, seed 1007, not by half. At 200, seed 1001, it rises a little. Height, trunk and leaf each move by under a quarter.

**What ran.**
- `curvature/r9-probe.py` patched a scratch checkout, `.worktrees/fn-364-r9` (`1c5981ee7` with the reader, never committed). The terms it moved and left are R9-READING.md's.
- It grew the preset oak at 300 on seed 1007 and at 200 on seed 1001, read CURVATURE.md's table (`curvature/r9-a<age>-s<seed>.md`), and drew the two trees whole and bare, with the `tip` shot at 200 (`raw/fn364-r9/stills/`, ignored).
- No PAUSE fell inside the 300 grow (251 s).

## The trees

| tree | run | height m | spread x / y m | trunk at 1.3 m, m | settled leaf m² | phytomers | zero-length phytomers | living tips at s = 0 |
|---|---|---|---|---|---|---|---|---|
| 300, 1007 | fn-363 | 38.8 | 48.7 / 51.7 | 3.13 | 1,111 | 6,324,353 | - | - |
| 300, 1007 | this branch | 26.1 | 32.2 / 36.2 | 2.53 | 806 | 6,304,859 | 495,132 | 33,173 of 388,356 |
| 300, 1007 | **R9 probe** | **27.4 (+5%)** | 27.6 / 35.3 | **2.49 (−2%)** | **685 (−15%)** | 5,252,048 (−17%) | 668,595 | 67,200 of 322,306 |
| 200, 1001 | fn-363 | 36.8 | 35.5 / 37.3 | 1.80 | 1,041 | 4,311,571 | - | - |
| 200, 1001 | this branch | 26.8 | 27.2 / 24.6 | 1.47 | 549 | 2,827,498 | 289,272 | 23,383 of 165,800 |
| 200, 1001 | **R9 probe** | **27.9 (+4%)** | 25.8 / 26.3 | **1.43 (−3%)** | **476 (−13%)** | 2,606,368 (−8%) | 385,654 | 32,515 of 154,819 |

## Curvature on CURVATURE.md's ten-tip rule

| tree | run | most tortuous: tortuosity | most: deg per metre | most: axes per metre | least (path ≥ 15 m): tortuosity, deg per metre |
|---|---|---|---|---|---|
| 300, 1007 | fn-363 | 47.1, 34.0, 32.9, 30.0, 29.7 | 168 to 173 | 4.2 to 4.4 | 1.00 to 1.01, 7 to 18 |
| 300, 1007 | this branch | 57.7, 50.0, 43.7, 43.1, 42.5 | 430 to 1,360 | 13.2 to 41.4 | 1.01 to 1.02, 24 to 71 |
| 300, 1007 | **R9 probe** | **40.3, 38.8, 38.4, 38.2, 37.9** | **318 to 324** | **9.4 to 9.6** | 1.02, 33 to 56 |
| 200, 1001 | fn-363 | 35.4, 34.5, 34.3, 30.4, 25.3 | 130 to 143 | 3.0 to 3.4 | 1.00, 87 to 88 |
| 200, 1001 | this branch | 13.9, 12.7, 12.6, 11.7, 11.7 | 231 to 237 | 5.4 to 5.6 | 1.01 to 1.02, 21 to 53 |
| 200, 1001 | **R9 probe** | **14.2, 14.0, 14.0, 13.8, 13.8** | **365 to 486** | **10.8 to 13.8** | 1.02, 78 to 88 |

**Against the bar:**
- **Tortuosity falls by under half.** At 300 the worst tip goes from 57.7 to 40.3 (−30%), and the fifth from 42.5 to 37.9 (−11%). At 200 the worst five rise from 11.7 to 13.9 to 13.8 to 14.2 (+2% to +18%).
- **Height, trunk and leaf move by under a quarter.** Height rises 5% and 4%, the trunk falls 2% and 3%, leaf falls 15% and 13%.

**Against the paper's other expectations** (read, not judged):
- The worst paths' degrees a metre fall to about 320 at 300, where about 1,000 was expected. At 200 they rise from about 235 to 365 to 486.
- The axes a metre fall from about 41 to about 9.5 at 300, and rise from about 5.5 to 11 to 14 at 200.
- The worst tips at 300 now stand 21.2 m of path from the collar, where they stood 16.0 m.
- The ten paths carry 547 relays and 122 laterals in all, where they carried 886 and 51 at 300 alone.
- The junction turn axes stay near random on the worst tips (mean resultant 0.04 to 0.16). On the straight tips they are 0.24 to 0.49, more of one hand than before (0.06 to 0.08).

## The four stills

- **300, seed 1007, whole and bare.** The crown is broad and full of wavy wood. Several outer limbs still run in S-bends and small loops over their last metres: a long wavy limb at the left edge, curls at the ends of the upper left and upper right limbs. The leaf sits in tufts along the outer wood.
- **200, seed 1001, whole.** An upright crown, its limbs waving in long bends, with fewer tight curls than at 300. A few fine shoots hang below the crown on the right.
- **200, seed 1001, `tip`.** The camera stands among thick limbs near the trunk and does not frame the limb's outer metre: the crop shows smooth, gently bending limbs and one short curl at the upper right.

## Not done (R9 is not built)

The R9 code stays in the probe checkout. The product and its tests, the oak's R5 rows and R6 tiles on R9, and the three-column sheets wait on the host's call.

R10 is built: `light_no_bud_reads_leaves_every_passed_species_to_the_bit` zeroes `inverse_height` too. It was red on `1c5981ee7` and passes on `8edcb4907` (21 s).
