# fn-361 R4: the oak's panel with the limb dominance (builder, Opus, 13 Nov 2026)

The panel of fn-352 (`scoreboard/run.sh`'s `_panel` stage and its probe), the oak at 40, 80, 120, 200 and 300 on seeds 1001 to 1012 (measures) and 7, 1001, 1007, 1012 (stills, whole and leafless), in three columns:

- **base-2026-11-11**: fn-352's base sheet run (`c187c68cd`), its stills read from the checkout that holds them.
- **no-row**: this branch's base `b13c0d1d0`, scoreboard label `fn361-norow`, its probe built from a detached checkout of that commit with its own target.
- **row**: `8f079717c`, label `fn361-d05`.

Sheets, height, trunk and leaf under each tile (`sheet.py`): `oak-40-80-{whole,bare}.jpg`, `oak-120-{whole,bare}.jpg`, `oak-200-{whole,bare}.jpg`, `oak-300-{whole,bare}.jpg`. Table: `measures.py` over each label's `measures.jsonl` and the trees' logs.

## Measures

Median (lowest to highest) over seeds 1001 to 1012. Crown spread is the mean of the two ground axes. "inside White's" counts the trees whose trunk at 1.3 m lies in White's open-grown *Q. robur* range at that age (0.28 to 0.40 m at 40, 0.56 to 0.79 at 80, 0.83 to 1.09 at 120, 1.21 to 1.52 at 200, 1.56 to 1.92 at 300; quoted in fn-352's SCOREBOARD.md). Peak GiB and seconds are each tree's own process (`peak.py`); seconds count only where no PAUSE of the yield watcher fell inside the tree's run. The base's seconds are from 11 Nov, before the watcher, under another gate's load.

| tree | run | drawn | height m | crown spread m | trunk 1.3 m, m | inside White's | leaf m2 | phytomers | peak GiB | seconds |
|---|---|---|---|---|---|---|---|---|---|---|
| oak 40 | base-2026-11-11 | 12/12 | 10.6 (10.3 to 11.1) | 8.4 (6.9 to 9.7) | 0.78 (0.56 to 0.94) | 0 of 12 in 0.28 to 0.40 | 185 (115 to 214) | 490,720 (325,454 to 654,115) | 0.58 (0.45 to 0.84) | 2 (2 to 3) |
| oak 40 | no-row | 12/12 | 10.6 (10.4 to 11.1) | 8.5 (7.4 to 9.9) | 0.33 (0.26 to 0.36) | 10 of 12 in 0.28 to 0.40 | 187 (88 to 240) | 505,004 (326,166 to 674,392) | 0.70 (0.50 to 0.83) | 2 (2 to 3) |
| oak 40 | row | 12/12 | 11.1 (10.6 to 12.0) | 8.9 (8.0 to 10.3) | 0.28 (0.22 to 0.31) | 5 of 12 in 0.28 to 0.40 | 119 (66 to 160) | 489,855 (320,827 to 678,090) | 0.65 (0.47 to 0.79) | 2 (2 to 3) |
| oak 80 | base-2026-11-11 | 12/12 | 19.4 (17.5 to 22.1) | 23.1 (16.1 to 27.7) | 1.99 (1.10 to 2.52) | 0 of 12 in 0.56 to 0.79 | 1,120 (548 to 1,417) | 3,284,404 (1,593,559 to 4,288,137) | 3.20 (1.89 to 3.88) | 41 (17 to 62) |
| oak 80 | no-row | 12/12 | 19.8 (18.1 to 22.0) | 24.1 (18.1 to 26.6) | 0.94 (0.69 to 1.01) | 3 of 12 in 0.56 to 0.79 | 1,435 (753 to 1,664) | 3,690,198 (1,852,188 to 4,980,525) | 4.05 (2.29 to 4.77) | 27 (15 to 41); 1 not timed |
| oak 80 | row | 12/12 | 19.4 (18.0 to 20.9) | 20.4 (15.6 to 24.6) | 0.75 (0.58 to 0.85) | 9 of 12 in 0.56 to 0.79 | 931 (526 to 1,187) | 3,940,601 (1,779,662 to 5,374,370) | 3.88 (2.04 to 4.77) | 27 (13 to 40); 2 not timed |
| oak 120 | base-2026-11-11 | 12/12 | 27.6 (23.9 to 29.2) | 30.6 (23.0 to 38.8) | 2.02 (1.43 to 2.80) | 0 of 12 in 0.83 to 1.09 | 1,409 (796 to 2,460) | 4,801,158 (2,370,405 to 7,357,477) | 5.00 (2.81 to 6.81) | 85 (50 to 123) |
| oak 120 | no-row | 12/12 | 28.9 (25.6 to 31.4) | 35.2 (24.2 to 38.9) | 1.42 (1.07 to 1.50) | 1 of 12 in 0.83 to 1.09 | 1,842 (1,085 to 2,163) | 6,162,578 (3,100,796 to 9,175,834) | 5.95 (3.59 to 8.10) | 86 (44 to 132) |
| oak 120 | row | 12/12 | 27.3 (25.5 to 29.3) | 31.8 (25.5 to 35.9) | 1.23 (0.93 to 1.32) | 3 of 12 in 0.83 to 1.09 | 1,606 (886 to 1,811) | 9,499,612 (3,969,512 to 14,392,838) | 7.92 (4.14 to 10.44) | 126 (43 to 160) |
| oak 200 | base-2026-11-11 | 12/12 | 32.4 (21.4 to 37.1) | 25.5 (19.8 to 32.4) | 1.35 (1.02 to 2.74) | 5 of 12 in 1.21 to 1.52 | 638 (309 to 1,834) | 1,766,405 (849,410 to 4,573,179) | 4.54 (2.57 to 5.87) | 175 (78 to 335) |
| oak 200 | no-row | 12/12 | 30.2 (25.6 to 34.8) | 28.3 (23.3 to 36.3) | 1.80 (1.33 to 2.06) | 2 of 12 in 1.21 to 1.52 | 1,213 (297 to 2,026) | 7,099,312 (988,917 to 21,847,299) | 6.83 (1.81 to 17.60) | 221 (67 to 442) |
| oak 200 | row | 10/12 | 28.8 (23.1 to 30.4) | 29.4 (18.2 to 33.1) | 1.76 (1.33 to 1.86) | 2 of 10 in 1.21 to 1.52 | 1,380 (872 to 1,537) | 17,915,169 (4,850,195 to 30,278,650) | 13.58 (5.09 to 23.94) | 471 (133 to 724); 1 not timed |
| oak 300 | base-2026-11-11 | 12/12 | 35.3 (26.3 to 48.1) | 26.4 (20.6 to 38.4) | 1.28 (0.89 to 1.73) | 4 of 12 in 1.56 to 1.92 | 561 (245 to 1,249) | 1,499,942 (681,125 to 3,151,362) | 4.48 (2.96 to 6.45) | 300 (208 to 394) |
| oak 300 | no-row | 10/12 | 27.7 (7.8 to 30.7) | 23.4 (3.0 to 26.7) | 2.01 (1.35 to 2.39) | 3 of 10 in 1.56 to 1.92 | 512 (0 to 1,822) | 2,795,280 (84,740 to 31,028,589) | 3.77 (1.54 to 23.86) | 214 (73 to 841); 2 not timed |
| oak 300 | row | 5/12 | 29.7 (27.8 to 31.9) | 24.4 (18.8 to 30.2) | 1.81 (1.63 to 2.26) | 3 of 5 in 1.56 to 1.92 | 700 (523 to 1,003) | 12,745,620 (3,877,576 to 23,870,486) | 9.27 (4.29 to 16.97) | 425 (225 to 913) |

## Trees that could not be drawn in 24 GB

Each tree ran in its own `telperion-heavy 24` scope; the kernel OOM killer ended these at the scope's 24 GB (journal `oom-kill`, memory peak 24G); `OOM.txt` lists them with times.

- **row, 200:** seeds 1002 (also killed at the panel's first 16 GB scope) and 1010.
- **row, 300:** seeds 7, 1002, 1005, 1006, 1008, 1009, 1010, 1011 (8 of 13; 5 of the 12 measured seeds drawn).
- **no-row, 300:** seeds 1002 and 1010.

## Notes

- no-row at 300, seed 1001, is the known zero-size drawing (7.8 m, no leaf; PHASE3-TRIALS.md); it is in the no-row 300 medians.
- The six other presets were not redrawn (R5): their both-ends pins are unchanged (PINS.md).
- The stop check at 40 and 80 is STOPCHECK.md (the host ruled on it: go on).
