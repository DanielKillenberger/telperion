# The scoreboard, first run: `base-2026-11-11`

Drafted by the builder (Opus) on 11 Nov 2026 for the host, who views the sheets and gives every verdict. Nothing here calls a tree better or worse. Run on the integration branch at `c187c68cd` (`goal/fn-321-020-one-grown-tree-at-any-scale-in`). Crate source was the same as at `ff3f3cabf`; the commits between them touch `.flow/` only.

## How to run it

```
bash .flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard/run.sh <label>
python3 -I .flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard/board.py compare scoreboard/<a> scoreboard/<b>
```

- **`run.sh <label>`** builds the probe, then runs the panel, cost, wasm, sheets and tables stages. Each stage can also be named alone. `SMOKE=1` draws one seed per preset.
- **Outputs.** It writes `scoreboard/<label>/`: `RUN.md` (commit, machine, seeds, the load at each stage), `measures.jsonl`, `cost.jsonl`, `many.json`, `wasm.jsonl`, `TABLES.md` and `sheets/*.jpg`. Raw stills and logs go to the ignored `raw/<label>/`.
- **Heavy jobs.** Every heavy job goes through `telperion-heavy`, one at a time. The whole panel is one GPU job, with each tree in its own process. A rerun keeps any tree already measured.
- **Options (fn-354).** `run.sh`'s header lists them: another checkout's crates (`CRATES`), a variant's environment per tree (`TREE_ENV`), a panel or cost subset, and extra shots (`foot`, the trunk base leafless; `joint`, the largest limb's junction). Each tree also records its three largest limbs' base diameters, the largest limb's chord from vertical, and the phytomers under 0.2 mm. `variants.py` lays several runs side by side, a column each. Unset, the base run is as it was; `engine.rs` reads the supernatural scale since fn-353 deleted `Growth::scale`.
- **`compare`** prints every median, cost and Wasm time that moved by more than 5%, and anything present in only one label. Tested on the base against itself (0 of 178 moved) and against the smoke run.
- **The probe** (`scoreboard/probe/`) is a crate outside the workspace. It reaches the checkout's crates by relative path and builds into `raw/target`, so no landing crate changes. Every tree goes through the pipeline: `executor::grow`, its expansion, and `mesh`. Stills are framed and lit as `examples/space/still.rs` frames them.
- **Check against earlier runs (measured).** The oak at 40 on seed 1001 and the beech at 40 on seed 1001 reproduce fn-345's run-A logs exactly: height, spread, nodes, leaves and trunk.
- **0.1.4.** `scoreboard/v014.sh` draws 0.1.4's stills from the `v0.1.4` tag in its own checkout (`.worktrees/v014-scoreboard`, its own `target/`). It times 0.1.4's Wasm from the published package with `wasm014.mjs`.

### What each measure is

- **Height and spread.** Read from the dressed mesh's bounds, leaves included, and both ground axes are given.
- **Trunk at 1.3 m.** The thickest wood from the root, read on the drawn tree. The diameter is interpolated along the edge that spans 1.3 m. The mesh's root flare is not included.
- **Lowest living lateral.** Read on the engine structure: the lowest junction on the thickest-wood trunk path whose lateral system grew in the last cycle. The **limb** version also requires 5 cm or more at the lateral's base.
  - This is not fn-341's bole-only read. It walks the whole trunk path, so it always gives a number.
  - A branch where the trunk path leaves its own axis is skipped.
- **Leaf area.** The sum of the dressed leaves' blade triangles, one-sided. For the spruce it is each needle's whole surface, which is not projected area.
- **LAI.** Leaf area over the convex hull of the leaves' ground positions. It is one tree's index, while the published values are for stands.
- **New shoots.** Engine axes that grew any phytomer in the last cycle, one current-year shoot each.
- **Drawn nodes.** The pipeline tree's node count.
- **The two grows.** Each tree is grown twice: once through the pipeline, and once through `telperion_space::grow` with the pipeline's own species and request, because the pipeline keeps its structure private (FRICTION.md).

## The tables

Median (lowest to highest) over seeds 1001 to 1012, from `scoreboard/base-2026-11-11/measures.jsonl`. All 162 trees were drawn and none was refused. The full tables, with every published range and its source, are in `scoreboard/base-2026-11-11/TABLES.md`.

| tree | drawn | height m | spread x m | spread z m | trunk 1.3 m, m | lowest living lateral m | lowest living limb 5 cm+ m | leaf area m2 | LAI | new shoots | drawn nodes |
|---|---|---|---|---|---|---|---|---|---|---|---|
| oak 40 | 12/12 | 10.6 (10.3 to 11.1) | 8.4 (6.7 to 9.5) | 8.4 (7.1 to 10.6) | 0.78 (0.56 to 0.94) | 0.4 (0.3 to 1.1) | 0.4 (0.3 to 1.1) | 185 (115 to 214) | 3.5 (2.8 to 4.3) | 26,950 (17,914 to 34,785) | 368,222 (266,656 to 426,124) |
| oak 80 | 12/12 | 19.4 (17.5 to 22.1) | 23.3 (17.9 to 27.8) | 22.6 (14.4 to 27.6) | 1.99 (1.10 to 2.52) | 4.6 (4.5 to 4.8) | 4.6 (4.5 to 4.8) | 1,120 (548 to 1,417) | 2.7 (2.3 to 3.6) | 158,926 (77,978 to 206,430) | 2,234,731 (1,283,980 to 2,663,560) |
| oak 120 | 12/12 | 27.6 (23.9 to 29.2) | 31.2 (22.1 to 36.2) | 30.5 (20.5 to 41.3) | 2.02 (1.43 to 2.80) | 5.0 (4.5 to 8.9) | 5.0 (4.5 to 15.0) | 1,409 (796 to 2,460) | 2.5 (1.8 to 3.8) | 213,402 (111,452 to 355,816) | 3,243,572 (1,876,936 to 4,930,342) |
| oak 200 | 12/12 | 32.4 (21.4 to 37.1) | 24.0 (19.1 to 38.4) | 25.0 (18.1 to 34.0) | 1.35 (1.02 to 2.74) | 15.9 (8.1 to 23.0) | 16.5 (8.1 to 23.0) | 638 (309 to 1,834) | 2.3 (1.2 to 3.2) | 86,240 (41,765 to 222,059) | 1,383,738 (743,514 to 3,337,920) |
| oak 300 | 12/12 | 35.3 (26.3 to 48.1) | 25.9 (19.3 to 51.9) | 24.5 (18.7 to 37.2) | 1.28 (0.89 to 1.73) | 20.2 (9.5 to 31.4) | 20.2 (9.7 to 31.4) | 561 (245 to 1,249) | 2.1 (1.0 to 3.3) | 73,442 (33,412 to 156,465) | 1,248,522 (595,901 to 2,523,496) |
| spruce 40 | 12/12 | 11.8 (11.5 to 12.4) | 7.3 (6.4 to 7.9) | 6.9 (6.1 to 7.8) | 0.35 (0.33 to 0.37) | 0.4 (0.3 to 0.4) | 0.4 (0.3 to 0.5) | 232 (215 to 255) | 6.7 (5.8 to 7.3) | 94,720 (88,302 to 105,563) | 989,462 (918,478 to 1,067,890) |
| beech 40 | 12/12 | 10.1 (9.2 to 11.4) | 6.5 (5.1 to 8.3) | 8.1 (5.5 to 9.3) | 0.32 (0.26 to 0.51) | 3.1 (1.0 to 4.4) | 3.2 (1.0 to 6.0) | 114 (56 to 261) | 3.4 (2.0 to 6.3) | 8,608 (4,079 to 17,291) | 110,924 (57,263 to 250,095) |
| birch 40 | 12/12 | 16.9 (16.0 to 17.7) | 8.2 (7.4 to 12.0) | 8.5 (7.2 to 10.7) | 0.30 (0.25 to 0.31) | 4.9 (3.8 to 5.5) | 5.6 (4.9 to 7.2) | 157 (97 to 197) | 3.1 (2.1 to 4.0) | 28,820 (20,645 to 33,605) | 308,086 (217,149 to 375,258) |
| palm 40 | 12/12 | 21.7 (21.5 to 22.0) | 18.8 (18.5 to 19.4) | 18.7 (18.3 to 19.2) | 0.59 (0.59 to 0.59) | - | - | 763 (757 to 768) | 3.9 (3.9 to 3.9) | 1 (1 to 1) | 7,605 (7,605 to 7,605) |
| ordinary 40 | 12/12 | 12.7 (11.8 to 13.2) | 8.6 (8.2 to 10.8) | 8.7 (8.1 to 9.8) | 0.25 (0.22 to 0.27) | 0.4 (0.3 to 1.0) | 0.7 (0.3 to 1.2) | 45 (35 to 56) | 0.8 (0.7 to 0.9) | 8,126 (6,182 to 10,192) | 111,452 (85,294 to 145,332) |
| Laurelin 80 | 12/12 | 45.5 (41.7 to 47.7) | 50.2 (45.0 to 54.4) | 50.1 (40.0 to 56.2) | 3.06 (2.85 to 3.83) | 11.4 (9.5 to 13.6) | 11.4 (9.5 to 13.6) | 9,561 (7,303 to 13,093) | 5.5 (4.0 to 7.6) | 141,644 (94,223 to 176,918) | 2,161,766 (1,604,907 to 2,895,724) |
| Telperion 82 | 12/12 | 46.4 (42.7 to 48.9) | 50.9 (46.0 to 55.0) | 50.9 (39.8 to 56.8) | 2.83 (2.65 to 3.54) | 11.4 (9.5 to 13.6) | 11.4 (9.5 to 13.6) | 9,479 (7,152 to 12,979) | 5.2 (3.6 to 6.9) | 153,346 (100,598 to 190,622) | 2,334,617 (1,711,214 to 3,102,380) |

Every preset's own age is 40, except Laurelin (80) and Telperion (82).

### Beside the published ranges

These are the checked values from fn-339 and fn-341, set beside the medians. TABLES.md quotes each source. The numbers are given without a verdict.

- **Oak trunk at 1.3 m.**
  - At 40: 0.78 m, against White's open-grown *Q. robur* 0.28 to 0.40 m, the *Q. garryana* ceiling of 0.26 m, and Phillips's 0.23 m at 10 m and 40 years.
  - At 80: 1.99 m, against 0.56 to 0.79 m (ceiling 0.52).
  - At 120: 2.02 m, against 0.83 to 1.09 m (ceiling 0.77).
  - At 300: 1.28 m, against 1.56 to 1.92 m (ceiling 1.94).
  - At 200: no checked value.
- **Oak height**, against Silvics' values: 14 m at 47 to 70 years and 24 m at 95 to 135 years (forest), 15 to 27 m when mature on favourable sites, a record of 36.6 m and a 34.8 m potential maximum.
  - Medians: 10.6, 19.4, 27.6, 32.4 and 35.3 m.
  - At 300 the range reaches 48.1 m.
- **Oak spread**, against Pretzsch's crown for White's DBH: 7.2 to 9.6 m at 40, 12.7 to 16.8 at 80, 17.4 to 21.8 at 120 and 29.3 to 34.6 at 300.
  - The two axes' medians: 8.4 / 8.4, 23.3 / 22.6, 31.2 / 30.5 and 25.9 / 24.5 m.
- **Oak crown ratio** (1 − lowest living lateral / height), against the 0.80 stand mean (Gould 2008).
  - Medians: 0.97, 0.76, 0.81, 0.50 and 0.39.
- **Oak LAI.** 3.5 to 2.1 across the ages, against one Belgian stand's 2.4 to 3.3 (unchecked) and a mixed stand's 5.
- **Beech.**
  - Trunk 0.32 m at 40, against White's 0.32 to 0.48 m.
  - LAI 3.4, against closed stands' 7.4 (5.6 to 9.5).
- **Spruce.**
  - 94,720 new shoots in the last year, against 691 to 2,372 per tree a year in a closed stand of about 36 years and 15.5 m (Bellan 2019).
  - LAI 6.7 as whole needle surface, against a plantation's projected 10.5.
- **No checked range** exists for the birch, palm, the ordinary tree, Laurelin or Telperion.

## Cost (R3)

Seed 1001, one thread pinned to one core. Each tree is grown, expanded and dressed in one process (`TABLES.md`, "Cost").

| tree | seconds | peak GiB |
|---|---|---|
| oak 40 | 2.41 (2.02 on its second run, as the oak at 40) | 0.40 |
| spruce 40 | 12.67 | 1.61 |
| beech 40 | 0.28 | 0.09 |
| birch 40 | 2.07 | 0.44 |
| palm 40 | 0.02 | 0.01 |
| ordinary 40 | 0.68 | 0.12 |
| Laurelin 80 | 17.31 | 2.51 |
| Telperion 82 | 17.25 | 2.75 |
| oak 120 | 45.09 | 2.55 |
| oak 300 | 170.73 | 2.53 |

**A thousand distinct oaks at 40** (seeds 1 to 1000), grown and dressed in one process across 32 threads: **493 s**, none refused, 13.0 GiB peak.

### Wasm

Medians of the three seeds' warm medians, fn-329's protocol, Node v26.12.1, one thread pinned. `npm run build` was run at this commit.

| preset (age) | this build, fidelity 0 | this build, fidelity 1 | 0.1.4 (its age) |
|---|---|---|---|
| ordinary (40) | 426 ms | 849 ms | 53 ms (100) |
| oak (40) | 2,073 ms | 310 ms | 124 ms (100) |
| spruce (40) | 7,548 ms | 1,180 ms | 122 ms (100) |
| beech (40) | 262 ms | 499 ms | not in 0.1.4 |
| birch (40) | 1,087 ms | 425 ms | 239 ms (70) |
| palm (40) | 17 ms | 27 ms | 5 ms (100) |
| Telperion (82) | 19,433 ms | 12,130 ms | 447 ms (100) |
| Laurelin (80) | 16,866 ms | 7,765 ms | 56 ms (100) |

The 0.1.4 call is `build(family, { field: true })`, because 0.1.4's field takes no grid size and 0.1.4 has no fidelity. 0.1.4's presets are other trees at other ages. Read the column as what 0.1.4 cost, not as like for like.

### Load and caveats on the times

- **Load.** Another session's gate (`cargo nextest ... -p telperion-core`, four threads) was in the heavy queue during the cost and Wasm stages, so the load average was 36 to 48 (`RUN.md`).
  - Several Wasm samples spread widely. Telperion at fidelity 1 on seed 1234 ran 8.9 to 23.9 s.
  - Laurelin's fidelity-1 times overlapped my own 0.1.4 timing on the same core for about a minute; that 0.1.4 run was stopped and rerun alone (FRICTION.md).
  - **Take these times as this afternoon's**, not as quiet-machine numbers.
- **The panel's own wall time.** About 2.5 hours, three quarters of it the oak at 200 and 300 (FRICTION.md).

## 0.1.4 beside it (R4)

- **Wasm times.** In the table above, from the published `telperion@0.1.4` package (`npm pack`, nothing else fetched).
- **Stills** (`sheets/v014-whole.jpg`). Drawn from the `v0.1.4` tag's own headless example, at the hero pose, whole view, size 960×720 and the same sun, on seeds 1001, 1007 and 1012 (and 7 for the oak).
  - Every preset 0.1.4 has, at its own preset age: 100, and 70 for the birch.
  - It took about 10 minutes of effort. The build took 22 s in its own `target/`, and all 22 stills drew.
- **Not done:** 0.1.4's oak at 40 to 300. The task asks for 0.1.4's trees, and its headless example sets no age, so its presets were drawn at their own ages. The tiles carry no height, spread or trunk, because 0.1.4's still prints none and the probe does not build against 0.1.4's API.

## Plain read per tree (R6)

From the whole-view sheets, the oak's leafless sheet and the 0.1.4 sheet. The mid and close sheets were made but not viewed by the builder. This describes what is drawn and does not judge it.

- **Oak 40.** A single upright stem with a crown about as tall as it is wide, leafy from near the ground. Low laterals splay out at the base on seeds 1001 and 1012. The trunk reads stout for the tree (56 to 75 cm on the sheet seeds).
- **Oak 80.** A broad, spreading crown on a short trunk forking at about 4.6 m (the lowest living lateral on every seed). The spread is wider than the height. Seed 7 is a dome; seed 1001 is a V of two leaders.
- **Oak 120.** Two to three long leaning leaders from a low fork. Seed 7 has a few bare, whip-like limbs out to the right. Seeds 1007 and 1012 are lopsided, with the crown off to one side of the base.
- **Oak 200.** One long, curving bare stem carrying a compact crown at the top.
  - Seed 1007's stem runs along the ground for several metres before rising.
  - Seed 1012's stem is S-shaped.
  - The crown ratio's median is 0.50.
- **Oak 300.** The same form, thinner and higher.
  - Seed 1001 is a tall, sinuous stem with two separate crown clumps.
  - Seed 1007's stem loops back on itself near the ground and carries a small crown at the end of a long diagonal limb.
  - The lowest living lateral's median is 20 m on a 35 m tree.
- **Spruce 40.** A narrow cone of tiered whorls from the ground up with a clear leader, much alike on all three seeds.
- **Beech 40.** A tall bare stem with a small, dark, rounded crown in its upper third and few low branches. Seed 1012 has a long thin shoot arching out low on the right.
- **Birch 40.** A white stem under a narrow, pale, open crown of hanging shoots. It is the tallest of the age-40 trees (16.9 m).
- **Palm 40.** A slender, straight, patterned stem with a wide, symmetrical crown of fronds. The three seeds are nearly identical (trunk 0.59 m and 7,605 nodes on every seed).
- **Ordinary 40.** A slim, single straight stem under a sparse, columnar crown of small leaves. The lightest leafage of the panel (LAI 0.8).
- **Laurelin 80.** A massive, short trunk with a very broad, golden crown wider than its 45 m height. The three seeds share one form.
- **Telperion 82.** The same frame as Laurelin, with a silver-white crown.
- **0.1.4, for comparison of form only.**
  - The oak is a dense, shrub-like green mass about as wide as tall, with no visible bole.
  - The spruce is a feathery cone with a thin spike above.
  - The birch is a broad, weeping curtain.
  - The palm is a slender stem under a frond crown.
  - The ordinary tree is a straight stem under a small, light crown.
  - Laurelin and Telperion are drawn without leaves: Laurelin as a tangle of twisted bare limbs, Telperion as a spiral-twisted trunk under a small bare crown.

## Reference photographs (R5)

`R5-REFERENCES.md`. The oak, beech and birch have photographs at all five scales, the spruce one of each scale, and the palm three (whole, trunk, bases). The ordinary tree, Laurelin and Telperion have none. No photograph shows an oak of known age.

## What R1 to R6 delivered

- **R1** `scoreboard/run.sh <label>` and `board.py compare`, as above.
- **R2** All of the panel: 162 trees, 26 sheets with labelled tiles.
  - No tree was refused, so the refused-tile path (a grey tile with the error) is written but was not exercised on this run.
- **R3** All three: native one-thread times, the thousand oaks, and the Wasm times. The times were taken under load (above).
- **R4** 0.1.4's Wasm times for its seven presets, and its whole-view stills at its own preset ages.
- **R5** `R5-REFERENCES.md`.
- **R6** This file and `scoreboard/base-2026-11-11/`.
