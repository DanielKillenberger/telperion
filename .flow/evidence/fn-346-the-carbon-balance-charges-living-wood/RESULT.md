# fn-346: the charge by pipe path, P1 and P2 on the oak and every preset

Drafted by the worker on 11 Nov 2026 at `bd2d1bb03`, for the host. This is the run in `R1-ANSWERS.md`, steps 1 to 6. The probe is `probes/pipes.patch`, reverted; `git status --short crates` is empty. The variants are instruments; nothing here proposes a design.

Marks: **measured** (printed by a run), **arithmetic** (from run files), **eye** (read off a still), **read** (from the code).

## Headline

- **Both variants bound the oak, then take it apart.** Under P1 the whole tree's charge reaches its income at 99, stays at 0.98 to 1.10 of it to 199, and rises to 1.5 to 2.0 after the last limb is gone. Living tips then fall every decade, from 63,625 at 99 to 5,202 at 199 and 149 at 299 (measured). P2 takes the same path sooner and further.
- **The oak at 40 holds.** Height is 10.6 m in all three, and current-year leaf falls 6%, from 47,279 to 44,495 (measured). So the step-3 stop did not apply.
- **From 80 every oak is worse than today's (eye):**
  - **At 80** the crown is thinner: P1 has 39% fewer leaves and P2 48%.
  - **At 120** the crown is stripped from the inside: bare limbs with tufts at their ends, and 69% and 82% fewer leaves.
  - **At 200 and 300** the tree is wreckage or a bare pole. P1 at 300 on seed 1007 was refused by the pipeline because a limb lies below the ground.
- **Every other preset is the same as today's** (eye; measures within 0 to 5%, spruce leaves +5%).
- **The doubled instrument (P2, L_max = 69.6 m)** holds the young oak and its low limbs to 300.
  - It keeps 4 or 5 limbs from 4.7 m.
  - Held phytomers peak at 3.90 million at 159 and fall to 1.18 million at 299.
  - Its charge sits at 0.80 to 0.88 of income from 139 on.
  - The tree still declines after 160, more slowly.

## 1. The recount (arithmetic, from R1's census files)

**The restated charge.** A tip costs size × L / 34.8, where size = `base_scale × running`, summed over a subtree's living tips over the whole path to the root, with dead wood adding nothing. The script is `probes/recount.py`.

| tree, year | living tips | income | restated charge | charge / income | rank ≤ 7 on the main line below zero | all main-line laterals below zero |
|---|---|---|---|---|---|---|
| A, 39 | 24,622 | 6,382 | 4,521 | 0.71 | 0 of 9 | 6 of 48 |
| A, 79 | 111,239 | 26,907 | 33,646 | 1.25 | 10 of 16 (the three fork limbs at −0.10 to −0.18) | 29 of 45 |
| A, 119 | 158,355 | 40,960 | 71,195 | 1.74 | 21 of 25 (fork limbs −0.27 and −0.31) | 47 of 56 |
| B, 119 | 307,400 | 61,484 | 119,080 | 1.94 | 29 of 32 | 59 of 67 |
| B, 179 | 803,025 | 142,382 | 473,460 | 3.33 | 47 of 47 (fork limbs −0.50 to −0.60) | 82 of 82 |

The share of tips whose lattice light is under L / 34.8, by band of path L:

| L m | A 39 | A 79 | A 119 | B 119 | B 179 |
|---|---|---|---|---|---|
| 0 to 5 | 0.00 | | | | |
| 5 to 10 | 0.51 | 0.81 | | 0.00 | |
| 10 to 15 | 0.02 | 0.90 | 1.00 | 0.96 | 1.00 |
| 15 to 20 | | 0.69 | 1.00 | 1.00 | 1.00 |
| 20 to 25 | | | 0.98 | 0.99 | 1.00 |
| 25 to 30 | | | 0.95 | 0.95 | 1.00 |
| 30 to 35 | | | 0.24 | 0.72 | 1.00 |
| 35 to 55 | | | | | 1.00 |

Read against today's tree, the rule leaves 40 nearly untouched and puts most of the crown below zero by 80.

## 2. The probe (read)

`probes/pipes.patch` is `census.patch` plus the following.

- **The path.** The lay's walk (`geometry/lay.rs` `Layer`) carries each axis's path from the root collar as H did: a base path from its bearer at its node (or the bearer's end), then one f32 per phytomer laid. Under letter `P` it never changes a length. A tip's L is its pencil's `layer.path_end()`. The pencils move with compaction, so the path survives retirement. It was available where the balance is summed, so the rule could be built as written.
- **The charge** (`grow/cycle/passes.rs` `balance_one`).
  - A lit axis's own cost is `base_scale × running × L / L_max`, and an unlit axis's own cost is 0, so there is no per-metre upkeep.
  - A closed child adds nothing (`working.rs`, as B).
  - Light and every other value are as today.
  - `L_max` comes from `PIPES_LMAX`, times `PIPES_LMAX_FACTOR`. Unset, the charge is today's per-metre upkeep with dead wood not charged.
- **P2 (letter `R`).** An axis whose link is a continuation or relay also decides on its own subtree. Only the seed's axis, which has no bearer, never decides.
- **L_max per species**, as fn-345 sourced them: oak 34.8 m, beech 40, birch 30, spruce 61. Palm and ordinary are unset, so the rule is dormant there.
  - The `R` letter is not dormant on palm and ordinary under P2, but neither tree moved.
- **Runs.** All runs are `probes/pipes.sh` under `telperion-heavy`. The census runs used `12 --gpu 2 --threads 4`, stopping at eight times the most A had held or 8 GiB; the stills used `22 --gpu 2`; the one-thread seconds used `12 --threads 1` with `taskset -c 3`. No run was stopped by its census, and none was killed. Logs are in `raw/stills/`.
- **Today's stills.** Today's stills and measures are fn-345's `A` runs: the crates are unchanged since, and fn-345's probe was dormant for A. The oak at 80, which fn-345 did not draw, was drawn here.

## 3 and 4. The census curves, oak seed 1001 (measured; every 20 years)

Height and spread are the rough layout's, before the settle. Limbs are the living limb-row laterals on the stem line. "Charge" is the root's whole charged upkeep (`w.cost[0]`). Every 10-year row comes from `python3 probes/curves.py <log>`.

**Today (A)** (from R1's `raw/A-1001/run.log`)

| year | height m | spread m | held | living tips | limbs | lowest limb m | income | charge | charge / income |
|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 6.8 × 8.0 | 302,606 | 24,676 | 3 | 4.7 | 6,382 | 1,987 | 0.31 |
| 79 | 17.4 | 16.4 × 14.1 | 1,645,899 | 114,047 | 3 | 4.7 | 26,907 | 17,775 | 0.66 |
| 119 | 25.9 | 24.4 × 22.4 | 2,506,773 | 165,292 | 4 | 4.7 | 40,960 | 53,245 | 1.30 |
| 159 | 32.8 | 19.5 × 27.1 | 1,958,386 | 136,033 | 3 | 18.1 | 37,752 | 80,383 | 2.13 |
| 199 | 38.7 | 26.0 × 22.9 | 2,179,327 | 151,418 | 3 | 22.5 | 35,012 | 123,586 | 3.53 |
| 239 | 44.9 | 19.2 × 25.8 | 2,225,565 | 174,416 | 2 | 25.4 | 23,841 | 162,892 | 6.83 |
| 299 | 50.2 | 27.2 × 29.0 | 1,598,756 | 95,211 | 1 | 32.2 | 37,230 | 210,724 | 5.66 |

**P1** (`raw/stills/oak-300-1001-P1-census.log`; reached 300 in 15 s at four threads, 0.75 GiB)

| year | height m | spread m | held | living tips | limbs | lowest limb m | income | charge | charge / income |
|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.5 | 6.7 × 7.8 | 277,319 | 21,639 | 3 | 4.7 | 6,480 | 4,067 | 0.63 |
| 59 | 13.9 | 11.9 × 11.9 | 634,888 | 43,651 | 3 | 4.7 | 13,994 | 11,094 | 0.79 |
| 79 | 17.0 | 14.9 × 12.4 | 923,899 | 57,261 | 3 | 4.7 | 21,882 | 19,471 | 0.89 |
| 99 | 20.6 | 18.3 × 15.8 | 1,052,193 | 63,625 | 4 | 4.7 | 26,250 | 25,852 | 0.98 |
| 119 | 23.0 | 20.8 × 19.0 | 895,234 | 51,099 | 4 | 4.7 | 21,524 | 21,001 | 0.98 |
| 139 | 25.1 | 22.6 × 21.9 | 596,663 | 32,839 | 3 | 4.7 | 13,172 | 13,158 | 1.00 |
| 159 | 28.2 | 20.2 × 17.7 | 359,919 | 20,876 | 1 | 5.0 | 8,364 | 8,439 | 1.01 |
| 179 | 32.2 | 17.4 × 17.7 | 199,924 | 10,543 | 1 | 5.0 | 4,552 | 4,681 | 1.03 |
| 199 | 35.9 | 18.1 × 17.7 | 124,152 | 5,202 | 1 | 5.0 | 2,401 | 2,629 | 1.10 |
| 209 | 36.8 | 18.2 × 17.7 | 29,661 | 229 | 0 | - | 192 | 287 | 1.49 |
| 259 | 42.4 | 14.9 × 12.4 | 5,314 | 170 | 0 | - | 96 | 164 | 1.71 |
| 299 | 47.3 | 17.1 × 6.1 | 2,895 | 149 | 0 | - | 126 | 254 | 2.02 |

**P2** (`raw/stills/oak-300-1001-P2-census.log`; reached 300 in 12 s, 0.68 GiB)

| year | height m | spread m | held | living tips | limbs | lowest limb m | income | charge | charge / income |
|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.5 | 6.7 × 7.8 | 277,193 | 21,628 | 3 | 4.7 | 6,479 | 4,066 | 0.63 |
| 59 | 13.8 | 11.8 × 11.8 | 611,430 | 40,659 | 3 | 4.7 | 13,322 | 10,222 | 0.77 |
| 79 | 16.8 | 14.7 × 12.2 | 868,358 | 51,234 | 3 | 4.7 | 19,708 | 16,912 | 0.86 |
| 99 | 20.0 | 17.9 × 15.7 | 921,934 | 52,546 | 4 | 4.7 | 22,079 | 20,870 | 0.95 |
| 119 | 22.9 | 20.7 × 18.1 | 762,363 | 36,284 | 3 | 4.7 | 15,329 | 14,057 | 0.92 |
| 139 | 25.7 | 21.5 × 18.9 | 564,002 | 21,068 | 3 | 4.7 | 8,800 | 7,831 | 0.89 |
| 159 | 24.1 | 17.8 × 16.9 | 276,914 | 10,040 | 2 | 4.7 | 4,038 | 3,441 | 0.85 |
| 179 | 24.1 | 17.7 × 17.6 | 136,938 | 2,919 | 2 | 4.7 | 1,164 | 965 | 0.83 |
| 199 | 19.6 | 15.3 × 14.4 | 81,517 | 1,624 | 2 | 4.7 | 586 | 474 | 0.81 |
| 249 | 19.1 | 15.3 × 8.8 | 38,440 | 440 | 1 | 5.0 | 75 | 45 | 0.60 |
| 299 | 19.1 | 15.3 × 8.8 | 31,578 | 199 | 1 | 5.0 | 9 | 5 | 0.58 |

- **The top stops at 139 to 149.** The stem line's top holds at 19.1 m from about 99, and the tree's height falls from 25.7 m at 139 to 19.1 m.
- **The crown keeps only fragments.** From 159, every decade it keeps fewer tips.

**The instrument: P2 with L_max doubled to 69.6 m** (`raw/stills/oak-300-1001-P2x2-census.log`; census only, not drawn; 95 s, 2.47 GiB)

| year | height m | spread m | held | living tips | limbs | lowest limb m | income | charge | charge / income |
|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 6.8 × 8.0 | 304,231 | 24,702 | 3 | 4.7 | 6,400 | 2,268 | 0.35 |
| 79 | 17.4 | 17.6 × 14.3 | 1,745,879 | 115,553 | 3 | 4.7 | 28,712 | 17,213 | 0.60 |
| 119 | 25.4 | 24.3 × 22.3 | 3,242,481 | 191,775 | 5 | 4.7 | 54,624 | 40,812 | 0.75 |
| 139 | 28.5 | 26.6 × 26.2 | 3,743,131 | 210,840 | 5 | 4.7 | 63,482 | 50,816 | 0.80 |
| 159 | 31.5 | 27.8 × 30.3 | 3,901,146 | 211,591 | 5 | 4.7 | 66,351 | 55,553 | 0.84 |
| 179 | 34.4 | 28.5 × 34.1 | 3,756,068 | 200,981 | 5 | 4.7 | 67,033 | 57,834 | 0.86 |
| 199 | 37.7 | 32.0 × 34.5 | 3,323,395 | 167,683 | 5 | 4.7 | 59,114 | 51,763 | 0.88 |
| 219 | 40.8 | 36.7 × 37.5 | 2,897,931 | 142,987 | 5 | 4.7 | 49,356 | 43,604 | 0.88 |
| 239 | 40.1 | 37.8 × 36.7 | 2,140,125 | 93,420 | 5 | 4.7 | 25,539 | 20,998 | 0.82 |
| 259 | 41.6 | 36.9 × 35.6 | 1,676,267 | 70,577 | 5 | 4.7 | 15,110 | 12,075 | 0.80 |
| 299 | 41.6 | 34.4 × 29.0 | 1,183,699 | 53,774 | 4 | 4.7 | 8,692 | 7,034 | 0.81 |

The settled tree at 300 stands 23.0 m tall with 4 living limbs (`SETTLED` line).

**How the tree answers the doubling** (arithmetic).
- **It answers strongly, in the expected direction.** At 119 the doubled tree holds 3.24 million phytomers against P2's 0.76 million and today's 2.51 million.
- **It keeps its fork limbs at 4.7 m to 300.**
- **Its height grows to 41.6 m by 259 and then stops.** Its spread reaches 38 m by 239.
- **The decline is only delayed.** Held phytomers fall 15% from 159 to 199 and 64% from 199 to 299. Income falls from 68,389 at 169 to 8,692 at 299 while the ratio stays at 0.80 to 0.88.

**The common pattern (arithmetic).**
- **Charge tracks income.** In P1, P2 and the instrument alike, the whole charge climbs to a fixed share of income (about 1.0 in P1, 0.85 to 0.95 in P2, 0.80 to 0.88 doubled) and then tracks income downward.
- **Tips decline, never level.** The living tips fall every decade once that share is reached. Nothing in the record replaces the tips the balance sheds.

## 5. Stills and measures (measured)

**Sheets.** Each has one row per tree, with leafless A, P1 and P2, then leafed A, P1 and P2.
- **`sheet-oak-1001.jpg`:** the oak at 40, 80, 120, 200 and 300.
- **`sheet-oak-1007.jpg`:** the oak at 120 and 300. P1 at 300 is a grey tile, refused.
- **`sheet-others.jpg`:** the beech at 40 and 80, and the birch, spruce, palm and ordinary at 40, the age fn-345 drew them at.

**Measures** (`python3 -I probes/measures.py`).
- **Height and spread** are the mesh's. **Trunk** is the diameter at 1.3 m.
- **Lowest living limb and limbs** are the settled stem line's limb row.
- **Leaves** are the current year's. **Held** is the settled phytomers.
- **Peak GiB** is the drawn run's. **1-thread s** is the grow alone, pinned to one core.

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB | 1-thread s |
|---|---|---|---|---|---|---|---|---|---|---|
| oak-40-1001 | A | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,279 | 325,454 | 0.7 | 1.2 |
|  | P1 | 10.6 | 7.4 × 7.8 | 52 | 4.7 | 3 | 44,495 | 289,829 | 0.6 | 1.2 |
|  | P2 | 10.6 | 7.4 × 7.8 | 52 | 4.7 | 3 | 44,513 | 289,629 | 0.6 | 1.2 |
| oak-80-1001 | A | 17.5 | 17.9 × 14.4 | 110 | 4.7 | 3 | 209,845 | 1,593,559 | 2.0 | 10.5 |
|  | P1 | 16.8 | 16.6 × 13.8 | 77 | 4.7 | 3 | 128,433 | 874,551 | 1.4 | 7.2 |
|  | P2 | 16.6 | 16.3 × 13.4 | 74 | 4.7 | 3 | 109,316 | 808,382 | 1.3 | 6.7 |
| oak-120-1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 2,370,405 | 3.1 | 32.4 |
|  | P1 | 21.4 | 26.6 × 22.1 | 76 | 4.7 | 4 | 95,300 | 795,831 | 1.3 | 15.3 |
|  | P2 | 20.9 | 26.2 × 21.2 | 67 | 4.7 | 3 | 54,501 | 589,387 | 1.0 | 13.2 |
| oak-200-1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2,108,291 | 2.8 | 70.6 |
|  | P1 | 18.2 | 31.4 × 19.1 | 44 | 5.0 | 1 | 7,959 | 89,834 | 0.7 | 21.4 |
|  | P2 | 12.3 | 13.0 × 17.2 | 27 | 5.0 | 1 | 1,604 | 52,382 | 0.7 | 17.3 |
| oak-300-1001 | A | 40.2 | 26.0 × 37.2 | 120 | 25.5 | 1 | 233,403 | 1,537,433 | 2.9 | 133.8 |
|  | P1 | 14.5 | 15.5 × 11.2 | 43 | none | 0 | 108 | 2,834 | 0.7 | 21.6 |
|  | P2 | 7.7 | 0.4 × 0.4 | 20 | 5.0 | 1 | 0 | 8,017 | 0.7 | 17.5 |
| oak-120-1007 | A | 25.5 | 28.0 × 20.5 | 150 | 5.6 | 5 | 386,363 | 2,879,040 | 4.1 | 57.0 |
|  | P1 | 21.0 | 32.0 × 24.5 | 86 | 4.5 | 5 | 94,044 | 881,347 | 1.4 | 19.0 |
|  | P2 | 20.4 | 27.2 × 27.0 | 80 | 4.5 | 6 | 66,776 | 574,424 | 1.1 | 17.0 |
| oak-300-1007 | A | 31.9 | 34.9 × 24.0 | 106 | none | 0 | 94,815 | 681,125 | 4.0 | 193.6 |
|  | P1 | refused: "axis 12 (PA 6, born in cycle 74, base at 0.035 m) reaches 0.023 m below the ground" | | | | | | | 0.9 | 22.6 |
|  | P2 | 7.4 | 0.5 × 0.7 | 20 | none | 0 | 0 | 164 | 0.9 | 18.6 |
| beech-40-1001 | A | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,359 | 0.4 | 0.4 |
|  | P1 | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,365 | 0.3 | 0.5 |
|  | P2 | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,365 | 0.4 | 0.4 |
| beech-80-1001 | A | 17.4 | 19.6 × 20.2 | 130 | n/a | n/a | 463,750 | 1,887,629 | 2.6 | 7.6 |
|  | P1 | 17.4 | 19.6 × 20.2 | 128 | n/a | n/a | 454,904 | 1,902,265 | 2.6 | 8.0 |
|  | P2 | 17.4 | 19.6 × 20.2 | 128 | n/a | n/a | 450,557 | 1,889,081 | 2.6 | 7.8 |
| birch-40-1001 | A | 16.7 | 7.7 × 8.5 | 30 | n/a | n/a | 139,929 | 374,880 | 0.7 | 1.4 |
|  | P1 | 16.7 | 7.7 × 8.4 | 30 | n/a | n/a | 139,336 | 374,871 | 0.7 | 1.5 |
|  | P2 | 16.7 | 7.7 × 8.4 | 30 | n/a | n/a | 139,336 | 374,871 | 0.7 | 1.5 |
| spruce-40-1001 | A | 12.4 | 7.9 × 7.3 | 34 | n/a | n/a | 4,185,635 | 1,137,653 | 2.0 | 5.1 |
|  | P1 | 12.4 | 7.8 × 7.3 | 35 | n/a | n/a | 4,406,389 | 1,211,003 | 2.1 | 5.2 |
|  | P2 | 12.4 | 7.8 × 7.3 | 35 | n/a | n/a | 4,406,389 | 1,211,003 | 2.1 | 5.3 |
| palm-40-1001 | A | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 | 0.3 |
|  | P1 | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 | 0.3 |
|  | P2 | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 | 0.3 |
| ordinary-40-1001 | A | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 | 0.6 |
|  | P1 | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 | 0.6 |
|  | P2 | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 | 0.6 |

## 6. The read per tree (eye, against today's, with the measures beside)

**Oak 40, seed 1001: the same, P1 and P2 alike.**
- **Unchanged:** the same young tree, the same leader, the same low limbs along the ground, and the same height.
- **Slightly fewer leaves and thinner trunk:** 6% fewer leaves and 52 cm against 56 cm of trunk. The leafed crowns cannot be told apart at this size.

**Oak 80, seed 1001: worse, P2 more than P1.**
- **Where it falls short:** the crown. The open fork V and its spread are kept, but the crown is see-through, with each limb carrying sparse leaf clusters and gaps between them, where today's is a dense mass.
- **How much:** leaves fall 39% (P1) and 48% (P2), and the trunk goes from 110 cm to 77 and 74 cm.
- **It has thinned from the inside.** The lower and inner twigs are gone first, consistent with the recount: at 79, 90% of the tips at 10 to 15 m of path stood under L / 34.8.

**Oak 120, seeds 1001 and 1007: worse.**
- **The crown is stripped from the inside.** The limbs stand bare along most of their length and carry tufts only at their ends. The form reads as a skeleton with leaf at the rim.
- **On 1001** leaves fall 69% (P1) and 82% (P2). The trunk halves (143 cm to 76 and 67 cm). Height falls 15 to 17%.
- **On 1007** leaves fall 76% and 83%. Today's dense, irregular dome becomes a broad open frame, wider than today's (32.0 m against 28.0 m under P1).
- **No limb is lost whole at 120 on either seed**, since 4 to 6 limbs are living. The loss is the fine wood between and along them.
- **The top has not died back yet:** the stem line still carries a living top.

**Oak 200, seed 1001: much worse.**
- **What is left:** a few long, snaking bare limbs with small leaf tufts at the ends.
- **P1:** one limb, 18.2 m tall, 7,959 leaves.
- **P2:** a bare spike of a stem with one limb lying out to the side, 12.3 m tall, 1,604 leaves.
- **Gone:** the crown and the top. The limbs are lost whole and leave nothing in their place. The long limb that remains on P1 coils back on itself in a loop.
- **Today's oak at 200 is a pole with a crown on top. Under the probe the crown is gone.**

**Oak 300, seeds 1001 and 1007: much worse.**
- **P1 on 1001:** a 14.5 m stump-like trunk carrying one thick limb that loops twice in the air, with 108 leaves.
- **P2 on 1001 and 1007:** a bare tapering spike of 7.4 to 7.7 m with no leaf, the seed's axis alone. Its subtree above was shed, as P2 allows everywhere above the seed's own axis.
- **P1 on 1007:** refused by the pipeline. A limb born in year 74 has its last point 0.023 m below the ground.
- **Nothing reads as an oak at 300.**

**Beech 40 and 80, birch 40, spruce 40, palm 40, ordinary 40: the same as today's, P1 and P2.**
- **Every tile is the same tree** at the same height and spread.
- **Leaf and held phytomers differ by 0 to 5%.** The spruce has 5% more leaves and 6% more held phytomers, because no dead wood is charged and its own per-metre upkeep is replaced. The beech at 80 has 2 to 3% fewer leaves.
- **Why the beech and birch barely move** (arithmetic, inferred). At 40 and 80, their tips' paths are short against their L_max of 40 and 30 m.

**What the stills and curves point to (measured and inferred, no proposal).**
- **No regrowth.** Under the restated charge the crown's balance sits at about income from 99 (P1). The balance sheds or fades whatever is negative, and nothing in the record replaces it: living tips fall every decade.
- **Where the loss shows (eye).** The loss shows first as thinning from inside the crown (80, 120), then as whole limbs lost with nothing in their place (200), then as the bare stem (300).
- **Doubling L_max delays the fall.** It does not change it: the doubled tree also declines after 160.

## Open for the host

- **Whether the decline is the rule's or the shedding's.** The curves show the charge tracking income downward, while the tips shed are not replaced. Whether a bounded tree needs the shed tips replaced (sleeping buds, or a tip's growth reading its balance) is not settled in the spec ("whether shedding alone carries the bound"). That is the host's.
- **The refused tree.** P1 on 1007 at 300 is refused for a limb below the ground. Today's tree on 1007 at 300 also has a limb running along the ground (eye, `sheet-oak-1007.jpg`), but it is drawn.
- **P2 on palm and ordinary.** The `R` letter is active on presets whose charge is dormant. Neither tree moved at 40.

## Friction

None new in this run. See `FRICTION.md` for R1's two hook refusals.
