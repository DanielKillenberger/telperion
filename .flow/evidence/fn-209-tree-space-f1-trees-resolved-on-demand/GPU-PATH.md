# fn-209: the engine's trees on the harness's GPU path (worker, 2026-10-06)

The owner asked (2026-10-06) whether dressing is fast enough when the tree-space engine's trees go through the fast GPU path that the harness uses. This file measures that question. It changes no product behaviour and proposes no design. BASELINE.md, beside it, measured the CPU reference path.

## Answer in brief

- **An engine tree cannot reach the GPU path today.** The entry takes a family, never a tree (see "What is missing" below). So the GPU placement itself is not measured for engine trees.
- **What could be measured:**
  - every CPU step the GPU path runs before and around its dispatches, on the engine's own expansion;
  - the upload and the draw, through the same renderer and passes.
- **The CPU preparation is the wall.** For the GPU path it is three passes over every node: the compact rings, the stations and the curve. On the spruce at 80 they take 3.9 s, against 15.9 s for the CPU mesh. On the beech at 80 they take 1.1 s, against 1.8 s.
- **Why it scales so badly:** the cost per node matches the shipped presets (about 0.2 to 0.26 µs per node per step). The engine's trees simply carry 7 to 48 times the nodes.
- **The shipped spruce and oak build in 0.15 to 0.17 s** from parameters to first frame on this path. A spruce at 40 from the engine needs at least 1.4 s after growth.
- **The steady frame breaks 16.7 ms** for the engine's spruce at 80 (58 ms) and its spruce at 40 (17.9 ms). The oak at 80 sits at the edge (16.3 ms).

## How measured

- **Build:** this worktree (master `428009eb` plus the baseline commit), release profile, its own `target/`.
- **Machine:** Ryzen 9 5950X with 32 threads, an NVIDIA GeForce RTX 3080 (10 GB), Vulkan, driver 610.57.04.
- **Probe:** `gpu-path-probe.rs`, copied to `crates/telperion-render/examples/f1_gpu_path.rs` to run, and not committed there. `gpu-path-run.sh` reruns everything.
- **GPU lock:** every run held the GPU lock (`flock .../scratchpad/gpu.lock`).
- **Load:**
  - The 1-minute load was 76 when the work began, with another worker on the desk.
  - The lock was held elsewhere for about 49 minutes (FRICTION.md). By the time the runs began (15:47), the load had fallen to 0.6.
  - It stayed between 0.6 and 2.0 for every timing. Each log line records it.
- **Seed:** 1.
- **Rows and light:**
  - The engine's families are overlaid with the same rows as `baseline-probe.rs` (the `space_*.rs` examples' `ROWS`).
  - The oak grows under light 0.5,0.5, as in round 10. The others grow under neutral light.
- **Cold and repeated:** "cold" is the first build in a process and "repeated" is the next. The tables give the repeated figures. The spruce at 80 was built once, so its figures are cold.
- **The harness's GPU path:**
  - `harness/GrowerDev.tsx:78` calls `setTreeGpu`.
  - That reaches `crates/telperion-render/src/web/generation.rs:41`: `Generator::prepare_async(&family, Delivery::Resident)`, then `Renderer::submit_prepared`.
  - The native entry is `Generator::prepare` (`src/generation.rs`). Its body is `src/generation/preparation.rs`.
  - The WGSL is in `src/generation/*.wgsl`: placement, prefix and pack, scatter (compaction), mass, and the positions of the ring words.
  - The wood is drawn from its curve by the surfacing compute pass (fn-208).
- **Presets:** the probe runs exactly that native entry, then submits, then draws one frame at the hero pose to device completion.
- **Engine trees:**
  1. The probe grows the tree with `telperion_space::grow`, then runs `tree::convert` and `executor::expand`.
  2. It runs the expansion's public CPU steps that `prepare_async` would run, in the same branch order: `supports_stations`, `round_section` and `seats` pick the branch, then `compact` or `compact_with_contacts`, then `stations` or `compact_stations`, then `curve`.
  3. The GPU placement cannot be reached (see below). The leaves therefore come from the CPU reference (`Expansion::mesh`), with its triangle wood dropped. They are drawn through `Renderer::submit`, which feeds the same selection, shadow, vegetation and surfacing passes as `submit_prepared`.
- **First frame:** `Renderer::draw` at `hero_pose`, 960x720 (the stills' size), then a wait for the device to finish.
- **Steady frame:** `telperion_render::measure`: 8 conditioning frames, 8 warm-up frames and 120 measured frames, with GPU timestamps around each pass. Every session's verdict was valid.
- **Memory:**
  - Peak RSS is the process's `VmHWM`.
  - For engine trees, "to prep" is the high-water mark before the CPU reference ran. It is the figure a GPU path would see.
  - GPU memory is device `memory.used` from `nvidia-smi`, sampled every 100 ms, peak minus idle. It is a whole-device figure.
  - "Tree buffers" is `Generator::tree_buffer_bytes`.
- **Raw data:** `raw/gpu-path/` (gitignored) holds one `.log` per run (JSON lines and the steady report) and one `.smi` per run.

## 1. Shipped presets through the harness's GPU path (ms, repeated)

`Generator::prepare` takes parameters to a resident tree. Its stage timers overlap, as `Metrics` documents. "GPU work" is the sum of the upload, the dispatches and the waits for positions, placement, compaction and mass. "Wood" includes the curve and the early ring preparation.

| Preset | Backend | Prepare | Skeleton | Ring prep | Descriptors | GPU work | Wood | Submit | First frame | **To first frame** | CPU path (BASELINE) | Leaves |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| date-palm | CPU fallback (rosette) | 4.5 | 1.6 | – | – | – | – | 5.5 | 1.5 | **11.5** | 4.4 | 6.6k |
| european-beech | CPU fallback (short shoots and clumping fail `plan::supports`) | 3,332 | 48 | – | – | – | – | 165 | 160 | **3,657** | 3,374 | 5.83M |
| norway-spruce | GPU | 128 | 30 | 25 | 25 | 21 | 55 | 17 | 29 | **174** | 4,805 | 7.41M |
| oregon-white-oak | GPU | 119 | 32 | 30 | 17 | 4.7 | 67 | 18 | 16 | **153** | 413 | 767k |

The cold builds took 191 ms for the spruce and 191 ms for the oak.

| Preset | Steady frame p50 / p95 | Vegetation | Selection | Shadow | Surfacing | Peak RSS | GPU memory | Tree buffers | GPU compute peak |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| date-palm | 1.04 / 1.20 | 0.23 | 0.01 | 0.02 | 0.79 | 142 MB | 236 MiB | 81 MB | – |
| european-beech | 14.2 / 14.6 | 9.53 | 0.27 | 1.87 | 2.54 | 880 MB | 1,531 MiB | 1.14 GB | – |
| norway-spruce | 20.4 / 20.6 | 14.96 | 0.33 | 3.45 | 1.63 | 271 MB | 1,511 MiB | 948 MB | 259 MB |
| oregon-white-oak | 6.7 / 6.9 | 4.01 | 0.04 | 0.31 | 2.37 | 263 MB | 1,511 MiB | 967 MB | 79 MB |

## 2. Engine trees: from the grown tree to the first frame (ms)

"CPU prep" is what the GPU path would spend on the CPU for this tree: the conversion, `expand`, the rings, the stations and the curve. "GPU placement" is not reachable. Submit and first frame go through `Renderer::submit` with CPU-placed instances. The resident path skips that instance upload, so treat these two columns as an upper reference.

### Age 80

| | GPU path branch | Grow | Convert + expand | Rings | Stations | Curve | **CPU prep** | GPU placement | Submit | First frame | **Grown to first frame, at least** | CPU dress (BASELINE) |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| palm | CPU fallback (rosette): runs `Expansion::mesh` | 1.0 | 1.7 | – | – | 0.3 | **4.4** with the fallback's mesh | – | 5.5 | 1.4 | **11** | 4.4 |
| beech | `compact` + `stations` | 724 | 144 | 322 | 305 | 350 | **1,121** | not reachable | 154 | 166 | **1,441** | 1,843 |
| oak (lit) | `compact` + `stations` | 5,588 | 93 | 168 | 186 | 193 | **639** | not reachable | 83 | 116 | **838** | 861 |
| spruce (cold) | `compact_with_contacts` + `compact_stations` | 3,253 | 497 | 1,080 | 1,184 | 1,125 | **3,886** | not reachable | 645 | 418 | **4,949** | 15,864 |

### Age 40

| | Grow | Convert + expand | Rings | Stations | Curve | **CPU prep** | Submit | First frame | **Grown to first frame, at least** | CPU dress (BASELINE) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| palm (CPU fallback) | 0.4 | 1.2 | – | – | 0.3 | **3.9** with the fallback's mesh | 5.6 | 1.5 | **11** | 3.7 |
| beech | 71 | 6.8 | 16.7 | 14.5 | 17.0 | **55** | 17 | 12 | **84** | 65 |
| oak (lit) | 467 | 14.8 | 22.5 | 29.2 | 26.6 | **93** | 21 | 16 | **130** | 135 |
| spruce | 735 | 123 | 277 | 290 | 291 | **980** | 195 | 230 | **1,405** | 4,065 |

- **Parameters to first frame** is the grow time plus the last column. That gives, at 80: beech 2.2 s, oak 6.4 s, spruce 8.2 s. At 40: beech 0.16 s, oak 0.60 s, spruce 2.1 s.
- **On the 80 ms live bar:** only the palm meets it. The beech at 40 comes close, at about 155 ms before GPU placement.
- **Not measured: GPU placement for the engine's spruce.** The shipped spruce's GPU work is about 21 ms for 7.4M input stations. Scaling linearly to the engine spruce's 16.9M stations gives about 50 ms. That figure is an extrapolation, not a measurement.
- **Not checked: whether 16.9M stations fit the resident path's buffer and binding limits.** This is unknown.

### Steady frame and memory

| | Age | Steady p50 / p95 | Vegetation | Selection | Shadow | Surfacing | Leaves | Nodes | Peak RSS to prep | Peak RSS with the CPU reference | GPU memory | Tree buffers |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| palm | 80 | 0.84 / 0.87 | 0.16 | 0.01 | 0.02 | 0.65 | 6.6k | 961 | 136 MB | 140 MB | 492 MiB | 131 MB |
| beech | 80 | 10.7 / 11.0 | 6.62 | 0.11 | 0.90 | 3.09 | 2.47M | 1.53M | 1.05 GB | 2.77 GB | 1,900 MiB | 1.33 GB |
| oak | 80 | 16.3 / 17.0 | 13.56 | 0.05 | 0.40 | 2.28 | 951k | 869k | 2.02 GB | 2.77 GB | 1,337 MiB | 1.01 GB |
| spruce | 80 | **58.2 / 58.7** | 38.68 | 0.88 | 8.24 | 10.35 | 16.9M | 4.64M | 3.50 GB | 8.69 GB | 2,551 MiB | 2.20 GB |
| palm | 40 | 0.98 / 1.00 | 0.25 | 0.01 | 0.02 | 0.71 | 6.6k | 481 | 135 MB | 140 MB | 492 MiB | 103 MB |
| beech | 40 | 2.78 / 2.83 | 1.70 | 0.01 | 0.04 | 1.04 | 66k | 102k | 191 MB | 312 MB | 1,004 MiB | 654 MB |
| oak | 40 | 6.63 / 7.18 | 5.40 | 0.01 | 0.08 | 1.14 | 166k | 158k | 322 MB | 498 MB | 1,004 MiB | 680 MB |
| spruce | 40 | 17.9 / 18.2 | 11.38 | 0.21 | 2.33 | 3.94 | 4.61M | 1.15M | 969 MB | 2.35 GB | 1,900 MiB | 1.38 GB |

- **Coarsened sun wood:** the spruce at 80 drew its sun's wood coarsened by 2 (`sun_wood_scale: 2`), because the shadow's wood budget ran out. Every other view drew at scale 1.
- **About 135 MB of the RSS is the GPU driver's own**, which is the palm's whole figure.
- **On the engine side, peak memory to prep is set by growth** (the oak at 80: 2.0 GB) or by the conversion and rings (the spruce at 80: 3.5 GB). The CPU reference's 8.7 GB for the spruce is what the GPU path would avoid.

## 3. What is missing for an engine tree to reach the GPU path (F4's job, not built)

1. **There is no entry for a tree.** `Generator::prepare` and `prepare_async` take `(&Family, Delivery)`. The first thing they do is `executor::grow(family)` (`preparation.rs:14`). An engine tree reaches the pipeline as a `telperion_core::tree::Tree`, which `executor::expand(tree, &family)` turns into an `Expansion`. Nothing in `Generator` accepts an `Expansion` or a `Tree`. Everything after `preparation.rs:17` already works on an `Expansion` (`x`). The renderer's private passes (`compute_async`, `begin_positions`, `emit_positions`, `upload_wood`) are reachable only through that entry.
2. **The pipeline boundary forbids this use today.** `docs/pipeline.md` sanctions `expand` for "the renderer's tests of the GPU kernels on hand-built trees" only: "growing one stays inside the pipeline, so `expand` opens no second chain". An engine tree entering through `expand` is the design question that design principle 1 answers. It is the host's to decide.
3. **The converter is example code.** `tree::convert` lives in `crates/telperion-render/examples/space/tree.rs`, and `telperion-space` is only a dev-dependency of the render crate. No product crate can turn a grown `Structure` into a pipeline `Tree`.
4. **The harness cannot ask for an engine tree.** `setTreeGpu` takes family JSON only (`web/generation.rs:13`), and the Wasm binding does not carry the engine.
5. **Branch eligibility:**
   - The engine's beech, oak and spruce qualify for GPU stations only because their overlay rows set `shortShootSpacing` and `limbClumping` to 0. The shipped European beech fails `plan::supports` and runs the CPU fallback, at 3.3 s.
   - The palm's rosette fails `supports_stations` in either form and always takes the CPU fallback. At 11 ms, that is no cost.

## What limits it

1. **CPU preparation that scales with nodes, not the GPU.**
   - The rings, the stations and the curve are each a CPU pass over every node, at about 0.21 to 0.26 µs per node each on the engine's trees. The shipped spruce costs about the same per node.
   - The engine's trees have 7x (the beech at 80: 1.53M against 213k) to 48x (the spruce at 80: 4.64M against 96k) the shipped presets' nodes.
   - So the GPU path cuts the spruce's dressing from 15.9 s to about 3.9 s, but no further.
   - fn-171's GPU ring preparation would take one of the three passes. The curve and the stations would remain.
2. **The conversion:** 0.1 to 0.5 s at 80, before any pipeline work.
3. **Growth:** for the oak, growth is 5.6 of 6.4 s, whatever the dressing does.
4. **The frame:**
   - At 80, the engine's spruce draws 16.9M needles in 58 ms a frame, 3.5x the 16.7 ms budget. Its vegetation pass alone takes 39 ms.
   - The oak at 80 (951k leaves, 16.3 ms) is at the budget's edge. The shipped spruce costs 20 ms for 7.4M needles.
   - Nothing reduces a tree's cost with distance yet (fn-176).
5. **Memory:**
   - At 80, the spruce needs 3.5 GB of CPU memory to reach the GPU and 2.5 GB of GPU memory.
   - The browser's 4 GB wasm32 ceiling (BASELINE, finding 4) still binds the preparation.
