# fn-212 R2: what can be deleted before porting, the fresh baseline, and the proposed R3 probe (task 1, worker, 2026-10-07)

This file covers the delete-first audit with measured costs, the baseline from settled geometry to draw-ready resident artifacts, and the R3 probe proposed to the host. It is analysis and measurement only: no product code changed, and every probe was a temporary patch, reverted (`git status` clean under `crates/`). Code claims cite `1419a832`. CONTRACT.md, beside this file, holds the input/output contract.

## Answer in brief

- **Baseline, settled geometry to resident:**
  - Beech at 80: **1.8 s** (seed 1) and **3.2 s** (seed 7), warm.
  - Spruce at 80: **17.3 s** (seed 1, cold) and **19.5 s** (seed 7, warm).
  - Native, all cores, the quietest of three passes (load 12 to 25). Across all three passes and every build the totals ranged 1.8 to 10.3 s (beech) and 16.3 to 31.6 s (spruce): the desk was shared (FRICTION.md).
- **The spruce at 80 does not take the path the spec assumed.**
  - Its GPU ring positions are refused for "compute limits" (`positions.rs:61-66`). The cause is the index buffer of a ring mesh nobody draws: 637,310,160 indices × 4 B = 2.55 GB, against this device's 2,147,483,644-byte binding limit (measured).
  - It then sweeps the canonical float32 wood on the CPU and prepares stations again: **9.0 to 9.8 s**, half its preparation.
  - The compact rings and stations computed before the refusal (**3.7 to 4.6 s**) are discarded.
- **The compact rings are the curve's points, bit for bit (measured).** All 5.3M rings of the spruce and 1.6 to 2.7M of the beech, every one of their twelve float words, in the same run order. Deriving them from the curve takes 22 to 249 ms, against 375 to 2,305 ms to sweep them again. One path sampling and framing serves both. Rings stop being a product of their own.
- **The beech's rings have no consumer.**
  - It seats no leaves (`surfaceContact` unset, so 0), and the placement reads the ring buffer only for a contact (`pack.wgsl:16`, `shape.z > 0`).
  - Its compact rings and their GPU positions (0.47 to 0.82 s, 26 % of its preparation) can go.
- **The foliage cull removes nothing on any fixture.**
  - All four presets have `shellDepth = 1`, and every placed leaf survived: the input count equals the output count, 2.47M to 16.9M (measured).
  - Its ranks, workgroup scan, serial `prefix` pass, raw buffer and scatter exist only for it (`place.wgsl:71-113`, `scatter.wgsl`).
  - The bounds reduction stays, because the mass grid and the framing read it.
- **What remains to port is smaller than R1's three passes.** It is:
  1. fine nodes assembled in the tree's frame;
  2. their curve points and clusters (one sweep);
  3. the whole-tree reductions;
  4. stations and placement without the cull;
  5. contact rings from curve points, for seated species only.
- **The R3 probe is proposed with its exact boundary below.** One question decides its denominator: are the CPU deletions above taken first? By arithmetic they would take the spruce at 80 from about 17 to 20 s to about 6 to 7 s. That arithmetic is not measured.

## 1. The baseline

### Method

- **Boundary.**
  - **Start:** the instant `telperion_space::grow` returns the settled, lineage-ordered `Structure` inside stage 2 (`pipeline/space.rs:71-72`). Growth and settling are outside.
  - **End:** after `Generator::prepare(&family, Delivery::Resident)` returned, `Renderer::submit_prepared` returned, an empty queue submission flushed its queued writes, and `device.poll(wait)` returned.
  - At the end, the curve, the leaves, the masses and the element are resident (`generation/submit.rs:16-24`). The first frame is excluded; R5a reports frames apart.
- **Stages.** Wall-clock marks between the existing steps, each with the process's peak RSS since the start:
  - The marks are in `baseline-marks.patch` (`telperion_core::probe_marks`, temporary).
  - The start resets the peak through `/proc/self/clear_refs`.
  - Marks were placed in `pipeline/space.rs` (conversion), `pipeline/executor.rs` (envelope, inputs, clothing, element, box) and `generation/preparation.rs` (each branch's steps).
  - The probe (`baseline-probe.rs`, run as an example) times the submit and the drain itself. It also prints the generator's own `Metrics`.
- **Runs.**
  - One preset and seed a process, three builds each: cold, then two warm.
  - Native, `--profile ci`, all 32 threads, the GPU lock held.
  - Device memory was sampled every 100 ms by `nvidia-smi` beside the process (`baseline-run.py`).
  - Three passes (`raw/pass1` to `raw/pass3`). The tables take pass 2, the quietest: load 11.8 to 24.6 at the start of each build (`baseline-table.py`).
- **Machine:** Ryzen 9 5950X (32 threads), RTX 3080 10 GB, Vulkan. Adapter limits (measured): `max_buffer_size` 4,294,967,295; `max_storage_buffer_binding_size` 2,147,483,644; 65,535 workgroups a dimension.
- **Fixtures:** the shipped `european-beech` and `norway-spruce` presets, age 80, seeds 1 and 7, through the pipeline as the harness builds them. The code is this tip with `build-fix.patch`, which is not committed.
- **Not the same as earlier figures.** fn-198 TIMING.md's 14.4 s for the spruce at 80 runs from the family and includes growth. GPU-PATH.md's 3.9 s is the CPU work only, on a different expansion path. Neither is this boundary.

### Beech at 80 (ms; pass 2)

| Tree | Build | Load | Convert | Envelope, inputs | Clothe, element, box | Rings (CPU) | Stations (CPU) | Ring positions (GPU) | Placement (GPU) | Curve (CPU) | Submit | Drain | **Total** | Peak RSS MB | Leaves |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 80 s1 | cold | 16.9 | 193 | 71 | 10 | 483 | 461 | 50 | 199 | 596 | 98 | 0 | **2,182** | 1,222 | 2,474,647 |
| beech 80 s1 | warm | 16.9 | 193 | 70 | 11 | 410 | 358 | 59 | 160 | 462 | 89 | 3 | **1,824** | 1,263 | 2,474,647 |
| beech 80 s7 | cold | 17.3 | 225 | 95 | 14 | 804 | 758 | 128 | 290 | 889 | 170 | 10 | **3,386** | 1,891 | 4,296,133 |
| beech 80 s7 | warm | 16.2 | 310 | 102 | 16 | 700 | 624 | 120 | 299 | 850 | 153 | 5 | **3,184** | 1,959 | 4,296,133 |

- **Branch taken:** `compact`, then `begin_positions` overlapped with `stations`, then `compute_buffer_async` on the uploaded positions (`preparation.rs:47`, `:64-99`, `:201-214`).
- **Placement's split** (`Metrics`, warm): upload and dispatch 104 to 245 ms, the wait for place and prefix 50 to 54, scatter 1 to 2, mass 0 to 1.
- **Device memory**, peak minus idle (whole device): 1,164 MiB (s1), 1,886 MiB (s7).
- **Tree buffers** after the submit: 141 MB (s1), 240 MB (s7).

### Spruce at 80 (ms; pass 2, the quietest build per seed)

| Step | s1, cold, load 12.6 | s7, warm, load 18.6 |
|---|--:|--:|
| Convert (and free the structure) | 835 | 1,103 |
| Envelope, inputs | 247 | 263 |
| Clothe, element, box | 34 | 46 |
| Compact rings with contacts (CPU), **discarded** | 1,800 | 2,224 |
| Compact stations (CPU), **discarded** | 1,863 | 2,323 |
| GPU positions refused, "compute limits" | 80 | 96 |
| Canonical float32 wood with contacts, shared stations, wood upload (CPU, `preparation.rs:122-163`) | 9,018 | 9,793 |
| Placement (GPU): upload and dispatch / wait / scatter / mass | 1,094 (557 / 517 / 6 / 2) | 1,282 (691 / 574 / 6 / 1) |
| Curve (CPU) | 1,708 | 1,926 |
| Assemble | 29 | 34 |
| Submit (curve packed, uploaded, pipelines built) | 506 | 404 |
| Drain | 53 | 25 |
| **Total** | **17,267** | **19,519** |
| Peak RSS since settled | 5,332 MB | 5,546 MB |

- **Leaves:** 16,912,892 (s1) and 16,848,780 (s7).
- **Device memory**, peak minus idle: 3,557 and 3,528 MiB.
- **Placement's compute peak** (`Metrics::gpu_compute_peak_bytes`): 2,241 and 2,231 MiB.
- **Tree buffers:** 759 and 756 MB.
- **Contact edges on the CPU:** 185 MB, 40 B a node (`Metrics::shared_contact_cpu_bytes`).
- **The other passes,** spruce totals by build: pass 1 18.0 / 19.2 (s1) and 30.5 / 16.3 s (s7) at load 38 to 50; pass 3 31.6 / 27.3 and 26.1 / 24.0 s at load 13 to 33.
  - Run-to-run spread on the spruce is about ±25 % on this desk. A gate measured against it needs interleaved runs (§4).

## 2. The delete-first audit

### a. Rings and curve share their path sampling and framing

- **The code already shares the functions:**
  - the compact rings: `paths` → `sample_path` → `frames` (`surface/compact.rs:89`, `:154-162`);
  - the curve: `paths` → `rings::rank` → `Scratch::sweep` → `frames` (`surface/curve.rs:90`, `:114-122`).
- **Each runs the whole sweep again, twice per run:** once to rank, once to emit (`compact.rs:154`, `:161`; `curve.rs:114`, `:122`).
- **Measured** (`shared-probe.rs`; `raw/shared-1.jsonl`; load 10 to 29):

| Tree | Rings | Curve points | Equal to the bit (12 words each) | Runs equal | Rings ms | Curve ms | Rings from the curve ms |
|---|--:|--:|--:|---|--:|--:|--:|
| beech 80 s1 | 1,647,289 | 1,647,289 | 1,647,289 | yes | 375 to 400 | 402 to 449 | 22 to 40 |
| beech 80 s7 | 2,735,730 | 2,735,730 | 2,735,730 | yes | 629 to 734 | 736 to 855 | 38 to 73 |
| spruce 80 s1 | 5,310,918 | 5,310,918 | 5,310,918 | yes | 1,780 to 1,911 (with contacts) | 1,788 to 1,807 | 84 to 138 |
| spruce 80 s7 | 5,285,504 | 5,285,504 | 5,285,504 | yes | 1,739 to 2,305 (with contacts) | 1,897 to 2,486 | 78 to 249 |

- **What the comparison covered:** the ring words are centre, radius, normal, distance along, binormal and twist phase. They were compared with the curve point's fields rounded to f32, the phase taken as `2π · twistRate · along / height` (`compact.rs:185-198`).
- **The remaining ring words** are vertex and cap offsets from a prefix sum over runs (`compact.rs:207-213`).
- **So a ring is the curve point on the device.** Rings stop being a second sweep: CPU or GPU, the reader derives them from the curve. This deletes 0.4 to 2.3 s of CPU work today and the separate ring port fn-171 proposed.
- **Two duplicates remain, not merged here:**
  - The leaf stations transport their own frames along the foliage's bearing runs (`foliage/station.rs:304-356`, `plan.rs:270`), a different split from the sweep's runs.
  - `AttachmentSurface::selected` sweeps a third time where stations build their own contacts (`surface/attachment.rs:66-132`, from `foliage/prepared.rs:60`). That path is not on either fixture's resident path. Its cost for the spruce, measured through `Expansion::stations`, is 5.8 to 11.0 s.

### b. Contact geometry only where consumed

- **Seated leaves are the only consumer:**
  - `pack.wgsl:16` blends a station's point with `contact(...)` only when `shape.z = surface_contact > 0` (`data.rs:57`).
  - The CPU reads them the same way (`station.rs:102-109`).
  - The wood itself never reads contacts: the curve has no edges (`surface/curve.rs`), and the CPU wood sweeps with `edges: false` (`build.rs:70-73`).
- **Which fixtures seat:** only the spruce (`norway-spruce.values:51`, `surfaceContact = 1.0`). The beech's is unset, so 0 (`canopy/params.rs:536`).
- **What a contact reads:** a station's node's ring pair and the neighbouring pair on its run (`contact.wgsl:29-63`).
- **What is built today:**
  - an edge for every node, 40 B on 64-bit: 185 MB at the spruce at 80, measured through `Metrics`;
  - every ring of every run;
  - for the beech, ring positions emitted and bound for a contact that is never read (`preparation.rs:62-114`, then `:205-212`).
- **Deletable now (beech):** the compact rings and the GPU positions for species that seat nothing. Measured cost 0.47 to 0.82 s (rings plus positions, pass 2), 26 % of the beech's preparation.
- **Spruce, after (a):** only the nodes stations stand on and their run neighbours need rings, derived from curve points.
  - The share of nodes that bear stations is **unknown**. Method: count the distinct `edge` nodes in `prepare_compact_stations`' segments against all nodes.
- **The refused positions.** The GPU positions pass also sizes and checks normals, coordinates and indices for a triangle mesh that no reader draws: the positions pass writes six buffers, and only positions are read (`positions.rs:48-55`, `positions.wgsl:26-33`; consumers: CONTRACT.md §5).
  - Its compute-limit test fails the spruce at 80 on the index buffer: 2.55 GB > 2.15 GB.
  - Emitting positions alone, 1.29 GB at the spruce at 80, would fit.
  - Estimated by arithmetic (not measured): the spruce would then take the compact path and skip the 9.0 to 9.8 s CPU wood path. After (a), the compact rings would also come from the curve.
  - The positions' admission (`positions.wgsl:49-92`, triangle and normal checks) guards a mesh that is not drawn. What, if anything, it still protects is **unknown**.

### c. The foliage cull's rejection policy

- **fn-198 R1** (`.flow/specs/fn-198-…md:37`): "the twig layer, its thickness gate, the tip shoot, girth patches and the foliage cull are removed for those presets' path".
- **The rejection is a no-op on the engine presets.**
  - `shellDepth = 1` for the beech, spruce and oak (`european-beech.values:87`, `norway-spruce.values:44`, `oregon-white-oak.values:44`). At 1 the shell test always holds: `radius_at ≤ max_radius = shell` (`place.wgsl:42-44`).
  - The palm has 0.85 and could cull.
  - Measured: `input_instances == instances` on every build (2,474,647; 4,296,133; 16,912,892; 16,848,780).
- **What exists only for the rejection:**
  - `in_shell` (`place.wgsl:42-53`, `:71`);
  - the alive flag that resets a culled leaf's full bounds (`:75`);
  - the workgroup scan and `ranks` (`:80-86`);
  - the serial `prefix` pass, one thread over every workgroup (66,066 at the spruce at 80; `:106-113`);
  - the `raw` buffer and `ranks` buffer, 16 B a leaf (270 MB at the spruce; `compute.rs:126-127`);
  - the scatter (`scatter.wgsl`, `compute.rs:188-200`, 6 to 8 ms measured);
  - the survivor count's read before the output is sized (`compute.rs:163-187`).
- **What stays:**
  - the bounds reduction: the crown bounds size the mass grid (`compute.rs:244-262`) and set the scene's crown (`submit.rs:27`), and the full bounds frame the tree (`preparation.rs:275`);
  - the error flags (`summary[1]`).
- **Cost:** the wait for place and prefix is 517 to 574 ms at the spruce. How much of it the serial prefix takes is **unknown**: no GPU timestamps. Method: `profile.rs`'s timestamp writes around the two passes.
- **On both executors:** the CPU's `foliage::cull` (`foliage.rs:255-309`, from `drawn.rs:252`) goes with it.

### d. Other deletions found on the way

- **The curve's shader and its five compute pipelines are built at every submit** (`crates/telperion-render/src/curve.rs:79-105`, in `CurveGpu::new`). The submit is 89 to 506 ms; the share is **unknown**. The driver's cache hides most of it after the first build: the first submit of a process took 737 ms on the beech at 40.
- **Conversion** is 0.19 to 1.1 s. It is a port item, not a deletion, but decision 2 removes its seven geometry decisions (CONTRACT.md §1).

### What remains to port, after the deletions

| Step | Today (CPU) | On the device for fine roots |
|---|---|---|
| Fine nodes in the tree's frame: identity, parent, tip, radius, start radius | conversion (`space.rs:190-323`) | from the laid buffers plus each root's bearer record (CONTRACT.md §5) |
| Curve points: samples (girth easing, flare, socket), transported frames | `curve.rs:86-147` | one sweep per run; runs crossing the seam per host question 4 |
| Clusters, bounds, radius extrema, error ladder | `curve.rs:150-236` | per 32 points of a run |
| Whole-tree reductions | `input.rs:58-86`, `reference.rs:91-106`, `tree.rs:151`, `rings.rs:292` | folds plus the shoulder's front (≤ 4k pairs) to the CPU |
| Stations: bearing runs, frames, segments | `foliage/prepared.rs:129-229`, `plan.rs:270` | per bearing run |
| Placement | GPU already (`place.wgsl`, `pack.wgsl`) | without the cull |
| Contact rings | compact rings (`compact.rs`) | from curve points, seated species only |

## 3. Not covered

- **Ages 40, the oak and the palm** were not timed here; R3 reports them.
- **Steady-frame cost** (R5a) was not measured. GPU-PATH.md's 58.2 ms for the spruce at 80 stands until retaken.
- **The browser:** nothing here ran in Wasm.

## 4. The proposed R3 probe (for the host)

### What it builds

A native vertical probe on the GPU executor's device buffers. No per-node readback.

1. **Settle on the device.** `telperion_space::settle` with the GPU executor (`generation/fine/device.rs`), as `realized_tests.rs` drives it.
2. **Fine nodes.** For every fine root on the device:
   - nodes with botanical identities (decision 2, once built);
   - tree-frame tips from each root's bearer node record;
   - parent indices from a prefix over roots' kept counts;
   - start radius as the conversion defines it.
   - Ground-routed roots are handled per host question 3.
3. **Curve on the device.** Curve points, transported frames and clusters for every run inside the fine roots. Runs that cross the seam are handled per host question 4.
4. **The rest on the CPU.** The structure above the cut and the ground-routed roots go through today's CPU steps, conversion and curve.
5. **Reductions.** The whole-tree reductions are folded on the device and combined with the CPU's: height, crown base, widest, the shoulder's front, the leaf box, the stem and root radii. Only these summaries cross.
6. **Leaves.** Stations for every bearing run on the device, then placement without the cull. For the spruce, contact rings come from curve points at the station nodes.
7. **Resident.** The curve is handed to the renderer in its packed form without a CPU copy. A `submit_prepared` path takes device buffers.

### Measurement boundary

- **Start, t0:**
  - **probe:** `settle` has returned with the device's phase-4 bends complete (`Device::wait`, `device.rs:126-131`);
  - **reference:** the CPU executor's settled `Structure` is returned (`grow_fine`, then `lineage_order`).
  - Both trees are the same by lineage. The probe's geometry is within REALIZED.md's screen of the reference's.
- **End, t1:** draw-ready resident artifacts. That means `submit_prepared` has returned (curve, leaves, masses, element and bounds on the device), an empty queue submission has been made, and `device.poll(wait)` has returned. The first frame is excluded and reported apart, as are the steady frame's p50 and p95 (R5a).
- **Inside the boundary:**
  - every CPU step for the structure above the cut and the ground-routed roots;
  - the CPU halves of the reductions;
  - every dispatch, buffer allocation (pool cold and warm, reported apart), upload, totals readback and wait;
  - the curve's pipeline creation at submit, as today's code does it.
- **Outside the boundary:**
  - growth and settling;
  - `Generator::new`'s pipelines, built before t0 on both sides.
- **Method:**
  - this file's marks, the same probe skeleton, `--profile ci`, all cores, the GPU lock held, load logged per build;
  - five builds a side per tree, the sides interleaved in alternate processes, with cold and the median of the four warm reported.
  - This desk's ±25 % spread needs interleaving.
- **Gate:** the probe's median warm at most a third of the reference's, at **both** spruce-80 seeds (the host's proposed feasibility target). The beech at 80 and the ages-40 trees are reported, not gated. A miss ends the probe with its stage profile, the marks plus GPU timestamps.
- **Parity, spent once on the final output against the CPU reference build of the same tree:**
  - equal leaf counts;
  - leaves within fn-125's leaf bounds (`generation/tests.rs`);
  - curve points within 50 µm max and 10 µm RMS, and frames within 0.01 rad;
  - contacts: each seated leaf's point within the leaf bounds;
  - seams: a run crossing the cut continuous in position and frame within the same bounds.
- **Memory, recorded:**
  - peak RSS from t0;
  - device memory peak minus idle;
  - each buffer's size against `max_storage_buffer_binding_size` (2,147,483,644 here; the browser's default is 128 MiB, a figure for R4).

### The denominator: the host's decision

| Option | Denominator for the spruce at 80 | Note |
|---|---|---|
| A. Today's code (this file) | 17.3 to 19.5 s (pass 2) | The probe's deletions count toward the 3x. The CPU path stays as it is. |
| B. The CPU deletions first: (a) rings from the curve, (b) positions alone and none for unseated species, (c) the cull removed | estimated about 6 to 7 s | The estimate is today's steps minus the deleted ones, by arithmetic, not measured. Both executors gain. The 3x then has to come from the device port alone. |

Astra's review and `docs/principles.md` ("delete, then optimise") point to B. B moves today's CPU outputs only where (b) changes which path the spruce takes: its leaves would seat on the compact rings instead of the canonical float32 wood, and the bytes of that are **unknown**.

### Before the probe can start (CONTRACT.md, "For the host")

1. Decision 2 built, so the reference is frozen on botanical identities.
2. The ground-routed roots (question 3).
3. The run graph across the seam (question 4).
4. The branch's compile seam fixed (FRICTION.md).

## Files

| File | What |
|---|---|
| `baseline-probe.rs`, `baseline-run.py`, `baseline-table.py` | the baseline's probe (an example while run), driver and table |
| `baseline-marks.patch` | the temporary stage marks and the shoulder's candidate count |
| `shared-probe.rs` | rings against curve points, CPU only |
| `share-probe.patch` | the routing-share test (CONTRACT.md §2) |
| `build-fix.patch` | the one-line compile fix every run needed |
| `raw/` (ignored) | `pass1` to `pass3` (JSON lines, `nvidia-smi` samples, stderr), `shared-1.jsonl`, `share.txt` |
