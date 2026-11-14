# fn-211 task 1: where a growth year's time goes (worker, 2026-10-06)

A dispatched worker wrote this. It measures; it designs nothing. Every gain below is an **estimate**, labelled so, for the host to weigh. F1 has not landed, so the tree profiled is today's whole tree (R1's fallback).

## How measured

- **Build:** `f9ac2d6d` (branch `fn-211-tree-space-f2b-growth-years-cheap`), release profile, this worktree's own `target/`.
- **Machine:** Ryzen 9 5950X, 32 threads. "all" is 32 threads; "1" is `taskset -c 0`.
- **Load:** another worker shared the desk. The 1-minute load stayed between 1.77 and 3.87 for every build, so no build needed a repeat (the run script waits while the load is 4 or more). Each JSON line in `raw/profile.jsonl` records its load.
- **Trees:** `telperion_space::{beech, spruce, oak, palm}()`, budget 20M. The oak is lit at extinction 0.5, sky 0.5 (round 10). The other three grow in neutral light, as in BASELINE.md. Ages 40 and 80, seeds 1 and 7.
- **Builds:** two builds per process. The tables give the second (repeated) build. Cold totals are within 0 to 10% (`raw/profile.jsonl`, rep 0).
- **Timers:** perf is not on the desk, so the passes were timed by hand.
  - `profile-instrumentation.patch` puts a wall-clock span around every pass. It also adds up the caller's time inside `cores::spread`, plus a share's growth on one core. That spread time is the pass's **parallel** part, and the rest of the pass is its **serial share**.
  - The patch was reverse-applied after the runs. Nothing under `crates/` changed (`git diff HEAD -- crates/` is empty). The probe is `profile-probe.rs`, and `profile-tables.py` makes every table here from the raw lines.
  - To rerun: apply the patch, copy the probe to `crates/telperion-space/examples/profile.rs`, then run `profile-run.sh`.
- **Counts per year:** come from a second run with `PROF_COUNTS=1` (`raw/counts.jsonl`; its timings are not used).

## 1. The table: each pass per tree (ms, repeated build)

The pass groups:

- **setup:** `Windows::new`.
- **balance, allot:** `grow.rs::step`.
- **grow:** `homes` + share growth (`units`) + `merge`.
- **wake:** waking buds.
- **sketch:** the yearly rough layout.
- **relay:** the full re-lay every 10 years. It covers its settle (standing, clone, assign, shed, scale, thicken), `place`, sag `levers`, sag `place` and write-back.
- **shade:** the lattice build and sweep (`field`) and the reads.
- **settle:** mark, assign, attachments, shed, disused, scale, thicken, leaf mean.
- **lay:** final `place`, `levers`, sag `place`.
- **Other:** wall time no span covers. It is unattributed; it is likely freeing the grower's per-axis state and the bookkeeping between spans, but that is not verified.
- **Serial:** the share of the build's wall time outside parallel regions.

| Tree | Cores | Load | setup | balance | allot | grow | wake | sketch | relay | shade | settle | lay | other | **Total** | serial |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| palm 40 s1 | all | 2.03 | 0.02 | 0.00 | 0.00 | 0.06 | 0.00 | – | – | – | 0.02 | 0.10 | 0.06 | **0.26** | 43% |
| palm 40 s1 | 1 | 2.03 | 0.03 | 0.00 | 0.00 | 0.06 | 0.00 | – | – | – | 0.02 | 0.10 | 0.07 | **0.28** | 46% |
| palm 40 s7 | all | 2.03 | 0.02 | 0.00 | 0.00 | 0.05 | 0.00 | – | – | – | 0.02 | 0.10 | 0.06 | **0.26** | 44% |
| palm 40 s7 | 1 | 2.03 | 0.03 | 0.00 | 0.00 | 0.06 | 0.00 | – | – | – | 0.02 | 0.12 | 0.08 | **0.31** | 44% |
| beech 40 s1 | all | 2.03 | 1.68 | 0.00 | 0.00 | 20.3 | 0.00 | – | – | – | 12.0 | 11.4 | 2.80 | **48.2** | 52% |
| beech 40 s1 | 1 | 2.03 | 1.75 | 0.00 | 0.00 | 21.4 | 0.00 | – | – | – | 11.1 | 17.4 | 1.86 | **53.5** | 37% |
| beech 40 s7 | all | 2.03 | 1.75 | 0.00 | 0.00 | 28.3 | 0.00 | – | – | – | 21.9 | 14.8 | 3.72 | **70.5** | 57% |
| beech 40 s7 | 1 | 2.03 | 1.69 | 0.00 | 0.00 | 43.6 | 0.00 | – | – | – | 24.5 | 30.3 | 3.60 | **104** | 40% |
| oak 40 s1 | all | 2.03 | 2.84 | 10.5 | 17.9 | 82.1 | 0.09 | 85.8 | 56.4 | 28.5 | 86.8 | 40.5 | 21.4 | **433** | 77% |
| oak 40 s1 | 1 | 2.03 | 3.64 | 9.08 | 17.8 | 105 | 0.10 | 71.1 | 67.6 | 28.2 | 83.8 | 72.1 | 17.2 | **475** | 63% |
| oak 40 s7 | all | 2.03 | 2.99 | 18.9 | 28.9 | 111 | 0.14 | 138 | 80.1 | 37.0 | 140 | 67.0 | 33.7 | **657** | 82% |
| oak 40 s7 | 1 | 2.11 | 2.69 | 19.5 | 28.7 | 182 | 0.29 | 128 | 99.1 | 36.7 | 141 | 132 | 31.1 | **802** | 63% |
| spruce 40 s1 | all | 2.11 | 5.61 | 0.01 | 0.00 | 176 | 15.1 | – | – | – | 187 | 320 | 55.3 | **758** | 79% |
| spruce 40 s1 | 1 | 2.11 | 5.86 | 0.00 | 0.00 | 332 | 21.6 | – | – | – | 186 | 824 | 53.1 | **1,423** | 40% |
| spruce 40 s7 | all | 2.18 | 5.71 | 0.00 | 0.00 | 166 | 8.86 | – | – | – | 176 | 301 | 52.9 | **711** | 78% |
| spruce 40 s7 | 1 | 2.18 | 5.60 | 0.00 | 0.00 | 320 | 8.42 | – | – | – | 175 | 765 | 49.0 | **1,322** | 40% |
| palm 80 s1 | all | 2.08 | 0.04 | 0.00 | 0.00 | 0.15 | 0.00 | – | – | – | 0.03 | 0.19 | 0.07 | **0.50** | 33% |
| palm 80 s1 | 1 | 2.08 | 0.04 | 0.00 | 0.00 | 0.15 | 0.00 | – | – | – | 0.03 | 0.18 | 0.07 | **0.48** | 34% |
| palm 80 s7 | all | 2.08 | 0.05 | 0.00 | 0.00 | 0.15 | 0.00 | – | – | – | 0.03 | 0.19 | 0.07 | **0.49** | 35% |
| palm 80 s7 | 1 | 2.08 | 0.04 | 0.00 | 0.00 | 0.14 | 0.00 | – | – | – | 0.03 | 0.18 | 0.07 | **0.46** | 34% |
| beech 80 s1 | all | 2.08 | 7.63 | 0.01 | 0.00 | 206 | 0.00 | – | – | – | 226 | 103 | 47.4 | **591** | 69% |
| beech 80 s1 | 1 | 2.08 | 7.46 | 0.01 | 0.00 | 416 | 0.01 | – | – | – | 232 | 265 | 39.4 | **960** | 39% |
| beech 80 s7 | all | 2.08 | 7.80 | 0.01 | 0.00 | 307 | 0.01 | – | – | – | 410 | 175 | 75.9 | **976** | 72% |
| beech 80 s7 | 1 | 2.08 | 7.44 | 0.01 | 0.00 | 739 | 0.01 | – | – | – | 393 | 462 | 74.9 | **1,677** | 39% |
| oak 80 s1 | all | 1.99 | 8.81 | 347 | 459 | 482 | 0.57 | 1,052 | 996 | 287 | 950 | 215 | 347 | **5,145** | 91% |
| oak 80 s1 | 1 | 2.66 | 8.62 | 361 | 457 | 1,374 | 1.78 | 1,002 | 1,356 | 279 | 960 | 483 | 294 | **6,576** | 68% |
| oak 80 s7 | all | 2.58 | 9.48 | 791 | 1,106 | 1,139 | 1.39 | 2,327 | 2,082 | 596 | 2,263 | 568 | 742 | **11,625** | 92% |
| oak 80 s7 | 1 | 2.27 | 8.72 | 731 | 1,047 | 3,006 | 5.23 | 2,184 | 2,895 | 578 | 2,272 | 1,223 | 710 | **14,660** | 69% |
| spruce 80 s1 | all | 1.91 | 15.3 | 0.02 | 0.00 | 709 | 46.3 | – | – | – | 740 | 1,117 | 206 | **2,833** | 77% |
| spruce 80 s1 | 1 | 1.91 | 14.7 | 0.02 | 0.00 | 1,850 | 41.5 | – | – | – | 782 | 3,153 | 188 | **6,028** | 35% |
| spruce 80 s7 | all | 1.77 | 14.5 | 0.02 | 0.00 | 700 | 48.9 | – | – | – | 751 | 1,097 | 205 | **2,816** | 77% |
| spruce 80 s7 | 1 | 3.80 | 14.4 | 0.01 | 0.00 | 1,830 | 43.5 | – | – | – | 732 | 3,165 | 193 | **5,978** | 35% |

Only the oak runs balance, allot, sketch, relay and shade: it is the one tree that reads light. The others' balance and allot return at once.

## 2. Each pass's serial share (oak 80, seed 1; spruce 80, seed 1)

The serial share is the part of the pass outside spread regions, on 1 core and on 32. Seed 7 and the beech are in `raw/passes-*.md`; their shares match these within a few points.

| Pass | Oak 1 core | Oak all | Oak serial (1 / all) | Spruce 1 core | Spruce all | Spruce serial (1 / all) |
|---|--:|--:|--:|--:|--:|--:|
| balance | 361 | 347 | 100% / 100% | 0.02 | 0.02 | 100% |
| allot | 457 | 459 | 100% / 100% | 0 | 0 | – |
| grow.homes | 44.8 | 47.2 | 100% / 100% | 87.6 | 94.7 | 100% / 100% |
| grow.units | 1,236 | 336 | 0% / 2% | 1,665 | 477 | 0% / 2% |
| grow.merge | 93.4 | 98.6 | 100% / 100% | 97.5 | 137 | 100% / 100% |
| wake | 1.78 | 0.57 | 100% / 100% | 41.5 | 46.3 | 100% / 100% |
| sketch | 1,002 | 1,052 | 100% / 100% | – | – | – |
| relay.settle | 474 | 490 | 100% / 100% | – | – | – |
| relay.place | 345 | 147 | 22% / 63% | – | – | – |
| relay.levers | 90.6 | 111 | 100% / 100% | – | – | – |
| relay.sagplace | 351 | 149 | 22% / 62% | – | – | – |
| relay.write | 94.3 | 99.5 | 100% / 100% | – | – | – |
| shade.field | 233 | 240 | 100% / 100% | – | – | – |
| shade.read | 46.4 | 47.3 | 100% / 100% | – | – | – |
| settle.assign | 149 | 147 | 100% / 100% | 143 | 139 | 100% / 100% |
| settle.attachments | 461 | 467 | 100% / 100% | – | – | – |
| settle.shed | 218 | 219 | 100% / 100% | 234 | 207 | 100% / 100% |
| settle.scale | 23.5 | 23.3 | 100% / 100% | 114 | 115 | 100% / 100% |
| settle.thicken | 97.5 | 84.3 | 100% / 100% | 282 | 270 | 100% / 100% |
| lay.place | 197 | 76.6 | 23% / 71% | 1,260 | 389 | 23% / 80% |
| lay.levers | 80.4 | 64.2 | 100% / 100% | 281 | 322 | 100% / 100% |
| lay.sagplace | 205 | 74.0 | 24% / 71% | 1,612 | 406 | 19% / 76% |

- **Serial on all cores:** the oak's build is 91 to 92% serial, the spruce's 77%, and the beech's 69 to 72%.
- **The two parallel passes run at a few times one core, not 32:**
  - Growth (`grow.units`) runs 2.4 to 3.8x faster on 32 threads than on one.
  - The lay (`place`) runs 2.4 to 4.0x faster.
  - Within the lay, the serial 20 to 25% is the per-depth framing loop (`frame`, `bend`, the starts). It is the 63 to 80% that remains once the pieces run on 32 threads.

## 3. A growth year: per year and against the tree's size

This is the oak at 80, seed 1, on all cores: each pass per year (ms), with the tree that year. Years 10, 20 and so on are re-lay years, and year 80 is the last, with no sketch, relay or shade (fn-210 lever c1).

| Year | balance | allot | grow | sketch | relay | shade | **Year** | axes | living apexes | axes in living subtrees | new phytomers / all |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 10 | 0.01 | 0.04 | 0.40 | 0.30 | 2.88 | 0.15 | **3.78** | 1,149 | 689 | 746 (65%) | 2,115 / 6,990 (30%) |
| 20 | 0.11 | 0.24 | 1.45 | 1.46 | 15.6 | 0.53 | **19.4** | 8,017 | 2,586 | 2,939 (37%) | 8,369 / 63,486 (13%) |
| 30 | 0.30 | 0.69 | 2.50 | 3.27 | 38.0 | 1.22 | **45.9** | 25,676 | 6,268 | 7,203 (28%) | 20,387 / 211,865 (9.6%) |
| 40 | 1.33 | 1.70 | 3.51 | 7.16 | 83.4 | 2.52 | **99.7** | 62,917 | 12,218 | 14,294 (23%) | 41,079 / 523,404 (7.8%) |
| 50 | 3.31 | 3.97 | 15.4 | 20.6 | 164 | 4.24 | **211** | 132,947 | 22,654 | 26,481 (20%) | 72,883 / 1.08M (6.7%) |
| 60 | 7.59 | 10.8 | 9.45 | 25.4 | 267 | 6.49 | **327** | 259,734 | 38,122 | 45,156 (17%) | 122,829 / 2.08M (5.9%) |
| 70 | 12.2 | 16.7 | 13.5 | 34.2 | 425 | 8.09 | **510** | 450,924 | 54,151 | 64,873 (14%) | 175,378 / 3.59M (4.9%) |
| 80 | 19.4 | 24.9 | 17.0 | – | – | – | **61.3** | 698,112 | 66,123 | 79,512 (11%) | 212,958 / 5.53M (3.8%) |

**The oak's years in total (all cores):**

- 3,624 ms over the 80 years: relay 996, sketch 1,052, grow 482, allot 459, balance 347, shade 287.
- Then 1,165 ms of settle and lay, once.
- For seed 7, the years take 8,042 ms and settle and lay 2,831 ms. At year 70 the re-lay takes 1,020 ms and the year 1,226 ms.

**What scales with what:**

- **Balance and allot** grow with the whole tree (all axes, 698k at 80). Only the subtrees holding a living apex change, and they are 11 to 14% of the axes in the last years: dead axes the oak keeps until settle (lever 1 excludes the oak).
- **The re-lay** grows with the standing tree. Every tenth year it is 80 to 85% of that year's cost.
- **Sketch and growth** grow with the year's new growth.
- **Shade** grows with the lattice's box (the crown's extent over the cell), not with the leaves. The field holds only the year's leaves.

**The spruce at 80, seed 1 (all cores):**

- Its years take 756 ms in all, all of it growth (and 46 ms of wake), against 1,857 ms of settle and lay once.
- The beech's years take 206 ms, against 330 ms.
- For both, the last year is the largest. It is 65 ms for the spruce, against 11 ms at year 70, with 157k new axes. Lever 1 cannot skip the laterals made in the last cycles: nothing makes them certain to be shed by the tree's age. The beech shows the same (37k new axes).
- **So for the spruce, the beech and the palm, a growth year is already only its new growth.** Their cost is the settle and lay done once.

## 4. Each pass: what it reads and writes per year, and its layer

The gains are **estimates**. Each is computed from the table above, at 32 threads, with the assumption stated in its row. None was measured.

- Layer 1 is exact parallelism, hash-identical.
- Layer 2 is incremental years.
- "Per year" is the cost each year; "once" is per build.

| Pass | Runs | Reads | Writes | Serial today | Layer | Estimated gain (all cores) |
|---|---|---|---|--:|---|---|
| **balance** (`grow/balance.rs`) | per year, oak | every pencil's `wood`; living apexes' `base_scale`, `running`, `light`; every axis's parent and PA; the shed draw per living lateral (lineage, cycle) | each living lateral's `memory`; each live apex's `kept`; `live` (shed apexes leave, `apex_end` set) | 100% | **2**, then 1 | **Layer 2 (est.):** the sums and memory update run over the axes in living subtrees only (11 to 14% of axes late) rather than all axes: oak s1 347 to about 60 to 90 ms, s7 791 to about 130 to 200 ms. This assumes the cost falls in proportion to the axes visited; a dead subtree's `light` is 0, but its `wood` cost sum still enters its living ancestors, so it needs a kept running sum per axis. **Layer 1 (est.):** a tree reduction by depth, 2 to 3x on what remains. |
| **allot** (`sketch.rs::allot`, `allocation.rs`) | per year, oak | every axis's link and λ (all axes); living apexes' presence, light, ψ | living apexes' `size` | 100% | **2**, then 1 | **Layer 2 (est.):** as for balance. `vigours` builds its links, `kids` sort and arrays over all n each year; only living subtrees carry light. Oak s1 459 to about 70 to 110 ms, s7 1,106 to about 170 to 270 ms. The link table and sort change only by the year's new axes, so they could be kept rather than rebuilt. **Layer 1 (est.):** the basipetal sum and acropetal split by depth, 2 to 3x. |
| **grow.units** | per year | `Rules`, pencils (`light`, `size`, `kept`), `roots`, `horizon`, each apex's own axis and draws | each apex's own axis, draws, units and successor; new axes, apexes, sleepers | 2% | **1** (efficiency) | Already parallel, but at 2.4 to 3.8x on 32 threads. The cause is unknown: shares of 1,024 apexes mean the oak has 1 to 12 shares before year 40, and memory may also be a factor. **Est.:** reaching 6x would save oak s1 about 140 ms and spruce s1 about 190 ms. |
| **grow.homes, grow.merge** | per year | the live list; each share's made lines | appends to axes, draws, units, successor, roots, horizon, live, asleep, woken | 100% | 1 | **Est.:** 146 ms on oak s1 and 232 ms on spruce s1; at most about half could be saved. The append order is the hash, so only the index shifting and copying parallelise. |
| **wake** | per year | `asleep[cycle]`; bearer chains (axes, draws, successor) | new axes, through one share | 100% | none | Under 50 ms everywhere. |
| **sketch** (`sketch.rs::sketch`) | per year, oak | the axes that grew this year (draws, phytomers, pencil); new axes' parents' layers | those axes' phytomer tip, frame, scale, size, rank; pencils; `leaves` | 100% | **1** | It is already incremental: it lays only this year's units and axes, so layer 2 does not apply. **Layer 1 (est.):** the advanced axes are independent, and new axes depend only on their parents' layers, so they can be laid by depth as `place` does. If it reaches `place`'s 2.5x: oak s1 1,052 to about 420 ms, s7 2,327 to about 930 ms. `leaves` must be gathered in today's order, because the lattice sums them. |
| **relay** (every 10 years) | per 10 years, oak | the whole standing tree (standing, clone, draws), live | every standing axis's base, frame and phytomer tips; `pencils[i].layer`; `leaves` | 91% (settle 100%, place about 63%) | **1**, and 2 open | **Layer 1 (est.):** its settle (490 ms on s1) and levers (111) are the same passes as the final settle and lay, below, at 2 to 3x; with `place`'s serial framing as well, s1 996 to about 450 ms. **Layer 2:** a re-lay of only the changed chains is not exact. A chain's place reads every ancestor's frame and sag, and the trunk's load changes every year, so every standing chain moves. It is a host design question (spec "Design questions"), so no gain is estimated. |
| **shade** (`sketch.rs::shade`, `light.rs::Field`) | per year, oak | the year's leaves; live apexes' tips; the light's directions | living apexes' `light` | 100% | **1** | The field already holds only the year's leaves, so layer 2 does not apply beyond the host's open "coarser time step". **Layer 1 (est.):** the sweep's directions are independent, and P4 made them parallel bit-identically, if the per-direction buffers are summed in order. With 1 + 4 × rings directions: s1 287 to about 100 ms. |
| **settle.assign** | once | each axis's draws; its parent's end presence | phytomers' scale, size, light, rank; axes' units, alive, vigour | 100% | **1** | **Est.:** by depth, 2 to 3x: oak s1 147 to about 60 ms, spruce s1 139 to about 55 ms. |
| **settle.attachments** (oak only: retained pipes) | once | the grown tree (all axes, phytomers) | the grown tree's sizes and girth | 100% | **1** | **Est.:** 2 to 3x: oak s1 467 to about 180 ms, s7 1,040 to about 400 ms. |
| **settle.shed** (`fades`) | once | all axes' units, birth, alive, vigour, parent phytomer scale | the kept axes and index; fades | 100% | **1** | `fades` climbs from every axis to the root, which costs n × depth, and each climb is independent. **Est.:** 3 to 5x, as axes are independent: oak s1 219 to about 55 ms, spruce s1 207 to about 50 ms. |
| **settle.scale, thicken, disused** | once | parents' scale (scale); children's sections (thicken) | phytomer scale; radius | 100% | **1** | **Est.:** by depth, 2 to 3x. Thicken's sums into a parent must keep today's order. Spruce s1 385 to about 150 ms; oak s1 115 to about 45 ms. |
| **lay.place, lay.sagplace** | once | every axis's frame from its parent's layer | every phytomer's tip and frame | 20 to 25% (1 core), 63 to 80% (all) | **1** | Today's parallel lay is bound by its serial framing per depth. **Est.:** parallel framing at 3x would take spruce s1 795 to about 350 ms and oak s1 151 to about 70 ms. |
| **lay.levers** (`sag::levers`) | once (and per re-lay) | all phytomers' scale, frame, mass | per-phytomer levers | 100% | **1** | **Est.:** by depth, 2 to 3x: spruce s1 322 to about 130 ms. |
| **setup.windows** | once | species, age | the windows table | 100% | none | 9 to 15 ms. |

### What the two layers might give (estimates, all cores, repeated build)

- **Layer 1 on every serial pass above, at 2.5 to 3x** (what `place` and growth reach today):
  - Oak s1: about 4.3 s of 5.15 s is serial. 5.1 s falls to about 2.3 to 2.6 s; s7's 11.6 s falls to about 5 to 6 s.
  - Spruce s1: about 2.0 s of 2.8 s. 2.8 s falls to about 1.4 to 1.6 s.
  - Beech s1: 0.59 s falls to about 0.35 s.
  - Palm: nothing to gain (0.5 ms).
- **Layer 2:**
  - It applies only to the oak's balance and allot, in this tree. That is about 0.6 to 0.7 s on s1 and 1.5 s on s7 before layer 1.
  - Sketch, growth and shade already work only on the year's new growth or leaves.
  - Settle and lay run once per build. Layer 2 would touch them only in an age walk, where age a + 1 could reuse age a's settle.
  - **The re-lay is the one year-pass whose cost follows the whole tree** (oak s1 996 ms, s7 2,082 ms). Incremental re-lays are not exact, so the host has to settle them first.
- **The spruce and beech gain nothing from layer 2 in their years** (756 and 206 ms, all new growth). Their cost is the once-per-build settle and lay, which is layer 1's.

## 5. Files

- `PROFILE.md`: this file.
- `profile-instrumentation.patch`: the timers, reverse-applied after the runs.
- `profile-probe.rs`: the probe.
- `profile-tables.py`: makes the tables.
- `profile-run.sh`: runs the matrix.
- `FRICTION.md`
- `raw/` (gitignored):
  - `profile.jsonl`: 64 builds, with the spans per cycle.
  - `counts.jsonl`
  - `run.log`
  - `summary.md`, `passes-*.md`, `years-*.md`
