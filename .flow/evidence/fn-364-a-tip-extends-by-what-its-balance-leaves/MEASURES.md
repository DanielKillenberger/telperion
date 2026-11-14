# fn-364 R5: grower and settle, fn-363's branch against this one (builder, Opus, 14 Nov 2026)

**The spec's second Stop fires before any drawing, so R6 is not drawn.** Laurelin at its preset age (80) and Telperion at its preset age (82) both lose more than 15% of their height and more than 25% of their leaf, on both seeds:

| tree | height m, fn-363 to this | leaf m², fn-363 to this |
|---|---|---|
| Laurelin 80, seed 1 | 43.6 to 32.1 (−26%) | 3,473 to 1,393 (−60%) |
| Laurelin 80, seed 7 | 45.1 to 33.3 (−26%) | 5,217 to 2,417 (−54%) |
| Telperion 82, seed 1 | 43.8 to 32.4 (−26%) | 3,393 to 1,338 (−61%) |
| Telperion 82, seed 7 | 45.5 to 33.7 (−26%) | 5,181 to 2,299 (−56%) |

Both were over their potential height of 40 m on fn-363's branch (43.5 to 45.5 m), and are under it on this one (32.1 to 33.7 m). Between 43% and 68% of their living tips are laid at s = 0 at 80 and 82. The natural presets at their preset age (40) stay inside the Stop: the largest height fall is the birch's (16.7 to 15.2 m, −9%), the largest leaf move also the birch's (−11%). At 80, which is past every natural preset's age, the birch falls 18% in height and 32% in leaf.

**The host's reading of the Stop (14 Nov 2026):** the rule does to the Two Trees what it was designed to do, against a potential height that is not theirs. Their values carry the beech's 40 m ("it may reach over 40 m", a lower bound for the beech, fn-345), and the base's Two Trees stand at 45.5 and 46.4 m at 80 and 82 (fn-352's SCOREBOARD.md) where the beech at 80 stands at about 17 to 18 m. Their potential height is an open value, the host's to settle, not a fault of the rule. How they get their size from the beech's rows: TWO-TREES-HEIGHT.md. Their draws wait for that value.

**The oak at 40** is within 5% of fn-363's, as the spec expects: height 10.7 to 10.4 m and 11.1 to 10.8 m (−3%), leaf 115 to 112 m² and 180 to 175 m² (−3%).

## R8: the Two Trees' potential height lengthened with them (host, 14 Nov 2026)

With R8 in (`be4357b7e`, measured on `8e22c8a17`), the Two Trees stand against 104 m (40 m × 0.182 / 0.07). At their preset ages they stand 41.7 to 43.7 m, inside the host's 38 to 46 m around the paper's 41 m:

| tree | height m (base median / fn-363 / this, R8) | leaf m² (base ~9,500 / fn-363 / this, R8) | trunk m (base median / this, R8) | phytomers (fn-363 / this, R8) | living tips at s = 0 |
|---|---|---|---|---|---|
| Laurelin 80, seed 1 | 46.4 / 43.6 / **41.7** | 3,473 / **4,875** | 2.83 / - | 3,368,079 / 5,087,956 | 29,497 of 229,943 |
| Laurelin 80, seed 7 | 46.4 / 45.1 / **43.2** | 5,217 / **7,485** | 2.83 / 2.05 | 4,810,022 / 7,596,573 | 46,069 of 345,598 |
| Telperion 82, seed 1 | 45.5 / 43.8 / **42.2** | 3,393 / **4,859** | 3.06 / - | 3,701,260 / 5,591,003 | 29,145 of 249,228 |
| Telperion 82, seed 7 | 45.5 / 45.5 / **43.7** | 5,181 / **7,469** | 3.06 / 2.06 | 5,264,660 / 8,344,437 | 45,329 of 375,488 |

The base's medians are fn-352's SCOREBOARD.md figures as the host quoted them (height 46.4 m Laurelin and 45.5 m Telperion, trunks 2.83 and 3.06 m, leaf about 9,500 m²). The top's rows are fn-363's MEASURES.md "before" rows. At 40 the Two Trees stand 25.3 and 26.3 m on R8, against fn-363's 25.8 and 26.9 m. Every row is in the table below, beside fn-363's and this branch's without R8. The browser's ceiling test with R8: see R4 (PINS.md).

## Method

- **fn-363's rows** are its builder's (`scratchpad/fn363/logs/after-*`, fn-363's product at `c2d897a37` with the probe's reader). None were grown again.
- **This branch's rows** come from `.worktrees/fn-364-after`, a detached checkout of `6c7093f96` with the same reader applied (never committed). Its grower and settle are those of `f6c00cc49`: the later commits change only the pipeline's conversion and foliage, which these rows do not read. The runner, the scripts and the table are in `r5/`.
- **Drawn leaf stations** are the pipeline's own station plan (`Expansion::stations`, no drawing): fn-363's from a runner built on its checkout, this branch's on `f6c00cc49`. "-" means not yet measured when this was written.
- **Seconds** are wall time inside the queue scope. "not timed" means a PAUSE of the yield watcher fell inside the tree's window.
- **Living tips at s = 0** counts the living apexes laid at no length in the age's own cycle. **Zero-length phytomers** counts settled phytomers with scale above 0 and length 0.
- **White's open-grown ranges** for the oak's trunk at 1.3 m are as in fn-363's MEASURES.md. The potential height is each species' `1 / inverse height`.

## R3

The oak grown to 80 at once equals the oak paused at 40, settled there and grown on, to the bit (seed 1001, the preset: `tree_digest` `b6f1d607a86ea1c6` both ways; at 40, direct and paused `980c7b3fe04dffc4` both ways).

## Table

| preset | age | seed | run | height m | mean crown spread m | trunk at 1.3 m, m | settled leaf m² | drawn leaf stations | leafed phytomers | phytomers | leaf per leafed phytomer, cm² | phytomers per m² of leaf | tallest living tip m / potential | apexes shed by the balance, last 20 cycles | living tips at s = 0 | zero-length phytomers | peak GiB | seconds |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| oregon-white-oak | 120 | 1001 | fn-363 | 27.4 | 25.6 | 1.15 (out of 0.83 to 1.09) | 968 | - | 770,086 | 3,984,042 | 12.6 | 4,117 | 27.4 / 34.8 | 68,302 | - | - | 2.2 | 26 |
| oregon-white-oak | 120 | 1001 | this | 23.4 | 23.0 | 1.06 (in) | 741 | - | 622,555 | 3,324,494 | 11.9 | 4,484 | 23.4 / 34.8 | 48,391 | 32,372 of 211,352 | 367,847 | 1.9 | not timed |
| oregon-white-oak | 120 | 1007 | fn-363 | 28.4 | 36.2 | 1.65 (out of 0.83 to 1.09) | 1,974 | - | 1,528,168 | 8,025,988 | 12.9 | 4,065 | 28.4 / 34.8 | 229,484 | - | - | 4.6 | not timed |
| oregon-white-oak | 120 | 1007 | this | 23.6 | 35.5 | 1.49 (out of 0.83 to 1.09) | 1,474 | - | 1,228,385 | 6,754,339 | 12.0 | 4,583 | 23.6 / 34.8 | 140,834 | 87,365 of 452,010 | 965,658 | 4.2 | not timed |
| oregon-white-oak | 120 | 1012 | fn-363 | 26.0 | 30.7 | 1.34 (out of 0.83 to 1.09) | 1,389 | - | 1,120,897 | 5,738,685 | 12.4 | 4,133 | 26.0 / 34.8 | 99,134 | - | - | 3.2 | 40 |
| oregon-white-oak | 120 | 1012 | this | 22.9 | 28.4 | 1.22 (out of 0.83 to 1.09) | 1,059 | - | 902,928 | 4,792,917 | 11.7 | 4,528 | 22.9 / 34.8 | 85,415 | 41,002 of 320,638 | 513,532 | 2.9 | not timed |
| oregon-white-oak | 200 | 1001 | fn-363 | 36.8 | 36.4 | 1.80 (out of 1.21 to 1.52) | 1,041 | - | 831,090 | 4,311,571 | 12.5 | 4,140 | 36.8 / 34.8 | 292,088 | - | - | 2.6 | not timed |
| oregon-white-oak | 200 | 1001 | this | in progress | | | | | | | | | | | | | | | |
| oregon-white-oak | 200 | 1007 | fn-363 | 37.1 | 53.1 | 2.59 (out of 1.21 to 1.52) | 2,198 | - | 1,730,661 | 9,068,912 | 12.7 | 4,126 | 37.1 / 34.8 | 787,984 | - | - | 5.6 | 166 |
| oregon-white-oak | 200 | 1007 | this | not run | | | | | | | | | | | | | | | |
| oregon-white-oak | 200 | 1012 | fn-363 | 39.4 | 48.4 | 2.27 (out of 1.21 to 1.52) | 1,887 | - | 1,800,898 | 9,573,949 | 10.5 | 5,073 | 39.4 / 34.8 | 730,492 | - | - | 5.8 | 136 |
| oregon-white-oak | 200 | 1012 | this | not run | | | | | | | | | | | | | | | |
| oregon-white-oak | 300 | 1001 | fn-363 | 36.0 | 34.5 | 2.20 (out of 1.56 to 1.92) | 590 | - | 507,823 | 2,625,032 | 11.6 | 4,450 | 36.1 / 34.8 | 314,626 | - | - | 2.0 | 121 |
| oregon-white-oak | 300 | 1001 | this | not run | | | | | | | | | | | | | | | |
| oregon-white-oak | 300 | 1007 | fn-363 | 38.8 | 50.2 | 3.13 (out of 1.56 to 1.92) | 1,111 | - | 1,172,088 | 6,324,353 | 9.5 | 5,692 | 38.8 / 34.8 | 329,036 | - | - | 4.3 | 273 |
| oregon-white-oak | 300 | 1007 | this | not run | | | | | | | | | | | | | | | |
| oregon-white-oak | 300 | 1012 | fn-363 | 38.2 | 53.9 | 2.90 (out of 1.56 to 1.92) | 1,371 | - | 2,029,980 | 10,589,928 | 6.8 | 7,725 | 38.2 / 34.8 | 735,834 | - | - | 6.3 | 290 |
| oregon-white-oak | 300 | 1012 | this | not run | | | | | | | | | | | | | | | |
| oregon-white-oak | 40 | 1 | fn-363 | 10.7 | 7.5 | 0.27 (out of 0.28 to 0.4) | 115 | 31,925 | 79,721 | 353,937 | 14.4 | 3,084 | 10.7 / 34.8 | 0 | - | - | 0.2 | 1 |
| oregon-white-oak | 40 | 1 | this | 10.4 | 7.4 | 0.27 (out of 0.28 to 0.4) | 112 | 31,423 | 77,536 | 346,875 | 14.5 | 3,096 | 10.4 / 34.8 | 0 | 0 of 25,747 | 0 | 0.2 | 1 |
| oregon-white-oak | 40 | 7 | fn-363 | 11.1 | 7.4 | 0.33 (in) | 180 | 71,162 | 114,782 | 497,320 | 15.7 | 2,761 | 11.1 / 34.8 | 0 | - | - | 0.3 | 1 |
| oregon-white-oak | 40 | 7 | this | 10.8 | 7.5 | 0.33 (in) | 175 | 70,439 | 110,655 | 482,313 | 15.8 | 2,760 | 10.8 / 34.8 | 0 | 0 of 40,566 | 0 | 0.3 | 1 |
| oregon-white-oak | 80 | 1 | fn-363 | 17.0 | 17.8 | 0.73 (in) | 562 | 317,459 | 424,715 | 2,110,204 | 13.2 | 3,756 | 17.0 / 34.8 | 531 | - | - | 1.2 | 10 |
| oregon-white-oak | 80 | 1 | this | 15.9 | 16.2 | 0.71 (in) | 515 | 340,733 | 393,366 | 1,976,704 | 13.1 | 3,842 | 15.9 / 34.8 | 842 | 3,351 of 137,206 | 25,972 | 1.1 | 11 |
| oregon-white-oak | 80 | 7 | fn-363 | 20.2 | 24.0 | 0.98 (out of 0.56 to 0.79) | 1,016 | 601,675 | 754,660 | 3,869,037 | 13.5 | 3,809 | 20.2 / 34.8 | 43,917 | - | - | 2.4 | 19 |
| oregon-white-oak | 80 | 7 | this | 18.4 | 22.8 | 0.92 (out of 0.56 to 0.79) | 879 | 754,764 | 654,394 | 3,429,765 | 13.4 | 3,900 | 18.5 / 34.8 | 43,293 | 42,363 of 258,645 | 386,755 | 2.2 | 21 |
| ordinary | 40 | 1 | fn-363 | 12.3 | 8.1 | - | 17 | 5,645 | 18,588 | 98,097 | 9.4 | 5,639 | 12.3 | 0 | - | - | 0.1 | 1 |
| ordinary | 40 | 1 | this | 12.3 | 8.1 | - | 17 | 5,645 | 18,588 | 98,097 | 9.4 | 5,639 | 12.3 | 0 | 0 of 6,402 | 0 | 0.1 | not timed |
| ordinary | 40 | 7 | fn-363 | 13.1 | 9.8 | 0.11 | 27 | 8,005 | 25,902 | 140,416 | 10.2 | 5,297 | 13.1 | 0 | - | - | 0.1 | 1 |
| ordinary | 40 | 7 | this | 13.1 | 9.8 | 0.11 | 27 | 8,005 | 25,902 | 140,416 | 10.2 | 5,297 | 13.1 | 0 | 0 of 8,938 | 0 | 0.1 | not timed |
| ordinary | 80 | 1 | fn-363 | 22.8 | 14.3 | - | 101 | 35,552 | 119,122 | 741,107 | 8.5 | 7,343 | 22.8 | 0 | - | - | 0.4 | 4 |
| ordinary | 80 | 1 | this | 22.8 | 14.3 | - | 101 | 35,552 | 119,122 | 741,107 | 8.5 | 7,343 | 22.8 | 0 | 0 of 41,732 | 0 | 0.4 | not timed |
| ordinary | 80 | 7 | fn-363 | 23.2 | 17.5 | 0.31 | 183 | 62,127 | 205,657 | 1,231,672 | 8.9 | 6,738 | 23.2 | 0 | - | - | 0.6 | 6 |
| ordinary | 80 | 7 | this | 23.2 | 17.5 | 0.31 | 183 | 62,127 | 205,657 | 1,231,672 | 8.9 | 6,738 | 23.2 | 0 | 0 of 71,821 | 0 | 0.7 | not timed |
| norway-spruce | 40 | 1 | fn-363 | 11.6 | 6.5 | 0.26 | 170 | 3,484,945 | 434,820 | 1,115,420 | 3.9 | 6,555 | 11.6 / 61.0 | 0 | - | - | 0.8 | 4 |
| norway-spruce | 40 | 1 | this | 11.5 | 6.4 | 0.26 | 170 | 3,483,100 | 434,685 | 1,115,083 | 3.9 | 6,556 | 11.5 / 61.0 | 0 | 0 of 109,104 | 0 | 0.8 | not timed |
| norway-spruce | 40 | 7 | fn-363 | 11.8 | 6.0 | 0.26 | 165 | 3,364,471 | 418,949 | 1,081,326 | 3.9 | 6,570 | 11.8 / 61.0 | 0 | - | - | 0.8 | 4 |
| norway-spruce | 40 | 7 | this | 11.7 | 5.9 | 0.26 | 165 | 3,363,119 | 418,769 | 1,080,871 | 3.9 | 6,570 | 11.7 / 61.0 | 0 | 0 of 105,258 | 0 | 0.8 | not timed |
| norway-spruce | 80 | 1 | fn-363 | 21.6 | 11.6 | 0.56 | 484 | 10,487,937 | 1,300,354 | 4,365,283 | 3.7 | 9,025 | 21.6 / 61.0 | 333 | - | - | 2.8 | 28 |
| norway-spruce | 80 | 1 | this | 20.7 | 11.5 | 0.56 | 481 | 10,446,434 | 1,287,603 | 4,324,563 | 3.7 | 8,995 | 20.7 / 61.0 | 2,358 | 6,256 of 326,312 | 37,430 | 2.8 | not timed |
| norway-spruce | 80 | 7 | fn-363 | 21.8 | 10.8 | 0.56 | 484 | - | 1,298,940 | 4,334,096 | 3.7 | 8,949 | 21.8 / 61.0 | 762 | - | - | 2.8 | not timed |
| norway-spruce | 80 | 7 | this | 20.9 | 10.7 | 0.56 | 481 | - | 1,288,858 | 4,296,978 | 3.7 | 8,937 | 20.9 / 61.0 | 3,017 | 8,297 of 326,593 | 54,251 | 2.8 | not timed |
| european-beech | 40 | 1 | fn-363 | 10.0 | 6.4 | - | 89 | 92,245 | 89,711 | 485,877 | 10.0 | 5,442 | 10.0 / 40.0 | 0 | - | - | 0.2 | 1 |
| european-beech | 40 | 1 | this | 9.8 | 6.3 | - | 88 | 90,872 | 88,299 | 478,333 | 10.0 | 5,426 | 9.8 / 40.0 | 0 | 0 of 25,009 | 0 | 0.2 | 1 |
| european-beech | 40 | 7 | fn-363 | 10.4 | 8.4 | - | 220 | 179,404 | 167,400 | 797,119 | 13.2 | 3,616 | 10.4 / 40.0 | 60 | - | - | 0.3 | 1 |
| european-beech | 40 | 7 | this | 10.2 | 8.2 | - | 216 | 173,927 | 163,249 | 779,750 | 13.2 | 3,613 | 10.2 / 40.0 | 126 | 2,941 of 46,662 | 20,324 | 0.3 | 1 |
| european-beech | 80 | 1 | fn-363 | 17.4 | 18.1 | - | 1,051 | 881,520 | 803,655 | 5,109,439 | 13.1 | 4,861 | 17.4 / 40.0 | 18,045 | - | - | 1.9 | not timed |
| european-beech | 80 | 1 | this | 16.6 | 16.9 | - | 915 | 737,650 | 699,112 | 4,524,403 | 13.1 | 4,944 | 16.6 / 40.0 | 23,649 | 29,625 of 199,761 | 429,820 | 1.7 | 12 |
| european-beech | 80 | 7 | fn-363 | 18.4 | 21.7 | - | 1,428 | 1,163,639 | 1,047,598 | 6,552,595 | 13.6 | 4,588 | 18.4 / 40.0 | 40,763 | - | - | 2.5 | not timed |
| european-beech | 80 | 7 | this | 17.3 | 20.4 | - | 1,227 | 953,967 | 896,921 | 5,697,708 | 13.7 | 4,642 | 17.3 / 40.0 | 40,390 | 38,839 of 256,765 | 581,792 | 2.2 | 23 |
| silver-birch | 40 | 1 | fn-363 | 16.7 | 7.6 | 0.21 | 55 | 76,148 | 60,557 | 262,553 | 9.0 | 4,803 | 16.7 / 30.0 | 0 | - | - | 0.1 | 1 |
| silver-birch | 40 | 1 | this | 15.2 | 7.1 | 0.20 | 49 | 68,525 | 54,498 | 240,800 | 9.0 | 4,899 | 15.2 / 30.0 | 0 | 0 of 23,673 | 0 | 0.1 | 1 |
| silver-birch | 40 | 7 | fn-363 | 16.7 | 7.1 | 0.23 | 71 | 123,454 | 101,702 | 463,021 | 6.9 | 6,555 | 16.7 / 30.0 | 0 | - | - | 0.2 | 1 |
| silver-birch | 40 | 7 | this | 15.3 | 6.5 | 0.22 | 64 | 110,483 | 91,950 | 422,318 | 6.9 | 6,643 | 15.3 / 30.0 | 0 | 0 of 40,841 | 0 | 0.2 | 1 |
| silver-birch | 80 | 1 | fn-363 | 25.4 | 16.4 | 0.47 | 218 | 356,713 | 275,360 | 1,486,565 | 7.9 | 6,816 | 25.4 / 30.0 | 700 | - | - | 0.7 | 6 |
| silver-birch | 80 | 1 | this | 20.8 | 12.6 | 0.40 | 148 | 223,658 | 184,814 | 1,016,485 | 8.0 | 6,857 | 20.8 / 30.0 | 1,433 | 76 of 81,452 | 6,587 | 0.5 | not timed |
| silver-birch | 80 | 7 | fn-363 | 26.3 | 15.8 | 0.50 | 240 | 425,077 | 343,042 | 1,853,242 | 7.0 | 7,719 | 26.3 / 30.0 | 175 | - | - | 0.9 | 8 |
| silver-birch | 80 | 7 | this | 21.1 | 11.8 | 0.44 | 175 | 295,958 | 254,924 | 1,406,601 | 6.9 | 8,018 | 21.1 / 30.0 | 1,920 | 1,186 of 112,129 | 14,881 | 0.7 | not timed |
| date-palm | 40 | 1 | fn-363 | 16.3 | 0.0 | 0.59 | 1,349 | 0 | 480 | 480 | 28100.0 | 0 | 16.3 | 0 | - | - | 0.0 | 0 |
| date-palm | 40 | 1 | this | 16.3 | 0.0 | 0.59 | 1,349 | 0 | 480 | 480 | 28100.0 | 0 | 16.3 | 0 | 0 of 1 | 0 | 0.0 | not timed |
| date-palm | 40 | 7 | fn-363 | 16.3 | 0.0 | 0.59 | 1,349 | 0 | 480 | 480 | 28100.0 | 0 | 16.3 | 0 | - | - | 0.0 | 0 |
| date-palm | 40 | 7 | this | 16.3 | 0.0 | 0.59 | 1,349 | 0 | 480 | 480 | 28100.0 | 0 | 16.3 | 0 | 0 of 1 | 0 | 0.0 | not timed |
| date-palm | 80 | 1 | fn-363 | 32.5 | 0.0 | 0.59 | 2,698 | 0 | 960 | 960 | 28100.0 | 0 | 32.5 | 0 | - | - | 0.0 | 0 |
| date-palm | 80 | 1 | this | 32.5 | 0.0 | 0.59 | 2,698 | 0 | 960 | 960 | 28100.0 | 0 | 32.5 | 0 | 0 of 1 | 0 | 0.0 | not timed |
| date-palm | 80 | 7 | fn-363 | 32.5 | 0.1 | 0.59 | 2,698 | 0 | 960 | 960 | 28100.0 | 0 | 32.5 | 0 | - | - | 0.0 | 0 |
| date-palm | 80 | 7 | this | 32.5 | 0.1 | 0.59 | 2,698 | 0 | 960 | 960 | 28100.0 | 0 | 32.5 | 0 | 0 of 1 | 0 | 0.0 | not timed |
| laurelin | 40 | 1 | fn-363 | 25.8 | 14.4 | - | 406 | 90,046 | 85,142 | 437,988 | 47.6 | 1,080 | 25.8 / 40.0 | 1,658 | - | - | 0.2 | 1 |
| laurelin | 40 | 1 | this | 22.8 | 11.9 | - | 355 | 64,577 | 72,687 | 374,298 | 48.9 | 1,053 | 22.8 / 40.0 | 2,961 | 8,537 of 20,674 | 114,482 | 0.2 | not timed |
| laurelin | 40 | 1 | this, R8 | 25.3 | 14.0 | - | 413 | 92,098 | 87,777 | 457,162 | 47.1 | 1,106 | 25.3 / 104.0 | 0 | 31 of 24,971 | 177 | 0.2 | 1 |
| laurelin | 40 | 7 | fn-363 | 26.9 | 21.6 | 0.66 | 984 | 172,315 | 158,945 | 725,951 | 61.9 | 738 | 26.8 / 40.0 | 5,016 | - | - | 0.3 | 1 |
| laurelin | 40 | 7 | this | 23.5 | 18.4 | 0.61 | 831 | 120,773 | 130,268 | 595,279 | 63.8 | 716 | 23.5 / 40.0 | 8,528 | 15,712 of 37,106 | 178,896 | 0.3 | not timed |
| laurelin | 40 | 7 | this, R8 | 26.3 | 21.1 | 0.67 | 1,029 | 177,751 | 168,128 | 787,446 | 61.2 | 765 | 26.2 / 104.0 | 172 | 2,000 of 47,965 | 16,673 | 0.3 | 1 |
| laurelin | 80 | 1 | fn-363 | 43.6 | 46.4 | - | 3,473 | 648,205 | 568,163 | 3,368,079 | 61.1 | 970 | 43.6 / 40.0 | 169,576 | - | - | 1.4 | 9 |
| laurelin | 80 | 1 | this | 32.1 | 30.9 | - | 1,393 | 180,417 | 235,966 | 1,535,576 | 59.0 | 1,102 | 32.1 / 40.0 | 114,674 | 43,810 of 66,824 | 841,255 | 0.7 | 8 |
| laurelin | 80 | 1 | this, R8 | 41.7 | 43.5 | - | 4,875 | 856,571 | 803,351 | 5,087,956 | 60.7 | 1,044 | 41.7 / 104.0 | 21,570 | 29,497 of 229,943 | 441,484 | 2.0 | 21 |
| laurelin | 80 | 7 | fn-363 | 45.1 | 55.7 | 1.76 | 5,217 | 940,724 | 814,335 | 4,810,022 | 64.1 | 922 | 45.1 / 40.0 | 275,564 | - | - | 2.1 | 15 |
| laurelin | 80 | 7 | this | 33.3 | 43.5 | 1.27 | 2,417 | 294,362 | 375,076 | 2,321,917 | 64.4 | 961 | 33.3 / 40.0 | 178,507 | 72,260 of 106,479 | 1,320,815 | 1.0 | 13 |
| laurelin | 80 | 7 | this, R8 | 43.2 | 53.4 | 2.05 | 7,485 | - | 1,204,193 | 7,596,573 | 62.2 | 1,015 | 43.2 / 104.0 | 33,319 | 46,069 of 345,598 | 659,110 | 2.9 | not timed |
| telperion | 40 | 1 | fn-363 | 25.8 | 14.4 | - | 382 | 93,168 | 88,506 | 457,100 | 43.2 | 1,196 | 25.8 / 40.0 | 1,183 | - | - | 0.2 | 1 |
| telperion | 40 | 1 | this | 22.8 | 11.9 | - | 338 | 68,128 | 76,043 | 393,545 | 44.4 | 1,165 | 22.8 / 40.0 | 2,627 | 8,308 of 21,619 | 111,586 | 0.2 | 1 |
| telperion | 40 | 1 | this, R8 | 25.3 | 14.0 | - | 387 | - | 90,156 | 469,735 | 42.9 | 1,215 | 25.3 / 104.0 | 0 | 14 of 25,582 | 91 | 0.2 | 1 |
| telperion | 40 | 7 | fn-363 | 26.9 | 21.6 | 0.64 | 930 | 177,244 | 164,121 | 753,965 | 56.6 | 811 | 26.8 / 40.0 | 4,531 | - | - | 0.3 | 1 |
| telperion | 40 | 7 | this | 23.5 | 18.5 | 0.59 | 796 | 127,277 | 137,412 | 630,501 | 58.0 | 792 | 23.5 / 40.0 | 7,703 | 16,088 of 39,139 | 181,110 | 0.3 | 1 |
| telperion | 40 | 7 | this, R8 | 26.3 | 21.1 | 0.65 | 964 | - | 171,979 | 804,743 | 56.0 | 835 | 26.2 / 104.0 | 113 | 1,587 of 49,014 | 12,546 | 0.3 | 1 |
| telperion | 80 | 1 | fn-363 | 43.5 | 46.4 | - | 3,382 | 686,557 | 605,873 | 3,608,122 | 55.8 | 1,067 | 43.5 / 40.0 | 166,463 | - | - | 1.5 | 10 |
| telperion | 80 | 1 | this | 32.2 | 30.9 | - | 1,403 | 198,309 | 256,849 | 1,672,944 | 54.6 | 1,192 | 32.2 / 40.0 | 116,656 | 46,639 of 72,725 | 890,999 | 0.8 | 7 |
| telperion | 80 | 1 | this, R8 | not run | | | | | | | | | | | | | | | |
| telperion | 80 | 7 | fn-363 | 45.1 | 55.7 | 1.73 | 5,078 | 1,002,912 | 869,384 | 5,169,814 | 58.4 | 1,018 | 45.1 / 40.0 | 270,502 | - | - | 2.2 | 16 |
| telperion | 80 | 7 | this | 33.4 | 43.6 | 1.26 | 2,385 | 322,564 | 406,977 | 2,526,224 | 58.6 | 1,059 | 33.4 / 40.0 | 181,502 | 76,501 of 115,633 | 1,388,443 | 1.1 | 12 |
| telperion | 80 | 7 | this, R8 | not run | | | | | | | | | | | | | | | |
| telperion | 82 | 1 | fn-363 | 43.8 | 47.8 | - | 3,393 | 697,902 | 611,270 | 3,701,260 | 55.5 | 1,091 | 43.8 / 40.0 | 185,576 | - | - | 1.6 | 10 |
| telperion | 82 | 1 | this | 32.4 | 31.6 | - | 1,338 | 193,560 | 245,675 | 1,629,240 | 54.5 | 1,218 | 32.4 / 40.0 | 120,270 | 45,240 of 69,810 | 882,438 | 0.7 | not timed |
| telperion | 82 | 1 | this, R8 | 42.2 | 44.6 | - | 4,859 | - | 870,456 | 5,591,003 | 55.8 | 1,151 | 42.2 / 104.0 | 21,240 | 29,145 of 249,228 | 444,471 | 2.2 | 19 |
| telperion | 82 | 7 | fn-363 | 45.5 | 57.5 | 1.76 | 5,181 | 1,020,863 | 887,895 | 5,264,660 | 58.4 | 1,016 | 45.5 / 40.0 | 294,680 | - | - | 2.2 | 16 |
| telperion | 82 | 7 | this | 33.7 | 44.4 | 1.26 | 2,299 | 315,314 | 395,657 | 2,481,927 | 58.1 | 1,080 | 33.7 / 40.0 | 185,940 | 73,967 of 112,444 | 1,366,173 | 1.1 | 13 |
| telperion | 82 | 7 | this, R8 | 43.7 | 54.3 | 2.06 | 7,469 | - | 1,311,070 | 8,344,437 | 57.0 | 1,117 | 43.7 / 104.0 | 33,431 | 45,329 of 375,488 | 682,804 | 3.2 | 27 |
