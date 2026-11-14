# Phase 3 probes: the Oregon white oak's balance, bare wood, sleepers and limbs, 12 Nov 2026

Measurement only: no product change, nothing committed, no verdicts. Checkout: `/home/daniel/Projects/telperion/.worktrees/census-p5`, detached at `487266f8f` (`fn-357-a-retired-root-keeps-only-what-is-read`), with uncommitted worktree-only readers. Scratch: `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad` (called `$S` below).

## How each number was read

- **[grower]**: from the grower's own stores between years. The probe takes the species and request `executor::growth` gives the pipeline, then `Paused::begin(.., Retiring::AtAge, false)` and `grow_to(y)` one year at a time, reading between years. Seed 1001's readings at 120 from the run to 300 are identical, line for line (A1, sheds, A4 tops), to a run asked for 120, so one long run stands for the shorter ones.
- **[recorder]**: counted as the grower grows by worktree-only hooks that only record: the balance's own sheds (in `balance_one`, after D16's walk), apexes ended by a shed, and sleepers recorded and buds woken (in `absorb`).
- **[settled]**: `Paused::settle()` at the run's last year, the structure the pipeline converts.
- **[drawn]**: the pipeline whole (the scoreboard probe's `tree`: `executor::grow`, expansion, mesh). Height and spread come from the mesh bounds; trunk at 1.3 m and limbs from the expanded tree; drawn leaf is the mesh's leaf instances.
- **[analytic]**: arithmetic on preset values.

The definitions:

- Grower leaf is the sum of the cycle's deposited leaf (`g.leaves`), the leaf light is read from.
- "h all held" is the highest rough-layout tip of any phytomer the grower holds, closed dead wood not yet retired included. "h open wood" counts only axes on the way to a living apex or a sleeping bud.
- A shed is an axis the balance shed by its own decision; the descendants it takes with it are counted under "ended". Its path is the rough layout's path to its tip. "Tip light" is the light at its own tip, read only where its own apex was alive ("lit"); for a limb or bough that has relayed, the decision rests on its subtree's Q and C.
- Bare wood is a structural node (trunk, fork, leader, limb, bough) more than 16 years old that bears no lateral whose subtree holds a living apex. A segment's length is its tip less the previous tip.
- A woken line is a woken lateral and the continuations and relays that carry it on. It is alive while any of them has a living apex, checked every year.
- A limb or bough line is a run of one PA joined by continuation or relay. It is living if an axis of the run has a living apex. Its base diameter is 2√(T/π) of the run's first phytomer's laid section in the grower, and its length is the path from the run's base to its furthest tip.
- Every leaf, height and light here is at the grower's natural size; the supernatural scale is 1.

## A. The mechanism, on the unchanged preset

### A1. Leaf, height, tips, woken buds by decade [grower, recorder]

The decade row at 80 counts sleepers and woken buds from year 1.

| run | age | grower leaf m2 | h all held | h open wood | tips | limb tips | bough tips | sleepers recorded | buds woken |
|---|---|---|---|---|---|---|---|---|---|
| 1001 | 80 | 509 | 17.8 | 17.8 | 129,311 | 6 | 70 | 64 | 48 |
| 1001 | 90 | 609 | 20.6 | 20.6 | 153,568 | 8 | 90 | 20 | 13 |
| 1001 | 100 | 695 | 23.0 | 23.0 | 175,650 | 9 | 100 | 24 | 23 |
| 1001 | 110 | 753 | 25.1 | 25.1 | 187,846 | 10 | 110 | 36 | 20 |
| 1001 | 120 | 757 | 27.0 | 27.0 | 189,672 | 11 | 130 | 32 | 31 |
| 1001 | 130 | 733 | 28.7 | 28.7 | 183,493 | 13 | 141 | 34 | 36 |
| 1001 | 140 | 662 | 30.1 | 30.1 | 164,114 | 15 | 154 | 39 | 28 |
| 1001 | 150 | 583 | 31.8 | 31.8 | 146,156 | 10 | 130 | 40 | 32 |
| 1001 | 160 | 500 | 32.1 | 31.8 | 126,891 | 7 | 107 | 22 | 21 |
| 1001 | 170 | 424 | 31.6 | 30.8 | 108,167 | 7 | 100 | 21 | 20 |
| 1001 | 180 | 373 | 31.7 | 31.0 | 95,872 | 7 | 105 | 15 | 16 |
| 1001 | 190 | 269 | 31.7 | 31.2 | 70,941 | 3 | 73 | 10 | 13 |
| 1001 | 200 | 206 | 32.6 | 32.6 | 55,499 | 3 | 57 | 7 | 7 |
| 1001 | 210 | 154 | 32.8 | 29.1 | 42,639 | 3 | 49 | 9 | 6 |
| 1001 | 220 | 120 | 31.6 | 24.6 | 35,685 | 3 | 36 | 6 | 8 |
| 1001 | 230 | 79 | 31.6 | 25.7 | 25,489 | 1 | 27 | 8 | 4 |
| 1001 | 240 | 41 | 31.6 | 26.3 | 15,259 | 0 | 10 | 0 | 0 |
| 1001 | 250 | 17 | 31.6 | 24.9 | 8,955 | 0 | 4 | 0 | 0 |
| 1001 | 260 | 12 | 31.6 | 24.1 | 7,982 | 0 | 3 | 0 | 0 |
| 1001 | 270 | 11 | 31.6 | 21.6 | 7,271 | 0 | 3 | 0 | 0 |
| 1001 | 280 | 7 | 31.6 | 22.0 | 5,711 | 0 | 2 | 0 | 0 |
| 1001 | 290 | 5 | 31.6 | 19.6 | 4,971 | 0 | 2 | 0 | 0 |
| 1001 | 300 | 4 | 31.6 | 17.8 | 4,334 | 0 | 1 | 0 | 0 |
| 1007 | 80 | 919 | 18.2 | 18.2 | 241,146 | 16 | 183 | 212 | 150 |
| 1007 | 100 | 1154 | 22.2 | 22.2 | 297,312 | 17 | 221 | 51 | 41 |
| 1007 | 120 | 1135 | 26.4 | 26.4 | 295,913 | 26 | 261 | 75 | 58 |
| 1007 | 140 | 929 | 30.3 | 30.3 | 258,269 | 19 | 225 | 45 | 50 |
| 1007 | 160 | 686 | 32.1 | 32.1 | 209,471 | 14 | 169 | 47 | 35 |
| 1007 | 180 | 476 | 32.0 | 28.2 | 165,699 | 5 | 100 | 15 | 14 |
| 1007 | 200 | 329 | 32.0 | 30.5 | 132,423 | 4 | 77 | 13 | 8 |
| 1012 | 80 | 615 | 17.9 | 17.9 | 162,404 | 9 | 100 | 106 | 77 |
| 1012 | 100 | 839 | 21.5 | 21.5 | 221,719 | 15 | 154 | 39 | 22 |
| 1012 | 120 | 905 | 24.7 | 24.7 | 244,453 | 17 | 181 | 45 | 37 |

The odd decades for 1007 are in the logs. Seed 1001's grower leaf at 120 reads 757 m² here; the survey's 737 came from fn-354's traces on an earlier branch.

### A1. The balance's own sheds by decade, structural and branch [recorder]

"Lit tip under" counts the lit tips whose light was below (path/34.8)²; "Q<C" counts the sheds whose subtree's Q was below its C in the shedding year.

| seed | age | kind | shed | lateral / cont / relay | in a woken line | lit tip | lit tip under (path/34.8)² | Q<C | path/34.8 p10/50/90 | tip light p10/50/90 (lit) | memory p10/50/90 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1001 | 80 | branch | 163 | 157/0/6 | 1 | 57 | 57 | 158 | 0.38 / 0.40 / 0.45 | 0.09 / 0.12 / 0.17 | -0.22 / -0.10 / -0.04 |
| 1001 | 90 | branch | 411 | 406/0/5 | 3 | 173 | 173 | 398 | 0.42 / 0.44 / 0.52 | 0.11 / 0.15 / 0.22 | -0.22 / -0.11 / -0.04 |
| 1001 | 100 | bough | 6 | 3/0/3 | 0 | 4 | 4 | 6 | 0.50 / 0.57 / 0.63 | 0.16 / 0.21 / 0.33 | -0.20 / -0.10 / -0.05 |
| 1001 | 100 | branch | 730 | 717/0/13 | 9 | 335 | 335 | 693 | 0.46 / 0.50 / 0.63 | 0.14 / 0.19 / 0.28 | -0.21 / -0.12 / -0.04 |
| 1001 | 110 | bough | 7 | 5/0/2 | 0 | 2 | 2 | 7 | 0.51 / 0.52 / 0.53 | 0.16 / 0.21 / 0.21 | -0.19 / -0.17 / -0.14 |
| 1001 | 110 | branch | 984 | 969/0/15 | 3 | 486 | 486 | 956 | 0.50 / 0.55 / 0.70 | 0.18 / 0.24 / 0.36 | -0.20 / -0.11 / -0.04 |
| 1001 | 120 | bough | 7 | 4/0/3 | 0 | 1 | 1 | 7 | 0.60 / 0.62 / 0.71 | 0.44 | -0.12 / -0.05 / -0.03 |
| 1001 | 120 | branch | 1297 | 1282/0/15 | 7 | 693 | 693 | 1249 | 0.53 / 0.60 / 0.77 | 0.21 / 0.28 / 0.43 | -0.20 / -0.10 / -0.04 |
| 1001 | 130 | limb | 1 | 0/0/1 | 0 | 0 | 0 | 1 | 0.58 | - | -0.03 |
| 1001 | 130 | bough | 14 | 5/0/9 | 0 | 1 | 1 | 14 | 0.56 / 0.67 / 0.85 | 0.21 | -0.19 / -0.09 / -0.03 |
| 1001 | 130 | branch | 1624 | 1619/0/5 | 4 | 964 | 964 | 1567 | 0.57 / 0.73 / 0.86 | 0.25 / 0.37 / 0.55 | -0.21 / -0.11 / -0.04 |
| 1001 | 140 | bough | 18 | 12/0/6 | 0 | 3 | 3 | 18 | 0.60 / 0.82 / 0.96 | 0.65 / 0.73 / 0.93 | -0.18 / -0.10 / -0.04 |
| 1001 | 140 | branch | 2149 | 2138/0/11 | 13 | 1378 | 1377 | 2094 | 0.61 / 0.82 / 0.94 | 0.29 / 0.46 / 0.66 | -0.23 / -0.12 / -0.04 |
| 1001 | 150 | limb | 5 | 2/0/3 | 0 | 1 | 1 | 5 | 0.71 / 0.77 / 0.93 | 0.39 | -0.25 / -0.09 / -0.04 |
| 1001 | 150 | bough | 31 | 27/0/4 | 0 | 5 | 5 | 31 | 0.65 / 0.90 / 0.97 | 0.31 / 0.64 / 0.89 | -0.18 / -0.07 / -0.02 |
| 1001 | 150 | branch | 1960 | 1952/0/8 | 7 | 1285 | 1285 | 1913 | 0.64 / 0.81 / 0.97 | 0.30 / 0.50 / 0.73 | -0.21 / -0.11 / -0.04 |
| 1001 | 160 | limb | 4 | 0/0/4 | 0 | 1 | 1 | 4 | 0.77 / 0.98 / 1.01 | 0.99 | -0.07 / -0.01 / -0.01 |
| 1001 | 160 | bough | 25 | 14/0/11 | 0 | 2 | 2 | 25 | 0.70 / 0.84 / 1.00 | 0.73 / 0.94 | -0.11 / -0.07 / -0.02 |
| 1001 | 160 | branch | 1626 | 1621/0/5 | 7 | 1035 | 1035 | 1586 | 0.66 / 0.81 / 0.98 | 0.34 / 0.52 / 0.76 | -0.20 / -0.11 / -0.04 |
| 1001 | 170 | limb | 1 | 0/0/1 | 0 | 0 | 0 | 1 | 0.65 | - | -0.02 |
| 1001 | 170 | bough | 16 | 8/0/8 | 0 | 1 | 1 | 16 | 0.77 / 0.89 / 0.99 | 0.62 | -0.13 / -0.07 / -0.03 |
| 1001 | 170 | branch | 1401 | 1396/0/5 | 8 | 894 | 894 | 1365 | 0.70 / 0.84 / 0.97 | 0.36 / 0.51 / 0.75 | -0.20 / -0.11 / -0.04 |
| 1001 | 180 | limb | 1 | 0/0/1 | 0 | 1 | 1 | 1 | 0.98 | 0.77 | -0.12 |
| 1001 | 180 | bough | 15 | 7/0/8 | 0 | 1 | 1 | 15 | 0.54 / 0.82 / 0.94 | 0.50 | -0.22 / -0.06 / -0.03 |
| 1001 | 180 | branch | 1483 | 1477/0/6 | 8 | 1017 | 1016 | 1448 | 0.74 / 0.89 / 0.95 | 0.40 / 0.55 / 0.71 | -0.21 / -0.11 / -0.05 |
| 1001 | 190 | limb | 4 | 3/0/1 | 0 | 0 | 0 | 4 | 0.81 / 0.91 / 0.93 | - | -0.05 / -0.04 / -0.04 |
| 1001 | 190 | bough | 20 | 13/0/7 | 0 | 3 | 3 | 20 | 0.65 / 0.89 / 0.97 | 0.35 / 0.54 / 0.96 | -0.14 / -0.07 / -0.02 |
| 1001 | 190 | branch | 1333 | 1330/0/3 | 5 | 901 | 901 | 1304 | 0.77 / 0.86 / 0.98 | 0.41 / 0.57 / 0.76 | -0.21 / -0.11 / -0.04 |
| 1001 | 200 | bough | 20 | 13/0/7 | 0 | 0 | 0 | 20 | 0.69 / 0.82 / 0.90 | - | -0.13 / -0.08 / -0.05 |
| 1001 | 200 | branch | 1034 | 1033/0/1 | 1 | 731 | 730 | 1010 | 0.79 / 0.87 / 1.02 | 0.43 / 0.58 / 0.83 | -0.21 / -0.11 / -0.04 |
| 1001 | 220 | bough | 14 | 11/0/3 | 0 | 3 | 3 | 14 | 0.54 / 0.89 / 1.01 | 0.75 / 0.85 | -0.17 / -0.08 / -0.02 |
| 1001 | 220 | branch | 677 | 676/0/1 | 1 | 481 | 481 | 661 | 0.86 / 0.92 / 0.97 | 0.51 / 0.64 / 0.77 | -0.19 / -0.11 / -0.04 |
| 1001 | 240 | limb | 1 | 0/0/1 | 0 | 0 | 0 | 1 | 0.93 | - | -0.02 |
| 1001 | 240 | bough | 11 | 7/0/4 | 0 | 0 | 0 | 11 | 0.69 / 0.85 / 0.91 | - | -0.07 / -0.05 / -0.02 |
| 1001 | 240 | branch | 261 | 261/0/0 | 0 | 197 | 197 | 257 | 0.87 / 0.96 / 1.01 | 0.59 / 0.72 / 0.83 | -0.16 / -0.10 / -0.04 |
| 1001 | 260 | branch | 39 | 39/0/0 | 0 | 28 | 28 | 39 | 0.87 / 0.91 / 0.93 | 0.52 / 0.64 / 0.70 | -0.18 / -0.13 / -0.04 |
| 1001 | 280 | branch | 36 | 36/0/0 | 0 | 29 | 29 | 36 | 0.93 / 0.96 / 0.98 | 0.63 / 0.72 / 0.79 | -0.14 / -0.09 / -0.04 |
| 1001 | 300 | branch | 7 | 7/0/0 | 0 | 6 | 6 | 7 | 0.52 / 1.01 / 1.01 | 0.23 / 0.72 / 0.84 | -0.14 / -0.12 / -0.09 |
| 1007 | 80 | limb / bough | 2 / 14 | 0/0/2 · 7/0/7 | 0 | 0 / 2 | 0 / 2 | 2 / 14 | 0.40 · 0.42 | - · 0.15 | |
| 1007 | 120 | limb / bough | 2 / 24 | 0/0/2 · 12/0/12 | 0 | 0 / 6 | 0 / 6 | 2 / 24 | 0.83 · 0.70 | - · 0.43 | |
| 1007 | 160 | limb / bough | 4 / 59 | 2/0/2 · 47/0/12 | 0 | 0 / 9 | 0 / 9 | 4 / 59 | 0.84 · 0.87 | - · 0.94 | |
| 1007 | 200 | limb / bough | 3 / 21 | 3/0/0 · 13/0/8 | 0 | 0 / 3 | 0 / 3 | 3 / 20 | 0.96 · 0.89 | - · 0.93 | |
| 1012 | 120 | limb / bough | 5 / 19 | 0/0/5 · 13/0/6 | 0 | 0 / 7 | 0 / 7 | 5 / 19 | 0.79 · 0.77 | - · 0.47 | |

For seed 1001, the decades 210 to 290 not shown carry 1 to 16 bough and 19 to 800 branch sheds, all at a path/34.8 median of 0.78 to 0.99. The 1007 and 1012 rows give medians only; every decade is in the logs (`tables.py sheds`).

Fine kinds, seed 1001 [recorder]:

| age | kind | shed | lit tip under (path/34.8)² / lit | path/34.8 p10/50/90 | tip light p10/50/90 |
|---|---|---|---|---|---|
| 80 | twig / short | 274 / 60 | 230/230 · 60/60 | 0.38-0.46 | 0.09 / 0.11 / 0.16 |
| 120 | twig / short / shoot | 2476 / 1243 / 17 | 1922/1922 · 1243/1243 · 14/14 | 0.54 / 0.69 / 0.79 (twig) | 0.21 / 0.32 / 0.43 (twig) |
| 160 | twig / short | 2808 / 1602 | all | 0.67 / 0.80 / 0.97 | 0.32 / 0.47 / 0.70 |
| 200 | twig / short | 1770 / 966 | all | 0.79 / 0.87 / 1.01 | 0.41 / 0.54 / 0.74 |
| 240 | twig / short | 391 / 138 | all | 0.88 / 0.96 / 0.99 | 0.57 / 0.68 / 0.77 |
| 280 | twig / short | 45 / 12 | all | 0.93 / 0.96 / 0.97 | 0.63 / 0.70 / 0.76 |

Apexes ended by a shed, its own or a bearer's, seed 1001 [recorder]:

| age | ended |
|---|---|
| 80 | branch 217, shoot 31, twig 1,214, short 4,705 |
| 120 | bough 7, branch 1,843, shoot 234, twig 10,612, short 58,813 |
| 150 | limb 7, bough 48, branch 3,308, shoot 288, twig 17,511, short 102,930 |
| 200 | bough 20, branch 1,758, shoot 137, twig 9,289, short 56,118 |
| 250 | bough 6, branch 278, twig 1,330, short 8,091 |
| 300 | bough 1, branch 29, twig 126, short 702 |

What this shows:

- Of the lit tips the balance shed, all but a handful had light below (path/34.8)² [recorder, `check.py`]:
  - seed 1001 to 300: 3 of 57,378 above (branch tips at 140, 180 and 200);
  - seed 1007 to 200: 8 of 97,272 above;
  - seed 1012 to 120: 0 of 21,661 above.
- Q<C held in the shedding year for 87 to 91% of all sheds. For twigs and shorts, Q<C equals the lit count exactly: an unlit subtree has Q = C = 0 and is shed on its remembered balance.
- The median path/34.8 at shedding rises from 0.40 at 80 to 0.85 to 0.95 from 180 on, and the lit-tip light at shedding rises with it, from 0.12 to about 0.7.
- Limb and bough sheds are mostly subtrees whose own apex was no longer alive; under half of those per decade have a lit own tip.
- No limb or bough line in a woken line was shed on the unchanged preset.

### A2. Bare structural wood and leaf by the age of the wood it stands on [grower]

| seed | age | wood | structural m | older than 16 y, m | older than 16 y with no living lateral at the node, m |
|---|---|---|---|---|---|
| 1001 | 120 | held | 1106.1 | 762.4 | 731.1 |
| 1001 | 120 | open | 1092.6 | 753.6 | 722.4 |
| 1001 | 200 | held | 1363.5 | 1191.4 | 1183.9 |
| 1001 | 200 | open | 1057.5 | 906.5 | 899.1 |
| 1001 | 300 | held | 289.8 | 288.4 | 288.1 |
| 1001 | 300 | open | 50.4 | 49.8 | 49.5 |
| 1007 | 120 | held | 2467.2 | 1722.0 | 1678.2 |
| 1007 | 120 | open | 2358.4 | 1639.4 | 1595.7 |
| 1007 | 200 | held | 1527.9 | 1357.3 | 1345.3 |
| 1007 | 200 | open | 998.2 | 860.1 | 848.1 |
| 1012 | 120 | held | 1571.1 | 1042.7 | 1009.2 |
| 1012 | 120 | open | 1491.6 | 997.6 | 964.1 |

By kind, held, seed 1001 (m; older than 16 y; of it bare):

| kind | at 120 | at 200 |
|---|---|---|
| trunk | 4.4; 4.4; 4.4 | 4.4; 4.4; 4.4 |
| fork | 0.6; 0.6; 0.5 | 0.6; 0.6; 0.5 |
| leader | 2.0; 2.0; 1.9 | 2.0; 2.0; 1.9 |
| limb | 122.8; 89.1; 84.7 | 181.9; 173.4; 171.6 |
| bough | 976.2; 666.2; 639.5 | 1174.5; 1010.9; 1005.5 |

Leaf in m² by the age of the wood it stands on. "Bearer" is the node the leafy axis's line leaves from. "Structural" is the node where its chain of laterals leaves the nearest trunk, fork, leader, limb or bough; leaf on a structural axis's own new wood counts as 0.

| seed | age | grower leaf | bearer 0-5 / 6-16 / 17-26 / 27+ y | structural 0-5 / 6-16 / 17-26 / 27+ y | sleepers pending (to wake later) |
|---|---|---|---|---|---|
| 1001 | 120 | 757 | 735 / 22 / 0 / 1 | 52 / 313 / 152 / 241 | 41 |
| 1001 | 200 | 206 | 200 / 5 / 0 / 0 | 23 / 87 / 32 / 64 | 8 |
| 1001 | 300 | 4 | 4 / 0 / 0 / 0 | 0 / 1 / 1 / 2 | 0 |
| 1007 | 120 | 1135 | 1099 / 34 / 0 / 2 | 122 / 540 / 211 / 261 | 79 |
| 1007 | 200 | 329 | 319 / 9 / 0 / 0 | 24 / 114 / 54 / 137 | 13 |
| 1012 | 120 | 905 | 878 / 26 / 0 / 1 | 89 / 423 / 189 / 205 | 54 |

Sleepers recorded and woken per decade are the last two columns of A1 [recorder]. At 120: 32, 75 and 45 recorded, and 31, 58 and 37 woken, on seeds 1001, 1007 and 1012.

What this shows:

- 66 to 70% of structural wood is older than 16 years at 120, and 96 to 97% of that bears no living lateral at its node. At 200, 87 to 89% is older than 16, and 99% of that is bare.
- Leaf is on young bearers: 97% stands on a bearer node 5 years old or less.
- Counted from the structural node the leaf's chain leaves, 23 to 32% of leaf at 120 hangs from structural wood older than 26 years.

### A3. Light at trunk-line, limb and bough nodes under 12 m of path [grower]

Light is read on a lattice built from the cycle's leaves (`Field::quantised`, the same light the tips read) at each held node of those PAs whose path is under 12 m. "Above (path/34.8)²" counts nodes whose light exceeds the break-even of a tip standing at that node's path. No trunk-line node lies at 8 to 12 m of path, and no limb or bough node under 4 m.

| seed | age | wood | path m | nodes | light p10 / 50 / 90 | nodes above (path/34.8)² |
|---|---|---|---|---|---|---|
| 1001 | 120 | trunk line | 0-4 / 4-8 | 88 / 65 | 0.70 / 0.75 / 0.80 · 0.48 / 0.57 / 0.66 | all |
| 1001 | 120 | limb | 4-8 / 8-12 | 331 / 511 | 0.45 / 0.51 / 0.59 · 0.36 / 0.43 / 0.48 | all |
| 1001 | 120 | bough | 4-8 / 8-12 | 803 / 3899 | 0.43 / 0.48 / 0.52 · 0.33 / 0.42 / 0.52 | all |
| 1001 | 160 | trunk line | 0-4 / 4-8 | 88 / 65 | 0.80 / 0.82 / 0.87 · 0.77 / 0.78 / 0.79 | all |
| 1001 | 160 | limb | 4-8 / 8-12 | 331 / 511 | 0.76 / 0.79 / 0.82 · 0.54 / 0.66 / 0.75 | all |
| 1001 | 160 | bough | 4-8 / 8-12 | 650 / 3642 | 0.73 / 0.78 / 0.81 · 0.57 / 0.67 / 0.76 | all |
| 1001 | 200 | trunk line | 0-4 / 4-8 | 88 / 65 | 0.85 / 0.87 / 0.89 · 0.77 / 0.81 / 0.84 | all |
| 1001 | 200 | limb | 4-8 / 8-12 | 331 / 511 | 0.76 / 0.78 / 0.87 · 0.81 / 0.86 / 0.89 | all |
| 1001 | 200 | bough | 4-8 / 8-12 | 525 / 2787 | 0.76 / 0.78 / 0.83 · 0.77 / 0.86 / 0.93 | all |
| 1007 | 120 | trunk line | 0-4 / 4-8 | 89 / 62 | 0.58 / 0.69 / 0.72 · 0.30 / 0.44 / 0.54 | all |
| 1007 | 120 | limb | 4-8 / 8-12 | 840 / 1427 | 0.17 / 0.33 / 0.46 · 0.13 / 0.33 / 0.42 | all |
| 1007 | 120 | bough | 4-8 / 8-12 | 2370 / 11416 | 0.14 / 0.29 / 0.43 · 0.14 / 0.26 / 0.39 | all |
| 1007 | 160 | trunk line | 0-4 / 4-8 | 89 / 62 | 0.72 / 0.79 / 0.81 · 0.44 / 0.58 / 0.67 | all |
| 1007 | 160 | limb | 4-8 / 8-12 | 840 / 1509 | 0.30 / 0.45 / 0.59 · 0.19 / 0.44 / 0.57 | all |
| 1007 | 160 | bough | 4-8 / 8-12 | 2186 / 12391 | 0.28 / 0.40 / 0.56 · 0.19 / 0.30 / 0.55 | all |
| 1007 | 200 | trunk line | 0-4 / 4-8 | 89 / 62 | 0.81 / 0.88 / 0.92 · 0.51 / 0.67 / 0.76 | all |
| 1007 | 200 | limb | 4-8 / 8-12 | 782 / 1285 | 0.41 / 0.58 / 0.70 · 0.26 / 0.46 / 0.72 | all |
| 1007 | 200 | bough | 4-8 / 8-12 | 1606 / 10375 | 0.40 / 0.50 / 0.64 · 0.26 / 0.36 / 0.64 | all |
| 1012 | 120 | trunk line | 0-4 / 4-8 | 88 / 62 | 0.55 / 0.60 / 0.67 · 0.31 / 0.34 / 0.50 | all |
| 1012 | 120 | limb | 4-8 / 8-12 | 569 / 949 | 0.27 / 0.33 / 0.41 · 0.09 / 0.29 / 0.42 | all; 894 of 949 at 8-12 |
| 1012 | 120 | bough | 4-8 / 8-12 | 3074 / 8605 | 0.26 / 0.31 / 0.39 · 0.13 / 0.24 / 0.41 | all; 8453 of 8605 at 8-12 |

At 12 m the break-even is (12/34.8)² = 0.119 [analytic]. Every node's light is above it, except 55 limb and 152 bough nodes at 8 to 12 m of path on seed 1012 at 120. Light at these nodes rises from 120 to 200 on seeds 1001 and 1007; 1012 was read at 120 only.

### A4. Living limb and bough lines at 80 and 120 (and 200) [grower]

| seed | age | kind | living lines | on the trunk line: lateral / carried on | bearer: trunk line / limb / bough | base diameter p10/50/90 cm | length p10/50/90 m |
|---|---|---|---|---|---|---|---|
| 1001 | 80 | limb | 6 | 3 / 1 | 4 / 2 / 0 | 4.5 / 31.3 / 34.7 | 1.9 / 11.3 / 15.5 |
| 1001 | 80 | bough | 70 | 3 / 0 | 3 / 67 / 0 | 1.4 / 6.1 / 12.0 | 1.0 / 4.2 / 10.2 |
| 1001 | 120 | limb | 11 | 3 / 1 | 4 / 7 / 0 | 4.1 / 16.3 / 53.7 | 4.1 / 8.8 / 21.0 |
| 1001 | 120 | bough | 130 | 2 / 0 | 2 / 128 / 0 | 1.1 / 5.5 / 14.9 | 0.7 / 6.5 / 15.1 |
| 1001 | 200 | limb | 3 | 1 / 0 | 1 / 2 / 0 | 4.8 / 20.7 / 66.6 | 3.6 / 11.8 / 25.5 |
| 1001 | 200 | bough | 57 | 1 / 0 | 1 / 56 / 0 | 1.9 / 5.5 / 15.1 | 1.7 / 7.2 / 21.7 |
| 1007 | 80 | limb | 16 | 6 / 1 | 7 / 9 / 0 | 7.1 / 23.4 / 38.8 | 3.6 / 11.3 / 14.8 |
| 1007 | 80 | bough | 183 | 0 / 0 | 0 / 183 / 0 | 1.0 / 4.8 / 10.3 | 0.6 / 4.1 / 10.0 |
| 1007 | 120 | limb | 26 | 3 / 1 | 4 / 22 / 0 | 1.8 / 6.7 / 53.0 | 0.4 / 4.1 / 18.6 |
| 1007 | 120 | bough | 261 | 0 / 0 | 0 / 261 / 0 | 1.2 / 4.9 / 12.1 | 0.9 / 5.3 / 14.9 |
| 1007 | 200 | limb | 4 | 2 / 0 | 2 / 2 / 0 | 1.7 / 42.7 / 66.6 | 0.8 / 11.0 / 30.5 |
| 1012 | 80 | limb | 9 | 4 / 1 | 5 / 4 / 0 | 2.0 / 18.2 / 44.0 | 1.6 / 9.8 / 13.3 |
| 1012 | 80 | bough | 100 | 2 / 0 | 2 / 98 / 0 | 1.1 / 4.9 / 12.0 | 0.5 / 3.5 / 9.4 |
| 1012 | 120 | limb | 17 | 4 / 1 | 5 / 12 / 0 | 2.5 / 6.2 / 50.4 | 1.7 / 6.0 / 19.6 |
| 1012 | 120 | bough | 181 | 1 / 0 | 1 / 180 / 0 | 1.5 / 5.2 / 13.7 | 1.0 / 5.4 / 14.7 |

First-order limbs leaving the trunk line and living at 120 number 3, 3 and 4 laterals, plus the leader carried on as a limb, on seeds 1001, 1007 and 1012. The count is of lines living at the read; dead ones are not counted.

The largest living lines at 120 (base diameter cm, length m):

| seed | limbs 1-10 | boughs 1-10 |
|---|---|---|
| 1001 | 54.8/16.9, 53.7/21.0, 49.6/24.7, 47.1/15.4 (trunk line); 23.4/12.4, 16.3/8.8, 14.9/8.6, 6.3/5.7, 5.7/5.0, 4.1/4.1 (on limbs) | 19.4/21.5, 19.3/19.9, 18.7/15.7, 18.3/18.3, 17.7/17.1, 17.1/13.1, 16.4/15.1, 16.2/15.8, 15.7/15.4, 15.6/15.1 |
| 1007 | 75.6/23.8, 62.6/18.6 (trunk line), 53.0/19.1, 39.4/17.9 (limb), 37.7/17.8 (trunk line), ... | 21.6/21.3, 20.0/20.6, 19.1/20.2, 18.6/21.4, 18.2/19.7, ... |
| 1012 | 74.3/23.2, 60.8/20.6, 50.4/19.6, 36.0/15.5, 21.9/6.8 (all trunk line), ... | 24.2/22.4, 23.1/21.9, 20.0/20.4, 19.7/18.9, 19.4/22.3, ... |

At 120 the ten largest boughs on 1001 span 15.6 to 19.4 cm at the base and 13 to 22 m in length. The fourth-largest limb and the largest bough are 47 and 19 cm.

### A1/A2. Woken lines on the unchanged preset [grower, yearly]

Sleepers here are branch buds.

| seed | read at | woken lines | dead | alive | woken 41+ years before the read: lived ≥10 / ≥20 / ≥40 y |
|---|---|---|---|---|---|
| 1001 | 300 | 326 | 326 | 0 | 0 / 0 / 0 of 326 |
| 1007 | 200 | 572 | 569 | 3 | 0 / 0 / 0 of 511 |
| 1012 | 120 | 194 | 188 | 6 | 0 / 0 / 0 of 77 |

Life of the dead by decade woken, seed 1001: median 1 to 8 years, 90th percentile 4 to 9 years in every cohort from 21-30 to 221-230. No woken line on any seed lived 10 years. Woken lines are 0 to 2% of the balance's sheds, column "in a woken line" in the shed table. Per-cohort rows are in the logs (`tables.py lines`).

### A5. Why the drawn tree at 300 is shorter than the grower's height

**When the leaf went** (seed 1001):

| age | 200 | 220 | 240 | 250 | 260 | 270 | 280 | 290 | 300 |
|---|---|---|---|---|---|---|---|---|---|
| grower leaf m² [grower] | 206 | 120 | 41 | 17 | 12 | 11 | 7 | 5 | 4.1 |
| living tips [grower] | 55,499 | 35,685 | 15,259 | 8,955 | 7,982 | 7,271 | 5,711 | 4,971 | 4,334 |
| grower height, all held / open wood m [grower] | 32.6 / 32.6 | 31.6 / 24.6 | 31.6 / 26.3 | 31.6 / 24.9 | 31.6 / 24.1 | 31.6 / 21.6 | 31.6 / 22.0 | 31.6 / 19.6 | 31.6 / 17.8 |
| drawn height m [drawn] | 32.7 | 25.4 | 27.2 | 28.2 | 24.3 | 21.2 | 21.7 | 20.1 | 7.8 |
| drawn leaf m² [drawn] | 290 | 170 | 52 | 13 | 8.4 | 6.9 | 2.9 | 0.0 | 0 |

The drawn leaf is gone between 280 (2.9 m²) and 290 (0.0 m²). The drawn height falls from 20.1 m at 290 to 7.8 m at 300, and the drawn spread from about 22 m at 260 to under 10 m from 270 on [drawn]. The grower never reaches zero leaf by 300: 4.1 m² and 4,334 living tips remain [grower]. The drawn tree at 300 has no leaf and stands 7.8 m tall, 2.8 x 2.8 m: the leafless still shows a bare trunk post [drawn]. Grower leaf by year from 200 to 300 is in `census-base-300-1001.log` (`YEAR` lines).

**What the grower's height reads** [grower, 300]: its 31.6 m is a limb that last grew in cycle 152, and below it shorts, limbs and boughs that died between 150 and 164. Every one of them is closed: dead, nothing in it living or sleeping, held but not yet retired. The grower's height counts them (`h all held`); it has held 31.60 m unchanged from 220 to 300. The open wood, on the way to a living apex, reaches 17.8 m.

**What the settle makes of it** [settled]:

| at | living tips | grower tip height p10/50/90/max | settled tip height p10/50/90/max | settled/grower path to the tip p10/50/90 | settled tip phytomer scale p10/50/90 | settled tree height |
|---|---|---|---|---|---|---|
| 120 | 189,672 | 11.4 / 16.6 / 22.8 / 27.0 | 11.3 / 16.5 / 23.0 / 27.1 | 0.96 / 1.00 / 1.00 | 0.03 / 0.62 / 1.29 | 27.1 |
| 200 | 55,499 | 13.5 / 20.1 / 22.7 / 32.6 | 13.5 / 20.0 / 22.4 / 32.6 | 0.96 / 1.00 / 1.00 | 0.03 / 0.56 / 1.10 | 32.6 |
| 250 | 8,955 | 12.6 / 13.2 / 21.4 / 24.9 | 5.2 / 5.2 / 21.0 / 25.1 | 0.30 / 0.34 / 1.00 | 0.00 / 0.00 / 0.69 | - |
| 300 | 4,334 | 12.9 / 13.3 / 13.4 / 17.5 | 4.8 / 4.8 / 4.8 / 5.1 | 0.25 / 0.26 / 0.27 | 0.00 / 0.00 / 0.00 | 7.7 |

Tips are matched by lineage. Relays carry their line's lineage, so 60% of the living tips found a match (2,632 of 4,334 at 300). At 300 the settled tree's tallest phytomers are a dead limb last grown in cycle 23 (7.69 m) and the dead leader (7.08 m); the living wood stands at or under 5.1 m.

**Where the size goes** [grower and settled, 300]: the chain from the tallest living tip (a short shoot born 298) to the seed runs through a twig, 13 branch axes (one a relay), a bough lateral born 148 with 6 relays, a limb lateral born 123 with 3 relays, and the limb line it stands on, a lateral born 14 with 18 relays. Then come fork and trunk.

- In the grower every axis on it keeps its rough-layout scale: 0.79 on the limb line, 0.48 on the boughs, 0.10 to 0.48 on the branches, and running and kept presence 1.0.
- In the settled tree the same chain's vigour is 0.104 on the limb lateral born 14 and 0.132 on every limb axis after it (relays and the lateral born 123), 0.217 on the boughs and on the branches up to 236, 0.30 at 237, and 1.0 above.
- Phytomer scale falls along the limb relays: 0.107 on the lateral (born 14), 0.013 at the relay born 15, 0.0017 at 26, 0.0002 at 27, then 0.0000. Every axis above it is drawn at scale 0.
- The settle multiplies each relay's base scale by its vigour (`geometry::scale`). A relay's vigour is its parent's end presence times its own birth draws (`presence::assign`).
- At 250 the same limb line's relays have vigour 1.0, scale 0.43 to 0.92, and radius 0.35 to 0.06 m.

So between 250 and 300 the settle's end presence on this line's relays fell from 1.0 to 0.13 to 0.22, and it compounds down the chain to zero. Which input of `assign_one` moves it is not determined [U]: the living a fade reads at the age, the windows taken at the request age, or something else. The drawn height at 300 is then the dead stub of the old limb and leader, and the leafless crown is drawn at scale 0.

## B. Preset-only probes

Scratch value files written by `$S/variants.py` from the checked-out `oregon-white-oak.values`. Each was swapped into the worktree for its build and the base file restored after; `cmp` confirms the restore.

- **B1**: the three limb zones' `dormant/9 = 0.1` become `dormant/7 = 0.1` (bough buds, same probability); delay 6 and rate 0.04 unchanged.
- **B2**: as B1, with the three limb zones' `rate = 0.012` [analytic, window from `dormant::draw` → `dormant::aged`: a bough bud is recorded only if it wakes before the bough's 60 years].
  - At 0.04 the untruncated mean wait is 6 + 25 = 31 y. A bough bud is recorded 88.5% of the time, and its mean wait is then 24.0 y.
  - At 0.012, 47.7% are recorded, with a mean wait of 30.1 y. That is the rate set, so that the mean wait of a recorded bud is about 30 years.
  - On the unchanged preset, by the survey's 16-year window, 33% are recorded, with a mean wait of 10.7 y.
- **B3**: `/space/ages/6/form/dominance = 0.5` added (limb row only); sleepers unchanged.

### B. Leaf, height, tips, woken buds, structural sheds [grower, recorder]

| variant | seed | age | grower leaf m² | grower height m | living tips | limb / bough tips | buds woken in the decade | woken lines alive | limb / bough shed in the decade |
|---|---|---|---|---|---|---|---|---|---|
| base | 1001 | 80 | 509 | 17.8 | 129,311 | 6 / 70 | 48* | 4 | 0 / 0 |
| base | 1001 | 120 | 757 | 27.0 | 189,672 | 11 / 130 | 31 | 11 | 0 / 7 |
| base | 1001 | 160 | 500 | 32.1 | 126,891 | 7 / 107 | 21 | 9 | 4 / 25 |
| base | 1001 | 200 | 206 | 32.6 | 55,499 | 3 / 57 | 7 | 3 | 0 / 20 |
| base | 1007 | 120 | 1135 | 26.4 | 295,913 | 26 / 261 | 58 | 19 | 2 / 24 |
| base | 1012 | 120 | 905 | 24.7 | 244,453 | 17 / 181 | 37 | 6 | 5 / 19 |
| B1 | 1001 | 80 | 779 | 19.2 | 231,244 | 6 / 174 | 109* | 104 | 0 / 5 |
| B1 | 1001 | 100 | 1108 | 23.8 | 333,401 | 8 / 225 | 28 | 133 | 0 / 21 |
| B1 | 1001 | 120 | 1194 | 26.6 | 355,102 | 7 / 252 | 47 | 153 | 2 / 36 |
| B1 | 1001 | 140 | 1055 | 30.1 | 317,748 | 7 / 223 | 21 | 140 | 0 / 57 |
| B1 | 1001 | 160 | 867 | 32.3 | 255,078 | 4 / 176 | 15 | 130 | 2 / 38 |
| B1 | 1001 | 180 | 707 | 32.2 | 207,555 | 5 / 165 | 26 | 107 | 0 / 40 |
| B1 | 1001 | 200 | 502 | 32.2 | 150,594 | 6 / 135 | 21 | 72 | 5 / 54 |
| B1 | 1007 | 120 | 1726 | 29.8 | 532,035 | 27 / 548 | 109 | 313 | 1 / 123 |
| B1 | 1012 | 120 | 1443 | 29.1 | 492,820 | 12 / 359 | 52 | 228 | 2 / 70 |
| B2 | 1001 | 80 | 630 | 18.1 | 172,804 | 6 / 120 | 50* | 50 | 0 / 0 |
| B2 | 1001 | 100 | 936 | 22.8 | 259,099 | 8 / 177 | 19 | 80 | 1 / 7 |
| B2 | 1001 | 120 | 1054 | 27.0 | 298,484 | 9 / 207 | 23 | 97 | 0 / 28 |
| B2 | 1001 | 140 | 1006 | 30.2 | 291,885 | 9 / 217 | 22 | 115 | 0 / 39 |
| B2 | 1001 | 160 | 837 | 32.1 | 242,055 | 4 / 155 | 13 | 99 | 1 / 46 |
| B2 | 1001 | 180 | 661 | 32.7 | 188,410 | 1 / 122 | 9 | 85 | 1 / 28 |
| B2 | 1001 | 200 | 474 | 32.7 | 138,138 | 0 / 74 | 1 | 59 | 2 / 27 |
| B2 | 1007 | 120 | 1597 | 28.5 | 463,459 | 23 / 403 | 28 | 176 | 3 / 61 |
| B2 | 1012 | 120 | 1318 | 27.0 | 389,778 | 16 / 300 | 33 | 141 | 1 / 45 |
| B3 | 1001 | 80 | 352 | 19.0 | 129,156 | 6 / 70 | 48* | 4 | 0 / 0 |
| B3 | 1001 | 100 | 521 | 22.7 | 208,112 | 9 / 106 | 23 | 7 | 0 / 0 |
| B3 | 1001 | 120 | 599 | 25.4 | 265,228 | 10 / 137 | 27 | 9 | 0 / 5 |
| B3 | 1001 | 140 | 561 | 29.1 | 289,613 | 12 / 162 | 22 | 11 | 1 / 9 |
| B3 | 1001 | 160 | 505 | 29.2 | 295,913 | 13 / 161 | 30 | 12 | 1 / 13 |
| B3 | 1001 | 180 | 545 | 29.6 | 328,673 | 10 / 197 | 34 | 12 | 4 / 19 |
| B3 | 1001 | 200 | 570 | 30.0 | 351,497 | 11 / 202 | 26 | 7 | 0 / 10 |
| B3 | 1007 | 120 | 981 | 25.7 | 611,185 | 36 / 385 | 72 | 29 | 3 / 25 |
| B3 | 1012 | 120 | 818 | 25.4 | 284,863 | 15 / 177 | 25 | 8 | 2 / 8 |

\* From year 1. Every decade from 80 to 200 for 1001, and 80 and 120 for 1007 and 1012, is in the logs (`cmp.py`).

Bough sheds in woken lines [recorder, `check.py`] take 71 to 72% of all bough sheds in B1 (367 of 508 on 1001 to 200) and 40 to 45% in B2 (155 of 342); there are none on base or B3. At 200 on seed 1001 the bough sheds' median path/34.8 is 0.82 in B1, 0.76 in B2, 0.82 on base and 0.49 in B3. Lit tips shed with light above (path/34.8)²: 3 to 15 per run in B1, 5 to 10 in B2, 1 to 2 in B3, of 9,117 to 70,310 lit sheds.

### B. How long a woken line lives [grower, yearly]

| variant | seed | read at | woken lines | dead | alive | woken 41+ years before the read: lived ≥10 / ≥20 / ≥40 y |
|---|---|---|---|---|---|---|
| base | 1001 | 300 | 326 | 326 | 0 | 0 / 0 / 0 of 326 |
| base | 1007 | 200 | 572 | 569 | 3 | 0 / 0 / 0 of 511 |
| base | 1012 | 120 | 194 | 188 | 6 | 0 / 0 / 0 of 77 |
| B1 | 1001 | 200 | 460 | 388 | 72 | 229 / 193 / 167 of 359 |
| B1 | 1007 | 120 | 623 | 310 | 313 | 240 / 212 / 177 of 272 |
| B1 | 1012 | 120 | 381 | 153 | 228 | 144 / 140 / 134 of 170 |
| B2 | 1001 | 200 | 230 | 171 | 59 | 157 / 128 / 114 of 199 |
| B2 | 1007 | 120 | 263 | 87 | 176 | 102 / 93 / 88 of 111 |
| B2 | 1012 | 120 | 180 | 39 | 141 | 65 / 64 / 63 of 69 |
| B3 | 1001 | 200 | 350 | 343 | 7 | 0 / 0 / 0 of 239 |
| B3 | 1007 | 120 | 387 | 358 | 29 | 0 / 0 / 0 of 156 |
| B3 | 1012 | 120 | 173 | 165 | 8 | 0 / 0 / 0 of 79 |

B1, seed 1001, by decade woken (life of the dead p10/50/90 years; lived ≥40 of woken):

| woke in | 31-40 | 51-60 | 71-80 | 91-100 | 111-120 | 131-140 | 151-160 | 171-180 |
|---|---|---|---|---|---|---|---|---|
| woken / dead | 13 / 11 | 20 / 15 | 25 / 16 | 28 / 26 | 47 / 44 | 21 / 21 | 15 / 13 | 26 / 24 |
| life of the dead | 67 / 106 / 140 | 49 / 81 / 140 | 5 / 57 / 98 | 5 / 9 / 79 | 3 / 8 / 67 | 6 / 9 / 33 | 4 / 7 / 13 | 3 / 6 / 9 |
| lived ≥40 | 13 | 20 | 18 | 8 | 16 | 1 | 2 | 0 (≤29 y possible) |

B2, seed 1001: lines woken from 31 to 80 that died lived a median 87 to 117 years, and 3 to 12 per cohort still live at 200. Those woken from 81 to 140 that died lived a median 12 to 68 years. Of those woken from 111 to 150, 5 to 9 per cohort of 18 to 25 lived 40 years or more.

### B. The drawn trees [drawn]

| run | tree | height m | spread x / z m | trunk at 1.3 m cm | drawn leaf m² | three largest limbs' base cm |
|---|---|---|---|---|---|---|
| base | 120 seed 1001 | 27.3 | 25.4 / 23.1 | 107 | 1,069 | 55 / 50 / 47 |
| base | 120 seed 1007 | 27.1 | 32.5 / 31.1 | 138 | 1,644 | 76 / 54 / 53 |
| base | 120 seed 1012 | 25.6 | 31.9 / 27.2 | 118 | 1,303 | 74 / 61 / 36 |
| base | 200 seed 1001 | 32.7 | 30.3 / 29.1 | 133 | 290 | 77 / 67 / 55 |
| base | 300 seed 1001 | 7.8 | 2.8 / 2.8 | 135 | 0 | 36 / 25 |
| B1 | 120 seed 1001 | 27.1 | 29.5 / 30.8 | 133 | 1,752 | 65 / 59 / 55 |
| B1 | 120 seed 1007 | 29.4 | 39.4 / 37.8 | 166 | 2,579 | 85 / 70 / 68 |
| B1 | 120 seed 1012 | 29.4 | 38.6 / 33.3 | 147 | 2,165 | 93 / 77 / 40 |
| B1 | 200 seed 1001 | 30.3 | 33.6 / 30.8 | 172 | 702 | 95 / 86 / 71 |
| B2 | 120 seed 1001 | 27.4 | 27.0 / 27.0 | 123 | 1,539 | 61 / 59 / 54 |
| B2 | 120 seed 1007 | 28.1 | 38.3 / 36.0 | 157 | 2,374 | 87 / 64 / 58 |
| B2 | 120 seed 1012 | 27.7 | 35.9 / 30.6 | 136 | 1,959 | 87 / 71 / 41 |
| B2 | 200 seed 1001 | 31.3 | 33.5 / 30.6 | 163 | 652 | 88 / 79 / 76 |
| B3 | 120 seed 1001 | 26.9 | 25.3 / 26.2 | 93 | 873 | 39 / 29 / 25 |
| B3 | 120 seed 1007 | 25.5 | 27.0 / 30.2 | 117 | 1,497 | 56 / 48 / 35 |
| B3 | 120 seed 1012 | 25.6 | 26.7 / 24.3 | 109 | 1,190 | 65 / 52 / 22 |
| B3 | 200 seed 1001 | 23.1 | 17.6 / 18.8 | 133 | 861 | 77 / 60 / 46 |

The drawn leaf (mesh leaf instances) is a different measure from the grower's leaf, and runs 1.4 to 1.5 times it at 120.

### B3. Limb and bough lines with dominance 0.5 on the limb row [grower]

| seed | age | kind | living lines | on trunk line: lateral / carried on | base diameter p10/50/90 cm | length p10/50/90 m |
|---|---|---|---|---|---|---|
| 1001 | 80 | limb | 6 | 3 / 1 | 2.6 / 15.4 / 36.4 | 0.7 / 6.5 / 15.0 |
| 1001 | 80 | bough | 70 | 3 / 0 | 0.9 / 3.6 / 11.9 | 0.3 / 2.3 / 7.5 |
| 1001 | 120 | limb | 10 | 3 / 1 | 1.5 / 8.1 / 39.1 | 0.9 / 4.6 / 11.4 |
| 1001 | 120 | bough | 137 | 3 / 0 | 0.9 / 4.0 / 11.5 | 0.4 / 3.1 / 10.3 |
| 1007 | 80 | limb | 18 | 7 / 1 | 1.1 / 15.0 / 27.4 | 0.6 / 4.3 / 8.9 |
| 1007 | 80 | bough | 204 | 0 / 0 | 0.4 / 2.2 / 7.8 | 0.1 / 1.7 / 5.5 |
| 1007 | 120 | limb | 36 | 6 / 1 | 0.4 / 2.5 / 35.4 | 0.1 / 1.1 / 12.9 |
| 1007 | 120 | bough | 385 | 0 / 0 | 0.3 / 2.5 / 9.4 | 0.1 / 1.7 / 8.7 |
| 1012 | 80 | limb | 10 | 4 / 1 | 1.4 / 12.2 / 36.7 | 0.5 / 5.0 / 12.9 |
| 1012 | 80 | bough | 103 | 2 / 0 | 0.7 / 4.2 / 10.6 | 0.2 / 2.8 / 8.9 |
| 1012 | 120 | limb | 15 | 4 / 1 | 0.8 / 5.0 / 60.1 | 0.2 / 2.5 / 21.1 |
| 1012 | 120 | bough | 177 | 2 / 0 | 1.1 / 4.6 / 12.2 | 0.5 / 4.1 / 12.3 |

The largest limbs at 120 (base cm / length m):

| seed | base | B3 |
|---|---|---|
| 1001 | 54.8/16.9, 53.7/21.0, 49.6/24.7, 47.1/15.4, 23.4/12.4 | 61.8/24.3, 39.1/11.4, 28.5/10.6, 25.4/8.5, 8.1/2.3 |
| 1007 | 75.6/23.8, 62.6/18.6, 53.0/19.1, 39.4/17.9, 37.7/17.8 | 79.7/25.9, 56.2/22.4, 47.8/15.2, 35.4/12.9, 34.3/10.0 |
| 1012 | 74.3/23.2, 60.8/20.6, 50.4/19.6, 36.0/15.5, 21.9/6.8 | 64.6/21.1, 60.1/21.6, 51.9/15.9, 41.5/19.0, 22.2/7.7 |

On seed 1001 at 120, the trunk line's four limbs measure 62/39/29/25 cm in B3 against 55/54/50/47 cm on base. The drawn tree's three largest limbs measure 39/29/25 cm against 55/50/47.

### B. Stills

One sheet per variant: `$S/sheets/B1.jpg`, `$S/sheets/B2.jpg` and `$S/sheets/B3.jpg`. Each is oak seed 1001, the unchanged preset on the left and the variant on the right; the rows are 120 whole, 120 leafless, 200 whole and 200 leafless. The stills are in `$S/stills/<run>/oak-<age>-1001-{whole,bare}.png`, and the base tree at 300 is `$S/stills/base/oak-300-1001-{whole,bare}.png`. What they show:

- **B1**: at 120 the crown is wider (29.5 x 30.8 against 25.4 x 23.1 m) and denser, with more limbs inside it. At 200 the crown holds leaf through its middle and upper part, where base has sparse leaf at its periphery and long bare wandering limbs.
- **B2**: like B1 at 120 and 200, a little less dense.
- **B3**: at 120 the crown is more open, with fewer heavy limbs and a thinner trunk (93 against 107 cm). At 200 it is shorter (23.1 against 32.7 m) and narrower (17.6 x 18.8 m), with a compact, dense leafy crown.
- **Base at 300**: a bare trunk post, 7.8 m tall, with no limbs and no leaf.

## Commands

Worktree setup (as asked):

```
git -C census-p5 stash push -u -m census-p5-readers-phase3      # 6ed959c80e65bc63abccf6e08101159c4530ce60
git -C census-p5 checkout --detach fn-357-a-retired-root-keeps-only-what-is-read   # 487266f8f
git -C census-p5 stash apply 6ed959c80e65bc63abccf6e08101159c4530ce60             # applied clean
git -C census-p5 stash drop stash@{0}                           # re-found by its tag first
```

`phase5.rs` was adapted to the narrower phytomer: `p.scale` and `p.tip` became `Node::scale()` and `Node::tip()`.

Uncommitted readers and recorders in the worktree (no product behaviour changed; they only record or read):

- `crates/telperion-space/src/grow/cycle/phase3.rs` holds the recorders and the yearly woken-line tracker.
- `crates/telperion-space/src/grow/cycle/phase3_read.rs` holds the readers A1, A2, A3, A4, lines, leaf, top, live tips and chain.
- Hooks are one line each:
  - `grow/cycle/passes.rs`: the sheds after `walk`, and the ended apexes.
  - `grow/parallel.rs`: the counts in `absorb`.
- `grow/cycle/paused.rs` gains `phase3_year`, `phase3(parts)`, `phase3_live_tips` and `phase3_chain`.
- `grow/cycle.rs` gains the `mod` lines and `phase3_count`.

The scratch probe, `$S/probe`, is the scoreboard probe copied and pointed at census-p5's crates, with target `$S/probe-target`. It gains `census.rs`, a year-by-year census, and `a5.rs`, the settled comparison and chain.

The scripts, all heavy work through `telperion-heavy`, one job at a time:

```
bash $S/build.sh <variant>              # telperion-heavy 8 --threads 16 -- cargo build --profile ci --manifest-path $S/probe/Cargo.toml --target-dir $S/probe-target
bash $S/batch-a.sh                      # base censuses: 120/1001, 120/1012, 200/1007, 300/1001
bash $S/batch-b.sh                      # per variant B1 B2 B3: swap values, build, restore; censuses 200/1001, 120/1007, 120/1012; trees.sh; then base trees (+300)
bash $S/batch-c.sh                      # base2: A5 tip comparison at 300, 200, 120
bash $S/batch-d.sh                      # base3: A5 chains at 300 and 250
bash $S/batch-e.sh                      # base drawn trees at 220..290 (no stills)
# census.sh: telperion-heavy 16 --threads 16 -- python3 $S/peak.py $S/bin/probe-<v> census oregon-white-oak <age> <seed> <from> <a2> <a3> <a4> <yearly> <settle>
# trees.sh:  telperion-heavy 16 [--gpu 2] --threads 16 -- python3 $S/peak.py $S/bin/probe-<v> tree oregon-white-oak <age> <seed> <prefix> whole,bare|none
python3 -I $S/tables.py {a1|sheds[:kinds]|ended|lines|a2|a3|a4|a4top:<age>|years:<from>:<step>} $S/logs/census-*.log
python3 -I $S/cmp.py; python3 -I $S/life.py; python3 -I $S/chain.py $S/logs/census-base3-300-1001.log
python3 -I $S/sheet.py sheets B1 B2 B3; python3 -I $S/sheet.py measures base B1 B2 B3
```

The logs are in `$S/logs/census-<run>-<age>-<seed>.log`, and the drawn trees' logs in `$S/stills/<run>/oak-<age>-<seed>.log`.

Cost:

- The census to 300 on seed 1001 took 59 s and 1.4 GiB, and to 200 on 1007 216 s and 2.2 GiB.
- B censuses took 77 to 194 s and 2.0 to 3.7 GiB.
- Drawn trees took 38 to 318 s and up to 7.9 GiB; many waited on the queue beside a running gate.
- No job was killed.
