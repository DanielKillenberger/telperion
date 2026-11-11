# fn-346: wood that stays (G) and sleeping buds by light (B) on SR

Drafted by the worker on 11 Nov 2026 at `a1e5bde22` plus the host's R7 answers, for the host. This is `R7-ANSWERS.md`, "The two rules" and "The run". The probe is `probes/stays.patch`, and it was reverted; `git status --short crates` is empty. Nothing here proposes a design.

Marks: **measured** (printed by a run), **read** (from the code), **eye** (read off a still).

## Headline

- **G holds the wood, and the trunk no longer shrinks (measured).** On seed 1001 the trunk at 1.3 m stands 138 cm at 200 and 137 cm at 300 under SRG, against SR's 82 and 36. On 1007 it stands 205 cm at both ages, against SR's 89 and 51.
- **G does not stand the tree up (measured, eye).** SRG's settled heights equal SR's within 0.4 m at every age drawn (22.4 m at 200 and 7.9 m at 300 on 1001; 22.8 and 15.3 m on 1007). At 300 on 1001 the crown still arcs over to the ground from the top of the stem.
- **G makes a new fault (eye).** The held old wood meets the thinner wood that was laid later in hard steps. The trunk becomes a stack of cylinders with flat shoulders, a column with a wide flared foot at 300 on both seeds (worst on SRG at 300 on 1001, a 137 cm stub topped by a thin stem). The same collar shows at the foot of the beech at 40 and 80.
- **B with G (SRBG) clothes the crown more and holds more (measured).**
  - **Steady state:** living tips hold at 142,000 to 155,000 from 219 on seed 1001, against SR's 103,000 to 117,000, with leaves at 445,000 to 471,000 (rough layout). Seed 1007 holds 174,000 to 181,000 from 259 on.
  - **Settled leaf at 200:** 177,266 on 1001 and 241,977 on 1007, against SR's 97,454 and 156,803 and today's 270,545 and 619,213.
- **The young oak does not move from SR's (measured, eye).**
  - **SRG at 40, 80 and 120:** SR's tree.
  - **SRBG:** +4% to +14% leaf over SR's and +1 to +24 cm of trunk.
- **G's cost is the yearly settle it needs (measured).** One thread, oak seed 1001, SRBG against today's: 3.1 s against 1.3 s at 40, 132 s against 35 s at 120, 531 s against 146 s at 300. Peak memory is 4.0 GiB against 2.7 at 300.
- **The other presets (eye):**
  - **Palm, spruce, ordinary and birch:** the same.
  - **Beech:** the same tree, but with a step ring at the foot of the trunk under SRG and SRBG.

## The radii before building (measured, from the existing logs)

The logs hold no trunk radius at its base or at 5 m: the stills print the trunk's diameter at 1.3 m, and the shape runs print each stem limb's base radius. Both are given here at the ages the logs hold, not every 20 years from 100. A 20-year series would need new runs; none was made before building.

**Fork limbs' base radius, cm, settled tree** (limbs born in 13 and 14; `raw/oldtree/shape-*`). Today's runs printed no shape.

| variant | seed | 120 | 160 | 200 | 240 | 280 | 300 |
|---|---|---|---|---|---|---|---|
| Q | 1001 | 20.4, 34.4, 42.9 | 20.4, 29.9, 46.7 | 20.6, 30.4, 43.1 | 22.0, 27.6, 29.1 | 9.5, 19.6, 23.1 | |
| Q | 1007 | 12.9 to 63.7 (5 limbs) | 14.4 to 54.3 | 15.5 to 49.0 | refused | refused | |
| QR | 1001 | | 21.6, 32.1, 53.3 | 21.1, 32.7, 53.0 | 20.3, 35.7, 54.8 | | 19.5, 38.8, 47.0 |
| SR | 1001 | | 20.5, 34.8, 37.3 | 14.7, 22.2, 34.1 | 1.1, 13.1, 25.3 | | 5.0, 15.6 |
| SR | 1007 | | 21.4 to 38.5 (5 limbs) | 7.2 to 36.9 | 0.3 to 38.1 | | 0.0, 10.4 |

**Trunk diameter at 1.3 m, cm, drawn trees:**

| variant | seed | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| today | 1001 | 55.7 | 110.2 | 142.6 | 128.8 | 120.4 |
| today | 1007 | 74.9 | | 149.9 | 225.8 | 106.1 |
| Q | 1001 | 55.7 | 109.1 | 131.0 | 128.2 | 64.4 |
| QR | 1001 | 55.7 | 110.5 | 139.4 | 152.8 | 141.0 |
| SR | 1001 | 55.8 | 115.3 | 137.2 | 81.5 | 36.4 |
| SR | 1007 | | | 188.9 | 88.8 | 50.7 |

**What the tables show.** Today's trunk shrinks too: 142.6 cm at 120, 120.4 at 300 on 1001, and 225.8 at 200, 106.1 at 300 on 1007. SR's shrinks most: 137.2 to 36.4 cm on 1001.

## How the girth is computed, and how G was built (read, then measured)

**How the girth is computed today (read).**
- **When.** The girth is computed once, at the settle (`grow/settle.rs` `settle_lay` → `girth::thicken`), from the tree as it stands after the settle's mark, assign, shed and scale. Nothing during growth computes or keeps a radius.
- **What it reads.** Each phytomer's radius is the pipe model's: the sections of everything it bears, each child's base radius raised to the bearer's exponent, plus its own pipe. Its own pipe is `pipe × scale × ripened × leaf factor`, weighed against its established radius by `secondary`. A shed branch keeps `retained` of its pipe (0 for the oak). A retired subtree stands for itself by a frozen radius (`retire.rs`).
- **Who reads the radius after.** Sag (`sag.rs`, `geometry/sagged.rs`, through `geometry::place` with levers) reads it, and so does the drawn surface.

**A running largest radius therefore needs a girth every year (read).** In this code that means a settle every year. `Paused::settle` already settles a copy of the grower at the age grown (`settled_copy`). The probe runs that settle unlaid each year and keeps each phytomer's largest radius.

**How G was built (probe code, letter `G`).**
- **The yearly settle.** In `Paused::cycles`, after each cycle, the probe settles a copy of the grower at that age with `lay` false: mark, assign, shed, scale and thicken, no place and no sag. It then keeps every settled phytomer's radius in a map from its axis's lineage to one f64 per phytomer (`probe_ablate::hold_radii`). Axes no longer settled are let go from the map.
- **The raise.** In `settle_lay`, right after `thicken`, every phytomer's radius is raised to the largest kept for it (`probe_ablate::raise_radii`). The yearly settle passes through it too, so the kept radius is the largest so far. The final settle passes through it, and so its place and sag read the raised radii.
- **What it does not raise.** A relay's join (`Join` below and above, computed inside `thicken`) is not raised.
- **Rule B as built in `renew.patch`,** with the cost at the bud's place now the square charge (path / L_max)² under letter `U` (`grow/wake.rs`).
- **Variants.** SRG is letters `PRMUG`; SRBG is `PRMUSG`; both at `L_max` 34.8 m undoubled.

**What G costs (measured).**
- **Memory:** one f64 per held phytomer, plus a copy of the grower's axes and draws each year for the settle.
- **Time:** the yearly settles took 208 s of a 335 s SRG grow to 300 on seed 1001 at four threads (SR: 114 s), and 349 s of 557 s on 1007. At one thread they make the grow 2.4 to 3.6 times today's (cost table below).

**G does not change growth (measured).** Growth reads no radius, so SRG's census is SR's line for line (`YEAR` and `BAL` lines identical on both seeds).

## 1. The census to 300 (measured)

**SRG equals SR** (the tables in SQUARE-RESULT.md). Its rough layout holds at 32.8 m (1001) and 31.7 m (1007), and living tips hold at 103,000 to 117,000 and 136,000 to 139,000.

**SRBG, seed 1001** (every 20 years; height and spread are the rough layout's)

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | buds held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 7.0 × 8.1 | 315,267 | 25,934 | 4 / 23 / 878 | 3 | 4.7 | 71,798 | 6,543 | 1,136 | 4,676 | 204 |
| 79 | 17.5 | 18.6 × 15.1 | 2,045,683 | 141,904 | 6 / 68 / 4,137 | 3 | 4.7 | 400,950 | 29,032 | 15,694 | 27,867 | 956 |
| 119 | 25.9 | 24.2 × 22.6 | 3,804,109 | 227,493 | 10 / 116 / 5,845 | 5 | 4.7 | 698,033 | 39,550 | 30,086 | 43,394 | 1,009 |
| 159 | 31.7 | 28.4 × 27.9 | 3,331,215 | 185,089 | 7 / 93 / 4,475 | 5 | 4.7 | 591,831 | 31,691 | 23,695 | 35,419 | 891 |
| 199 | 32.9 | 30.2 × 27.5 | 2,663,523 | 144,386 | 12 / 84 / 3,600 | 4 | 4.7 | 464,683 | 24,174 | 17,876 | 27,466 | 738 |
| 219 | 32.9 | 31.1 × 27.5 | 2,428,229 | 142,671 | 16 / 113 / 3,948 | 3 | 4.7 | 444,757 | 19,482 | 14,420 | 28,263 | 559 |
| 239 | 32.9 | 31.4 × 27.5 | 2,442,962 | 147,240 | 18 / 140 / 4,253 | 3 | 4.7 | 451,788 | 14,620 | 11,002 | 30,928 | 416 |
| 259 | 32.9 | 31.2 × 26.5 | 2,481,344 | 151,615 | 23 / 155 / 4,368 | 3 | 4.7 | 463,497 | 11,427 | 8,805 | 32,073 | 387 |
| 279 | 32.9 | 34.0 × 25.6 | 2,510,796 | 155,443 | 24 / 178 / 4,673 | 3 | 4.7 | 471,400 | 8,754 | 6,883 | 34,070 | 385 |
| 299 | 32.9 | 25.7 × 21.8 | 2,484,017 | 154,734 | 32 / 209 / 4,825 | 3 | 4.7 | 467,891 | 6,082 | 4,902 | 12,844 | 353 |

The longest path holds at 33 to 39 m from 139 (`REACH` lines).

**SRBG, seed 1007**

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | buds held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.1 | 7.7 × 7.4 | 454,595 | 41,941 | 11 / 54 / 1,753 | 8 | 4.5 | 104,220 | 9,165 | 2,050 | 8,627 | 322 |
| 79 | 18.4 | 24.6 × 25.9 | 3,529,140 | 254,662 | 15 / 165 / 8,521 | 8 | 4.5 | 670,566 | 42,686 | 27,072 | 57,911 | 1,253 |
| 119 | 25.8 | 32.3 × 31.8 | 5,905,916 | 367,158 | 24 / 244 / 10,906 | 10 | 4.5 | 1,054,407 | 49,855 | 40,073 | 79,452 | 1,807 |
| 159 | 31.9 | 34.9 × 38.1 | 5,469,366 | 308,448 | 11 / 144 / 8,301 | 9 | 4.5 | 917,340 | 44,129 | 33,057 | 65,022 | 2,112 |
| 199 | 30.4 | 33.9 × 34.3 | 4,494,894 | 248,526 | 9 / 125 / 6,603 | 8 | 4.5 | 740,742 | 28,860 | 19,630 | 50,627 | 1,846 |
| 239 | 31.7 | 31.6 × 34.0 | 3,559,121 | 190,232 | 7 / 85 / 5,141 | 7 | 4.5 | 558,339 | 19,046 | 12,073 | 39,018 | 1,420 |
| 259 | 31.7 | 31.3 × 34.4 | 3,206,516 | 176,536 | 4 / 75 / 4,763 | 7 | 4.5 | 518,721 | 15,672 | 9,695 | 36,122 | 1,064 |
| 279 | 31.7 | 26.2 × 34.5 | 3,122,521 | 174,450 | 4 / 74 / 4,516 | 5 | 4.5 | 528,043 | 15,955 | 10,166 | 34,068 | 1,000 |
| 299 | 31.7 | 23.1 × 34.5 | 3,230,063 | 181,369 | 5 / 73 / 4,553 | 5 | 4.5 | 564,104 | 17,682 | 12,282 | 14,229 | 932 |

**The settled tree beside the rough layout** (the drawn height against the census height; a laid settle each year was not cheap).

| tree | rough height m (census) | settled SR | settled SRG | settled SRBG |
|---|---|---|---|---|
| 120, seed 1001 | 25.7 (SR), 25.9 (SRBG) | 24.7 | 24.5 | 24.9 |
| 200, seed 1001 | 32.8, 32.9 | 22.6 | 22.4 | 23.2 |
| 300, seed 1001 | 30.2, 32.9 | 8.3 | 7.9 | 18.3 |
| 120, seed 1007 | 25.6, 25.8 | 24.9 | 24.8 | 25.1 |
| 200, seed 1007 | 31.7, 30.4 | 23.0 | 22.8 | 23.2 |
| 300, seed 1007 | 31.3, 31.7 | 15.3 | 15.3 | 14.1 |

## 2 and 3. Measures (measured)

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB |
|---|---|---|---|---|---|---|---|---|---|
| oak-40-1001 | A | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,279 | 325,454 | 0.7 |
|  | SR | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,532 | 329,158 | 0.7 |
|  | SRG | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,465 | 329,158 | 0.8 |
|  | SRBG | 10.6 | 7.7 × 8.0 | 57 | 4.7 | 3 | 49,380 | 341,526 | 0.8 |
| oak-80-1001 | A | 17.5 | 17.9 × 14.4 | 110 | 4.7 | 3 | 209,845 | 1,593,559 | 2.0 |
|  | SR | 17.5 | 19.1 × 14.9 | 115 | 4.7 | 3 | 227,393 | 1,856,352 | 2.3 |
|  | SRG | 17.5 | 19.2 × 14.9 | 115 | 4.7 | 3 | 227,157 | 1,856,352 | 2.8 |
|  | SRBG | 17.6 | 19.6 × 15.3 | 124 | 4.7 | 3 | 250,054 | 2,035,357 | 2.9 |
| oak-120-1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 2,370,405 | 3.1 |
|  | SR | 24.7 | 27.4 × 25.9 | 137 | 4.7 | 5 | 363,719 | 3,159,003 | 3.8 |
|  | SRG | 24.5 | 27.7 × 26.1 | 139 | 4.7 | 5 | 364,078 | 3,159,003 | 4.6 |
|  | SRBG | 24.9 | 27.2 × 25.3 | 149 | 4.7 | 5 | 413,716 | 3,573,380 | 5.1 |
| oak-200-1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2,108,291 | 2.8 |
|  | SR | 22.6 | 29.1 × 37.2 | 82 | 4.7 | 3 | 97,454 | 1,541,098 | 2.3 |
|  | SRG | 22.4 | 33.7 × 32.5 | 138 | 4.7 | 3 | 108,448 | 1,541,098 | 3.7 |
|  | SRBG | 23.2 | 30.5 × 42.3 | 148 | 4.7 | 4 | 177,266 | 2,192,188 | 4.0 |
| oak-300-1001 | A | 40.2 | 26.0 × 37.2 | 120 | 25.5 | 1 | 233,403 | 1,537,433 | 2.9 |
|  | SR | 8.3 | 13.7 × 20.2 | 36 | 4.7 | 2 | 66,310 | 1,655,424 | 2.4 |
|  | SRG | 7.9 | 14.1 × 20.0 | 137 | 4.7 | 2 | 67,014 | 1,655,424 | 3.6 |
|  | SRBG | 18.3 | 16.9 × 12.7 | 148 | 4.7 | 3 | 7,509 | 2,306,836 | 4.0 |
| oak-120-1007 | A | 25.5 | 28.0 × 20.5 | 150 | 5.6 | 5 | 386,363 | 2,879,040 | 4.1 |
|  | SR | 24.9 | 35.4 × 35.2 | 189 | 4.5 | 10 | 599,507 | 4,881,176 | 5.5 |
|  | SRG | 24.8 | 35.8 × 35.7 | 204 | 4.5 | 10 | 603,325 | 4,881,176 | 6.6 |
|  | SRBG | 25.1 | 34.7 × 35.4 | 213 | 4.5 | 10 | 680,179 | 5,561,522 | 7.4 |
| oak-200-1007 | A | 37.1 | 29.7 × 34.0 | 226 | 14.4 | 2 | 619,213 | 4,121,440 | 5.0 |
|  | SR | 23.0 | 29.2 × 24.9 | 89 | 4.5 | 8 | 156,803 | 2,823,600 | 3.4 |
|  | SRG | 22.8 | 30.2 × 27.5 | 205 | 4.5 | 8 | 172,574 | 2,823,600 | 5.5 |
|  | SRBG | 23.2 | 40.5 × 29.9 | 214 | 4.5 | 9 | 241,977 | 4,355,513 | 6.7 |
| oak-300-1007 | A | 31.9 | 34.9 × 24.0 | 106 | none | 0 | 94,815 | 681,125 | 4.0 |
|  | SR | 15.3 | 16.2 × 13.6 | 51 | 4.9 | 4 | 102,800 | 2,290,382 | 3.5 |
|  | SRG | 15.3 | 16.3 × 13.7 | 205 | 4.9 | 4 | 107,417 | 2,290,382 | 5.7 |
|  | SRBG | 14.1 | 14.4 × 11.8 | 213 | 4.5 | 5 | 130,511 | 2,984,933 | 6.7 |
| beech-40-1001 | A | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,359 | 0.4 |
|  | SRG | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,622 | 83,365 | 0.4 |
|  | SRBG | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 17,428 | 86,014 | 0.4 |
| beech-80-1001 | A | 17.4 | 19.6 × 20.2 | 130 | n/a | n/a | 463,750 | 1,887,629 | 2.6 |
|  | SRG | 17.4 | 19.6 × 20.2 | 132 | n/a | n/a | 480,555 | 2,055,505 | 3.2 |
|  | SRBG | 17.4 | 19.6 × 20.2 | 136 | n/a | n/a | 510,242 | 2,196,334 | 3.4 |
| birch-40-1001 | A | 16.7 | 7.7 × 8.5 | 30 | n/a | n/a | 139,929 | 374,880 | 0.7 |
|  | SRG | 16.7 | 8.0 × 8.4 | 30 | n/a | n/a | 139,471 | 378,165 | 0.8 |
|  | SRBG | 16.8 | 7.9 × 8.5 | 31 | n/a | n/a | 150,666 | 412,191 | 0.9 |
| spruce-40-1001 | A | 12.4 | 7.9 × 7.3 | 34 | n/a | n/a | 4,185,635 | 1,137,653 | 2.0 |
|  | SRG | 12.4 | 7.8 × 7.3 | 35 | n/a | n/a | 4,400,182 | 1,211,003 | 2.4 |
|  | SRBG | 12.4 | 7.7 × 7.2 | 37 | n/a | n/a | 4,790,404 | 1,288,524 | 2.5 |
| palm-40-1001 | A | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 |
|  | SRG | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 |
|  | SRBG | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 |
| ordinary-40-1001 | A | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 |
|  | SRG | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 8,157 | 92,345 | 0.4 |
|  | SRBG | 13.3 | 9.1 × 8.4 | 23 | n/a | n/a | 9,426 | 104,604 | 0.4 |

**SRBG at 300 on 1001: a measure to check.** It holds 7,509 current-year leaves in the drawn tree, where its census holds about 468,000 leaves in the rough layout at 299. The difference is measured and not explained; the still shows a nearly bare crown.

## 4. Each fork limb's shape under SRG (measured)

Fork limbs born in 13 and 14, settled (`raw/oldtree/shape-SRG-*`), as base radius cm / length m / tip − base m / lowest of the subtree m.

| seed | 160 | 200 | 240 | 300 |
|---|---|---|---|---|
| 1001 | 20.5 / 13.1 / −5.0 / 0.0; 37.3 / 15.7 / +1.8 / 4.75; 34.8 / 31.6 / +14.8 / 4.75 | 22.2 / 13.1 / −5.0 / 0.0; 34.1 / 15.7 / +2.5 / 3.31; 14.7 / 5.5 / −1.0 / 0.01 | 25.3 / 13.0 / −5.0 / 0.0; 13.1 / 11.1 / −4.6 / 0.0; 10.3 / 0.4 / +0.4 / 4.73 | 7.8 / 2.2 / +1.6 / 5.06; 15.6 / 11.1 / −3.9 / 0.0 |
| 1007 | 31.8 to 38.5; tips −4.4 to +8.2; lowest 0.0 to 4.9 | 11.8 to 36.9; tips −1.5 to +11.4; lowest 0.02 to 4.8 | 13.1 to 38.1; tips −2.6 to +1.1; lowest 2.3 to 4.6 | 5.0 to 21.8; tips −0.5 to 0.0; lowest 1.4 to 4.9 |

**What the shapes show** (measured).
- **G raises a limb only where its radius was larger in an earlier year on the same axis.** The stub on 1001 at 240 goes from SR's 1.1 to 10.3 cm.
- **The other fork limbs read as SR's** (25.3, 13.1 at 240; 15.6 at 300). The parts of SR's that were thicker earlier were shed whole and are gone, so no radius was kept for them.
- **The limb born in 14 on 1001 still lies on the ground** (tip 5 m below its base, lowest point 0.0 m) at 160 to 240, as under SR.

## 5. Cost, as a pair in one session (measured)

**How it was measured.** One binary, the `stays.patch` build. Today's tree ran with no letters, and SRBG with `PRMUSG`. Each run was pinned to one core and alternated age by age, with no other heavy job running.

| oak, seed 1001 | today s | SRBG s | SRBG / today | today peak GiB | SRBG peak GiB |
|---|---|---|---|---|---|
| 40 | 1.3 | 3.1 | 2.4 | 0.35 | 0.47 |
| 80 | 11.3 | 36.8 | 3.3 | 1.07 | 2.17 |
| 120 | 34.5 | 132.4 | 3.8 | 1.80 | 3.79 |
| 200 | 75.9 | 329.4 | 4.3 | 2.14 | 3.90 |
| 300 | 146.0 | 530.7 | 3.6 | 2.72 | 3.99 |

**Where the cost comes from (measured).** Most of it is G's yearly settle: at four threads, the yearly settles took 208 s and 263 s of SRG's and SRBG's grows to 300 on 1001. SR's cost alone is in SQUARE-RESULT.md (131 s at 300).

## 6. The read per tree and variant (eye, with the measures)

**Oak 40, seed 1001: the same as SR's and today's, under SRG and SRBG.** SRBG has 4% more leaves.

**Oak 80, seed 1001.**
- **SRG:** SR's tree.
- **SRBG:** the same, 10% more leaf than SR and a 124 cm trunk against 115.

**Oak 120, seeds 1001 and 1007: better than today's, as SR.**
- **SRG:** SR's.
- **SRBG:** fuller (+14% leaf on 1001, +13% on 1007). The trunk is a little heavier (149 and 213 cm against SR's 137 and 189).

**Oak 200, seed 1001: worse than today's, as SR.**
- **SRG:** SR's straggly crown, with limbs to the ground on one side. The trunk is now a thick stub with a hard shoulder where it meets the thinner stem above.
- **SRBG:** better clothed than SR (177,266 leaves against 97,454) and wider (30 × 42 m). It is still thin, with crooked limbs, and lower limbs reach the ground on the left.

**Oak 200, seed 1007: about as SR (not worse than today's, not clearly better).**
- **What it is:** an upright 23 m tree, with the crown a little fuller under SRBG.
- **What is wrong:** under G the trunk is a swollen column (205 to 214 cm) carrying a thin crown, and the base reads too heavy for the crown above it.

**Oak 300, seed 1001: worse than today's.**
- **SRG:** SR's crown arced to the ground, now on a 137 cm cylinder stub with two stepped shoulders and a flared foot, from whose top the thin stem bends over.
- **SRBG:** an 18 m stem rises from the same stepped stub, crooked and nearly bare (7,509 leaves). A few short limbs lie around its foot.
- **What is wrong:** the stepped trunk is new; the arc and the bare crown are SR's.

**Oak 300, seed 1007: worse than SR's.**
- **SR:** a small upright old tree on a straight trunk.
- **SRG and SRBG:** the same crown on a 205 to 213 cm smooth column with a wide flared disc at its foot and a hard step at about 4 m, where the crown's wood starts. The column reads as a stump with a tree on it.

**Other presets:**
- **Palm, spruce and ordinary:** the same as today's.
- **Birch:** the same (8.0 × 8.4 m under SRG).
- **Beech at 40 and 80:** the same tree, with a visible step ring at the foot of the trunk under SRG and SRBG. A thin collar sits below where the trunk's held radius meets its present one.

**The points the host named.**
- **Do the trunk and limbs stand at 200 and 300?**
  - **The trunk stands** under G at every age.
  - **The limbs:** the stem above about 8 m still arcs to the ground at 300 on 1001, under SRG as under SR. On 1007 the crown stands, as under SR.
- **Is the crown clothed through its depth?** At 120, yes. At 200 and 300 it is thin, least so under SRBG at 200.
- **A thicket on the bole or under the limbs?** None seen. SRBG's extra leaf sits in the crown.
- **Does the trunk or a limb base swell out of scale?** Yes, under G.
  - **The trunk at 300:** 137 cm on 1001 against SR's 36 and today's 120, and 205 to 213 cm on 1007 against SR's 51 and today's 106. It carries a crown a third of today's or less.
  - **The joins:** stepped shoulders where held wood meets later wood.
- **Has the oak at 40, 80 and 120 moved from SR's?** No under SRG. Under SRBG, +4 to +14% leaf.

## Open for the host (no rule proposed)

- **Holding the radius does not hold the tree up.** The settled height is SR's within 0.4 m. What lies down is the wood laid later on the stem line and the limbs, which never had a larger radius.
- **G steps the girth.** A held part keeps its radius while the part above or beside it carries the present pipes. The join is a hard step, and the relay joins (`Join`) were not raised in this probe.
- **G's cost.** A girth every year needs a settle every year: 2.4 to 4.3 times today's one-thread time on the oak, and up to 4.0 GiB at 300 against 2.7.
- **SRBG at 300 on 1001** draws 7,509 leaves against a census of about 468,000. That is not explained by this pass.

## Friction

One entry in FRICTION.md, written now: the hook refused a `diff <(…) > /dev/null` redirect, which cost one tool call.
