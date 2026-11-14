# fn-363 R5: grower and settle, the top against this branch (builder, Opus, 13 Nov 2026; partial)

**The spec's first Stop fired before any drawing. R6 was not drawn.** Telperion at its preset age, 82, seed 1, holds 3,393 m² of settled leaf on this branch against 2,400 on the top, +41% (Stop: more than 25%). The browser's ceiling test agrees at seed 42: Telperion keeps 6,120,872 phytomers against 3,614,135 (+69%) and Laurelin 5,422,804 against 3,382,695 (+60%), both over the 4,000,000 ceiling (PINS.md).

## The host's decision on the Stop (13 Nov 2026, evening)

The host's decision, in substance: go on. Telperion at 82, seed 1, rose from 2,400 to 3,393 m² of leaf (+41%) at an unchanged height of 43.8 m. The Two Trees stand on the beech's rows with a potential height of their own. Phase 2 recorded their trunks as too thin against the base (1.15 to 1.28 m against 2.7 to 3.0 m). More leaf under the height charge therefore moves them toward the base, not away from it, and the verdict is taken on the sheet. R6 adds Telperion and Laurelin at their preset ages on seeds 1 and 7, in three columns: base-2026-11-11, top, and this branch, both whole and leafless. Where the seed chain has no phytomer at 1.3 m, each tile prints the scoreboard tool's own trunk measure.

The host's message did not address the browser's ceiling test (PINS.md: Telperion 6.12 M and Laurelin 5.42 M phytomers at seed 42 against the 4 M ceiling), and it still fails on this branch.

## Method

- **before:** a detached checkout of `fn-351-phase-top` (`f17affbb3`) at `.worktrees/fn-363-before`. **after:** a detached checkout of `c2d897a37` (this branch's product; the later commits change only tests and evidence) at `.worktrees/fn-363-after`. Each has the surplus probe's reader applied uncommitted (`reader.patch`, `v4-census.patch`, `extras.patch` from the probe's scratch folder) and its own target.
- Each tree is the probe's runner (`oldoak/runner`, pointed at each checkout): `telperion_space::grow` on the preset's family at the age and seed, with `OLDOAK=1 SURPLUS=1`, one process per tree, each in its own `telperion-heavy` scope (`r5/runs.sh`, `r5/all.sh`). No tree is drawn. `r5/tables.py` builds the table from the logs.
- Height is the reader's top tip above the ground. Crown spread is the mean of its two ground axes. The trunk is twice the radius of the first phytomer at or above 1.3 m on the seed axis and its continuations. "-" means that chain carries no phytomer at 1.3 m, which happens on Telperion, Laurelin and some ordinary trees, and their trunk is **unknown** here. Leaf, leafed phytomers and phytomers come from the settled tree. The tallest living tip is above the root collar's base, beside the species' potential height. Sheds are the apexes the balance shed in the last 20 cycles, summed over every PA.
- Seconds are `peak.py`'s wall time inside the scope. "not timed" means a PAUSE of the yield watcher fell inside the tree's window.
- White's open-grown ranges for the oak's trunk at 1.3 m, in m, as quoted in fn-352's SCOREBOARD.md: 0.28 to 0.40 at 40, 0.56 to 0.79 at 80, 0.83 to 1.09 at 120, 1.21 to 1.52 at 200, 1.56 to 1.92 at 300.

## Not measured when this was written

The yield watcher froze every job from 19:33:16, and this file was written at 20:07 with the pause still on. The rows below "in progress" or missing were not grown. That covers the beech at 80 after; the birch, palm at 80, Laurelin and Telperion at 40 and 80 after; Telperion 82 seed 7 after; and the oak at 120, 200 and 300 on seeds 1001, 1007 and 1012, both runs. `r5/all.sh` resumes the batch and keeps every finished tree. **All of those numbers are unknown.**

## Table (what was measured)

| preset | age | seed | run | height m | mean crown spread m | trunk at 1.3 m, m | settled leaf m² | leafed phytomers | phytomers | tallest living tip m / potential | apexes shed by the balance, last 20 cycles | peak GiB | seconds |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ordinary | 40 | 1 | before | 12.3 | 8.1 | - | 17 | 18,588 | 98,097 | 12.3 | 0 | 0.1 | 1 |
| ordinary | 40 | 1 | after | 12.3 | 8.1 | - | 17 | 18,588 | 98,097 | 12.3 | 0 | 0.1 | 1 |
| ordinary | 40 | 7 | before | 13.1 | 9.8 | 0.11 | 27 | 25,902 | 140,416 | 13.1 | 0 | 0.1 | 1 |
| ordinary | 40 | 7 | after | 13.1 | 9.8 | 0.11 | 27 | 25,902 | 140,416 | 13.1 | 0 | 0.1 | 1 |
| ordinary | 80 | 1 | before | 22.8 | 14.3 | - | 101 | 119,122 | 741,107 | 22.8 | 0 | 0.4 | 3 |
| ordinary | 80 | 1 | after | 22.8 | 14.3 | - | 101 | 119,122 | 741,107 | 22.8 | 0 | 0.4 | 4 |
| ordinary | 80 | 7 | before | 23.2 | 17.5 | 0.31 | 183 | 205,657 | 1,231,672 | 23.2 | 0 | 0.7 | 5 |
| ordinary | 80 | 7 | after | 23.2 | 17.5 | 0.31 | 183 | 205,657 | 1,231,672 | 23.2 | 0 | 0.6 | 6 |
| oak | 40 | 1 | before | 10.9 | 7.7 | 0.24 (out of 0.28 to 0.40) | 88 | 76,250 | 342,606 | 10.9 / 34.8 | 0 | 0.2 | 1 |
| oak | 40 | 1 | after | 10.7 | 7.5 | 0.27 (out of 0.28 to 0.40) | 115 | 79,721 | 353,937 | 10.7 / 34.8 | 0 | 0.2 | 1 |
| oak | 40 | 7 | before | 10.8 | 7.8 | 0.29 (in) | 133 | 111,769 | 490,724 | 10.8 / 34.8 | 0 | 0.3 | 1 |
| oak | 40 | 7 | after | 11.1 | 7.4 | 0.33 (in) | 180 | 114,782 | 497,320 | 11.1 / 34.8 | 0 | 0.3 | 1 |
| oak | 80 | 1 | before | 17.7 | 14.6 | 0.61 (in) | 381 | 395,345 | 1,989,079 | 17.7 / 34.8 | 3,453 | 1.1 | 9 |
| oak | 80 | 1 | after | 17.0 | 17.8 | 0.73 (in) | 562 | 424,715 | 2,110,204 | 17.0 / 34.8 | 531 | 1.2 | 10 |
| oak | 80 | 7 | before | 19.4 | 23.8 | 0.79 (just out of 0.56 to 0.79) | 658 | 775,929 | 4,033,277 | 19.4 / 34.8 | 30,185 | 2.6 | 18 |
| oak | 80 | 7 | after | 20.2 | 24.0 | 0.98 (out of 0.56 to 0.79) | 1,016 | 754,660 | 3,869,037 | 20.2 / 34.8 | 43,917 | 2.4 | 19 |
| spruce | 40 | 1 | before | 11.6 | 6.5 | 0.26 | 170 | 434,820 | 1,115,420 | 11.6 / 61.0 | 0 | 0.8 | 3 |
| spruce | 40 | 1 | after | 11.6 | 6.5 | 0.26 | 170 | 434,820 | 1,115,420 | 11.6 / 61.0 | 0 | 0.8 | 4 |
| spruce | 40 | 7 | before | 11.8 | 6.0 | 0.26 | 165 | 418,949 | 1,081,326 | 11.8 / 61.0 | 0 | 0.8 | 3 |
| spruce | 40 | 7 | after | 11.8 | 6.0 | 0.26 | 165 | 418,949 | 1,081,326 | 11.8 / 61.0 | 0 | 0.8 | 4 |
| spruce | 80 | 1 | before | 21.6 | 11.6 | 0.56 | 483 | 1,297,614 | 4,358,762 | 21.6 / 61.0 | 1,026 | 2.8 | 22 |
| spruce | 80 | 1 | after | 21.6 | 11.6 | 0.56 | 484 | 1,300,354 | 4,365,283 | 21.6 / 61.0 | 333 | 2.8 | 28 |
| spruce | 80 | 7 | before | 21.8 | 10.8 | 0.56 | 484 | 1,298,226 | 4,331,400 | 21.8 / 61.0 | 1,010 | 2.8 | 22 |
| spruce | 80 | 7 | after | 21.8 | 10.8 | 0.56 | 484 | 1,298,940 | 4,334,096 | 21.8 / 61.0 | 762 | 2.8 | not timed |
| beech | 40 | 1 | before | 10.0 | 6.4 | - | 89 | 89,711 | 485,877 | 10.0 / 40.0 | 0 | 0.2 | 1 |
| beech | 40 | 1 | after | 10.0 | 6.4 | - | 89 | 89,711 | 485,877 | 10.0 / 40.0 | 0 | 0.2 | 1 |
| beech | 40 | 7 | before | 10.4 | 8.4 | - | 220 | 167,205 | 796,872 | 10.4 / 40.0 | 127 | 0.3 | 1 |
| beech | 40 | 7 | after | 10.4 | 8.4 | - | 220 | 167,400 | 797,119 | 10.4 / 40.0 | 60 | 0.3 | 1 |
| beech | 80 | 1 | before | 17.4 | 18.1 | - | 1,036 | 793,821 | 5,037,729 | 17.4 / 40.0 | 23,665 | 1.9 | 12 |
| beech | 80 | 7 | before | 18.4 | 21.7 | - | 1,352 | 996,269 | 6,170,796 | 18.4 / 40.0 | 78,212 | 2.4 | 18 |
| birch | 40 | 1 | before | 16.7 | 7.6 | 0.21 | 55 | 60,557 | 262,553 | 16.7 / 30.0 | 0 | 0.1 | 1 |
| birch | 40 | 7 | before | 16.7 | 7.1 | 0.23 | 71 | 101,702 | 463,021 | 16.7 / 30.0 | 0 | 0.2 | 1 |
| birch | 80 | 1 | before | 25.4 | 16.4 | 0.46 | 205 | 259,007 | 1,379,093 | 25.4 / 30.0 | 24,901 | 0.7 | 6 |
| birch | 80 | 7 | before | 26.3 | 15.8 | 0.49 | 227 | 329,317 | 1,768,402 | 26.3 / 30.0 | 18,883 | 0.9 | 8 |
| palm | 40 | 1 | before | 16.3 | 0.0 | 0.59 | 1,349 | 480 | 480 | 16.3 | 0 | 0.0 | 0 |
| palm | 40 | 1 | after | 16.3 | 0.0 | 0.59 | 1,349 | 480 | 480 | 16.3 | 0 | 0.0 | 0 |
| palm | 40 | 7 | before | 16.3 | 0.0 | 0.59 | 1,349 | 480 | 480 | 16.3 | 0 | 0.0 | 0 |
| palm | 80 | 1 | before | 32.5 | 0.0 | 0.59 | 2,698 | 960 | 960 | 32.5 | 0 | 0.0 | 0 |
| palm | 80 | 7 | before | 32.5 | 0.1 | 0.59 | 2,698 | 960 | 960 | 32.5 | 0 | 0.0 | 0 |
| Laurelin | 40 | 1 | before | 25.8 | 14.4 | - | 412 | 87,385 | 454,284 | 25.8 / 40 | 703 | 0.2 | 1 |
| Laurelin | 40 | 7 | before | 26.9 | 21.6 | 0.65 | 960 | 155,080 | 705,847 | 26.8 / 40 | 6,505 | 0.3 | 1 |
| Laurelin | 80 (preset) | 1 | before | 43.6 | 46.9 | - | 2,587 | 445,319 | 2,653,460 | 43.6 / 40 | 211,209 | 1.2 | 10 |
| Laurelin | 80 (preset) | 7 | before | 44.1 | 56.2 | 1.28 | 1,546 | 275,473 | 1,793,515 | 44.1 / 40 | 353,559 | 1.2 | 12 |
| Telperion | 40 | 1 | before | 25.8 | 14.4 | - | 388 | 90,434 | 469,672 | 25.8 / 40 | 392 | 0.2 | 1 |
| Telperion | 40 | 7 | before | 26.9 | 21.6 | 0.63 | 909 | 160,479 | 734,646 | 26.8 / 40 | 5,827 | 0.3 | 1 |
| Telperion | 80 | 1 | before | 43.5 | 46.9 | - | 2,522 | 476,314 | 2,856,172 | 43.5 / 40 | 208,856 | 1.3 | 10 |
| Telperion | 80 | 7 | before | 44.5 | 56.2 | 1.26 | 1,565 | 308,531 | 1,891,317 | 44.6 / 40 | 359,522 | 1.2 | 12 |
| Telperion | 82 (preset) | 1 | before | 43.8 | 48.1 | - | 2,400 | 461,574 | 2,834,654 | 43.8 / 40 | 229,554 | 1.3 | 11 |
| Telperion | 82 (preset) | 1 | **after** | 43.8 | 47.8 | - | **3,393 (+41%)** | 611,270 (+32%) | 3,701,260 (+31%) | 43.8 / 40 | 185,576 | 1.6 | 10 |
| Telperion | 82 (preset) | 7 | before | 45.1 | 57.1 | 1.26 | 1,405 | 287,197 | 1,729,973 | 45.1 / 40 | 363,068 | 1.2 | 13 |

Telperion's and Laurelin's potential height is 40 m (`inverseHeight = 0.025` in their values). Telperion's per-PA values are not all 0.025, which this reading did not check; the "/ 40" in their rows is an assumption.

## The first Stop, against these numbers

- **Fired: Telperion at 82, seed 1.** Leaf is +41% (2,400 to 3,393 m²). Height is unchanged (43.8 m). The trunk is unknown (no phytomer at 1.3 m on the seed chain). The seed-42 ceiling read gives +69% phytomers for Telperion and +60% for Laurelin at their preset ages (PINS.md), so Laurelin at 80 very likely fires too. Its seed 1 and 7 trees after were not grown before the pause, so that is unmeasured.
- **Held, measured:** the ordinary tree, the spruce, the palm, and the beech at 40 are unchanged or move under 1% at 40 (and 80 for the ordinary tree and spruce). The birch after is unmeasured.
- **The oak (exempt from the Stop):** at 40, leaf rises 31% and 35%, the trunk 0.24 to 0.27 and 0.29 to 0.33 (the limb row's removal; fn-361 measured it: leaf 119 against 187 m² on the median). At 80, seed 7's trunk goes from 0.79 (just past White's 0.56 to 0.79) to 0.98 m. Leaf rises 48% and 54%.
- No tallest living tip passes its potential height at 40 or 80 on any natural species. Telperion and Laurelin pass their 40 m on both runs at 80 (43.5 to 45.1 m). That predates this change.

## Why the Two Trees move (reading, not run)

Their crowns are wide (46 to 57 m at 80), so a tip's path from the collar runs far longer than its height. The path charged them heavily, and the top sheds 209,000 to 364,000 apexes by the balance in the last 20 cycles. Charged by height, the same tips pay less, and Telperion at 82, seed 1, sheds 185,576 against 229,554. The natural presets at 40 and 80 shed little or nothing either way, so they barely move. This is a reading of the table. Whether the Two Trees' rows (inverse height, hazard, tolerance) or the ceiling should answer it is the host's call, under the spec's fourth Stop: a preset row.
