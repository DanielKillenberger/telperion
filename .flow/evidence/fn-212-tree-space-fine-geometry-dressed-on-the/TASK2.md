# fn-212 task 2: delete first (worker, 2026-10-07)

The CPU deletions AUDIT.md found, taken under the host's decisions (CONTRACT.md, "Host decisions": option B, D6, the compile fix), each its own commit with its time before and after and the drawn output's digests. Then the new baseline by AUDIT.md's method.

## Answer in brief

- **The spruce at 80, seed 1, from the grown tree to resident: 16.0 to 18.3 s before, 6.2 s after** (median warm, interleaved). Its peak RSS fell from 5.6 to 4.7 GB, its GPU compute peak from 2,241 to 2,034 MiB.
- **The beech at 80, seed 1: 2.0 to 2.4 s before, 1.2 to 1.5 s after** (the quietest repetition).
- **The spruce at 80 now takes the GPU ring path** (`gpu_positions` true, no "compute limits"): the fix that gained most (8 s).
- **The drawn output is unchanged** at every deletion but one, at both ages, all four engine species, seeds 1 and 7: the pipeline's tree, wood, leaves, plan and structure, the CPU mesh, and the GPU executor's leaves, counts, bounds and curve at both deliveries.
  - The exception is the spruce at 80's seated leaves after the positions fix. It now seats on the GPU's ring positions, as every smaller seated tree already did. 9,551 of 16,912,892 leaves (0.06 %) moved by one position code step (one leaf by two). No rotation or scale moved, and the count and bounds are equal.
- **wasm32 grows engine trees in Node** (D6). The base build traps on the beech at 40 (`RuntimeError: unreachable`). With D6 the beech, spruce, oak and palm grow at 20 and at 40.
- **New baseline (option B), settled geometry to resident, AUDIT.md's method:**
  - spruce at 80: 8.1 to 8.2 s warm (8.4 and 8.2 s cold), against 17.3 to 19.5 s;
  - beech at 80: 1.8 s (seed 1) and 3.4 s (seed 7) warm, against 1.8 and 3.2 s. The beech's deletion shows in the interleaved timing, not across the two days' desks.
- **For the host:** `shellDepth` is now checked and read by nothing (below). Task 3's first decision, D1, collides with the ring path's float32 qualification; that is a design question (report).

## Method

- **States.** Each deletion is one commit. Each state's timing and digest probes were built at that commit:
  - s0 `7f0cade5`: the compile fix and D6;
  - s1 `1c3ccb93`: the cull;
  - s2 `f83e03f6`: no rings unseated;
  - s3 `471fb1d9`: the positions' sizes;
  - s4 `90c2cb8c`: rings from the curve (the timing probe at `884a0bb7`, the same render code).
- **Digests.** `generation_digest` with the age as a variable (`raw/task2/tools/build.py` derives it).
  - At 40: the beech, spruce, oak and palm, seeds 1 and 7. Compared fields: the pipeline's artifacts (tree, wood, leaves, plan, structure), the CPU mesh, and the GPU executor at both deliveries (instances, count, bounds, curve sizes, backend).
  - At 80: the beech and spruce, the GPU executor's fields only. The CPU reference build at 80 held the lock for minutes, and the CPU path is covered at 40.
- **Timing.** `raw/task2/tools/f212_time.rs`:
  - The boundary is the grown tree to resident: the generator's total less its skeleton stage, plus `submit_prepared` and the drain. This is AUDIT.md's boundary less the conversion, which no deletion touches.
  - One process per state and two builds (cold, warm). The five states ran back to back under one hold of the GPU lock, forward in the first repetition and reversed in the second.
  - The beech's first repetition ran at load 34 to 54 and is quoted only where it agrees with the second (load 13).
- **Baseline.** AUDIT.md's probe and marks (`baseline-probe.rs`, `baseline-marks.patch`), with the preparation's marks re-placed on the new code: "curve and rings with contacts" replaces "rings with contacts", and "positions emit" is unchanged. Two passes, three builds a tree.

## The deletions

### 1. The foliage cull's rejection, on both executors (`1c3ccb93`)

- **What went:**
  - on the GPU: the shell test, the alive flag, the workgroup scan, `ranks`, `raw`, the serial `prefix` pass, the `scatter` pass, and the survivor count's read before the output is sized;
  - on the CPU: `foliage::cull` left the pipeline for `foliage::admit`, the float32 overflow check alone;
  - `Metrics::compact_ms` and `input_instances`.
  - About 200 lines.
- **What stayed:** the bounds reduction and the error flags, which the mass grid, the crown and the framing read.
- **Removed nothing drawn:**
  - Placed leaves equal retained leaves for all four engine species at 20, 40 and 80, seeds 1 and 7, the palm's shell of 0.85 included (CPU path, measured).
  - Every digest is unchanged, at 40 and at 80.
- **Time:** no change beyond the desk's noise.
  - Spruce, repetition 1 (cold, warm): 14.8 and 20.1 s before, 15.1 and 15.1 s after.
  - Placement wait: 0.3 to 0.6 s on both sides.
  - The serial prefix pass cost less than the spread.
- **Memory:** the GPU compute peak fell 258 MiB (spruce, 2,241 to 1,983 MiB) and 38 MiB (beech).
- **Re-pin (`884a0bb7`):** the suite's scaffold silver birch is not an engine tree; the pipeline refuses it, so nothing draws it. Its pinned placement moved: the family's shell of 0.45 had dropped 47,978 of its 327,463 leaves. Its skeleton, bounds and element did not move, and no other pin moved.
- **Left for the host:** `shellDepth`. The row is checked where it was, so a request meets the same error, and nothing past the check reads it. Its catalogue text now says so. Whether the row goes (presets, dials, the browser's metadata) is the owner's call.

### 2. No ring positions where nothing seats (`f83e03f6`)

- **What changed:** the resident path builds compact rings and GPU positions only where `seats()`. The beech, oak and every unseated family place their stations as the CPU delivery always did. The overlap of the position upload with the stations went with its only caller, and so did the injected late-station test hook and its test.
- **Removed nothing drawn:** every digest is unchanged, the beech and oak resident at 40 and the beech at 80 included.
- **Time, beech at 80, repetition 1:** 2.26 and 2.38 s before, 1.17 and 1.28 s after (−1.1 s, about 48 %). The positions' preparation, 0.6 s, is gone.
- **Memory:** GPU compute peak 562 to 168 MiB.

### 3. Ring positions sized by what the pass binds (`471fb1d9`)

- **What changed:** the positions pass checked six buffer sizes against the binding limit but allocated two, the positions and their metadata. The ring mesh's normals, coordinates and indices are never built. The indices' 2.55 GB refused the spruce at 80.
- **Path:** the spruce at 80 now takes the compact path (`gpu_positions` true). It no longer sweeps the canonical float32 wood on the CPU, prepares stations twice, or discards the compact rings.
- **Drawn output:** unchanged everywhere except the spruce at 80's seated leaves (above).
  - 9,551 leaves moved by one code step of the leaf box, one by two.
  - fn-125's leaf bound is two steps plus 32 float32 epsilons (`generation/tests.rs`).
  - Counts, bounds, the curve and the CPU path are unchanged.
- **Time, spruce at 80:**
  - before: 15.5 / 16.4 s (repetition 0) and 18.0 / 15.6 s (repetition 1);
  - after: 8.2 / 7.7 s and 7.3 / 9.2 s;
  - −8 s, about 52 %.
  - Peak RSS 5.6 to 4.2 GB. The GPU compute peak is 1,983 to 2,034 MiB, the positions now resident during placement.

### 4. Compact rings read from the curve (`90c2cb8c`)

- **What changed:**
  - `surface::traced` returns the curve with its paths and each run's path.
  - `compact::prepare` and `prepare_with_contacts` read the rings and the node contacts from it; their own sweep is deleted.
  - `Expansion::curve_with_contacts` hands both from one sweep. The resident path keeps that curve for the drawn wood instead of sweeping it again after placement.
- **Removed nothing drawn:** every digest is unchanged, 40 and 80. The rings were the curve's points to the bit (AUDIT.md §2a).
- **Time, spruce at 80:** 7.3 to 9.2 s before, 5.9 to 6.6 s after (−1.5 to −2 s). The wood's own time (`wood_ms`) fell from 4.1 to 5.8 s to 2.7 to 3.3 s.
- **Memory cost:** peak RSS rose from 4.2 to 4.7 GB, because the curve (467 MB at the spruce) is now held through placement instead of built after it.

### 5. D6: wasm32 (`7f0cade5`), and the compile fix (`188cfc5e`)

- **Compile fix:** `Stages::add` assigned `Instant::now()` to an f64. The saved patch was applied as is.
- **D6:**
  - Where the plan has one thread, as on wasm32 always, each cycle's buds are sown as the cycle ends, on the calling thread.
  - Every timer reads `crate::clock::seconds`.
  - Phase 4's two halves and the spare roots' drop run in turn on wasm (`cores::beside`).
  - The realized suite's one-thread plan covers the calling-thread path natively against the whole-tree engine: 16 passed.
- **Proof in Node** (`wasm-grow.mjs`):
  - the base Wasm traps on the beech at 40;
  - D6's grows the beech (0.9 s), spruce (9.4 to 13.4 s), oak and palm at 40, status 0.
- **Not covered:** `settle` with the landing on still spawns (the reach and the spare cores). No Wasm path calls it before R4.

## The new baseline (option B)

AUDIT.md's boundary and marks, from settled geometry to resident (ms). The quietest warm build per tree; pass 1 for the spruce, pass 2 for the beech s1.

| Tree | Load | Convert | Envelope, inputs | Clothe, element, box | Curve and rings (CPU) | Stations (CPU) | Ring positions (GPU) | Placement (GPU) | Curve kept | Submit | Drain | **Total** | Peak RSS MB |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 80 s1 | 11.8 | 237 | 77 | 13 | – | 496 | – | 225 | 625 | 136 | 1 | **1,822** | 1,245 |
| beech 80 s7 | 18.5 | 483 | 156 | 24 | – | 968 | – | 389 | 1,108 | 261 | 23 | **3,442** | 1,976 |
| spruce 80 s1 | 13.5 | 965 | 262 | 39 | 2,665 | 2,145 | 495 | 934 | 24 | 526 | 36 | **8,151** | 4,738 |
| spruce 80 s7 | 12.5 | 967 | 254 | 34 | 2,553 | 2,062 | 470 | 1,017 | 29 | 589 | 42 | **8,089** | 4,697 |

- **The spruce's cold builds:** 8.4 s (s1) and 8.2 s (s7) in pass 1.
- **Device memory, peak minus idle:** spruce 3.3 GB, beech 0.75 to 1.0 GB.
- **R3's gate against this baseline:** a probe a third of it, about 2.7 s for the spruce at 80 warm.
- **Where the spruce's 8.1 s goes:** curve and rings 2.6 s, stations 2.1 s, conversion 1.0 s, placement 0.9 s. What remains to port is AUDIT.md's table less the deleted rows.

## Gate

- **Gate, in the host's two parts:** `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.
  - The render crate under the GPU lock: 160 passed, 0 failed.
  - The rest without it: 978 passed, 3 failed.
- **The three reds are not this task's:**
  - `generation_limits_are_classified`: fn-198's shoulder sites are missing from the limit inventory on this branch, before any commit of this task.
  - The two jev replay tests need the render crate's examples, which only the render half builds. Re-run after it: 3 passed.
- **npm test:** 13 files, 143 tests passed.
- FRICTION.md has each red.

## Files

| File | What |
|---|---|
| `wasm-grow.mjs` | D6's Node proof |
| `task2-tools/` | the timing probe, the digest builder, the session driver and the comparison |
| `raw/task2/` (ignored) | digests per state and age, timings per state and repetition, the baseline's passes, the gate's logs |
