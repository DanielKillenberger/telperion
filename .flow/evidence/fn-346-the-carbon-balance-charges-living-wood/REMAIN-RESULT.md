# fn-346: the balance counts what would remain (QR, QRB)

Drafted by the worker on 11 Nov 2026 at `e0de8f1fe` plus the host's R5 answers, for the host. This is `R5-ANSWERS.md`, "The run", steps 1 to 6. The probe is `probes/remain.patch`, and it was reverted; `git status --short crates` is empty. Nothing here proposes a design.

Marks: **measured** (printed by a run), **arithmetic** (from the run's lines), **read** (from the code), **eye** (read off a still).

## Headline

- **The rule does what it was written to do (measured).** It sheds the part that does not pay and keeps the rest.
  - **What is shed:** from 119 to 299 on seed 1001, QR sheds 669 boughs and 17 limbs by their own balance, against Q's 78 and 4. Only 12 of those boughs and 1 of those limbs had a paying child the year before (Q: 66 of 78 and 3 of 4).
  - **Seed 1007:** 34 of 2,196 boughs and 0 of 78 limbs.
  - **Whole limbs** are no longer lost while part of them pays.
- **The framework holds, then thins slowly (measured).**
  - **QR on 1001:** living bough tips rise from 118 at 119 to 441 at 259 (Q: 15 at 259), and branch tips from 5,295 to 7,914.
  - **Living tips** stay between 250,000 and 265,000 from 159 to 259, then fall to 186,300 at 299. Births over deaths stand at 0.98 to 1.00 from 159 on.
  - **Seed 1007** runs the same course, with living tips at 405,000 to 457,000 from 159 to 259 and 320,000 at 299.
- **The tree keeps spreading and lies on the ground (measured, eye).** QR's settled oak on 1001 is 51 × 50 m on 30.8 m at 200, and 82.7 × 67.8 m on 33.2 m at 300. QRB on 1007 is 82.5 × 81.4 m on 34.0 m at 300.
  - **The fork limbs lie down:** every fork limb measured on 1001 lies with its tip 3.6 to 5.0 m below its base, its lowest point 0.01 to 1.1 m above the ground, from 200 on (one at 200 still rises, at +7.4 m).
  - **Seed 1007 at 300:** both Q and QR are refused for a limb reaching under the ground; QRB was drawn.
  - **The rough layout keeps rising:** 57 m on 1001 at 299, still gaining about 3 m a decade, and 58.7 m on 1007.
- **By eye:**
  - **The young oak has not moved:** at 40 and 80 QR and QRB are Q and today.
  - **At 120 both are Q's better oak, fuller.**
  - **At 200 both are clothed through the middle, where Q is not, on Q's flat 50 m spread.** On 1007 they are a heaped mound with no clear trunk.
  - **At 300 both are a sprawl of limbs lying on the ground, 60 to 83 m wide, worse than today's.**
- **The other presets are the same as today's under QR and under QRB (eye, measured).** QRB adds leaf: the ordinary preset +16%, the birch +8%.
- **Cost (measured, one session, one thread, one build).** QR's grow costs the same as today's at 40, +9% at 80, +13% at 120, +61% at 200 and +53% at 300. Peak memory is the same at 300 and +42% at 200.

## How it was built

**The base.** The probe is `probes/oldtree.patch` (RENEWAL's census, the old-tree census and Q's letters) plus three things.
- **Rule B as built in `renew.patch`.** Its grower part in `dormant.rs` and `grow/wake.rs` keeps a bud from ageing and holds it while its bearer lives. It wakes at the rate × max(0, 1 − cost / light), and the lattice light is read at each held bud's place. Its preset part in the example's `tuning.rs` puts sleeping buds on the limb, bough and branch rows, laid as limb → branch, bough → branch, branch → twig. Rule B is letter `S`.
- **The rule (letter `M`), in `Working::totals` (`grow/cycle/working.rs`).** An open child adds its light and charge to its bearer as today, except a child lateral whose own light is below its own charge (each counted the same way, so recursively), which adds nothing.
  - **What counts as a lateral.** A lateral is a child whose link stands at a node: `Origin::Lateral`. A relay or continuation is the bearer's own line, "the tips on its own line, as today's sums take them", and is summed as today even where it does not pay. That is the worker's reading of "own"; it is stated here so the host can check it.
  - **What else changes.** The `living` flag is unchanged, and dead wood adds nothing, as Q.
- **Zero balance.** Where a decider's light and charge are both 0, its balance is 0 (`grow/cycle/passes.rs`); today's `shed::now` gives 1 there.

**Buildable as written (read).** `Working::totals` sums in reverse order of the open list, and a child always stands after its bearer in that list. So each child's result is known when its bearer is summed.

**Memory and the rest.** The memory, hazard, `kept`, tolerance and idle rule read the balance unchanged.

**Variants.** Q is letters `PR`; QR is `PRM`; QRB is `PRMS`. All three run with `PIPES_LMAX` 34.8 × `PIPES_LMAX_FACTOR` 2.

**Runs.** All runs went under `telperion-heavy`. No run was killed by the memory cap.
- **Census:** `probes/remain.sh`, logs in `raw/remain/`.
- **Stills:** `probes/pipes.sh`, logs in `raw/stills/`.
- **Shape:** `probes/oldtree.sh shape` with `LETTERS=PRM TAG=QR UNSAGGED=0`, logs in `raw/oldtree/shape-QR-*`.
- **Cost pair:** `pipes.sh` time mode, today (no letters) and QR alternated on the same binary, no other heavy job running.
- **Sheets and tables:** the sheets come from `probes/remain_sheets.sh`; the curves from `probes/renew_curves.py <log> 20` and `probes/remain_lines.py <log> 20`.

**Today's stills** are fn-345's (same crates), and today's oak at 80 was drawn in this spec's first run. Q's stills at 1001 are the last round's; Q on 1007 was drawn here.

## 1. The census to 300 (measured; every 20 years)

**Columns.**
- **Tips by row:** living tips of the limb, bough and branch rows.
- **Limbs on stem:** living limb-row laterals of the stem line.
- **Height and spread:** the rough layout's, before the settle.
- **B / D:** births over deaths in the decade.
- **Held and woken:** sleeping buds held, and buds woken in the decade.

**QR, seed 1001**

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | B / D | buds held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 6.8 × 8.0 | 304,231 | 24,702 | 4 / 23 / 842 | 3 | 4.7 | 69,004 | 6,367 | 2,232 | 1.21 | 56 | 9 |
| 79 | 17.4 | 17.7 × 14.3 | 1,777,493 | 118,256 | 6 / 65 / 3,335 | 3 | 4.7 | 348,590 | 27,011 | 15,419 | 1.06 | 97 | 8 |
| 119 | 25.6 | 24.7 × 22.7 | 3,416,271 | 207,847 | 9 / 118 / 5,295 | 5 | 4.7 | 646,693 | 45,874 | 31,981 | 1.03 | 214 | 26 |
| 159 | 32.3 | 28.6 × 31.4 | 4,366,343 | 253,484 | 11 / 174 / 6,467 | 5 | 4.7 | 808,201 | 52,883 | 42,090 | 1.00 | 276 | 31 |
| 199 | 39.9 | 33.1 × 38.4 | 4,533,934 | 264,493 | 20 / 228 / 7,185 | 5 | 4.7 | 840,353 | 45,205 | 38,499 | 0.99 | 302 | 29 |
| 239 | 46.8 | 42.4 × 40.5 | 4,287,766 | 260,734 | 44 / 374 / 7,824 | 5 | 4.7 | 806,170 | 28,302 | 24,852 | 1.00 | 718 | 64 |
| 259 | 50.9 | 45.6 × 42.1 | 4,034,947 | 249,672 | 55 / 441 / 7,914 | 5 | 4.7 | 753,819 | 20,227 | 17,706 | 0.99 | 823 | 112 |
| 279 | 54.3 | 47.4 × 45.0 | 3,654,643 | 224,312 | 48 / 376 / 7,139 | 5 | 4.7 | 677,784 | 16,137 | 14,188 | 0.98 | 679 | 96 |
| 299 | 57.4 | 46.4 × 47.2 | 3,101,607 | 186,300 | 44 / 320 / 5,964 | 5 | 4.7 | 564,783 | 14,656 | 12,662 | 0.98 | 36 | 88 |

**QRB, seed 1001**

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | B / D | buds held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 6.9 × 8.1 | 313,022 | 25,526 | 4 / 23 / 865 | 3 | 4.7 | 71,063 | 6,487 | 2,288 | 1.21 | 4,665 | 133 |
| 79 | 17.4 | 17.9 × 14.5 | 1,891,957 | 127,854 | 6 / 65 / 3,612 | 3 | 4.7 | 372,295 | 27,918 | 16,199 | 1.06 | 24,976 | 544 |
| 119 | 25.8 | 24.4 × 22.8 | 3,732,171 | 227,636 | 9 / 113 / 5,789 | 5 | 4.7 | 707,220 | 50,084 | 34,891 | 1.03 | 42,300 | 935 |
| 159 | 32.2 | 28.5 × 31.4 | 4,860,782 | 287,814 | 11 / 168 / 7,225 | 5 | 4.7 | 915,476 | 62,578 | 48,925 | 1.01 | 54,146 | 956 |
| 199 | 39.9 | 33.1 × 37.0 | 5,307,622 | 313,232 | 19 / 216 / 8,180 | 5 | 4.7 | 1,001,232 | 61,596 | 50,920 | 1.00 | 61,514 | 1,025 |
| 239 | 46.7 | 42.3 × 40.4 | 5,211,628 | 310,060 | 41 / 336 / 8,529 | 5 | 4.7 | 985,328 | 49,475 | 40,950 | 0.99 | 65,060 | 960 |
| 259 | 51.0 | 45.6 × 42.0 | 4,769,364 | 284,514 | 48 / 385 / 8,191 | 5 | 4.7 | 890,170 | 41,607 | 33,790 | 0.99 | 64,125 | 831 |
| 279 | 54.4 | 47.3 × 44.2 | 4,210,823 | 248,946 | 42 / 317 / 7,115 | 5 | 4.7 | 780,951 | 38,273 | 30,999 | 0.98 | 57,598 | 778 |
| 299 | 57.0 | 48.5 × 46.5 | 3,710,184 | 208,517 | 33 / 255 / 6,036 | 5 | 4.7 | 661,459 | 35,301 | 28,721 | 0.98 | 20,226 | 729 |

**QR, seed 1007** (the settled tree refused at 300; the census to 299 is complete)

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | B / D | buds held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.1 | 7.6 × 7.2 | 435,510 | 39,047 | 10 / 53 / 1,556 | 8 | 4.5 | 99,762 | 8,784 | 3,520 | 1.20 | 123 | 13 |
| 79 | 17.8 | 21.3 × 21.9 | 2,593,929 | 181,748 | 14 / 152 / 5,583 | 8 | 4.5 | 517,484 | 39,070 | 23,622 | 1.05 | 323 | 36 |
| 119 | 25.0 | 30.6 × 30.4 | 4,996,617 | 314,539 | 30 / 269 / 8,991 | 10 | 4.5 | 954,976 | 68,846 | 50,504 | 1.02 | 538 | 59 |
| 159 | 33.5 | 35.8 × 40.6 | 6,510,358 | 405,093 | 54 / 482 / 11,916 | 10 | 4.5 | 1,240,446 | 73,728 | 60,101 | 1.00 | 1,029 | 116 |
| 199 | 42.3 | 46.3 × 47.4 | 6,872,071 | 441,905 | 84 / 663 / 14,121 | 10 | 4.5 | 1,326,832 | 50,080 | 44,163 | 1.00 | 1,693 | 187 |
| 239 | 50.8 | 55.8 × 57.4 | 6,593,204 | 446,986 | 114 / 915 / 16,035 | 10 | 4.5 | 1,288,132 | 21,542 | 19,728 | 1.00 | 2,317 | 235 |
| 259 | 54.9 | 60.2 × 59.4 | 6,170,976 | 423,941 | 128 / 945 / 15,483 | 9 | 4.5 | 1,211,385 | 13,018 | 11,938 | 0.99 | 2,337 | 264 |
| 279 | 58.3 | 65.0 × 64.3 | 5,347,215 | 361,655 | 113 / 813 / 13,306 | 9 | 4.5 | 1,033,912 | 12,262 | 11,160 | 0.98 | 1,654 | 259 |
| 299 | 58.7 | 64.7 × 61.7 | 4,897,084 | 319,904 | 83 / 642 / 11,057 | 9 | 4.5 | 934,899 | 11,488 | 10,458 | 0.99 | 86 | 157 |

**QRB, seed 1007**

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | B / D | buds held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.1 | 7.6 × 7.3 | 441,601 | 39,764 | 10 / 53 / 1,579 | 8 | 4.5 | 101,225 | 8,842 | 3,559 | 1.20 | 8,195 | 135 |
| 79 | 17.8 | 21.5 × 22.2 | 2,701,424 | 192,490 | 14 / 150 / 5,937 | 8 | 4.5 | 541,595 | 40,196 | 24,576 | 1.06 | 39,873 | 613 |
| 119 | 25.1 | 30.3 × 30.4 | 5,427,697 | 346,164 | 28 / 257 / 9,774 | 10 | 4.5 | 1,044,604 | 74,770 | 55,161 | 1.02 | 68,745 | 1,064 |
| 159 | 33.5 | 35.6 × 40.4 | 7,328,724 | 457,465 | 49 / 452 / 13,018 | 10 | 4.5 | 1,412,390 | 88,300 | 71,158 | 1.01 | 94,462 | 1,165 |
| 199 | 42.3 | 46.0 × 47.2 | 8,064,445 | 517,235 | 84 / 653 / 15,733 | 10 | 4.5 | 1,575,198 | 70,284 | 59,750 | 1.00 | 115,545 | 1,025 |
| 239 | 50.6 | 55.5 × 56.3 | 7,781,300 | 514,972 | 125 / 931 / 17,447 | 10 | 4.5 | 1,518,265 | 43,081 | 36,694 | 1.00 | 127,149 | 950 |
| 259 | 54.7 | 59.9 × 59.7 | 7,212,965 | 483,692 | 141 / 963 / 16,904 | 9 | 4.5 | 1,415,584 | 33,166 | 27,895 | 0.99 | 127,300 | 783 |
| 279 | 58.5 | 64.9 × 62.5 | 6,591,924 | 437,991 | 137 / 890 / 15,193 | 8 | 4.5 | 1,287,235 | 29,505 | 25,065 | 0.99 | 116,932 | 773 |
| 299 | 61.2 | 65.3 × 61.1 | 5,928,174 | 383,103 | 113 / 734 / 12,999 | 8 | 4.5 | 1,135,354 | 27,154 | 23,546 | 0.99 | 38,374 | 642 |

**For comparison, Q.**
- **Seed 1001 (RENEW-RESULT.md):** held phytomers peak at 3.90 million at 159 and fall to 1.18 million at 299. Living tips fall to 53,774, with bough tips 15 and limb-row tips 0.
- **Seed 1007 (OLDTREE.md's history run):** living tips go from 252,919 at 159 to 61,619 at 279.

**Limb and bough lines ended, 119 to 299** (measured). In brackets: lines shed by their own balance that had a paying child the year before.

| variant | seed | limbs: own balance (paying child) | with bearer | died | boughs: own balance (paying child) | with bearer | died |
|---|---|---|---|---|---|---|---|
| Q | 1001 | 4 (3) | 6 | 3 | 78 (66) | 69 | 81 |
| QR | 1001 | 17 (1) | 17 | 3 | 669 (12) | 155 | 166 |
| QRB | 1001 | 19 (2) | 17 | 2 | 561 (7) | 168 | 154 |
| Q | 1007 | 13 (8) | 29 | 10 | 198 (176) | 254 | 176 |
| QR | 1007 | 78 (0) | 91 | 32 | 2,196 (34) | 717 | 429 |
| QRB | 1007 | 67 (1) | 78 | 24 | 2,176 (28) | 659 | 537 |

Per decade, from `remain_lines.py <log> 20`, the sheds rise from 199 on. QR on 1001 sheds 144 to 198 boughs every 20 years from 259 to 299, and limbs shed with their bearer rise to 8 at 279.

**What the curves show (measured, no proposal).**
- **What is shed now pays nothing.** Under QR, 2% of the boughs shed had a paying child (12 of 669 on 1001, 34 of 2,196 on 1007), against 85% under Q (66 of 78) and 89% (176 of 198).
- **QR sheds more, in smaller pieces.** It sheds about 9 times as many boughs as Q on 1001 and 11 times on 1007.
- **The crown holds 60 to 80 years longer than Q's.** Living tips peak at 264,493 (1001, 199) and 456,729 (1007, 219) and fall by about 30% by 299. Q's fell by 75% (1001) and 77% (1007, to 279) from its peak.
- **Income falls while leaves hold.** QR 1001's income falls from 52,883 at 159 to 14,656 at 299 while leaves fall only from 808,201 to 564,783. On 1007 income falls from 75,570 (139) to 11,488 while leaves stay above 900,000.
- **The extent does not settle.** Height (rough) gains 3 to 4 m every 20 years to 299 on 1001. Spread reaches 47 m (1001) and 65 m (1007) and is still growing at 279.

## 2. Stills (eye)

**Sheets.**
- **`remain-oak-1001-bare.jpg` and `remain-oak-1001-whole.jpg`:** rows 40, 80, 120, 200, 300; columns today, Q, QR, QRB.
- **`remain-oak-1007-bare.jpg` and `remain-oak-1007-whole.jpg`:** rows 120, 200, 300, same columns. Q and QR at 300 are grey tiles: the settled tree was refused for a limb reaching 0.015 and 0.002 m under the ground.
- **`remain-others.jpg`:** today, QR and QRB, leafless then leafed, for the other presets.

## 3. Measures (measured)

- **Height and spread** are the mesh's. **Trunk** is the diameter at 1.3 m.
- **Lowest living limb and limbs** are the settled stem line's limb row.
- **Leaves** are the current year's. **Held** is the settled phytomers. **Peak GiB** is the drawn run's.

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB |
|---|---|---|---|---|---|---|---|---|---|
| oak-40-1001 | A | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,279 | 325,454 | 0.7 |
|  | Q | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,432 | 328,331 | 0.7 |
|  | QR | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,394 | 328,565 | 0.7 |
|  | QRB | 10.6 | 7.7 × 8.0 | 57 | 4.7 | 3 | 48,857 | 338,042 | 0.7 |
| oak-80-1001 | A | 17.5 | 17.9 × 14.4 | 110 | 4.7 | 3 | 209,845 | 1,593,559 | 2.0 |
|  | Q | 17.5 | 18.8 × 14.5 | 109 | 4.7 | 3 | 217,924 | 1,719,507 | 2.2 |
|  | QR | 17.5 | 18.9 × 14.6 | 110 | 4.7 | 3 | 221,110 | 1,758,425 | 2.2 |
|  | QRB | 17.5 | 18.9 × 14.7 | 115 | 4.7 | 3 | 238,118 | 1,874,151 | 2.4 |
| oak-120-1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 2,370,405 | 3.1 |
|  | Q | 24.3 | 27.0 × 25.8 | 131 | 4.7 | 5 | 375,802 | 3,120,543 | 3.9 |
|  | QR | 24.8 | 27.6 × 26.1 | 139 | 4.7 | 5 | 413,650 | 3,360,155 | 4.1 |
|  | QRB | 25.1 | 27.4 × 25.7 | 145 | 4.7 | 5 | 462,005 | 3,698,998 | 4.5 |
| oak-200-1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2,108,291 | 2.8 |
|  | Q | 29.3 | 50.0 × 50.7 | 128 | 4.7 | 5 | 344,012 | 2,845,517 | 3.8 |
|  | QR | 30.8 | 51.0 × 50.1 | 153 | 4.7 | 5 | 601,248 | 4,455,756 | 5.6 |
|  | QRB | 31.8 | 49.0 × 49.8 | 160 | 4.7 | 5 | 712,359 | 5,044,141 | 6.4 |
| oak-300-1001 | A | 40.2 | 26.0 × 37.2 | 120 | 25.5 | 1 | 233,403 | 1,537,433 | 2.9 |
|  | Q | 23.1 | 40.3 × 24.8 | 64 | 4.7 | 4 | 33,737 | 1,030,396 | 2.5 |
|  | QR | 33.2 | 82.7 × 67.8 | 141 | 4.7 | 5 | 413,859 | 2,804,129 | 4.3 |
|  | QRB | 36.7 | 83.1 × 60.1 | 148 | 4.7 | 5 | 479,377 | 3,387,548 | 5.0 |
| oak-120-1007 | A | 25.5 | 28.0 × 20.5 | 150 | 5.6 | 5 | 386,363 | 2,879,040 | 4.1 |
|  | Q | 24.0 | 33.1 × 34.0 | 174 | 4.5 | 10 | 591,551 | 4,069,791 | 5.1 |
|  | QR | 24.9 | 34.1 × 35.0 | 192 | 4.5 | 10 | 732,660 | 4,711,934 | 5.9 |
|  | QRB | 25.0 | 34.4 × 35.2 | 198 | 4.5 | 10 | 794,393 | 5,175,669 | 6.3 |
| oak-200-1007 | A | 37.1 | 29.7 × 34.0 | 226 | 14.4 | 2 | 619,213 | 4,121,440 | 5.0 |
|  | Q | 29.7 | 38.9 × 55.0 | 150 | 3.6 | 9 | 363,650 | 3,229,617 | 4.3 |
|  | QR | 33.5 | 52.1 × 59.7 | 206 | 4.3 | 10 | 1,099,630 | 6,469,030 | 8.4 |
|  | QRB | 34.4 | 57.2 × 57.8 | 215 | 4.5 | 10 | 1,263,281 | 7,626,073 | 9.6 |
| oak-300-1007 | A | 31.9 | 34.9 × 24.0 | 106 | none | 0 | 94,815 | 681,125 | 4.0 |
|  | Q | refused: "axis 85 (PA 6, born in cycle 74, base at 0.657 m) reaches 0.015 m below the ground" | | | | | | | 3.0 |
|  | QR | refused: "axis 227 (PA 6, born in cycle 97, base at 0.147 m) reaches 0.002 m below the ground" | | | | | | | 4.5 |
|  | QRB | 34.0 | 82.5 × 81.4 | 190 | 1.9 | 8 | 868,948 | 5,453,614 | 7.8 |
| beech-40-1001 | A | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,359 | 0.4 |
|  | QR | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,365 | 0.4 |
|  | QRB | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 17,388 | 85,962 | 0.4 |
| beech-80-1001 | A | 17.4 | 19.6 × 20.2 | 130 | n/a | n/a | 463,750 | 1,887,629 | 2.6 |
|  | QR | 17.4 | 19.6 × 20.2 | 132 | n/a | n/a | 479,926 | 2,053,960 | 2.8 |
|  | QRB | 17.4 | 19.6 × 20.2 | 135 | n/a | n/a | 502,267 | 2,162,290 | 2.9 |
| birch-40-1001 | A | 16.7 | 7.7 × 8.5 | 30 | n/a | n/a | 139,929 | 374,880 | 0.7 |
|  | QR | 16.7 | 7.7 × 8.4 | 30 | n/a | n/a | 140,946 | 378,165 | 0.7 |
|  | QRB | 16.8 | 8.1 × 8.8 | 31 | n/a | n/a | 150,470 | 403,951 | 0.8 |
| spruce-40-1001 | A | 12.4 | 7.9 × 7.3 | 34 | n/a | n/a | 4,185,635 | 1,137,653 | 2.0 |
|  | QR | 12.4 | 7.8 × 7.3 | 35 | n/a | n/a | 4,406,389 | 1,211,003 | 2.1 |
|  | QRB | 12.4 | 7.7 × 7.2 | 37 | n/a | n/a | 4,776,400 | 1,283,348 | 2.1 |
| palm-40-1001 | A | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 |
|  | QR | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 |
|  | QRB | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 |
| ordinary-40-1001 | A | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 |
|  | QR | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 |
|  | QRB | 13.3 | 9.1 × 8.4 | 23 | n/a | n/a | 9,207 | 104,604 | 0.4 |

## 4. Cost, measured as a pair in one session (measured)

**How it was measured.**
- **One binary for both:** the `remain.patch` build, today's tree run with no letters (its probes dormant), QR with `PRM`.
- **One thread:** `taskset -c 3`, grow only, no yearly records.
- **Alternated:** the two alternated age by age, with no other heavy job running.

| oak, seed 1001 | today s | QR s | QR / today | today peak GiB | QR peak GiB |
|---|---|---|---|---|---|
| 40 | 1.3 | 1.3 | 1.00 | 0.35 | 0.35 |
| 80 | 11.4 | 12.4 | 1.09 | 1.07 | 1.27 |
| 120 | 34.6 | 39.1 | 1.13 | 1.81 | 2.30 |
| 200 | 75.7 | 121.7 | 1.61 | 2.12 | 3.00 |
| 300 | 145.5 | 221.9 | 1.53 | 2.77 | 2.78 |

The cost follows the held phytomers. At 200 QR holds 4.46 million settled phytomers against today's 2.11 million (section 3).

## 5. Each fork limb's shape under QR (measured)

The settled tree, the limbs born in years 13 and 14 on the stem at 4.5 to 5.0 m (`raw/oldtree/shape-QR-*`). Q's values are in OLDTREE.md.

| seed | age | born | base radius cm | length m | reach m | tip − base m | lowest of the line m | lowest of the subtree m | m a year, last 10 years |
|---|---|---|---|---|---|---|---|---|---|
| 1001 | 160 | 13 | 32.1 | 21.3 | 5.3 | −4.4 | 0.35 | 0.21 | 0.145 |
| 1001 | 160 | 13 | 53.3 | 31.4 | 18.1 | +19.0 | 4.75 | 3.46 | 0.190 |
| 1001 | 160 | 14 | 21.6 | 13.2 | 5.1 | −5.0 | 0.02 | 0.00 | 0.000 |
| 1001 | 200 | 13 | 32.7 | 18.0 | 5.4 | −4.7 | 0.02 | 0.00 | 0.000 |
| 1001 | 200 | 13 | 53.0 | 32.8 | 22.2 | +7.4 | 4.75 | 4.75 | 0.000 |
| 1001 | 200 | 14 | 21.1 | 13.1 | 3.4 | −5.0 | 0.03 | 0.00 | 0.000 |
| 1001 | 240 | 13 | 35.7 | 27.6 | 11.1 | −4.7 | 0.01 | 0.00 | 0.000 |
| 1001 | 240 | 13 | 54.8 | 33.0 | 19.3 | −3.6 | 1.12 | 0.00 | 0.000 |
| 1001 | 240 | 14 | 20.3 | 13.1 | 3.1 | −5.0 | 0.04 | 0.00 | 0.000 |
| 1001 | 300 | 13 | 38.8 | 18.0 | 5.2 | −4.7 | 0.05 | 0.00 | 0.000 |
| 1001 | 300 | 13 | 47.0 | 29.5 | 14.9 | −4.6 | 0.09 | 0.08 | 0.000 |
| 1001 | 300 | 14 | 19.5 | 13.1 | 5.0 | −5.0 | 0.05 | 0.05 | 0.000 |
| 1007 | 160 | 13 | 70.8 | 32.2 | 25.0 | +11.5 | 4.58 | 0.18 | 0.174 |
| 1007 | 200 | 13 | 60.8 | 39.8 | 19.9 | +9.0 | 4.58 | 0.04 | 0.138 |
| 1007 | 200 | 13 | 48.3 | 34.5 | 18.1 | +1.6 | 0.36 | 0.00 | 0.000 |
| 1007 | 240 | 13 | 47.0 | 35.2 | 19.3 | −1.8 | 0.19 | 0.05 | 0.000 |
| 1007 | 240 | 13 | 47.8 | 32.4 | 17.8 | −4.5 | 0.06 | 0.00 | 0.000 |

Seed 1007's other stem limbs (born in 14, 17 and 28) lie the same way from 200 (`shape-QR-*-1007.log`). The settled tree at 300 on 1007 was refused, as above.

**What the shapes show** (measured).
- **Under QR the fork limbs are thicker than Q's** (base radius up to 55 cm on 1001 and 71 cm on 1007, against Q's 47 and 64). They are as long (13 to 40 m) and reach 3 to 25 m out.
- **They lie on the ground.** From 200 on 1001, every fork limb but one lies with its tip 3.6 to 5.0 m below its base and its lowest point 0.01 to 1.1 m above the ground.
- **They stop extending** by 200 on 1001 and by 240 on 1007, as they did under Q.

## 6. The read per tree and variant (eye, with the measures)

**Oak 40, seed 1001: the same as today's and as Q, under QR and QRB.** QRB has 3% more leaves.

**Oak 80, seed 1001: the same as Q.** The leaf is +1% under QR and +9% under QRB.

**Oak 120, seed 1001: better than today's, as Q.**
- **Both:** the same many-limbed dome clothed through its depth.
- **QR** has 10% more leaves than Q, and **QRB** 23% more.

**Oak 120, seed 1007: better than today's.** QR and QRB are the broad, dense, many-limbed dome Q makes (34 × 35 m, 10 limbs, trunk 192 to 198 cm), denser than Q's.

**Oak 200, seed 1001: the same silhouette as Q, better clothed; still not clearly better than today's.**
- **The silhouette:** the same flat, 50 m wide crown on a 31 m tree, with limbs from 4.7 m.
- **Clothing:** leaves sit through the crown's middle and around the trunk, not only at the limb ends (QR 601,248 and QRB 712,359 leaves, against Q's 344,012).
- **What is wrong:**
  - **The flatness:** it is 50 m wide on 30 m of height.
  - **The limbs on the ground:** the lowest fork limbs lie on the ground.
  - **The dark low mass:** a dark mass of leaf and shoots fills under the main limbs around the trunk, most in QRB.

**Oak 200, seed 1007: worse than today's.**
- **What it is:** a heaped mound 52 to 57 m wide and 34 m tall, densely leafed (1.10 to 1.26 million leaves), with no trunk visible above the heap and limbs running out in every direction. Today's is a bent trunk with a dense crown on top.

**Oak 300, seed 1001: worse than today's.**
- **What it is:** a sprawl of long limbs lying on and along the ground, 83 m wide and 33 to 37 m tall (settled), clothed in leaf all along the limbs (414,000 to 479,000 leaves) but with no crown standing up. One limb rises to the right.
- **Whole limbs are not lost.** The form is a heap of limbs on the ground, not an old oak's dome.

**Oak 300, seed 1007.**
- **QRB: worse than today's.** It is a flat sprawl 82 × 81 m wide, with limbs on the ground and its lowest living limb at 1.9 m.
- **QR: refused** (a limb under the ground).

**The other presets under QR: the same as today's** in every tile (beech 40 and 80, birch, spruce, palm, ordinary). Measures move 0 to +9%.

**The other presets under QRB: the same.**
- **The beech at 80:** +8% leaves.
- **The birch:** slightly wider (8.1 × 8.8 m, +8% leaves), its trunk still clear.
- **The ordinary preset:** +16% leaves.
- **Palm and spruce:** the same as under QR.

**The points the host named.**
- **Limbs lost whole:** no, by the counts (2% of shed boughs had a paying child). Whole limbs are still shed with their bearer from 219 on, 1 to 8 every 20 years on 1001 and up to 47 on 1007 (280 to 299).
- **The crown clothed through its depth:** yes at 120 and 200 under QR and QRB. At 300 the leaf lies along limbs on the ground.
- **Gaps refilling:** no gap shows in the crown at 200, since no large limb was shed. At 300 no crown stands to show one.
- **How far the limbs run, and the ground:** 13 to 40 m long; the fork limbs lie on the ground from 200; the crown spreads to 83 m by 300.
- **The oak at 40, 80 and 120 against Q:** unmoved in form. Leaves are +1 to +10% under QR and +3 to +23% under QRB.

## Open for the host (no rule proposed)

- **The extent has no bound.** Under QR the rough layout's height gains 3 to 4 m every 20 years to 299, and the spread reaches 47 to 65 m and still grows. Income falls to 15 to 28% of its peak while the leaves hold at two thirds or more of theirs.
- **The fork limbs lie on the ground from 200** under QR as under Q. On 1007 the settled tree at 300 is refused for it (Q and QR); QRB was drawn.
- **The relay rule.** The rule sums a relay or continuation into its bearer whether it pays or not (the worker's reading of "own line"). Under P2's letter R, those children also decide on their own balance.

## Friction

None new in this run.
