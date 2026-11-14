# fn-211 task 2: every serial growth pass on every core, exactly (worker, 2026-10-06)

A dispatched worker wrote this. Every lever leaves every tree identical to the bit; what could not be made faster exactly is listed in section 4 with the reason. Questions for the host are in section 5.

## 1. What is built

Each lever is its own commit on `fn-211-tree-space-f2b-growth-years-cheap`, from `899c7186`.

| Commit | What |
|---|---|
| `239612a2` | **Per-pass reports.** `Stage::Pass(Pass)` names 26 passes (growth, rough layout, full lay, light, settle, final lay), each reported as it ends, before its stage. `measures passes` sums the time between reports (`gaps`: time before a stage's own report; `tail`: after the last report until the tree returns). Test `every_pass_is_reported`. The after table below is taken with it. |
| `fb5974ea` | **Light.** Each sky direction swept on its own core, summed per node in the directions' order; the buds read the field on every core. `cores::map` and `cores::fill` hand out pieces in order. |
| `f3375582` | **Shedding's fades in one backward pass.** The climb from every axis to the root (n x depth) becomes one pass: the carry from a child to its parent is monotone (weights are never negative), so the most carried up equals the carry of the most, to the bit. The units' prefix sums sit in one array. |
| `3c1c2ce4` | **Presences assigned on every core**; each vigour then reads its parent's end presence. |
| `ec0986bd` | The settle and final lay move to `grow/settle.rs`: `grow.rs` was over 400 lines. |
| `eee788d7` | **`upward::Tree`**: a pass that adds each child's share into its parent, last child first, runs a depth at a time, deepest first. Each parent takes its children's shares itself in that order, so every sum is the same. Used here for the tree-as-grown radii (`attachments`); the leaf term's pipes are taken on every core and summed in the tree's order. |
| `f271387d` | **Girth** (`thicken`) on `upward::Tree`: kept pipes first, then children, as before. |
| `55ff41de` | **Sag's levers** (`sag::gather`) on `upward::Tree`. Bounded chains pass one core. |
| `6944917c` | **The lay's framing on every core.** Each depth's starts are taken in parallel and kept in their pieces, so no start or walk is moved on one thread; the error is still the first axis in index order. |
| `63ce2c59` | **The rough layout on every core** (`grow/draw.rs`). The axes that grew are laid at once, from their own walks; new axes a generation at a time. Leaves are gathered afterwards in the serial order, and the error is the one serial laying meets first. |
| `91c7896a`, `90ad91d9` | **Balance and allotment read the tree packed.** Each axis's link and PA are gathered once a cycle on every core (pieces of 32k), and the sums run over the packed rows in the same order. |
| `305dca0d` | **The full lay's copy, write-back and leaf gathering on every core.** |
| `53445e1d` | **Scale on every core.** Base scales are taken in turn from packed rows; the phytomers are scaled, relay nodes found and sizes applied on every core. |
| `6b34fd6d` | The light's unit tests call the sweep with its cores (four). They did not compile from `fb5974ea` to here; see section 6. |

## 2. The hash proof

`measures hash 80 1,7 all both` (seeds 1 and 7, neutral and lit, four species). The table is taken at the base `899c7186`. The beech and oak hashes differ from fn-210's table because fn-206 and fn-210 landed after that table was made.

| Tree | Neutral | Lit |
|---|---|---|
| palm 1 / 7 | `d2cd16377eece1a5` / `c3dee32807ab1c0a` | same |
| beech 1 / 7 | `6dd1df9c29b0c77c` / `52ef8f3c61e2e248` | same |
| spruce 1 / 7 | `109d37dbc251a86d` / `0ffab05502299068` | same |
| oak 1 / 7 | `1f70e9375d8d9118` / `f0f7a8bd7069755b` | `05b689dbc94763fe` / `3fc3866e318b782f` |

- **All cores:** identical for every lever's tree, built and hashed before its commit (16 checks; the last at `53445e1d`).
- **One core** (`taskset -c 0`, so `cores::available()` is 1): identical at `53445e1d`.
- **Kept tests:** `many_cores_grow_the_tree_one_does` (one core against eight, in shares of seven) now drives every lever, because each reads the same `threads`. The light's unit tests sweep on four cores.

## 3. Timings

**Setup.**

- Release build in this worktree's own `target/`. Ryzen 9 5950X, 32 threads. "1" is `taskset -c 0`.
- The trees are the ones PROFILE.md used: the oak lit (extinction 0.5, sky 0.5); the beech, spruce and palm neutral.
- Each configuration runs seeds 1, 1, 7 and 7 in one process. The second build of each seed is kept.
- Before is `239612a2` (pass reports only), after is `53445e1d`.
- The run waits until the 1-minute load is below 4 before each configuration (`passes-run.sh`); tables come from `passes-tables.py`.
- Another worker (fn-209.1) shared the desk. Each cell's load is the one its process read at the start.
- Two after-configurations started under 4 but read 8.8 (oak 80 s7) and 6.3 (spruce 40 s7). Both were run again (`raw/task2-rerun.txt`) and came within 1%: oak 80 s7 at 7,381 ms against 7,300; spruce 40 s7 at 561 against 559.
- Raw lines are in `raw/task2-before.txt` and `raw/task2-after.txt` (gitignored).

**Totals (ms):**

| Tree | Cores | Load before / after | Total before | Total after | Change |
|---|--:|--:|--:|--:|--:|
| palm 40 s1 | all | 3.77 / 3.83 | 0.25 | 0.25 | +0% |
| palm 40 s1 | 1 | 3.77 / 3.83 | 0.25 | 0.26 | +4% |
| palm 40 s7 | all | 3.77 / 3.83 | 0.24 | 0.26 | +8% |
| palm 40 s7 | 1 | 3.77 / 3.83 | 0.24 | 0.25 | +4% |
| beech 40 s1 | all | 3.77 / 3.83 | 45.8 | 49.0 | +7% |
| beech 40 s1 | 1 | 3.77 / 3.83 | 48.5 | 46.9 | -3% |
| beech 40 s7 | all | 3.77 / 3.83 | 69.7 | 69.3 | -1% |
| beech 40 s7 | 1 | 3.77 / 3.83 | 89.1 | 88.9 | -0% |
| oak 40 s1 | all | 3.77 / 3.83 | 425 | 377 | -11% |
| oak 40 s1 | 1 | 4.03 / 3.92 | 471 | 466 | -1% |
| oak 40 s7 | all | 3.77 / 3.92 | 632 | 500 | -21% |
| oak 40 s7 | 1 | 4.03 / 3.92 | 815 | 769 | -6% |
| spruce 40 s1 | all | 3.50 / 3.92 | 806 | 600 | -26% |
| spruce 40 s1 | 1 | 3.46 / 3.73 | 1,407 | 1,400 | -0% |
| spruce 40 s7 | all | 3.46 / 6.33 | 779 | 559 | -28% |
| spruce 40 s7 | 1 | 3.34 / 3.59 | 1,342 | 1,331 | -1% |
| palm 80 s1 | all | 3.34 / 3.59 | 0.48 | 0.46 | -4% |
| palm 80 s1 | 1 | 3.34 / 3.59 | 0.44 | 0.46 | +5% |
| palm 80 s7 | all | 3.34 / 3.59 | 0.43 | 0.45 | +5% |
| palm 80 s7 | 1 | 3.34 / 3.59 | 0.42 | 0.43 | +2% |
| beech 80 s1 | all | 3.34 / 3.59 | 617 | 478 | -23% |
| beech 80 s1 | 1 | 3.23 / 3.47 | 978 | 942 | -4% |
| beech 80 s7 | all | 3.23 / 3.59 | 1,031 | 763 | -26% |
| beech 80 s7 | 1 | 3.13 / 3.47 | 1,649 | 1,675 | +2% |
| oak 80 s1 | all | 2.95 / 3.39 | 5,094 | 3,479 | -32% |
| oak 80 s1 | 1 | 2.30 / 3.62 | 7,047 | 6,687 | -5% |
| oak 80 s7 | all | 2.48 / 8.81 | 10,798 | 7,300 | -32% |
| oak 80 s7 | 1 | 1.84 / 3.16 | 14,523 | 14,621 | +1% |
| spruce 80 s1 | all | 4.18 / 3.16 | 2,998 | 2,154 | -28% |
| spruce 80 s1 | 1 | 3.27 / 2.90 | 6,202 | 6,308 | +2% |
| spruce 80 s7 | all | 3.92 / 3.07 | 2,899 | 2,143 | -26% |
| spruce 80 s7 | 1 | 2.92 / 2.84 | 6,082 | 6,180 | +2% |

**What changed:**

- **On all cores:**
  - The oak at 80 is 32% faster: s1 from 5.09 to 3.48 s, s7 from 10.8 to 7.3 s.
  - The spruce at 80 is 26 to 28% faster (3.0 to 2.15 s), and the beech at 80 23 to 26%.
  - At 40, the oak gains 11 to 21% and the spruce 26 to 28%. The beech at 40 (46 to 70 ms) and the palm (under 0.5 ms) do not change beyond noise.
- **On one core** the totals hold within -6% to +2% for every tree over 40 ms; the palm, under a millisecond, moves within ±5% of noise. Two exact serial gains (shedding's fades, the packed rows for allotment) pay for the by-depth passes. Those passes run 10 to 40% slower on one core than the index-order loops they replace:
  - one allocation per axis for its node sums;
  - visiting axes by depth rather than by index;
  - for `scale`, an extra pass.
  - Examples: the oak s1's sketch 1,010 to 1,210 ms, the spruce s1's thicken 280 to 339 and levers 282 to 383. See section 5.

**Per pass (ms, before / after):**

**oak 80** (ms; before / after)

| Pass | s1 all | s1 1 | s7 all | s7 1 |
|---|--:|--:|--:|--:|
| Windows | 13.9 / 9.0 | 8.6 / 9.2 | 9.1 / 8.8 | 8.5 / 8.7 |
| Balance | 352 / 307 | 355 / 376 | 736 / 677 | 775 / 822 |
| Allot | 454 / 295 | 464 / 294 | 1,032 / 726 | 1,040 / 700 |
| Homes | 47.4 / 48.3 | 46.5 / 44.0 | 103 / 105 | 101 / 97.9 |
| Units | 333 / 315 | 1,310 / 1,245 | 705 / 627 | 2,709 / 2,642 |
| Merge | 98.5 / 94.7 | 96.8 / 90.2 | 209 / 213 | 203 / 184 |
| Wake | 0.56 / 0.52 | 1.9 / 1.9 | 1.2 / 1.1 | 5.3 / 5.0 |
| Sketch | 1,033 / 643 | 1,010 / 1,210 | 2,153 / 1,304 | 2,116 / 2,629 |
| RelaySettle | 482 / 287 | 483 / 461 | 983 / 568 | 979 / 956 |
| RelayPlace | 145 / 120 | 361 / 346 | 288 / 200 | 735 / 722 |
| RelayLevers | 109 / 78.2 | 92.7 / 96.8 | 232 / 139 | 193 / 216 |
| RelaySag | 152 / 126 | 363 / 353 | 292 / 198 | 754 / 727 |
| RelayWrite | 142 / 123 | 137 / 144 | 284 / 242 | 270 / 290 |
| ShadeField | 271 / 177 | 281 / 283 | 552 / 367 | 557 / 587 |
| ShadeRead | 58.0 / 34.4 | 59.0 / 51.6 | 121 / 55.0 | 123 / 113 |
| Mark | 1.4 / 1.3 | 1.9 / 1.1 | 3.5 / 3.3 | 3.4 / 3.3 |
| Assign | 146 / 87.1 | 166 / 159 | 290 / 181 | 321 / 346 |
| Attachments | 467 / 151 | 496 / 555 | 936 / 321 | 1,007 / 1,184 |
| Shed | 220 / 164 | 245 / 157 | 495 / 380 | 526 / 363 |
| Disused | 7.4 / 7.5 | 10.6 / 19.6 | 16.6 / 16.4 | 25.0 / 37.6 |
| Scale | 23.3 / 17.9 | 28.1 / 34.1 | 56.8 / 45.3 | 62.6 / 89.7 |
| Thicken | 78.9 / 28.6 | 105 / 85.7 | 193 / 60.1 | 238 / 220 |
| Place | 77.9 / 53.3 | 213 / 194 | 188 / 112 | 496 / 492 |
| Levers | 61.3 / 40.7 | 88.2 / 68.5 | 142 / 85.4 | 199 / 177 |
| SagPlace | 73.5 / 52.5 | 242 / 196 | 188 / 111 | 509 / 497 |
| gaps | 58.1 / 36.2 | 59.2 / 33.4 | 115 / 75.1 | 120 / 68.9 |
| tail | 189 / 181 | 324 / 178 | 473 / 479 | 448 / 443 |
| total | 5,094 / 3,479 | 7,047 / 6,687 | 10,798 / 7,300 | 14,523 / 14,621 |

**spruce 80** (ms; before / after)

| Pass | s1 all | s1 1 | s7 all | s7 1 |
|---|--:|--:|--:|--:|
| Windows | 14.6 / 14.8 | 15.0 / 14.9 | 15.0 / 14.7 | 15.1 / 14.9 |
| Balance | 3.4 / 2.8 | 2.5 / 2.2 | 3.3 / 2.6 | 2.4 / 2.4 |
| Homes | 99.9 / 95.2 | 90.0 / 88.6 | 94.4 / 92.4 | 88.5 / 96.3 |
| Units | 484 / 493 | 1,664 / 1,627 | 471 / 474 | 1,631 / 1,753 |
| Merge | 149 / 126 | 135 / 142 | 131 / 131 | 111 / 105 |
| Wake | 45.0 / 47.4 | 41.4 / 39.8 | 44.1 / 45.5 | 44.6 / 44.4 |
| Mark | 9.2 / 9.4 | 8.0 / 13.7 | 9.3 / 9.2 | 8.7 / 10.6 |
| Assign | 142 / 97.9 | 144 / 186 | 138 / 97.5 | 134 / 193 |
| Shed | 211 / 142 | 235 / 154 | 209 / 141 | 202 / 168 |
| Scale | 118 / 88.5 | 120 / 168 | 118 / 87.0 | 117 / 159 |
| Thicken | 266 / 87.2 | 280 / 339 | 263 / 86.2 | 276 / 308 |
| Place | 408 / 230 | 1,258 / 1,255 | 396 / 215 | 1,252 / 1,212 |
| Levers | 330 / 198 | 282 / 383 | 279 / 203 | 315 / 292 |
| SagPlace | 419 / 230 | 1,633 / 1,608 | 425 / 232 | 1,609 / 1,549 |
| gaps | 21.0 / 17.0 | 19.0 / 24.5 | 22.8 / 16.6 | 21.8 / 23.0 |
| tail | 279 / 275 | 275 / 262 | 280 / 295 | 254 / 250 |
| total | 2,998 / 2,154 | 6,202 / 6,308 | 2,899 / 2,143 | 6,082 / 6,180 |

**beech 80** (ms; before / after)

| Pass | s1 all | s1 1 | s7 all | s7 1 |
|---|--:|--:|--:|--:|
| Windows | 7.7 / 7.8 | 7.5 / 7.5 | 7.7 / 7.5 | 7.5 / 14.1 |
| Balance | 0.53 / 0.47 | 0.34 / 0.41 | 0.82 / 0.87 | 0.82 / 0.52 |
| Homes | 17.6 / 15.3 | 12.8 / 13.5 | 27.8 / 29.1 | 21.9 / 21.8 |
| Units | 155 / 164 | 379 / 365 | 237 / 232 | 605 / 657 |
| Merge | 36.3 / 33.3 | 23.1 / 23.4 | 57.2 / 50.7 | 48.9 / 49.5 |
| Mark | 2.3 / 2.6 | 2.3 / 2.2 | 4.7 / 3.8 | 4.0 / 3.7 |
| Assign | 44.5 / 26.9 | 45.4 / 46.6 | 78.6 / 45.8 | 76.8 / 80.6 |
| Shed | 78.0 / 29.5 | 65.3 / 29.2 | 120 / 56.4 | 109 / 54.4 |
| Scale | 33.1 / 23.3 | 32.8 / 40.5 | 56.6 / 41.3 | 57.8 / 70.8 |
| Thicken | 82.4 / 35.6 | 85.5 / 86.2 | 140 / 53.4 | 146 / 150 |
| Place | 99.9 / 81.0 | 267 / 266 | 182 / 127 | 466 / 462 |
| tail | 59.4 / 58.0 | 57.3 / 61.7 | 118 / 115 | 105 / 110 |
| total | 617 / 478 | 978 / 942 | 1,031 / 763 | 1,649 / 1,675 |

**oak 40** (ms; before / after)

| Pass | s1 all | s1 1 | s7 all | s7 1 |
|---|--:|--:|--:|--:|
| Windows | 3.5 / 2.8 | 2.8 / 2.8 | 2.8 / 2.9 | 2.8 / 2.8 |
| Balance | 10.3 / 16.9 | 9.9 / 11.8 | 19.4 / 23.9 | 20.7 / 18.5 |
| Allot | 17.4 / 17.2 | 17.5 / 15.3 | 27.9 / 25.8 | 28.9 / 23.5 |
| Homes | 3.8 / 4.2 | 3.5 / 3.5 | 6.1 / 6.3 | 5.7 / 5.5 |
| Units | 70.4 / 54.9 | 94.9 / 90.7 | 78.0 / 66.3 | 164 / 152 |
| Merge | 10.7 / 10.1 | 6.3 / 5.5 | 15.7 / 14.7 | 15.2 / 10.2 |
| Sketch | 81.3 / 74.6 | 70.0 / 75.8 | 130 / 103 | 133 / 135 |
| RelaySettle | 20.6 / 22.4 | 18.4 / 18.0 | 30.9 / 26.4 | 31.5 / 28.0 |
| RelayPlace | 12.0 / 14.7 | 18.3 / 17.8 | 14.3 / 15.4 | 27.1 / 25.3 |
| RelayLevers | 4.3 / 6.8 | 3.2 / 2.8 | 5.5 / 7.7 | 5.7 / 4.9 |
| RelaySag | 12.2 / 14.0 | 18.5 / 17.8 | 15.3 / 16.0 | 27.6 / 25.7 |
| RelayWrite | 6.5 / 8.5 | 5.0 / 5.1 | 10.0 / 11.2 | 9.8 / 8.8 |
| ShadeField | 26.4 / 20.2 | 26.1 / 26.5 | 32.7 / 26.7 | 34.4 / 33.8 |
| ShadeRead | 3.7 / 3.9 | 3.7 / 4.0 | 6.4 / 5.7 | 7.3 / 6.5 |
| Assign | 13.9 / 8.0 | 12.5 / 14.1 | 19.7 / 12.4 | 20.6 / 23.4 |
| Attachments | 40.0 / 17.9 | 40.5 / 45.9 | 59.5 / 24.3 | 61.5 / 71.9 |
| Shed | 15.0 / 11.6 | 15.4 / 9.4 | 25.1 / 18.9 | 26.9 / 16.8 |
| Disused | 0.65 / 0.74 | 1.1 / 0.91 | 1.1 / 1.0 | 1.4 / 1.8 |
| Scale | 2.9 / 3.1 | 3.1 / 3.4 | 5.8 / 5.0 | 6.9 / 7.0 |
| Thicken | 11.0 / 6.7 | 12.2 / 11.8 | 19.9 / 9.1 | 22.0 / 22.1 |
| Place | 14.2 / 13.8 | 30.9 / 28.8 | 27.2 / 18.1 | 56.4 / 51.6 |
| Levers | 9.4 / 8.6 | 11.2 / 8.4 | 19.9 / 11.7 | 19.9 / 16.6 |
| SagPlace | 14.4 / 15.1 | 30.1 / 29.8 | 25.0 / 18.9 | 56.7 / 52.2 |
| gaps | 4.5 / 3.5 | 3.0 / 2.8 | 7.6 / 5.2 | 5.8 / 4.3 |
| tail | 15.4 / 17.0 | 13.2 / 12.7 | 25.3 / 23.1 | 23.0 / 20.4 |
| total | 425 / 377 | 471 / 466 | 632 / 500 | 815 / 769 |

**spruce 40** (ms; before / after)

| Pass | s1 all | s1 1 | s7 all | s7 1 |
|---|--:|--:|--:|--:|
| Windows | 6.5 / 5.8 | 5.6 / 5.8 | 6.0 / 5.8 | 5.7 / 5.8 |
| Balance | 0.65 / 0.66 | 0.47 / 0.38 | 0.61 / 0.62 | 0.65 / 0.43 |
| Homes | 19.2 / 20.9 | 16.8 / 16.6 | 18.4 / 19.2 | 16.5 / 16.3 |
| Units | 116 / 121 | 274 / 277 | 115 / 114 | 268 / 268 |
| Merge | 38.9 / 41.8 | 28.4 / 32.6 | 46.0 / 42.0 | 38.9 / 37.1 |
| Wake | 15.4 / 16.0 | 32.9 / 13.9 | 10.1 / 9.7 | 8.6 / 9.0 |
| Mark | 3.0 / 3.3 | 2.4 / 2.4 | 3.0 / 2.9 | 2.4 / 2.6 |
| Assign | 36.7 / 27.6 | 32.8 / 37.8 | 34.6 / 25.8 | 32.2 / 37.2 |
| Shed | 52.4 / 39.5 | 47.3 / 36.7 | 50.5 / 36.5 | 46.1 / 34.2 |
| Scale | 31.2 / 23.7 | 31.1 / 37.8 | 31.3 / 22.3 | 28.7 / 36.9 |
| Thicken | 70.9 / 27.0 | 69.2 / 82.9 | 68.5 / 26.6 | 65.0 / 80.6 |
| Place | 118 / 67.1 | 325 / 313 | 118 / 59.0 | 315 / 300 |
| Levers | 91.2 / 59.9 | 78.2 / 82.2 | 78.8 / 49.3 | 62.7 / 76.2 |
| SagPlace | 120 / 66.9 | 387 / 387 | 115 / 66.3 | 382 / 356 |
| gaps | 7.3 / 4.7 | 7.4 / 5.4 | 7.9 / 4.6 | 4.9 / 5.4 |
| tail | 79.1 / 74.0 | 68.0 / 68.6 | 75.6 / 74.2 | 65.4 / 64.8 |
| total | 806 / 600 | 1,407 / 1,400 | 779 / 559 | 1,342 / 1,331 |

**beech 40** (ms; before / after)

| Pass | s1 all | s1 1 | s7 all | s7 1 |
|---|--:|--:|--:|--:|
| Windows | 1.8 / 1.9 | 1.7 / 1.7 | 1.8 / 2.2 | 1.7 / 1.7 |
| Homes | 0.84 / 1.0 | 0.75 / 0.76 | 1.4 / 1.5 | 1.3 / 1.3 |
| Units | 17.2 / 18.2 | 18.5 / 18.3 | 22.2 / 24.9 | 32.1 / 33.5 |
| Merge | 2.4 / 2.2 | 0.99 / 0.80 | 4.4 / 3.7 | 3.9 / 3.5 |
| Assign | 2.2 / 1.5 | 1.7 / 1.7 | 4.4 / 2.2 | 4.2 / 4.3 |
| Shed | 3.2 / 1.9 | 2.5 / 1.3 | 5.4 / 3.0 | 4.5 / 2.7 |
| Scale | 1.0 / 1.9 | 0.91 / 0.97 | 2.4 / 2.8 | 2.3 / 2.7 |
| Thicken | 3.8 / 4.5 | 3.8 / 3.9 | 6.6 / 6.2 | 6.7 / 7.1 |
| Place | 9.6 / 11.5 | 15.9 / 15.8 | 14.2 / 15.5 | 27.4 / 27.0 |
| tail | 3.7 / 4.2 | 1.6 / 1.8 | 6.6 / 7.0 | 4.6 / 5.0 |
| total | 45.8 / 49.0 | 48.5 / 46.9 | 69.7 / 69.3 | 89.1 / 88.9 |


## 4. What stayed serial, and why

Each line names the pass, what remains on one thread, and why.

**Balance and allotment** (oak 80 s1, all cores: 307 and 295 ms):

- **What remains serial:** the sums over the tree, the forward split and `vigours`' sort of the living kids.
- **Why:** they run in index order over packed rows, a few nanoseconds an axis. Late in growth a year costs about 6 ms to gather the rows on every core, 7 ms for the sums with their allocations, and 3 ms forward.
  - Running them by depth would need the depth table rebuilt each year, which costs about what the sums do.
- **What would help:** keeping the rows across years, which is layer 2.

**Homes, merge and wake** (oak 95 to 210 ms; spruce 90 to 150):

- **Why:** the order of the merged shares is the tree's indices, so merging copies serially. Wake grows through one share.

**Shed** (oak 164; spruce 142):

- **Why:** the drain keeps index order. About 95 ms of the oak's is freeing the shed axes. This was measured by leaking them in a probe that was not committed.

**`tail`** (oak 181 to 479; spruce 275 to 295):

- **What it is:** freeing the grower's per-axis state when `run` returns.
- **Why it stayed:** freeing it on a helper thread while the settle went on was built, then reverted. It slowed the passes it overlapped by more than it saved: the spruce lost about 300 ms, and the oak's balance, allot and sketch each lost 25 to 30% (FRICTION.md).

**The light's lattice** (ShadeField, oak 177 ms):

- **What was tried:** the bounds were taken on every core and the leaves deposited a layer at a time. That version was hash-identical.
- **Why it was dropped:** it measured no faster (176 to 181 ms over 80 years). A year's lattice takes 1 to 5 ms, and spawning the threads eats the gain (section 5).
- **What runs in parallel now:** the sweep's directions and the reads.

**The rough layout** (oak s1 643 ms, from 1,033):

- **What remains serial:** sorting the axes that grew, picking them, and gathering the leaves.
- **What was tried:** gathering the leaves in parallel pieces. It was hash-identical and measured no different (A/B over three alternating runs), so it was dropped.

**Smaller serial passes** (each under 40 ms at 80, except `attachments`' 151):

- the windows (once);
- marking the living apexes and the woken buds' `kept`;
- `standing` in the full lay;
- `disused`;
- `attachments`' base scales, in turn from the parent;
- the leaf term's sum, in the tree's order;
- the all-zero check after the final lay.

**Growth units** (`Units`) were already parallel. Their scaling (2.4 to 3.8x on 32 threads) was not changed.

**The oak's unattributed 7%** (PROFILE.md "Other", 347 ms at 80 s1) is now itemised:

| Part | Oak 80 s1 | Where it is reported |
|---|--:|---|
| Freeing the grower when `run` returns | about 180 ms | `tail` |
| Freeing each full lay's tree | about 45 ms | `RelayWrite` |
| Between passes, the final all-zero check among them | 36 to 58 ms | `gaps` |
| Rebinding and bookkeeping | the remainder | spread over the passes now reported |

## 5. For the host

1. **The cost of a spread.**
   - **What:** `cores::spread` spawns its helpers with `std::thread::scope` on every call. This costs 60 us at 4 threads and 0.48 ms at 32 (200 spreads of 64 empty pieces on this desk).
   - **Why it matters:** a growth year late in growth runs about a dozen spreads whose work is 1 to 5 ms each, so spawning eats much of what the passes gain. That is why the light's lattice and the sketch's leaf gathering did not pay.
   - **The decision:** a pool of helpers kept for the process and handed scoped work needs either a dependency (rayon) or `unsafe` lifetime erasure in `cores.rs`.
2. **One core.**
   - **What:** the by-depth passes (`attachments`, `thicken`, `levers`, the sketch's generations, `scale`) cost 10 to 40% more on one core than the index-order loops did. The totals on one core still hold within -6% to +2%, because of the two serial gains.
   - **Why it matters:** wasm has one core. If one-core speed matters there, either keep the index-order loop where `threads` is 1, which is a second path for the same pass, or reuse per-piece scratch for the per-axis sums.
   - **The decision:** which of these, if either.
3. **Freeing.**
   - **What:** a quarter second per big tree, 5 to 10% of the build, is freeing per-axis vectors: each `Draws` holds five and each `Axis` two.
   - **Why it matters:** no exact parallel lever removes it.
   - **The decision:** the layout itself.

## 6. Notes

- **Broken test build:** from `fb5974ea` to `6b34fd6d`, the crate's unit tests did not compile, because the light's tests called the sweep without its cores. The release example and the hash were unaffected. Commits in between were checked with the example build alone. This was found and fixed at `6b34fd6d`. The workspace gate runs at the end.
- **Workspace gate:** `cargo test --profile ci --workspace --no-fail-fast`, run once at `53445e1d`: 1,113 passed, 0 failed, 20 ignored.
- **Not done:** Codex review (spec R5).

# fn-211 task 3: a kept pool, scratch reuse and fewer draw vectors (worker, 2026-10-06)

A dispatched worker wrote this, on the host's decisions of 2026-10-06. Every tree is identical to the bit at every commit.

## 1. What is built

| Commit | What |
|---|---|
| `a353489f` | **A kept pool.** `cores::spread` hands its helpers to rayon's global pool (`rayon::in_place_scope`) instead of spawning scoped threads each call. The pool holds one thread a core and is the process-wide budget the old helper count kept; at most `min(threads, pieces, pool size)` threads take tickets, the caller among them. One core, or one piece, runs on the calling thread without touching the pool. |
| `e8c7996b` | **Scratch reuse.** `upward::Tree::upward` hands each piece of a depth one scratch buffer in turn; `thicken`, sag's `gather` and the tree-as-grown radii (`attachments`) reuse it for their node sums instead of allocating one per axis. |
| `f55c91b1` | **Fewer draw vectors.** `Draws` held five vectors; survival, persistence and share are one `Unit` record, size and light one pair kept only where light shades (three vectors where there were five, two where nothing shades). The full lay every ten cycles no longer clones every standing axis's draws: `assign_living` reads them by reference and is told which apexes live. |

## 2. The hash proof

`measures hash 80 1,7 all both`, against task 2's base table (`raw/task2-hash-base.txt`): identical, 16 trees, at `a353489f`, `e8c7996b` and `f55c91b1` on all cores, and at `a353489f` and `f55c91b1` on one core (`taskset -c 0`). The crate's tests (`cargo test --profile ci -p telperion-space`, 131 passed, 3 ignored) were compiled and run at each of the three commits.

## 3. Timings and memory

Tables: `TASK3-PASSES.md` (totals before task 2 against now; every pass at the end of task 2 against now, ages 40 and 80, four species, 1 and all cores). Raw lines: `raw/task3-passes-{pre,t2,l1,l3}.txt`. The desk was shared with fn-209.2; each configuration started under load 4, each cell's load is in the tables.

**All cores, end of task 2 to now:** oak 80 -14% (s1 3.76 to 3.23 s) and -9% (s7 7.76 to 7.05 s); spruce 80 -4 to -7%; beech 80 -12 to -13%; oak 40 -17 to -24%; spruce 40 -8 to -12%; beech 40 -18 to -27%. Against before task 2: oak 80 -37 to -41%, spruce 80 -31 to -37%, beech 80 -35 to -39%.

**One core, against before task 2** (the acceptance's bar): beech -2 to -16%, spruce 40 -6 to -17%, spruce 80 -3% and +1%, oak 40 -6 to -8%, oak 80 s7 -6%. Oak 80 s1 read +5% in the table (6,574 to 6,907 ms, single cells); repeated three times alternating (`taskset -c 0`, loads 4.4 to 7.2, above 4 because fn-209.2 held the desk): before 7,363 / 6,889 / 7,802 ms, now 7,621 / 6,854 / 6,632 ms, means 7,351 and 7,036. No slower.

**Peak memory** (VmHWM, one tree a process, all cores, 80 years):

| Tree | Before task 2 | End of task 2 | Now |
|---|--:|--:|--:|
| oak s1 lit | 1.89 GB | 1.93 GB | 1.85 GB |
| oak s7 lit | 3.84 GB | 3.95 GB | 3.72 GB |
| beech s1 | 559 MB | 550 MB | 537 MB |
| beech s7 | 948 MB | 926 MB | 904 MB |
| spruce s1 | 2.27 GB | 2.06 GB | 1.99 GB |
| spruce s7 | 2.25 GB | 2.05 GB | 1.97 GB |

**Per lever.** The pool: 0 to -13% on all cores at 80 (`raw/task3-passes-l1.txt`), nothing on one core, peak unchanged. Scratch: sag's levers about -8% on one core, thicken unchanged (alternating A/B on spruce 80). Draw records: oak 80 s1 full re-lays 0.86 to 0.75 s and peak -60 MB; spruce 80 `tail` 295 to 245 ms and peak -65 MB; the oak's `tail` 200 to 156 ms (s1) and 626 to 430 (s7).

## 4. Findings

- **The by-depth passes' one-core cost is their order, not their allocations.** Host decision 2 expected scratch reuse to bring them back to the index-order loops' cost. It removed the allocations, and thicken on one core did not move (spruce 80 316 to 315 ms; before task 2 it was 280). What remains is visiting axes by depth rather than in the order they sit in memory. On one core the oak's `Sketch` (about +200 ms over before task 2) and `Attachments` (about +80) still carry it; the re-lays and the freeing now pay it back.
- **The sketch's extra 200 ms on one core** is mostly the leaves gathered after laying (145 ms on oak 80 s1, probed and reverted): laying in turn gathered them as it went. Gathering them in the jobs and copying in serial order is exact, but task 2 measured that shape no faster on all cores, so it was not built.
- **Wasm.** `telperion-space` is in no shipped Wasm module (`cargo tree` for `telperion-wasm` and the renderer's wasm target lists neither it nor rayon); it is a dev-dependency of the renderer's examples. It still compiles for `wasm32-unknown-unknown`, and `telperion-wasm` builds. In a browser `available_parallelism` fails, so `threads` is 1 and every spread runs on the calling thread without touching rayon; were the pool reached, rayon-core 1.13 falls back to the current thread where threads are unsupported.

## 5. For the host

1. **A cross-axis arena for the draws and per-axis vectors.** Decision 3 named flat arrays (struct of arrays or one arena). What was built is fewer vectors per axis, not one arena: each axis's units are appended one a cycle while it lives, by the parallel shares, and an arena that keeps an axis's records contiguous needs a relocation scheme (or a recycled pool of growing buffers, compacted into the arena when the apex stops) that the shares can append to without a lock. That is a layout design, so it was left. What it could still take: the oak's `tail` is 156 to 430 ms, the spruce's about 230; `Axis::units` and `Draws::nodes` are the vectors left per axis, and `Axis::phytomers` is public.
2. **One core.** If wasm's one core must match the index-order loops exactly, the remaining gap (oak 80 s1: sketch and attachments, about 280 ms of 7 s) is visiting order. Closing it without a second path would mean laying the tree out in memory by depth, which is the same layout question as 1.

## 6. Notes

- The crate's tests take about 7 minutes a commit on this desk (FRICTION.md).
- No profiler could be used (`perf_event_paranoid` 2); the two probes were timers patched in and reverted (FRICTION.md).
- Workspace gate: `cargo test --profile ci --workspace --no-fail-fast` at `f55c91b1`: 1,113 passed, 0 failed, 20 ignored.
