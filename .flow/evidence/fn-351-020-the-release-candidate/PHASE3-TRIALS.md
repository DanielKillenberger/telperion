# Phase 3 trials: preset variants B3 to B6 of the Oregon white oak, 12 to 13 Nov 2026

Measurement only: no product change, nothing committed, no verdicts. Checkout `/home/daniel/Projects/telperion/.worktrees/census-p5` (uncommitted readers left as they were). Scratch `$S` = `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad`. Readers, definitions and the B1 to B3 results are in `$S/PHASE3-PROBES.md` (section B). Grower numbers are [grower] from the census; drawn numbers are [drawn] from the pipeline whole.

## Variants (value lines, written by `$S/variants.py` into `$S/values/<V>.values`)

Index check: `crates/telperion-space/src/chain.rs` has `LIMB = 6` and `BOUGH = 7`. The preset's `/space/ages/6/` rows are the limb (its zones carry `dormant/9`, the bough-bud wake lines B1 moves to `dormant/7`), and `/space/ages/7/` is the bough row (`lifespan = 60`, matching the bough's 60 years in B2's analysis; `bendLength 1.5`, `primary 0.004`). The B3 line was placed after the last `/space/ages/6/form/` row (line 344, `sag`); the bough row's line goes after the last `/space/ages/7/form/` row (line 403, `sag`). `ordinary.values` and `silver-birch.values` use the same `/space/ages/<n>/form/dominance` key at 6 and 7.

- B3 (exists, binary reused): `+ /space/ages/6/form/dominance = 0.5`
- B4 (B1 + B3):
  - `/space/ages/6/zones/{0,1,2}/dormant/9 = 0.1` -> `/space/ages/6/zones/{0,1,2}/dormant/7 = 0.1`
  - `+ /space/ages/6/form/dominance = 0.5`
- B5 (B4 + bough dominance): the B4 lines and `+ /space/ages/7/form/dominance = 0.5`
- B6: `+ /space/ages/6/form/dominance = 0.25`

Each file was copied over the worktree's `oregon-white-oak.values` for its build only (`p3t.sh`: `cp values/<V>.values $VALUES; bash build.sh <V>; cp values/base.values $VALUES; cmp`). `cmp` printed nothing after each of B4, B5, B6 (restored). At resume, `git diff` of the worktree's values file was empty and `cmp` with `values/base.values` matched: it was not left swapped.

## Commands

```
python3 -I $S/variants.py
bash $S/p3t.sh       # builds B4 B5 B6 (telperion-heavy 8 cargo build --profile ci), then
                     # bash census.sh <V> 300 <seed> 80 120,200 - 80,120,200 1 0 for B3 B4 B5 B6 on 1001, B3 B4 on 1007 and 1012,
                     # then telperion-heavy 16 --gpu 2 --threads 16 -- python3 peak.py bin/probe-<V> tree oregon-white-oak <age> 1001 <prefix> whole,bare
bash $S/p3t2.sh      # the trees cut off (see Failures), TELPERION_HEAVY_RESERVE=8 telperion-heavy 16 --gpu 2 --threads 16 -- ...
python3 -I $S/t3.py {steps|leaf|lines|drawn|cost}
python3 -I $S/sheet.py tsheets B3 B4 B5
```

The base to 300 on seed 1001 is `logs/census-base-300-1001.log` (reused). B3 stills at 120 and 200 are the earlier agent's (same binary `bin/probe-B3`).

## What failed or went wrong

- `p3t.sh` was stopped (rc=143) during the B5 200 tree; its partial log was discarded and B5 200 was rerun whole in `p3t2.sh`. B5 120 (complete, rc 0, both PNGs) was kept. B5 300 and B6 200 had not started.
- No census was killed. B3 on 1007 peaked at 12.2 GiB, above its 16 GB request only in the sense of being close; the queue admitted it.
- The batch's `PEAK` echo in `p3t.sh` printed empty (peak.py prints `peakMiB`, not `PEAK`); the memory is read from the logs' last line.
- Base drawn tree at 300: the known zero-size drawing fault (7.8 m, no leaf, 187 nodes) against the grower's 31.6 m and 4 m2 leaf; both reported as measured.

## Timing rule

The yield watcher's log (`$S/yield.log`) begins 13 Nov 00:24:45. Everything before that (all censuses, builds, B3 and B4 trees, B5 120) ran with no PAUSE. B5 200 and B5 300 ran inside many PAUSE/RESUME windows: not timed. B6 200 ran 01:23:19 to 01:25:09 with no PAUSE line inside: 110 s wall.

## Grower leaf by age, m2 (all seed lists, 20-year steps)

| leaf m2, seed 1001 | 80 | 100 | 120 | 140 | 160 | 180 | 200 | 220 | 240 | 260 | 280 | 300 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base 1001 | 509 | 695 | 757 | 662 | 500 | 373 | 206 | 120 | 41 | 12 | 7 | 4 |
| B3 1001 | 352 | 521 | 599 | 561 | 505 | 545 | 570 | 562 | 509 | 444 | 365 | 344 |
| B4 1001 | 528 | 835 | 925 | 824 | 766 | 753 | 736 | 708 | 644 | 545 | 445 | 361 |
| B5 1001 | 301 | 499 | 645 | 696 | 697 | 682 | 662 | 666 | 650 | 609 | 572 | 511 |
| B6 1001 | 434 | 633 | 726 | 694 | 586 | 579 | 505 | 360 | 213 | 143 | 67 | 43 |
| B3 1007 | 550 | 820 | 981 | 960 | 953 | 978 | 944 | 892 | 821 | 763 | 700 | 627 |
| B4 1007 | 798 | 1224 | 1353 | 1315 | 1204 | 1147 | 1095 | 1064 | 1040 | 967 | 883 | 813 |
| B3 1012 | 495 | 717 | 818 | 851 | 743 | 631 | 588 | 521 | 492 | 454 | 388 | 346 |
| B4 1012 | 789 | 1164 | 1356 | 1296 | 1189 | 1033 | 896 | 758 | 685 | 637 | 589 | 563 |

## Grower tables by 20-year steps from 80 to 300

"Sheds in the step" is the sum of the two decade counts (age-10 and age). Heights are the grower's, all held (open wood in its own column).

| variant | seed | age | grower leaf m2 | height, all held m | height, open wood m | living tips | limb / bough tips | limb / bough sheds in the step (20 y) | woken lines alive |
|---|---|---|---|---|---|---|---|---|---|
| base | 1001 | 80 | 509 | 17.8 | 17.8 | 129,311 | 6 / 70 | 0 / 0 | 4 |
| base | 1001 | 100 | 695 | 23.0 | 23.0 | 175,650 | 9 / 100 | 0 / 6 | 5 |
| base | 1001 | 120 | 757 | 27.0 | 27.0 | 189,672 | 11 / 130 | 0 / 14 | 11 |
| base | 1001 | 140 | 662 | 30.1 | 30.1 | 164,114 | 15 / 154 | 1 / 32 | 9 |
| base | 1001 | 160 | 500 | 32.1 | 31.8 | 126,891 | 7 / 107 | 9 / 56 | 9 |
| base | 1001 | 180 | 373 | 31.7 | 31.0 | 95,872 | 7 / 105 | 2 / 31 | 8 |
| base | 1001 | 200 | 206 | 32.6 | 32.6 | 55,499 | 3 / 57 | 4 / 40 | 3 |
| base | 1001 | 220 | 120 | 31.6 | 24.6 | 35,685 | 3 / 36 | 1 / 30 | 4 |
| base | 1001 | 240 | 41 | 31.6 | 26.3 | 15,259 | 0 / 10 | 3 / 21 | 0 |
| base | 1001 | 260 | 12 | 31.6 | 24.1 | 7,982 | 0 / 3 | 0 / 7 | 0 |
| base | 1001 | 280 | 7 | 31.6 | 22.0 | 5,711 | 0 / 2 | 0 / 1 | 0 |
| base | 1001 | 300 | 4 | 31.6 | 17.8 | 4,334 | 0 / 1 | 0 / 1 | 0 |
| B3 | 1001 | 80 | 352 | 19.0 | 19.0 | 129,156 | 6 / 70 | 0 / 0 | 4 |
| B3 | 1001 | 100 | 521 | 22.7 | 22.7 | 208,112 | 9 / 106 | 0 / 0 | 7 |
| B3 | 1001 | 120 | 599 | 25.4 | 25.4 | 265,228 | 10 / 137 | 1 / 7 | 9 |
| B3 | 1001 | 140 | 561 | 29.1 | 29.1 | 289,613 | 12 / 162 | 2 / 17 | 11 |
| B3 | 1001 | 160 | 505 | 29.2 | 27.8 | 295,913 | 13 / 161 | 2 / 26 | 12 |
| B3 | 1001 | 180 | 545 | 29.6 | 29.6 | 328,673 | 10 / 197 | 4 / 27 | 12 |
| B3 | 1001 | 200 | 570 | 30.0 | 22.5 | 351,497 | 11 / 202 | 2 / 21 | 7 |
| B3 | 1001 | 220 | 562 | 30.0 | 24.4 | 357,996 | 13 / 227 | 7 / 32 | 10 |
| B3 | 1001 | 240 | 509 | 30.0 | 26.2 | 345,553 | 12 / 219 | 5 / 47 | 10 |
| B3 | 1001 | 260 | 444 | 30.0 | 28.4 | 332,763 | 16 / 224 | 2 / 67 | 7 |
| B3 | 1001 | 280 | 365 | 30.0 | 29.0 | 290,989 | 13 / 192 | 7 / 68 | 8 |
| B3 | 1001 | 300 | 344 | 30.0 | 29.2 | 287,182 | 10 / 176 | 4 / 62 | 5 |
| B4 | 1001 | 80 | 528 | 20.4 | 20.4 | 226,749 | 6 / 179 | 0 / 0 | 109 |
| B4 | 1001 | 100 | 835 | 25.4 | 25.4 | 390,857 | 9 / 271 | 0 / 8 | 165 |
| B4 | 1001 | 120 | 925 | 29.2 | 29.2 | 503,850 | 6 / 312 | 4 / 54 | 200 |
| B4 | 1001 | 140 | 824 | 30.9 | 30.9 | 526,652 | 5 / 294 | 3 / 87 | 193 |
| B4 | 1001 | 160 | 766 | 29.9 | 29.9 | 535,854 | 6 / 265 | 0 / 113 | 170 |
| B4 | 1001 | 180 | 753 | 30.6 | 30.6 | 541,883 | 4 / 255 | 2 / 99 | 156 |
| B4 | 1001 | 200 | 736 | 30.8 | 28.1 | 519,607 | 5 / 246 | 0 / 80 | 150 |
| B4 | 1001 | 220 | 708 | 30.8 | 23.3 | 484,900 | 5 / 233 | 2 / 94 | 139 |
| B4 | 1001 | 240 | 644 | 30.8 | 24.6 | 430,916 | 7 / 215 | 2 / 95 | 121 |
| B4 | 1001 | 260 | 545 | 30.8 | 26.1 | 366,177 | 4 / 187 | 5 / 82 | 105 |
| B4 | 1001 | 280 | 445 | 30.8 | 27.7 | 295,759 | 1 / 144 | 4 / 61 | 91 |
| B4 | 1001 | 300 | 361 | 30.8 | 29.4 | 249,918 | 0 / 104 | 1 / 45 | 72 |
| B5 | 1001 | 80 | 301 | 18.7 | 18.7 | 226,540 | 6 / 179 | 0 / 0 | 109 |
| B5 | 1001 | 100 | 499 | 21.7 | 21.7 | 409,607 | 9 / 276 | 0 / 3 | 173 |
| B5 | 1001 | 120 | 645 | 25.4 | 25.4 | 577,293 | 10 / 360 | 1 / 51 | 227 |
| B5 | 1001 | 140 | 696 | 29.4 | 29.4 | 681,011 | 9 / 381 | 4 / 104 | 243 |
| B5 | 1001 | 160 | 697 | 29.6 | 29.6 | 742,621 | 10 / 382 | 2 / 114 | 253 |
| B5 | 1001 | 180 | 682 | 31.8 | 31.8 | 789,796 | 11 / 412 | 1 / 156 | 254 |
| B5 | 1001 | 200 | 662 | 29.8 | 29.8 | 786,289 | 12 / 375 | 2 / 177 | 233 |
| B5 | 1001 | 220 | 666 | 29.8 | 28.4 | 796,719 | 13 / 391 | 4 / 166 | 234 |
| B5 | 1001 | 240 | 650 | 29.8 | 27.3 | 782,608 | 18 / 406 | 3 / 173 | 234 |
| B5 | 1001 | 260 | 609 | 29.8 | 25.8 | 744,273 | 15 / 387 | 7 / 217 | 206 |
| B5 | 1001 | 280 | 572 | 29.8 | 26.4 | 711,194 | 16 / 400 | 5 / 196 | 226 |
| B5 | 1001 | 300 | 511 | 29.8 | 28.1 | 665,537 | 14 / 361 | 5 / 235 | 207 |
| B6 | 1001 | 80 | 434 | 18.3 | 18.3 | 131,868 | 6 / 70 | 0 / 0 | 4 |
| B6 | 1001 | 100 | 633 | 22.0 | 22.0 | 198,582 | 9 / 106 | 0 / 0 | 5 |
| B6 | 1001 | 120 | 726 | 24.9 | 24.9 | 233,058 | 11 / 142 | 0 / 8 | 11 |
| B6 | 1001 | 140 | 694 | 28.1 | 28.1 | 229,080 | 14 / 153 | 2 / 41 | 10 |
| B6 | 1001 | 160 | 586 | 30.5 | 30.3 | 205,742 | 10 / 132 | 4 / 34 | 11 |
| B6 | 1001 | 180 | 579 | 31.3 | 31.3 | 203,459 | 9 / 150 | 2 / 29 | 10 |
| B6 | 1001 | 200 | 505 | 31.3 | 27.6 | 178,186 | 15 / 159 | 2 / 34 | 8 |
| B6 | 1001 | 220 | 360 | 31.3 | 29.4 | 137,443 | 12 / 152 | 6 / 62 | 10 |
| B6 | 1001 | 240 | 213 | 31.3 | 29.5 | 93,582 | 6 / 88 | 5 / 85 | 5 |
| B6 | 1001 | 260 | 143 | 31.3 | 28.9 | 70,306 | 8 / 78 | 3 / 29 | 8 |
| B6 | 1001 | 280 | 67 | 31.3 | 29.7 | 38,189 | 1 / 27 | 5 / 40 | 0 |
| B6 | 1001 | 300 | 43 | 31.3 | 29.7 | 26,383 | 0 / 13 | 1 / 12 | 0 |
| B3 | 1007 | 80 | 550 | 17.9 | 17.9 | 283,892 | 18 / 204 | 0 / 0 | 11 |
| B3 | 1007 | 100 | 820 | 21.3 | 21.3 | 467,130 | 26 / 291 | 3 / 9 | 25 |
| B3 | 1007 | 120 | 981 | 25.7 | 25.7 | 611,185 | 36 / 385 | 6 / 37 | 29 |
| B3 | 1007 | 140 | 960 | 28.5 | 28.1 | 702,909 | 40 / 445 | 11 / 93 | 17 |
| B3 | 1007 | 160 | 953 | 29.2 | 27.0 | 783,833 | 32 / 462 | 14 / 111 | 24 |
| B3 | 1007 | 180 | 978 | 28.4 | 26.0 | 843,164 | 39 / 511 | 11 / 89 | 32 |
| B3 | 1007 | 200 | 944 | 28.4 | 28.4 | 908,398 | 36 / 571 | 14 / 90 | 33 |
| B3 | 1007 | 220 | 892 | 29.9 | 29.9 | 1,011,383 | 46 / 625 | 5 / 122 | 29 |
| B3 | 1007 | 240 | 821 | 29.9 | 28.0 | 1,081,655 | 49 / 683 | 14 / 130 | 32 |
| B3 | 1007 | 260 | 763 | 29.9 | 26.0 | 1,277,534 | 64 / 803 | 9 / 121 | 45 |
| B3 | 1007 | 280 | 700 | 29.9 | 26.6 | 1,522,208 | 76 / 952 | 13 / 136 | 52 |
| B3 | 1007 | 300 | 627 | 29.9 | 27.9 | 1,762,710 | 96 / 1149 | 10 / 156 | 78 |
| B4 | 1007 | 80 | 798 | 19.5 | 19.5 | 434,977 | 18 / 476 | 0 / 16 | 272 |
| B4 | 1007 | 100 | 1224 | 24.1 | 24.1 | 687,502 | 21 / 617 | 5 / 140 | 377 |
| B4 | 1007 | 120 | 1353 | 29.8 | 29.8 | 851,701 | 15 / 628 | 9 / 208 | 371 |
| B4 | 1007 | 140 | 1315 | 30.0 | 29.4 | 914,744 | 19 / 620 | 2 / 243 | 365 |
| B4 | 1007 | 160 | 1204 | 29.9 | 28.8 | 919,509 | 18 / 595 | 5 / 273 | 334 |
| B4 | 1007 | 180 | 1147 | 30.9 | 27.7 | 921,288 | 25 / 595 | 6 / 283 | 308 |
| B4 | 1007 | 200 | 1095 | 29.9 | 28.4 | 898,709 | 17 / 555 | 14 / 292 | 280 |
| B4 | 1007 | 220 | 1064 | 29.9 | 27.1 | 880,791 | 17 / 563 | 5 / 253 | 288 |
| B4 | 1007 | 240 | 1040 | 29.9 | 26.7 | 869,369 | 17 / 571 | 5 / 252 | 285 |
| B4 | 1007 | 260 | 967 | 29.9 | 26.0 | 827,860 | 18 / 531 | 5 / 296 | 249 |
| B4 | 1007 | 280 | 883 | 29.9 | 26.9 | 784,622 | 15 / 509 | 10 / 237 | 244 |
| B4 | 1007 | 300 | 813 | 29.9 | 28.4 | 743,973 | 18 / 509 | 3 / 229 | 248 |
| B3 | 1012 | 80 | 495 | 18.3 | 18.3 | 162,254 | 10 / 103 | 0 / 4 | 9 |
| B3 | 1012 | 100 | 717 | 22.1 | 22.1 | 238,236 | 11 / 148 | 4 / 7 | 7 |
| B3 | 1012 | 120 | 818 | 25.4 | 25.4 | 284,863 | 15 / 177 | 4 / 16 | 8 |
| B3 | 1012 | 140 | 851 | 28.4 | 28.4 | 310,960 | 16 / 196 | 5 / 29 | 14 |
| B3 | 1012 | 160 | 743 | 31.9 | 31.9 | 315,701 | 12 / 199 | 5 / 35 | 9 |
| B3 | 1012 | 180 | 631 | 30.9 | 28.2 | 299,649 | 13 / 192 | 4 / 50 | 7 |
| B3 | 1012 | 200 | 588 | 30.9 | 26.9 | 304,168 | 19 / 202 | 5 / 60 | 18 |
| B3 | 1012 | 220 | 521 | 30.9 | 26.3 | 297,642 | 16 / 192 | 9 / 69 | 8 |
| B3 | 1012 | 240 | 492 | 30.9 | 23.7 | 287,774 | 13 / 194 | 5 / 46 | 9 |
| B3 | 1012 | 260 | 454 | 30.9 | 25.1 | 276,131 | 13 / 196 | 5 / 43 | 10 |
| B3 | 1012 | 280 | 388 | 30.9 | 25.6 | 248,825 | 14 / 199 | 5 / 57 | 12 |
| B3 | 1012 | 300 | 346 | 30.9 | 26.9 | 235,503 | 9 / 164 | 5 / 57 | 4 |
| B4 | 1012 | 80 | 789 | 20.9 | 20.9 | 294,705 | 7 / 257 | 2 / 7 | 162 |
| B4 | 1012 | 100 | 1164 | 25.7 | 25.7 | 463,305 | 11 / 352 | 0 / 45 | 215 |
| B4 | 1012 | 120 | 1356 | 28.5 | 28.5 | 580,639 | 13 / 409 | 4 / 87 | 261 |
| B4 | 1012 | 140 | 1296 | 31.8 | 30.4 | 604,133 | 14 / 381 | 5 / 158 | 250 |
| B4 | 1012 | 160 | 1189 | 30.6 | 29.1 | 598,103 | 10 / 349 | 5 / 170 | 231 |
| B4 | 1012 | 180 | 1033 | 30.6 | 28.3 | 565,691 | 11 / 331 | 2 / 162 | 204 |
| B4 | 1012 | 200 | 896 | 30.6 | 27.6 | 539,315 | 16 / 367 | 4 / 142 | 211 |
| B4 | 1012 | 220 | 758 | 30.6 | 27.8 | 489,584 | 15 / 321 | 6 / 198 | 171 |
| B4 | 1012 | 240 | 685 | 30.6 | 28.2 | 473,125 | 11 / 316 | 5 / 165 | 158 |
| B4 | 1012 | 260 | 637 | 30.6 | 28.1 | 456,654 | 13 / 303 | 4 / 171 | 153 |
| B4 | 1012 | 280 | 589 | 30.6 | 26.7 | 438,104 | 13 / 301 | 4 / 151 | 142 |
| B4 | 1012 | 300 | 563 | 30.6 | 26.7 | 418,219 | 11 / 292 | 4 / 144 | 151 |

## Living limb and bough lines at 120 and 200, seed 1001 [grower]

| variant | age | kind | living lines | base diameter p10/50/90 cm |
|---|---|---|---|---|
| base | 120 | limb | 11 | 4.1 / 16.3 / 53.7 |
| base | 120 | bough | 130 | 1.1 / 5.5 / 14.9 |
| base | 200 | limb | 3 | 4.8 / 20.7 / 66.6 |
| base | 200 | bough | 57 | 1.9 / 5.5 / 15.1 |
| B3 | 120 | limb | 10 | 1.5 / 8.1 / 39.1 |
| B3 | 120 | bough | 137 | 0.9 / 4.0 / 11.5 |
| B3 | 200 | limb | 11 | 1.1 / 17.6 / 60.5 |
| B3 | 200 | bough | 202 | 1.2 / 4.0 / 12.2 |
| B4 | 120 | limb | 6 | 3.4 / 28.2 / 42.5 |
| B4 | 120 | bough | 312 | 0.6 / 2.6 / 9.2 |
| B4 | 200 | limb | 5 | 2.2 / 47.9 / 82.9 |
| B4 | 200 | bough | 246 | 0.9 / 4.4 / 10.8 |
| B5 | 120 | limb | 10 | 1.6 / 9.6 / 32.4 |
| B5 | 120 | bough | 360 | 0.3 / 2.1 / 7.0 |
| B5 | 200 | limb | 12 | 0.7 / 7.8 / 58.4 |
| B5 | 200 | bough | 375 | 0.4 / 3.3 / 8.8 |
| B6 | 120 | limb | 11 | 2.7 / 9.3 / 47.1 |
| B6 | 120 | bough | 142 | 0.9 / 4.5 / 13.5 |
| B6 | 200 | limb | 15 | 1.0 / 5.2 / 61.8 |
| B6 | 200 | bough | 159 | 1.1 / 5.2 / 14.4 |

Five largest living limbs (base diameter cm / length m):

| variant | age | five largest limbs |
|---|---|---|
| base | 120 | 54.8/16.9, 53.7/21.0, 49.6/24.7, 47.1/15.4, 23.4/12.4 |
| base | 200 | 66.6/25.5, 20.7/11.8, 4.8/3.6 |
| B3 | 120 | 61.8/24.3, 39.1/11.4, 28.5/10.6, 25.4/8.5, 8.1/2.3 |
| B3 | 200 | 77.0/21.7, 60.5/20.1, 46.1/14.4, 21.8/9.2, 20.0/5.8 |
| B4 | 120 | 42.5/9.9, 36.1/10.3, 28.2/7.7, 5.8/1.8, 3.4/1.9 |
| B4 | 200 | 82.9/19.2, 77.4/19.4, 47.9/11.9, 23.4/8.0, 2.2/0.8 |
| B5 | 120 | 65.1/24.4, 32.4/9.2, 27.9/9.8, 27.5/9.3, 9.6/5.0 |
| B5 | 200 | 63.8/17.2, 58.4/18.6, 51.6/16.0, 15.5/7.6, 12.7/4.6 |
| B6 | 120 | 61.1/22.9, 47.1/14.4, 39.5/14.0, 36.1/15.7, 13.3/5.7 |
| B6 | 200 | 83.1/26.1, 61.8/22.2, 59.9/27.8, 34.4/15.5, 19.7/10.6 |

## Cost

Census seconds are the probe's `CLOCK 300` (compute only, queue wait excluded); peak is the census process's `peakMiB`. All censuses were before the watcher and are timed.

| census to 300 | seconds | peak GiB |
|---|---|---|
| base 1001 | 59 | 1.4 |
| B3 1001 | 446 | 2.6 |
| B3 1007 | 897 | 11.9 |
| B3 1012 | 247 | 2.2 |
| B4 1001 | 740 | 4.1 |
| B4 1007 | 1068 | 6.9 |
| B4 1012 | 695 | 4.5 |
| B5 1001 | 1030 | 6.0 |
| B6 1001 | 218 | 1.7 |

Drawn trees, seed 1001 at 200: phytomers drawn and peak memory (axes in brackets).

| tree at 200 | phytomers | nodes | axes | peak GiB | seconds |
|---|---|---|---|---|---|
| base | 988,917 | 818,736 | 122,500 | 1.8 | 38 (probe) |
| B3 | 5,492,748 | 3,826,468 | 757,677 | 5.6 | 130 (probe) |
| B4 | 8,303,641 | 4,680,178 | 1,164,868 | 7.6 | 343 (probe) |
| B5 | 11,988,545 | 5,983,900 | 1,809,089 | 9.8 | not timed |
| B6 | 2,963,392 | 2,385,355 | 378,134 | 3.6 | 54 (probe) |

Other drawn trees, peak GiB: B3 120 4.1 (319 s, 119 s wall), B3 300 5.2, B4 120 6.7, B4 300 5.2, B5 120 6.8, B5 300 9.2 (not timed), base 120 3.6, base 300 1.5. "seconds" in the drawn table below is the probe's own pipeline clock; for B5 200 and 300 it includes the watcher's freezes, so it is not timed.

## Drawn trees, seed 1001 [drawn]

Three largest limbs' base cm from the expanded tree.

| variant | tree | height m | spread x / z m | trunk at 1.3 m cm | drawn leaf m2 | three largest limbs' base cm | seconds |
|---|---|---|---|---|---|---|---|
| base | 120 seed 1001 | 27.3 | 25.4 / 23.1 | 107 | 1,069 | 55 / 50 / 47 | 52 |
| base | 200 seed 1001 | 32.7 | 30.3 / 29.1 | 133 | 290 | 77 / 67 / 55 | 38 |
| base | 300 seed 1001 | 7.8 | 2.8 / 2.8 | 135 | 0 | 36 / 25 | 42 |
| B3 | 120 seed 1001 | 26.9 | 25.3 / 26.2 | 93 | 873 | 39 / 29 / 25 | 56 |
| B3 | 200 seed 1001 | 23.1 | 17.6 / 18.8 | 133 | 861 | 77 / 60 / 46 | 130 |
| B3 | 300 seed 1001 | 29.9 | 19.4 / 18.3 | 163 | 525 | 99 / 81 / 65 | 221 |
| B4 | 120 seed 1001 | 29.2 | 32.6 / 35.7 | 114 | 1,411 | 43 / 36 / 28 | 85 |
| B4 | 200 seed 1001 | 28.1 | 21.3 / 19.5 | 160 | 1,095 | 83 / 77 / 48 | 343 |
| B4 | 300 seed 1001 | 29.7 | 20.2 / 22.0 | 191 | 510 | 108 / 97 / 69 | 343 |
| B5 | 120 seed 1001 | 25.9 | 21.1 / 26.4 | 91 | 1,007 | 32 / 30 / 28 | 92 |
| B5 | 200 seed 1001 | 29.5 | 20.8 / 20.4 | 141 | 1,022 | 64 / 58 / 52 | 1329 |
| B5 | 300 seed 1001 | 29.0 | 21.7 / 23.8 | 180 | 752 | 100 / 84 / 72 | 1076 |
| B6 | 200 seed 1001 | 28.0 | 25.3 / 25.5 | 142 | 737 | 83 / 62 / 60 | 54 |

## Sheets (`$S/sheets/T-B3.jpg`, `T-B4.jpg`, `T-B5.jpg`)

Rows: 120 whole, 200 whole, 200 leafless, 300 whole, 300 leafless; unchanged preset left, variant right. On the left at 300 the unchanged preset is a bare cylindrical trunk post with a few ring bands, 7.8 m tall, next to the human figure, with no limbs or leaf (the known drawing fault).

- T-B3: at 120 the right tree has a thinner trunk, a sparser crown with a few long thin drooping twigs below the canopy at the right. At 200 the right tree is shorter and narrower than the left (23.1 against 32.7 m), with a dense dark leafy dome on a thick trunk; the left is a tall sparse crown with long bare arching twigs. At 300 the right tree is 29.9 m with a leafy crown of moderate density and a few long bare twigs rising above; the leafless view shows a forked trunk with many fine branches.
- T-B4: at 120 the right tree is wider (32.6 x 35.7 m), with thin hanging twigs reaching low at the right and a dark dense lower crown. At 200 it is a compact dense dome, 28.1 m tall with one long bare shoot above the top; the leafless view shows a fine, thick tangle of twigs. At 300 it has a lighter leaf cover on an open branched crown, trunk 191 cm, with a base flare.
- T-B5: at 120 the right tree shows a few long low branches reaching to the right and a leafy upper left. At 200 it is a narrow upright leafy crown with leafless twigs at the top. At 300 it has two upward-spreading arms of leafy branches in a V shape on a thick trunk (180 cm); the leafless view shows many fine branches in a vase shape.
