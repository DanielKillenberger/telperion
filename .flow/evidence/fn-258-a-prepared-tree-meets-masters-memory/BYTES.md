# fn-258 R1: where a prepared tree's bytes and frame time go

Measured 2026-10-10 on the fn-258 branch at 7803f205 (F4) and on master at ebd2e37b (`.worktrees/master-base`), before any cut. RTX 3080 (10 GB, shared with other sessions), 1920×1080 with the renderer's 4× MSAA, `RAYON_NUM_THREADS=4`, warm NVIDIA shader cache, every run under the shared `gpu.lock` after GPU utilisation fell below 15 %. Seed 1.

## How it was measured

One probe (`tools/src/main.rs`) built against each tree, as the parity scorecard's was: `Generator::prepare(Resident)`, `submit_prepared`, `set_family` (F4) or `set_material` (master), one draw, then three more builds in the same process, as the harness retunes. Then, with the last tree resident:

- **GPU bytes by owner:** the device allocator's report (`wgpu::Device::generate_allocator_report`), each live allocation under its buffer label, plus the allocator's reserved total and `nvidia-smi`'s process figure. A label is grouped by the code that allocates it (below).
- **CPU bytes:** `VmRSS` and `VmHWM` after the builds. For the oak at 40 the probe also called `malloc_trim(0)`: RSS fell from 782 to 310 MB (master: 212 to 201 MB). So most of F4's resident CPU after a build is freed heap glibc keeps, and the cost that matters is the peak: the preparation's working set. In a browser that peak stays, because a Wasm memory never shrinks.
- **Frame time per pass:** the renderer's own timestamp protocol (`telperion_render::measure`, 8 conditioning, 8 warm-up, 120 measured frames, camera still) at the harness hero pose (whole), half its distance (mid) and a quarter (close), median of each pass. "Frame ms" is the best of three rounds of 120 pipelined draws, wall clock per frame (the scorecard's FPS figure inverted).
- **Ages:** each version at its own default (master 70-120, F4 40, Laurelin 80, Telperion 82) and F4 again at master's default age. F4 refused the spruce at 100 and the beech at 120 with `DeviceRefused { "experimental generation", "Out of Memory" }` while other sessions held about 4.5 GB of the card; those two rows are missing.

## Owners, consumers and lifetimes (checked against code)

| Group | Labels | Consumer | Lifetime |
|---|---|---|---|
| Drawn tree, wood | `curve points`, `curve clusters`, `curve sections` (`curve.rs` `CurveGpu`) | the surfacing passes, every frame | the tree |
| Drawn tree, leaves and flowers | `foliage *`, `generated foliage`, `generated mass`, `generated florets`, `leaf *`, `selection` | selection and the vegetation pass | the tree |
| Camera budget | `curve camera` ×9 (`curve/target.rs` `Target`): config, counts, offsets, blocks, totals, ring records, vertices, indices, args | counts, offsets, blocks and records: the same frame's compute pass only (scratch kept between frames). Vertices, indices, args: the vegetation pass | the tree; remade when the budget changes |
| Sun budget | `curve sun` ×9, the same nine | records and the rest: the compute pass of a frame that surfaces the sun. Vertices and indices: the shadow pass of a frame that redraws the map (`sun.rs`: the map is redrawn when the casters change, the sun's wood only when the light or tree does) | the tree |
| Growth scratch (regions) | `chain axes`, `chain phytomers`, `chain apexes`, `chain status`, `chain sleeping buds` (`fine/chains.rs` `Grower::pool`) | the next build's grower, which reuses them; `Grower::trim` cuts them to what the last build asked | the generator, i.e. the page |
| Pool | `laid scratch`, `fine laid`, `packed axes`, `packed phytomers` (`fine/pool.rs`, given back in `fine/device.rs` `Drop`) | the next build, by size class | the generator |
| Targets | `probe`, `probe depth`, `probe resolve`, `shadow map` | the frame; the harness's canvas holds its own | the page; identical on both versions |
| Programs | none (driver memory) | every build | the page. `nvidia-smi` after device, renderer, generator, `prepare_programs`: F4 201, 202, 234, 313 MB; master 201, 210, 211, 211 MB |

Master's drawn wood is one CPU-tessellated mesh (`wood *`, `resident wood *`) at a fixed detail. F4 keeps the wood twice in a different shape: the curve (small), and per view a surfaced mesh sized for the most that view could ever ask (`Demand::at` with every ring at 384 sides, capped by the screen's pixels and two 128 MiB bindings), once for the camera and once for the sun.

## Bytes per preset

"smi" is the process's GPU memory from `nvidia-smi`, as the scorecard reports it. "allocated / reserved" is the allocator's: reserved is the device memory blocks it holds, which `nvidia-smi` sees. The budget, scratch, pool and target columns are allocated MiB.

| Tree | Version, age | smi MB | allocated / reserved MiB | drawn tree MiB (curve data + foliage) | camera budget | sun budget | growth scratch (regions) | pool | targets | RSS / peak MB |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| palm | old 100 | 483 | 80 / 192 | 1.4 | 0 | 0 | 0 | 0 | 76 | 340 / 340 |
| palm | new 40 | 839 | 185 / 448 | 0.3 | 58 | 48 | 0 | 0 | 76 | 356 / 356 |
| palm | new 100 | 837 | 247 / 448 | 0.4 | 97 | 71 | 0 | 0 | 76 | 341 / 341 |
| ordinary | old 100 | 481 | 98 / 256 | 18.8 | 0 | 0 | 0 | 0 | 76 | 334 / 334 |
| ordinary | new 40 | 1607 | 754 / 1536 | 3.6 | 362 | 309 | 0 | 0 | 76 | 454 / 454 |
| ordinary | new 100 | 1865 | 1383 / 1792 | 57.0 | 740 | 508 | 0 | 0 | 76 | 3311 / 3591 |
| spruce | old 100 | 1251 | 476 / 1280 | 397.5 | 0 | 0 | 0 | 0 | 76 | 351 / 351 |
| spruce | new 40 | 3823 | 2929 / 3750 | 196.3 | 752 | 496 | 1383 | 22 | 76 | 662 / 662 |
| birch | old 70 | 739 | 223 / 768 | 144.5 | 0 | 0 | 0 | 0 | 76 | 353 / 353 |
| birch | new 40 | 3171 | 2382 / 3099 | 22.1 | 471 | 358 | 1221 | 232 | 76 | 503 / 503 |
| birch | new 70 | 4847 | 3240 / 4774 | 130.8 | 775 | 650 | 1383 | 223 | 76 | 925 / 925 |
| oak | old 100 | 739 | 322 / 768 | 243.0 | 0 | 0 | 0 | 0 | 76 | 349 / 349 |
| oak | new 40 | 1609 | 838 / 1536 | 26.6 | 394 | 338 | 0 | 0 | 76 | 859 / 899 |
| oak | new 100 | 2377 | 1591 / 2304 | 185.8 | 743 | 584 | 0 | 0 | 76 | 8270 / 9137 |
| beech | old 120 | 1059 | 636 / 1152 | 557.6 | 0 | 0 | 0 | 0 | 76 | 498 / 913 |
| beech | new 40 | 2799 | 2200 / 2726 | 5.9 | 365 | 315 | 1382 | 54 | 76 | 415 / 415 |
| Laurelin | old 100 | 483 | 155 / 512 | 76.1 | 0 | 0 | 0 | 0 | 76 | 343 / 343 |
| Laurelin | new 80 | 4847 | 3854 / 4774 | 124.5 | 773 | 581 | 1383 | 913 | 76 | 489 / 489 |
| Laurelin | new 100 | 5871 | 4872 / 5798 | 225.7 | 777 | 689 | 1383 | 1719 | 76 | 612 / 612 |
| Telperion | old 100 | 739 | 254 / 768 | 174.9 | 0 | 0 | 0 | 0 | 76 | 347 / 347 |
| Telperion | new 82 | 4849 | 3926 / 4774 | 114.5 | 773 | 601 | 1383 | 976 | 76 | 516 / 516 |
| Telperion | new 100 | 5873 | 4840 / 5798 | 193.9 | 777 | 689 | 1383 | 1719 | 76 | 630 / 630 |


What the table says:

- **The two view budgets are most of F4's tree.** The camera's and the sun's surfaced meshes are 106 MiB of the palm's 106 and 1,374 of Telperion's 1,470 (`tree_buffer_bytes`). The drawn tree itself, curve plus leaves, is 0.3-226 MiB, at or under master's on every preset at default age.
- **Use against budget at the hero, mid and close views** (curve reports, `raw/curves.jsonl`): the palm writes 17-21 thousand vertices of a 723 thousand budget and 37-42 thousand triangles of 2.9 million; the ordinary 116-184 thousand of 7.46 million vertices; the spruce, the largest, 1.23-2.46 million of 7.46 million vertices and at most 7.0 million of 20.4 million triangles. Every view chose scale 1 with no overrun, so none was drawn coarser.
- **Growth scratch and the pool stay after every build:** 1.2-1.4 GB of regions on the five trees the fine executor grows (spruce, birch, beech, Laurelin, Telperion), and up to 1.7 GB of pooled geometry scratch on the Two Trees. They are held for the next build, never read by a frame.
- **The allocator holds 0.7-1.0 GB above what is allocated** on the large trees: blocks left after buffers of three different builds came and went.
- **Programs cost 102 MB of driver memory** before any tree, which master does not pay because it has no fine kernels to compile.
- **CPU:** at default age F4's peak RSS is within 1.0-2.6× master's (oak 899 against 349 MB). At master's age it is not: the ordinary at 100 peaks at 3.6 GB and the oak at 100 at 9.1 GB, against 334 and 349 MB.

## Frame time per pass, ms

Medians from the timestamp protocol; "frame" is wall clock per pipelined frame. Master redraws its sun map every frame; F4 keeps it while the casters do not change (fn-246), so F4's shadow reads 0.00 with the camera still.

|---|---|--:|---|--:|--:|--:|--:|--:|
| palm | old 100 | 6 | whole | 0.79 | 0.64 | 0.01 | 0.02 | - |
| | | | mid | 1.64 | 1.49 | 0.01 | 0.02 | - |
| | | | close | 2.24 | 2.06 | 0.01 | 0.02 | - |
| palm | new 40 | 6 | whole | 0.69 | 0.14 | 0.01 | 0.00 | 0.38 |
| | | | mid | 0.60 | 0.29 | 0.01 | 0.00 | 0.18 |
| | | | close | 0.78 | 0.40 | 0.01 | 0.00 | 0.25 |
| palm | new 100 | 8 | whole | 0.60 | 0.08 | 0.01 | 0.00 | 0.36 |
| | | | mid | 0.37 | 0.09 | 0.01 | 0.00 | 0.15 |
| | | | close | 0.42 | 0.09 | 0.01 | 0.00 | 0.20 |
| ordinary | old 100 | 53 | whole | 0.51 | 0.31 | 0.01 | 0.04 | - |
| | | | mid | 0.75 | 0.58 | 0.01 | 0.04 | - |
| | | | close | 0.90 | 0.74 | 0.01 | 0.04 | - |
| ordinary | new 40 | 553 | whole | 0.69 | 0.12 | 0.01 | 0.00 | 0.43 |
| | | | mid | 0.87 | 0.23 | 0.01 | 0.00 | 0.52 |
| | | | close | 1.02 | 0.36 | 0.01 | 0.00 | 0.53 |
| ordinary | new 100 | 16471 | whole | 2.08 | 0.62 | 0.01 | 0.00 | 1.29 |
| | | | mid | 2.80 | 1.02 | 0.01 | 0.00 | 1.61 |
| | | | close | 3.69 | 1.50 | 0.01 | 0.00 | 2.02 |
| spruce | old 100 | 185 | whole | 23.07 | 19.29 | 0.33 | 3.32 | - |
| | | | mid | 22.90 | 19.08 | 0.32 | 3.35 | - |
| | | | close | 13.22 | 9.46 | 0.30 | 3.38 | - |
| spruce | new 40 | 398 | whole | 8.59 | 6.04 | 0.21 | 0.00 | 2.26 |
| | | | mid | 11.53 | 9.36 | 0.21 | 0.00 | 1.84 |
| | | | close | 8.13 | 6.60 | 0.20 | 0.00 | 1.20 |
| birch | old 70 | 244 | whole | 7.92 | 7.70 | 0.02 | 0.08 | - |
| | | | mid | 24.45 | 24.39 | 0.02 | 0.08 | - |
| | | | close | 38.37 | 38.11 | 0.02 | 0.08 | - |
| birch | new 40 | 767 | whole | 2.08 | 1.06 | 0.01 | 0.00 | 0.87 |
| | | | mid | 3.25 | 2.16 | 0.01 | 0.00 | 0.94 |
| | | | close | 4.35 | 3.24 | 0.01 | 0.00 | 0.96 |
| birch | new 70 | 2037 | whole | 5.78 | 3.48 | 0.05 | 0.00 | 2.12 |
| | | | mid | 8.74 | 6.08 | 0.05 | 0.00 | 2.48 |
| | | | close | 13.85 | 11.34 | 0.05 | 0.00 | 2.32 |
| oak | old 100 | 170 | whole | 16.40 | 15.98 | 0.04 | 0.23 | - |
| | | | mid | 29.37 | 28.89 | 0.04 | 0.23 | - |
| | | | close | 25.60 | 25.13 | 0.03 | 0.24 | - |
| oak | new 40 | 1527 | whole | 4.35 | 3.39 | 0.02 | 0.00 | 0.81 |
| | | | mid | 10.15 | 9.00 | 0.02 | 0.00 | 1.04 |
| | | | close | 14.16 | 12.91 | 0.02 | 0.00 | 1.09 |
| oak | new 100 | 25401 | whole | 12.76 | 9.76 | 0.12 | 0.00 | 2.79 |
| | | | mid | 23.45 | 19.42 | 0.11 | 0.00 | 3.80 |
| | | | close | 20.36 | 17.95 | 0.10 | 0.00 | 2.12 |
| beech | old 120 | 3415 | whole | 16.30 | 14.49 | 0.21 | 1.56 | - |
| | | | mid | 69.60 | 67.73 | 0.21 | 1.59 | - |
| | | | close | 132.44 | 131.26 | 0.20 | 1.58 | - |
| beech | new 40 | 167 | whole | 1.63 | 0.93 | 0.01 | 0.00 | 0.57 |
| | | | mid | 2.58 | 1.72 | 0.01 | 0.00 | 0.69 |
| | | | close | 2.99 | 2.09 | 0.01 | 0.00 | 0.74 |
| Laurelin | old 100 | 200 | whole | 1.35 | 1.04 | 0.02 | 0.16 | - |
| | | | mid | 1.66 | 1.34 | 0.02 | 0.16 | - |
| | | | close | 1.32 | 1.00 | 0.02 | 0.16 | - |
| Laurelin | new 80 | 546 | whole | 5.60 | 3.65 | 0.03 | 0.00 | 1.70 |
| | | | mid | 12.00 | 9.61 | 0.03 | 0.00 | 2.14 |
| | | | close | 12.99 | 10.69 | 0.03 | 0.00 | 2.13 |
| Laurelin | new 100 | 971 | whole | 8.43 | 5.53 | 0.05 | 0.00 | 2.61 |
| | | | mid | 15.85 | 12.26 | 0.05 | 0.00 | 3.29 |
| | | | close | 18.59 | 14.90 | 0.04 | 0.00 | 3.40 |
| Telperion | old 100 | 599 | whole | 1.93 | 1.55 | 0.03 | 0.22 | - |
| | | | mid | 1.62 | 1.25 | 0.03 | 0.23 | - |
| | | | close | 1.66 | 1.28 | 0.03 | 0.23 | - |
| Telperion | new 82 | 587 | whole | 5.41 | 3.46 | 0.03 | 0.00 | 1.73 |
| | | | mid | 11.50 | 9.09 | 0.03 | 0.00 | 2.22 |
| | | | close | 12.62 | 10.24 | 0.03 | 0.00 | 2.15 |
| Telperion | new 100 | 897 | whole | 7.80 | 4.99 | 0.05 | 0.00 | 2.57 |
| | | | mid | 14.72 | 11.23 | 0.05 | 0.00 | 3.23 |
| | | | close | 16.61 | 13.17 | 0.04 | 0.00 | 3.24 |

What the frame table says:

- F4 is faster than master at every view for the five real species and the palm, at default age and (for the birch, oak and palm) at master's age.
- F4 is slower for the ordinary (0.69-1.02 against 0.51-0.90 ms at default age), Laurelin (5.6-13.0 against 1.3-1.7) and Telperion (5.4-12.6 against 1.6-1.9).
- **Surfacing is a fixed cost master does not have:** 0.2-0.5 ms on the small trees, 1.7-2.2 ms on the Two Trees, every frame. It is what keeps the ordinary behind. The measure pass walks every visible cluster at four error scales and the count and rings passes walk it twice more, though scale 1 fitted in every view measured.
- **The Two Trees' vegetation pass** is 3.5-10.7 ms against master's 1.0-1.3: F4 draws a different tree there (a full, flowering crown against master's bare contorted limbs; scorecard stills), so that cost is what the new tree's leaves and blossom are, not overhead.

## What the cuts can take without changing what is drawn

| Candidate | Bytes it would free | Changes what is drawn? |
|---|---|---|
| Retire the regions and the pool after a build | 1.2-3.1 GB on the five fine-executor trees; 0 elsewhere | No. A retune pays their allocation again |
| One set of surfacing buffers for both views: the sun surfaced into the camera's buffers before the shadow pass, the camera after it | the sun budget: 48-689 MiB | No, if the sun is surfaced whenever its map is redrawn |
| Measure scale 1 first, the coarser scales only when it overruns | none; 1-3 of 6 cluster walks a frame | No: the same scale is chosen |
| A smaller camera budget than the worst view could ask | most of the camera budget | **Yes**: a view past it is drawn coarser. A level-of-detail choice for the owner |
| Programs compiled only when a tree needs them | about 100 MB | No, but it reverses fn-243's compile at page load |

## CPU memory by owner (added after review)

Taken from the R3 sweep (`RESULT.md`, `raw/r3.jsonl`, seed 1, F4 after the cuts; master unchanged). After the builds the probe read `VmRSS`, called `malloc_trim(0)` and read it again. What `malloc_trim` returns is freed heap glibc kept; what stays is live. Checked against code: after `submit_prepared` the renderer keeps no CPU copy of the wood or the placed leaves (`Prepared` is consumed; they live on the device), only the one leaf's element, the bounds and the level tolerances (`generation/submit.rs`), so F4's live CPU is the process (driver, programs, the generator's kit and tables), and its peak is the preparation's working set: the skeleton, the fine roots' CPU half and the meshes it builds, all dropped when the build returns. No CPU pool or scratch outlives a build.

| Tree | Version, age | RSS after the builds MB | live after `malloc_trim` MB | freed heap glibc kept MB | peak (preparation working set) MB |
|---|---|--:|--:|--:|--:|
| date-palm | old 100 | 135 | 134 | 1 | 337 |
| date-palm | new 40 | 160 | 160 | 0 | 345 |
| date-palm | new 100 | 162 | 161 | 1 | 336 |
| ordinary | old 100 | 218 | 199 | 19 | 329 |
| ordinary | new 40 | 352 | 250 | 102 | 373 |
| ordinary | new 100 | 3404 | 310 | 3094 | 3688 |
| norway-spruce | old 100 | 222 | 199 | 23 | 348 |
| norway-spruce | new 40 | 600 | 414 | 186 | 596 |
| silver-birch | old 70 | 246 | 200 | 46 | 352 |
| silver-birch | new 40 | 368 | 327 | 41 | 470 |
| silver-birch | new 70 | 867 | 515 | 352 | 933 |
| oregon-white-oak | old 100 | 211 | 200 | 11 | 347 |
| oregon-white-oak | new 40 | 834 | 297 | 537 | 906 |
| oregon-white-oak | new 100 | 8017 | 197 | 7820 | 9004 |
| european-beech | old 120 | 324 | 273 | 51 | 912 |
| european-beech | new 40 | 278 | 274 | 4 | 413 |
| european-beech | new 120 | 597 | 376 | 221 | 679 |
| laurelin | old 100 | 204 | 199 | 5 | 335 |
| laurelin | new 80 | 365 | 310 | 55 | 474 |
| laurelin | new 100 | 417 | 353 | 64 | 516 |
| telperion | old 100 | 261 | 200 | 61 | 344 |
| telperion | new 82 | 359 | 307 | 52 | 466 |
| telperion | new 100 | 449 | 376 | 73 | 535 |

The live CPU F4 holds after a build is 0-215 MB above master's at default age (the spruce the most, 414 against 199). The rest of the gap is the working set: the oak at 100 peaks at 9.0 GB and keeps 7.8 GB of freed heap until trimmed; in a browser a Wasm memory keeps its peak.
