# fn-346: the square charge (SR) against the straight one (PR), both with R

Drafted by the worker on 11 Nov 2026 at `a7d3dbf31` plus the host's R6 answers, for the host. This is `R6-ANSWERS.md`, "The run", steps 1 to 8. The probe is `probes/square.patch`, and it was reverted; `git status --short crates` is empty. Nothing here proposes a design.

Marks: **measured** (printed by a run), **arithmetic** (from the run's lines), **source** (quoted), **unchecked**, **eye** (read off a still).

## Headline

- **The extent levels off under SR, in the rough layout (measured).** The longest living path stops rising and holds at 34 to 38 m from about 140 on both seeds. The rough height holds at 32.8 m (1001) and 31.7 m (1007) from 159 to 179 on.
  - **The steady state:** living tips hold at 103,000 to 117,000 (1001, from 219) and 136,000 to 139,000 (1007, from 239) to 299, with births over deaths at 0.99 to 1.00 and held phytomers level at 1.74 to 1.84 million and 2.40 to 2.48 million. This is the first variant with a steady state.
- **The settled tree is another matter (measured, eye).** The sag rule lays down most of the standing crown the rough layout holds.
  - **Seed 1001:** 22.6 m tall at 200 and 8.3 m at 300, against 30 to 33 m rough. At 300 the stem line above about 8 m arcs over to the ground and carries the crown there.
  - **Seed 1007:** a small upright tree, 23.0 m at 200 and 15.3 m at 300.
- **The young oak holds under SR (measured, eye).**
  - **At 40 and 80:** SR is today's tree; height is unchanged and leaf moves +1% and +8%.
  - **At 120:** SR is QR's better many-limbed crown, with 19% more leaves than today's on 1001 and 55% more on 1007.
- **SR thins from 120 to 200 (measured).** SR at 200 carries 97,454 leaves on 1001 against today's 270,545, and 156,803 on 1007 against 619,213. The crown at 200 and 300 is thin and straggly.
- **PR (straight charge, L_max undoubled) starves.** Leaf is down 28% at 80 and 41% at 120 against today's. Seed 1007's tree is dead by 279 (no living tip), and 1001's holds 7,904 tips at 299.
- **The other presets under SR are the same as today's (eye, measured),** within 0 to 9% on every measure.
- **Cost (measured, one session, one thread).** SR costs the same as today's at 40, +13% at 80, +17% at 120, +15% at 200 and −11% at 300. Peak memory is 0.35 to 2.2 GiB against today's 0.35 to 2.7.
- **Source (abstract only).** McDowell et al. 2002 found the leaf area : sapwood area ratio falling with height in most of the species they compiled, which is "partial" compensation. No oak and no ring-porous species is named in what could be read.

## 1. The source (fifteen minutes)

**McDowell et al. 2002**, Oecologia 132: 12–20, "The relationship between tree height and leaf area: sapwood area ratio". The abstract was read on the publisher's page; the full text was not readable. The Andrews Forest PDF is a scan without text, and only its cover page came through.

**The abstract states (source):**
- "The leaf area to sapwood area ratio (Al:As) of trees has been hypothesized to decrease as trees become older and taller. Theory suggests that Al:As must decrease to maintain leaf-specific hydraulic sufficiency as path length, gravity, and tortuosity constrain whole-plant hydraulic conductance."
- "Whole-tree Al:As was measured on 15 individuals of Douglas-fir (Pseudotsuga menziesii var. menziesii) ranging in height from 13 to 62 m (aged 20–450 years). Al:As declined substantially as height increased (P=0.02)."
- "Our test of the hypothesis that Al:As declines with tree height was extended using a combination of original and published data on nine species across a range of maximum heights and climates. Meta-analysis of 13 whole-tree studies revealed a consistent and significant reduction in Al:As with increasing height (P<0.05). However, two species (Picea abies and Abies balsamea) exhibited an increase in Al:As with height, although the reason for this is not clear."
- "Maximum potential height was positively correlated with ΔAl:As/Δh."
- "The decrease in Al:As with increasing tree size that we observed in the majority of species may be a homeostatic mechanism that partially compensates for decreased hydraulic conductance as trees grow in height."

**Unchecked:**
- **The species list.** The nine species and the 13 studies are not named in the abstract. Whether any is a *Quercus* or ring-porous could not be read.
- **The slopes.** The size of ΔAl:As/Δh, and whether the relationship is linear in height, are not in the abstract.
- **The model's exponent.** The abstract supports the ratio falling with height and calls the compensation "partial". It gives no exponent, so the model's square, which is full compensation, is not stated there.

## 2. The probe (read)

**What `probes/square.patch` adds.** It is `remain.patch` (QR: R, Q's letters, rule B's code dormant) plus one letter, `U`.
- **The charge.** Under `U` a lit tip's own charge is `size × (L / L_max)²` (`grow/cycle/passes.rs`).
- **The census.** The old-tree census prints the reach every 10 years rather than every 20.

**`L_max` undoubled** (`PIPES_LMAX_FACTOR` 1): oak 34.8 m, beech 40, birch 30, spruce 61; palm and ordinary unset (charge dormant).

**Variants.** All have R, the balance counting what would remain; neither has rule A or rule B.
- **SR:** letters `PRMU`.
- **PR:** letters `PRM`, the straight charge.

**Runs.** All runs went under `telperion-heavy`. None was stopped by its census or killed.
- **Census:** `FACTOR=1 probes/remain.sh`.
- **Stills:** `probes/pipes.sh`.
- **Shape:** `FACTOR=1 LETTERS=PRMU TAG=SR UNSAGGED=0 probes/oldtree.sh shape <seed>`.
- **Cost pair:** `pipes.sh` time mode, today (no letters) and SR alternated on one binary. One `montage` of the sheets ran during the first two ages of this pair (about 10 s of CPU).

## 3. The census to 300 (measured; every 20 years and 299)

**Columns.** The longest path is the longest living tip's path along the wood. Height and spread are the rough layout's, before the settle. B / D is births over deaths in the decade. Every 10 years comes from the `REACH`, `YEAR` and `BAL` lines of `raw/remain/oak-300-<seed>-<SR|PR>.log`.

**SR, seed 1001**

| year | longest path m | height m | spread m | held | living tips | limb / bough / branch tips | lowest limb m | leaves | income | charge | B / D |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 11.1 | 10.6 | 6.8 × 8.0 | 304,231 | 24,702 | 4 / 23 / 842 | 4.7 | 69,004 | 6,400 | 1,099 | 1.21 |
| 79 | 19.7 | 17.4 | 17.9 × 14.6 | 1,877,353 | 127,311 | 6 / 68 / 3,684 | 4.7 | 367,104 | 27,781 | 14,748 | 1.07 |
| 119 | 30.2 | 25.7 | 24.2 × 23.0 | 3,370,052 | 197,330 | 10 / 113 / 5,026 | 4.7 | 615,753 | 36,350 | 28,187 | 1.01 |
| 139 | 35.1 | 29.1 | 26.6 × 27.7 | 3,303,417 | 184,044 | 14 / 143 / 4,850 | 4.7 | 584,224 | 27,859 | 22,402 | 0.98 |
| 159 | 37.2 | 31.6 | 28.1 × 24.5 | 2,776,641 | 146,905 | 8 / 97 / 3,600 | 4.7 | 474,427 | 21,747 | 17,300 | 0.97 |
| 179 | 33.8 | 32.8 | 27.5 × 25.7 | 2,287,531 | 123,751 | 6 / 90 / 3,192 | 4.7 | 395,513 | 17,502 | 14,169 | 0.99 |
| 199 | 37.1 | 32.8 | 28.6 × 26.2 | 1,910,850 | 106,211 | 12 / 87 / 2,896 | 4.7 | 333,943 | 12,240 | 9,887 | 0.98 |
| 219 | 35.2 | 32.8 | 31.5 × 24.5 | 1,757,552 | 103,262 | 15 / 103 / 3,042 | 4.7 | 315,020 | 9,036 | 7,241 | 1.00 |
| 239 | 34.5 | 32.8 | 28.5 × 23.0 | 1,741,240 | 107,533 | 18 / 139 / 3,433 | 4.7 | 318,208 | 6,310 | 5,161 | 1.00 |
| 259 | 36.1 | 32.8 | 27.8 × 23.0 | 1,805,957 | 111,991 | 23 / 151 / 3,564 | 4.7 | 331,172 | 5,108 | 4,197 | 1.00 |
| 279 | 34.4 | 30.2 | 27.7 × 17.0 | 1,807,366 | 115,151 | 24 / 185 / 3,901 | 4.7 | 336,376 | 4,242 | 3,501 | 1.00 |
| 299 | 29.5 | 30.2 | 22.5 × 17.3 | 1,836,625 | 116,724 | 29 / 193 / 3,924 | 4.7 | 343,025 | 3,673 | 3,121 | 1.00 |

**SR, seed 1007**

| year | longest path m | height m | spread m | held | living tips | limb / bough / branch tips | lowest limb m | leaves | income | charge | B / D |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 11.1 | 10.1 | 7.6 × 7.3 | 441,713 | 40,184 | 11 / 54 / 1,665 | 4.5 | 101,280 | 9,099 | 2,000 | 1.21 |
| 79 | 23.1 | 18.2 | 24.0 × 25.4 | 3,333,181 | 236,152 | 15 / 169 / 7,675 | 4.5 | 635,423 | 41,734 | 26,308 | 1.05 |
| 119 | 34.9 | 25.6 | 31.6 × 31.8 | 5,291,965 | 320,337 | 28 / 261 / 9,288 | 4.5 | 946,824 | 43,176 | 35,174 | 1.00 |
| 139 | 35.9 | 29.3 | 34.6 × 35.8 | 5,071,624 | 298,567 | 25 / 271 / 8,686 | 4.5 | 888,680 | 29,108 | 22,422 | 0.99 |
| 159 | 37.0 | 31.7 | 34.5 × 37.2 | 4,203,632 | 234,734 | 13 / 180 / 6,550 | 4.5 | 704,316 | 25,867 | 19,636 | 0.97 |
| 179 | 38.7 | 31.7 | 33.9 × 35.2 | 3,594,682 | 203,850 | 15 / 170 / 5,696 | 4.5 | 607,869 | 18,642 | 13,178 | 0.98 |
| 199 | 34.8 | 31.7 | 34.4 × 33.3 | 3,088,953 | 171,273 | 10 / 131 / 4,639 | 4.5 | 517,581 | 15,365 | 10,093 | 0.99 |
| 219 | 36.7 | 31.7 | 34.4 × 33.3 | 2,792,118 | 154,973 | 10 / 114 / 4,358 | 4.5 | 462,537 | 11,638 | 6,898 | 0.99 |
| 239 | 38.3 | 31.7 | 33.9 × 32.0 | 2,474,239 | 139,485 | 8 / 92 / 3,859 | 4.5 | 414,939 | 11,236 | 6,842 | 1.00 |
| 259 | 35.9 | 31.7 | 28.9 × 31.5 | 2,398,860 | 135,927 | 4 / 75 / 3,622 | 4.5 | 410,863 | 11,955 | 7,295 | 1.00 |
| 279 | 34.8 | 31.7 | 28.9 × 34.1 | 2,467,148 | 136,811 | 4 / 70 / 3,417 | 4.9 | 428,107 | 13,729 | 9,098 | 1.00 |
| 299 | 34.8 | 31.3 | 21.7 × 34.4 | 2,478,268 | 136,411 | 5 / 70 / 3,325 | 4.9 | 438,394 | 13,860 | 9,865 | 0.99 |

**PR, seed 1001**

| year | longest path m | height m | spread m | held | living tips | limb / bough / branch tips | leaves | income | charge | B / D |
|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 11.0 | 10.5 | 6.7 × 7.9 | 282,133 | 21,941 | 4 / 22 / 710 | 63,421 | 5,382 | 2,847 | 1.16 |
| 79 | 19.4 | 17.0 | 15.2 × 12.6 | 1,058,822 | 67,393 | 5 / 56 / 1,937 | 208,901 | 15,018 | 11,797 | 1.02 |
| 119 | 28.9 | 24.8 | 21.3 × 19.4 | 1,346,873 | 81,103 | 9 / 95 / 2,265 | 259,087 | 10,790 | 9,549 | 0.99 |
| 159 | 38.0 | 30.8 | 24.4 × 25.7 | 987,797 | 54,344 | 11 / 82 / 1,559 | 171,400 | 4,537 | 4,004 | 0.95 |
| 199 | 35.8 | 31.5 | 27.1 × 27.8 | 427,572 | 22,858 | 3 / 29 / 638 | 74,288 | 2,066 | 1,844 | 0.97 |
| 239 | 33.0 | 31.5 | 28.2 × 23.3 | 238,897 | 10,673 | 0 / 6 / 249 | 37,562 | 396 | 365 | 0.96 |
| 299 | 32.4 | 31.5 | 24.3 × 19.3 | 158,838 | 7,904 | 0 / 2 / 154 | 29,232 | 7 | 7 | 1.06 |

**PR, seed 1007**

| year | longest path m | height m | spread m | held | living tips | limb / bough / branch tips | leaves | income | charge | B / D |
|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 11.0 | 10.0 | 7.0 × 6.9 | 378,450 | 32,137 | 10 / 51 / 1,210 | 86,383 | 7,186 | 4,290 | 1.16 |
| 79 | 19.6 | 17.1 | 17.4 × 18.9 | 1,408,649 | 96,326 | 13 / 128 / 3,003 | 287,798 | 18,713 | 14,942 | 1.03 |
| 119 | 29.0 | 23.3 | 26.0 × 28.2 | 1,683,029 | 111,258 | 27 / 204 / 3,725 | 332,579 | 10,913 | 9,931 | 0.98 |
| 159 | 37.2 | 30.6 | 33.0 × 37.4 | 868,862 | 50,130 | 20 / 130 / 1,870 | 144,674 | 1,115 | 1,001 | 0.93 |
| 199 | 34.7 | 31.9 | 33.7 × 29.5 | 264,404 | 8,671 | 2 / 17 / 261 | 25,442 | 210 | 194 | 0.87 |
| 239 | 9.4 | 31.9 | 32.1 × 27.7 | 21,995 | 82 | 0 / 0 / 1 | 385 | 1 | 0 | 0.70 |
| 279 | 0.0 | 31.9 | 32.1 × 27.0 | 9,754 | 0 | 0 / 0 / 0 | 0 | 0 | 0 | 0.00 |

**What the curves show (measured, no proposal).**
- **SR's extent levels.** On both seeds the longest path rises to 35 to 39 m by about 140 to 180, then holds between 34 and 38 m (29.5 m at 299 on 1001). That is about one L_max (34.8 m).
  - **Height** (rough) stops at 32.8 m (1001, from 179) and 31.7 m (1007, from 159).
  - **Spread** holds at 28 to 34 m and narrows on one axis after 259.
- **SR reaches a steady state** from about 200. Living tips, framework tips and held phytomers stay within a few percent each decade from 219 to 299, with births over deaths at 0.99 to 1.00. The bough tips rise again on 1001 (87 at 199 to 193 at 299).
- **It settles lower than its peak.** It holds 54 to 59% (1001) and 43% (1007) of its peak living tips, which came at 119.
- **Income falls while the leaves hold.** On 1001 income falls from 36,350 at 119 to 3,673 at 299 while leaves hold at about 330,000 from 199.
- **PR starves the crown.** Its income falls to nothing (7 on 1001 at 299; 0 on 1007 from 279), and on 1007 the tree has no living tip from 279.

## 4. Stills (eye)

**Sheets.**
- **`square-oak-1001-bare.jpg` and `square-oak-1001-whole.jpg`:** rows 40, 80, 120, 200, 300; columns today, QR, SR, PR.
- **`square-oak-1007-bare.jpg` and `square-oak-1007-whole.jpg`:** rows 120, 200, 300. QR at 300 is a grey tile, refused for a limb under the ground (REMAIN-RESULT.md).
- **`square-others.jpg`:** today beside SR, leafless then leafed, for the other presets.

## 5. Measures (measured)

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB |
|---|---|---|---|---|---|---|---|---|---|
| oak-40-1001 | A | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,279 | 325,454 | 0.7 |
|  | QR | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,394 | 328,565 | 0.7 |
|  | SR | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,532 | 329,158 | 0.7 |
|  | PR | 10.6 | 7.5 × 7.8 | 53 | 4.7 | 3 | 45,371 | 297,094 | 0.7 |
| oak-80-1001 | A | 17.5 | 17.9 × 14.4 | 110 | 4.7 | 3 | 209,845 | 1,593,559 | 2.0 |
|  | QR | 17.5 | 18.9 × 14.6 | 110 | 4.7 | 3 | 221,110 | 1,758,425 | 2.2 |
|  | SR | 17.5 | 19.1 × 14.9 | 115 | 4.7 | 3 | 227,393 | 1,856,352 | 2.3 |
|  | PR | 16.9 | 16.8 × 14.1 | 82 | 4.7 | 3 | 150,199 | 1,019,038 | 1.5 |
| oak-120-1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 2,370,405 | 3.1 |
|  | QR | 24.8 | 27.6 × 26.1 | 139 | 4.7 | 5 | 413,650 | 3,360,155 | 4.1 |
|  | SR | 24.7 | 27.4 × 25.9 | 137 | 4.7 | 5 | 363,719 | 3,159,003 | 3.8 |
|  | PR | 21.8 | 25.2 × 26.8 | 91 | 4.7 | 5 | 179,913 | 1,267,110 | 1.8 |
| oak-200-1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2,108,291 | 2.8 |
|  | QR | 30.8 | 51.0 × 50.1 | 153 | 4.7 | 5 | 601,248 | 4,455,756 | 5.6 |
|  | SR | 22.6 | 29.1 × 37.2 | 82 | 4.7 | 3 | 97,454 | 1,541,098 | 2.3 |
|  | PR | 14.4 | 26.5 × 24.1 | 50 | 4.7 | 3 | 29,967 | 322,683 | 1.0 |
| oak-300-1001 | A | 40.2 | 26.0 × 37.2 | 120 | 25.5 | 1 | 233,403 | 1,537,433 | 2.9 |
|  | QR | 33.2 | 82.7 × 67.8 | 141 | 4.7 | 5 | 413,859 | 2,804,129 | 4.3 |
|  | SR | 8.3 | 13.7 × 20.2 | 36 | 4.7 | 2 | 66,310 | 1,655,424 | 2.4 |
|  | PR | 7.7 | 12.2 × 13.9 | 27 | 4.7 | 2 | 0 | 150,100 | 1.0 |
| oak-120-1007 | A | 25.5 | 28.0 × 20.5 | 150 | 5.6 | 5 | 386,363 | 2,879,040 | 4.1 |
|  | QR | 24.9 | 34.1 × 35.0 | 192 | 4.5 | 10 | 732,660 | 4,711,934 | 5.9 |
|  | SR | 24.9 | 35.4 × 35.2 | 189 | 4.5 | 10 | 599,507 | 4,881,176 | 5.5 |
|  | PR | 21.1 | 31.7 × 34.4 | 119 | 4.5 | 9 | 286,250 | 1,576,144 | 2.3 |
| oak-200-1007 | A | 37.1 | 29.7 × 34.0 | 226 | 14.4 | 2 | 619,213 | 4,121,440 | 5.0 |
|  | QR | 33.5 | 52.1 × 59.7 | 206 | 4.3 | 10 | 1,099,630 | 6,469,030 | 8.4 |
|  | SR | 23.0 | 29.2 × 24.9 | 89 | 4.5 | 8 | 156,803 | 2,823,600 | 3.4 |
|  | PR | 12.0 | 16.0 × 26.1 | 42 | 4.5 | 2 | 10,227 | 123,519 | 1.3 |
| oak-300-1007 | A | 31.9 | 34.9 × 24.0 | 106 | none | 0 | 94,815 | 681,125 | 4.0 |
|  | QR | refused: a limb 0.002 m under the ground | | | | | | | 4.5 |
|  | SR | 15.3 | 16.2 × 13.6 | 51 | 4.9 | 4 | 102,800 | 2,290,382 | 3.5 |
|  | PR | 7.4 | 0.4 × 0.7 | 19 | none | 0 | 0 | 164 | 1.2 |
| beech-40-1001 | A | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,359 | 0.4 |
|  | SR | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,365 | 0.4 |
| beech-80-1001 | A | 17.4 | 19.6 × 20.2 | 130 | n/a | n/a | 463,750 | 1,887,629 | 2.6 |
|  | SR | 17.4 | 19.6 × 20.2 | 132 | n/a | n/a | 480,147 | 2,055,505 | 2.8 |
| birch-40-1001 | A | 16.7 | 7.7 × 8.5 | 30 | n/a | n/a | 139,929 | 374,880 | 0.7 |
|  | SR | 16.7 | 7.7 × 8.4 | 30 | n/a | n/a | 140,946 | 378,165 | 0.7 |
| spruce-40-1001 | A | 12.4 | 7.9 × 7.3 | 34 | n/a | n/a | 4,185,635 | 1,137,653 | 2.0 |
|  | SR | 12.4 | 7.8 × 7.3 | 35 | n/a | n/a | 4,406,389 | 1,211,003 | 2.1 |
| palm-40-1001 | A | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 |
|  | SR | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 |
| ordinary-40-1001 | A | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 |
|  | SR | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 |

## 6. Each fork limb's shape under SR (measured, settled tree)

The limbs born in 13 and 14 on the stem (`raw/oldtree/shape-SR-*`). QR's are in REMAIN-RESULT.md, and Q's in OLDTREE.md.

| seed | age | born | base radius cm | length m | reach m | tip − base m | lowest of the line m | lowest of the subtree m | m a year |
|---|---|---|---|---|---|---|---|---|---|
| 1001 | 160 | 13 | 34.8 | 31.6 | 20.4 | +15.8 | 4.75 | 4.75 | 0.202 |
| 1001 | 160 | 13 | 37.3 | 15.7 | 10.2 | +2.0 | 4.75 | 4.75 | 0.000 |
| 1001 | 160 | 14 | 20.5 | 13.1 | 5.1 | −5.0 | 0.02 | 0.00 | 0.000 |
| 1001 | 200 | 13 | 34.1 | 15.7 | 10.8 | +2.8 | 4.75 | 3.42 | 0.000 |
| 1001 | 200 | 13 | 14.7 | 5.5 | 4.4 | −0.0 | 4.15 | 2.43 | 0.000 |
| 1001 | 200 | 14 | 22.2 | 13.1 | 3.8 | −5.0 | 0.04 | 0.00 | 0.000 |
| 1001 | 240 | 13 | 13.1 | 11.1 | 2.9 | −4.6 | 0.09 | 0.00 | 0.000 |
| 1001 | 240 | 13 | 1.1 | 0.4 | 0.2 | +0.4 | 4.73 | 4.73 | 0.000 |
| 1001 | 240 | 14 | 25.3 | 13.0 | 3.3 | −5.0 | 0.05 | 0.05 | 0.000 |
| 1001 | 300 | 13 | 15.6 | 11.1 | 5.3 | −3.9 | 0.85 | 0.00 | 0.000 |
| 1001 | 300 | 14 | 5.0 | 2.2 | 1.2 | +1.8 | 5.06 | 5.06 | 0.000 |
| 1007 | 160 | 13 | 34.6 | 14.8 | 13.1 | −2.4 | 2.12 | 2.12 | 0.000 |
| 1007 | 160 | 14 | 21.4 | 18.6 | 10.9 | +8.5 | 4.89 | 4.89 | 0.128 |
| 1007 | 200 | 14 | 36.9 | 20.1 | 12.2 | +11.6 | 4.89 | 4.89 | 0.000 |
| 1007 | 200 | 14 | 16.7 | 12.7 | 4.8 | −4.7 | 0.12 | 0.08 | 0.000 |
| 1007 | 240 | 14 | 38.1 | 20.2 | 5.9 | −2.0 | 2.90 | 1.89 | 0.000 |
| 1007 | 240 | 13 | 10.3 | 7.3 | 5.6 | +2.6 | 4.58 | 4.58 | 0.000 |
| 1007 | 300 | 14 | 10.4 | 3.7 | 3.3 | +0.2 | 4.88 | 2.77 | 0.000 |
| 1007 | 300 | 16 | 21.8 | 9.4 | 2.7 | −0.4 | 5.09 | 1.53 | 0.000 |

Fuller tables are in `shape-SR-*-<seed>.log`.

**What the shapes show** (measured).
- **SR's fork limbs are shorter** than QR's and Q's at the same ages: 0.4 to 15.6 m from 200 on 1001 (QR 13 to 33 m) and 3.7 to 20.2 m on 1007.
- **Some still lie on the ground.** The limb born in 14 on 1001 lies with its tip 5 m below its base and its lowest point 0.0 to 0.05 m from 160 to 240. Most of the others stand clear (lowest point 2 to 5 m) from 200 on 1007.
- **They do not extend** from 200 on either seed.

## 7. Cost, measured as a pair in one session (measured)

**How it was measured.** Both variants ran on one binary, the `square.patch` build, today's tree with no letters and SR with `PRMU`. Each run was pinned to one core and alternated age by age, with no other heavy job running.

| oak, seed 1001 | today s | SR s | SR / today | today peak GiB | SR peak GiB |
|---|---|---|---|---|---|
| 40 | 1.4 | 1.4 | 1.00 | 0.35 | 0.35 |
| 80 | 11.3 | 12.8 | 1.13 | 1.08 | 1.29 |
| 120 | 34.6 | 40.6 | 1.17 | 1.80 | 2.19 |
| 200 | 76.4 | 87.5 | 1.15 | 2.13 | 2.22 |
| 300 | 146.4 | 131.0 | 0.89 | 2.71 | 2.20 |

## 8. The read per tree and variant (eye, with the measures)

**Oak 40, seed 1001.**
- **SR: the same.**
- **PR: the same,** with 4% less leaf.

**Oak 80, seed 1001.**
- **SR: the same.** It is today's and QR's tree, leaf +8%.
- **PR: worse.**
  - **What is wrong:** the crown is thinner and more open, with leaf −28% and trunk 82 cm against 110.

**Oak 120, seed 1001.**
- **SR: better than today's.** It is QR's many-limbed crown, clothed through its depth, with 12% fewer leaves than QR (363,719 against 413,650) and 19% more than today's.
- **PR: worse.** A thin, bare-limbed open crown with leaf at the ends only (−41% leaf), 21.8 m tall.

**Oak 120, seed 1007.**
- **SR: better than today's.** It is QR's broad full dome on 10 limbs.
- **PR: worse than QR.** A thinner, more open dome.

**Oak 200, seed 1001.**
- **SR: worse than today's.**
  - **The crown:** a sparse, straggly crown 22.6 m tall and 29 × 37 m, thin throughout (97,454 leaves against today's 270,545), with short tufts on long bare wood.
  - **The limbs:** the lower limbs reach the ground on one side, and one long limb runs out along the ground to the right.
  - **The upside:** the stem line stands as a trunk to the crown.
- **PR: much worse.** A tangle of bare looping limbs on a spike (14.4 m, 29,967 leaves).

**Oak 200, seed 1007.**
- **SR: not worse than today's, not clearly better.** An upright tree 23 m tall and 29 × 25 m, with a central trunk and an open crown of crooked limbs, thinly clothed (156,803 leaves against 619,213). The limbs mostly stand clear of the ground. Today's is a bent trunk with a dense crown on top.
- **PR: much worse.** Limbs on the ground around a short trunk.

**Oak 300, seed 1001.**
- **SR: worse than today's.** A bare 8 m spike from whose top the stem line arcs over and down to the ground on the left, carrying the crown's leaf there (8.3 m, 66,310 leaves).
  - **Why, by the measures:** the census holds a 30 m rough tree with 116,724 living tips in a steady state. In the settled tree the sag lays that crown down.
- **PR: much worse.** A bare spike with one dead looping limb (0 leaves).

**Oak 300, seed 1007.**
- **SR: different from today's, not worse.** A small upright tree 15.3 m tall and 16 × 14 m on a straight trunk, with a crooked, knotted, open crown and some leaf (102,800 leaves). Today's is a 32 m twisted pole with a high tuft and a limb on the ground.
  - **What is wrong:** the crown is small, crooked and thin.
- **PR: dead.** A 7.4 m spike.

**Other presets under SR: the same as today's.** I checked every tile (beech 40 and 80, birch, spruce, palm, ordinary); measures are within 0 to 9%.

**The points the host named.**
- **Does the extent level off?** Yes, under SR in the census on both seeds: longest path 34 to 38 m, rough height 31.7 to 32.8 m, spread under 35 m.
- **Do the limbs stand clear of the ground at 200 and 300?**
  - **Seed 1007:** mostly yes.
  - **Seed 1001:** no. One fork limb lies on the ground from 160, and at 300 the stem line itself lies down with the crown.
- **Has the crown thinned at 80 and 120?**
  - **SR:** no (+8% and +19% leaf against today's).
  - **PR:** yes, by 28% and 41%.
- **Does the top die back?**
  - **SR on 1001:** the top is laid down rather than dead. The census's height holds at 30.2 m, but the settled tree stands 8.3 m at 300.
  - **SR on 1007:** the settled tree stands 23 m at 200 and 15.3 m at 300, against a rough height of 31.7 m. The top is lower than the census's by half.
  - **PR:** the top dies (1007: no living tip from 279).

## Open for the host (no rule proposed)

- **The settled tree against the census.** Under SR the census holds a 30 to 33 m crown in a steady state, while the settled tree stands 8 to 23 m from 200, so the sag decides what is seen at 200 and 300. Girth and sag were out of this probe's scope.
- **SR's leaf at 200.** The crown's leaf at 200 is a third (1001) and a quarter (1007) of today's settled count, with income at 12,240 and 15,365 against 36,350 and 43,176 at 119.

## Friction

None new.
