# fn-209 baseline: parameters to a usable tree, before on-demand work (worker, 2026-10-06)

The owner asked (2026-10-06) whether F1 will be faster than today, and whether near-instant retuning and many trees in a game are within reach. This file records what happens today. It proposes no design.

## How measured

- **Build:** master `428009eb` (F2's speed levers are in), release profile, this worktree's own `target/`.
- **Machine:** AMD Ryzen 9 5950X, 16 cores and 32 threads, 31 GB of RAM.
- **Load:** every timing records the 1-minute load. The load stayed between 1.2 and 3.4 throughout. One spruce repeat ran at 4.29, so the spruce at 80 was measured again at 2.2 to 2.8 (`raw/baseline/engine-repeat.log`). Both passes agree within 3%.
- **Seed:** seed 1 everywhere.
- **Cores:** "all" means 32 threads. "1" means `taskset -c 0`, which every `available_parallelism` call obeys (the probe reports `threads: 1`).
- **Cold and repeated:**
  - Cold is the first build in a fresh process.
  - Repeated is the mean of the following builds of the same tree in that process.
  - No stage caches anything across builds: no cache exists in `telperion-space`, the pipeline or `web.rs` (checked). So any parameter change reruns everything, and cold is the honest cost of a retune. Cold differs from repeated only by allocator and page-fault warm-up, 0 to 15%.
- **Engine (part 1):**
  - Measured with `baseline-probe.rs`, which descends from fn-198's `cost-probe.rs`.
  - It times `grow_staged`'s stages, then the stills runner's dressing: `tree::convert`, `executor::expand` and `Expansion::mesh` (`examples/space/still.rs:195-204`).
  - The family is overlaid with each `space_*.rs` example's `ROWS`.
  - The oak grows under light 0.5,0.5, as in round 10. The beech, spruce and palm grow under neutral light.
  - Peak memory is the process's `VmHWM` after the first build.
- **Today (part 2):**
  - Native timings come from `examples/generation_stages.rs`. It runs `pipeline::build(Request::mesh())`, the build behind both `mesh::build` and the Wasm binding's `build()`, and reports the pipeline's own stage timers.
  - The Wasm timings come from `baseline-wasm.mjs`. It loads the core binding in Node, with no browser, and calls `build()` with `{surface, foliage}`.
  - The dev harness reaches the same `mesh::build` through the render crate's Wasm (`crates/telperion-render/src/web.rs:210`).
- **Raw data:** the logs are in `raw/baseline/` (gitignored). `baseline-run.sh` reruns everything.

## 1. The tree-space engine: parameters to a dressed mesh (ms)

Cold / repeated. "Grow" is the sum of the engine's stages. "Dress" is the conversion, the expansion and the CPU mesh (wood and leaves concurrently, then the cull).

### Age 80

| | Cores | Growth | Rough layout | Re-lays | Light | Settle | Final lay | **Grow** | Convert | Mesh | **Dress** | **Total** | Peak RSS |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| palm | all | 0.6 / 0.5 | – | – | – | 0.0 | 0.2 | **0.8 / 0.7** | 0.0 | 2.8 / 2.7 | **4.7 / 4.4** | **5.5 / 5.1** | 7 MB |
| palm | 1 | 0.6 / 0.4 | – | – | – | 0.0 | 0.2 | **0.8 / 0.7** | 0.0 | 4.7 / 4.4 | **6.5 / 6.1** | **7.3 / 6.8** | 7 MB |
| beech | all | 429 / 351 | – | – | – | 236 / 228 | 101 / 100 | **805 / 717** | 101 / 104 | 1,735 / 1,731 | **1,843 / 1,843** | **2,648 / 2,560** | 2.6 GB |
| beech | 1 | 689 / 572 | – | – | – | 232 / 240 | 268 / 271 | **1,227 / 1,123** | 117 / 138 | 2,901 / 3,148 | **3,025 / 3,293** | **4,251 / 4,416** | 2.6 GB |
| oak (lit) | all | 1,937 / 1,897 | 1,071 / 1,104 | 1,172 / 1,164 | 325 / 327 | 1,010 / 1,032 | 230 / 237 | **5,927 / 5,961** | 70 / 72 | 771 / 784 | **845 / 861** | **6,772 / 6,822** | 2.5 GB |
| oak (lit) | 1 | 3,516 / 2,815 | 1,210 / 1,040 | 2,090 / 1,470 | 367 / 318 | 1,284 / 1,000 | 562 / 514 | **9,231 / 7,327** | 83 / 83 | 1,546 / 1,457 | **1,634 / 1,545** | **10,864 / 8,871** | 2.4 GB |
| spruce | all | 977 / 865 | – | – | – | 796 / 780 | 1,204 / 1,228 | **3,149 / 3,067** | 342 / 371 | 15,580 / 15,468 | **15,946 / 15,864** | **19,094 / 18,931** | 8.5 GB |
| spruce | 1 | 2,412 / 2,104 | – | – | – | 779 / 794 | 3,434 / 3,388 | **6,775 / 6,443** | 400 / 425 | 19,928 / 19,877 | **20,352 / 20,326** | **27,126 / 26,769** | 8.1 GB |

- **Expansion** is 2 to 24 ms everywhere, so the table folds it into "Dress".
- **Dropping the built spruce** takes another 350 ms.
- **The spruce repeat** (load 2.2 to 2.8) gave a cold total of 19,604 ms, a repeated total of 18,615 ms, and a mesh of 15.3 to 15.8 s.

### Age 40

| | Cores | Growth | Rough | Re-lays | Light | Settle | Final lay | **Grow** | Convert | Mesh | **Dress** | **Total** | Peak RSS |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| palm | all | 0.3 / 0.2 | – | – | – | 0.0 | 0.1 | **0.4 / 0.35** | 0.0 | 3.5 / 2.7 | **4.6 / 3.7** | **5.0 / 4.0** | 6 MB |
| palm | 1 | 0.3 / 0.2 | – | – | – | 0.0 | 0.1 | **0.4 / 0.3** | 0.0 | 4.5 / 4.1 | **5.5 / 5.0** | **6.0 / 5.4** | 6 MB |
| beech | all | 64 / 51 | – | – | – | 14 / 11 | 12 / 11 | **93 / 75** | 8 / 6 | 63 / 58 | **71 / 65** | **164 / 140** | 185 MB |
| beech | 1 | 66 / 51 | – | – | – | 10 / 10 | 18 / 16 | **95 / 78** | 6 / 6 | 138 / 136 | **145 / 143** | **240 / 221** | 183 MB |
| oak (lit) | all | 173 / 167 | 85 / 90 | 72 / 71 | 29 / 31 | 85 / 88 | 44 / 44 | **508 / 508** | 13 / 10 | 132 / 124 | **146 / 135** | **655 / 643** | 369 MB |
| oak (lit) | 1 | 209 / 206 | 74 / 81 | 80 / 81 | 30 / 31 | 91 / 95 | 77 / 79 | **572 / 585** | 13 / 11 | 268 / 256 | **282 / 269** | **854 / 854** | 350 MB |
| spruce | all | 243 / 210 | – | – | – | 201 / 197 | 386 / 326 | **878 / 777** | 87 / 78 | 3,982 / 3,981 | **4,075 / 4,065** | **4,953 / 4,841** | 2.2 GB |
| spruce | 1 | 463 / 372 | – | – | – | 203 / 190 | 878 / 825 | **1,588 / 1,429** | 102 / 98 | 4,896 / 4,874 | **5,004 / 4,978** | **6,592 / 6,407** | 2.1 GB |

### What the engine builds

| | Age | Kept phytomers | Pipeline nodes | Wood triangles | Leaves |
|---|--:|--:|--:|--:|--:|
| palm | 80 / 40 | 960 / 480 | 961 / 481 | 69k / 50k | 6.6k / 6.6k |
| beech | 80 / 40 | 1.60M / 113k | 1.53M / 102k | 65.9M / 4.4M | 2.47M / 66k |
| oak | 80 / 40 | 1.10M / 191k | 869k / 158k | 37.6M / 6.8M | 951k / 166k |
| spruce | 80 / 40 | 4.78M / 1.18M | 4.64M / 1.15M | 212.4M / 54.5M | 16.9M / 4.6M |

## 2. Today's production path: the shipped presets (ms)

Seed 1, `pipeline::build(Request::mesh())`, cold / repeated. The stage columns are repeated means.

| Preset | Native, all cores | Native, 1 core | Wasm in Node (1 thread) | Skeleton | Rings | Surface | Placement | Cull | Nodes | Wood tris | Leaves | Wasm memory |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| date-palm | **4.6 / 4.4** | 5.9 / 5.5 | 7.3 / 6.1 | 1.6 | 0.7 | 0.7 | 1.5 | 1.0 | 552 | 32k | 6.6k | 2 MB |
| ordinary | **117 / 107** | 123 / 118 | 156 / 118 | 41 | 5 | 8 | 12 | 50 | 11.6k | 0.60M | 41k | 27 MB |
| laurelin | **420 / 387** | 459 / 452 | 488 / 475 | 97 | 11 | 8 | 110 | 168 | 53.9k | 2.23M | 393k | 88 MB |
| oregon-white-oak | **422 / 413** | 571 / 600 | 626 / 598 | 37 | 25 | 26 | 223 | 104 | 134k | 7.71M | 767k | 265 MB |
| telperion | **917 / 941** | 1,102 / 1,071 | 1,111 / 1,077 | 359 | 19 | 20 | 162 | 403 | 77.3k | 5.61M | 550k | 255 MB |
| silver-birch | **2,759 / 2,749** | 2,911 / 2,869 | 2,708 / 2,684 | 23 | 13 | 17 | 93 | 2,611 | 86.3k | 4.99M | 258k | 165 MB |
| norway-spruce | **4,805 / 4,805** | 4,914 / 4,874 | 5,687 / 5,582 | 32 | 19 | 23 | 4,240 | 486 | 96.2k | 5.94M | 7.41M | 283 MB |
| european-beech (in work) | **3,361 / 3,374** | 3,562 / 3,611 | not in the binding's catalogue | 54 | 40 | 44 | 2,454 | 799 | 213k | 12.2M | 5.83M | – |

- **The harness rebuilds on every dial notch** only when the last build took under 80 ms (`harness/GrowerDev.tsx:40`, `BUILD_LIVE_MS`). Slower builds wait 250 ms after the dial rests.
- **Only the date palm retunes live today.** The ordinary preset, at 107 to 118 ms, misses the 80 ms bar.
- **Wasm memory** is the binding's linear memory after the builds. It stands in for native peak RSS, which `generation_stages` does not report.

## 3. Side by side: parameters to a usable tree, all cores, repeated

| Species | Today (preset) | Engine at 80 | Engine at 40 | Engine at 80 against today |
|---|--:|--:|--:|--:|
| palm | 4.4 ms | 5.1 ms | 4.0 ms | 1.2x |
| beech | 3.37 s (in work) | 2.56 s | 0.14 s | **0.76x (the engine is faster)** |
| oak | 0.41 s | 6.82 s | 0.64 s | 16.5x |
| spruce | 4.81 s | 18.9 s | 4.84 s | 3.9x |

## What stands out

1. **Neither path's time is mostly growth.**
   - Today, the skeleton takes 23 to 359 ms. Leaf placement and the cull take the rest: the spruce's placement 4.2 s, the birch's cull 2.6 s, the beech's placement 2.5 s.
   - In the engine, dressing is 70% of the beech's time and 84% of the spruce's.
   - The spruce's mesh at 80 years runs 15.5 s for 212M wood triangles and 16.9M needles.
   - The oak is the one tree whose own growth dominates: 5.9 of 6.8 s, of which 2.6 s is the light (the rough layout, re-lays and sweep).
2. **Neither path uses the cores much.**
   - The engine runs 1.3 to 1.7x faster on 32 threads than on 1. Growth gains 1.5 to 2.4x, the final lay 2.2 to 2.8x, and settling gains nothing.
   - The spruce's mesh gains 1.3x on 32 threads, and today's path gains 1.0 to 1.5x. Today's placement and cull, the stages that dominate it, run serially.
3. **The engine is faster than today for the beech at 80 (2.6 s against 3.4 s).** At 40 years every engine tree but the spruce builds in about 0.65 s or less (the beech in 0.14 s).
4. **Memory is the hard wall for the browser and for many trees.**
   - At 80 years the engine peaks at 8.5 GB for one spruce and 2.5 to 2.6 GB for an oak or a beech.
   - wasm32 addresses 4 GB at most, so the spruce at 80 cannot be built whole in the browser as the engine stands.
   - Today's presets fit in 2 to 283 MB of Wasm memory.
5. **Engine growth is much faster than fn-198 measured.**
   - fn-198 measured the oak at 10.8 s, the spruce at 11.4 s and the beech at 4.1 s.
   - Today the oak takes 5.9 s, the spruce 3.1 s and the beech 0.8 s (F2's levers).
   - The spruce's CPU mesh went from 13.2 to 15.5 s over the same period: it now carries 16.9M needles, against 15.5M then.
6. **Only the palm meets the harness's 80 ms live bar,** on both paths. Retuning any other species today takes 0.1 to 5 s on today's path, and 0.14 to 19 s on the engine's.
