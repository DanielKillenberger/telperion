# fn-212 task 10: the device path in production, then in the browser (worker, 2026-10-08)

## Part 1: native wiring (host decision 2; R2's gate)

### What was built

- **The fine module is production code.** `generation.rs` no longer gates `mod fine` behind `cfg(test)`. Natively, the dress, sweep, stations, rings, handoff, settle kernels and `resident::build` are compiled into the library. The tests and probes stay `cfg(test)`. Two modules were split:
  - `units.rs` keeps the table packer; the unit-kernel tests moved to `units_tests.rs`.
  - `stubs.rs` takes the seam stubs out of the probe's fixture.
- **Routing.** `Generator::prepare_async` with resident delivery takes `prepare_fine` (`fine.rs`) wherever `executor::apart(family)` is `Ok`. `prepare_fine` is `resident::build` inside the device's error scopes, keeping only the prepared tree. Every other request takes `prepare_whole`, the body that ran before.
- **The gap, named** (`executor::apart`, `telperion_space::apart`). This is a capability selection made before growth, never a fallback from a failure:

  | Family | Gap |
  |---|---|
  | No engine species (birch, Laurelin, Telperion, ordinary) | "no engine species" (refused by `grow`, as before) |
  | Oregon white oak | "its buds read light, which fn-211 carries to fine roots (F2b)" |
  | A species with retained pipes or leaf girth | "its shed pipes and leaf girth read the tree as grown (fn-211)" |
  | Date palm | "its leaves stand on no regular twig stations". The device path refused it with "the fine path needs regular stations"; the whole path draws its fronds with the CPU foliage fallback, as it did before. |

  `grow_marked` reads the same `telperion_space::apart`, so the CPU executor's partition and the device path select by one rule.
- **The kernels.** Each generator makes the kernels once, on its first tree that comes apart (`OnceCell`). Their pool keeps one build's buffers for the next build (fn-209 task 8). The test lock `whole_tree` is now test-only.
- **Dependency.** `telperion-render` depends on `telperion-space` (it implements `Roots` on the engine's tables); before, it was a dev-dependency.
- **Wasm32 in part 1.** The browser still takes `prepare_whole`: `mod fine` is gated off wasm32 until part 2 makes its readbacks async.

### Tests

- `route_tests.rs` (in the gate):
  - One generator builds the spruce at 30, the beech at 30 and 3, the spruce at 1, the oak, the palm and the beech at 12, in that order. The large tree comes before the small ones, as a page's dials do.
  - Each tree takes the path `apart` names, its leaf count equals the whole path's, and it is submitted.
  - A CPU delivery still grows the tree whole.
- `drawn_tests.rs` (ignored, one side a process): a parameter change to a drawn tree, and the steady frame.

### R2: drawn natively within tolerance (`raw/task10/`)

**Parity probe on the real path** (`real_path_at_80`, `parity-80.txt`):

| | Spruce 80 s1 | Beech 80 s1 | Beech 80 s7 | Beech 40 s1 |
|---|--:|--:|--:|--:|
| Leaves (device, CPU) | 16,870,695 / 16,870,693 | 2,474,227 / 2,474,225 | 4,294,851 / equal | 65,633 / equal |
| Over the bound | 0 | 0 | 0 | 0 |
| Left out (device run unmatched, count apart, reference run unmatched) | 0 / 332 / 0 | **16 / 0 / 14** | 0 | 0 |
| Runs matched / parted / missing | all / 0 / 0 | all / 0 / 0 | | |
| Placement flags | 0 | 0 | 0 | 0 |

- **Spruce 80 s1** is task 7's figure after task 8: the half-micrometre rounding class (332 leaves, 2 more on the device).
- **Beech 80 s1 has 30 leaves on runs the other side lacks:** 16 on the device's runs, 14 on the reference's. That is 1.2e-5 of its leaves, and they make its 2 extra leaves. No earlier task measured the real path on this tree: task 7 took the beech at 30, and TASK6's station-count class names one key on beech 80 s1. **Not traced: unclassified.**

**Stills through `Generator::prepare`,** hero camera, 1280 × 900, against the whole path's (`stills/`, all four viewed, `stills-diff.txt`):

| Tree | Exact pixels apart | Under 6 % fuzz |
|---|--:|--:|
| Spruce 80 s1 | 0.15 % | 0.03 % |
| Beech 80 s1 | 0.27 % | 0.07 % |

Indistinguishable by eye on both trees.

### R5a natively: build latency and steady frame, before and after (`drawn-*.txt`, `report-*.json`)

- **Method.** One process a build, both sides on the same binary:
  - before is `prepare_whole` (the CPU executor's path);
  - after is `prepare`.
- **Build latency** runs from the parameter change to the first still drawn and read back. A small tree is built first, so the generator, its pipelines and the device's kernels already exist, as on a page.
- **Frame** is `timing::run`'s p50 at the hero camera: 120 frames, every verdict valid.
- **Conditions.** Spruce passes ran at a load of 12 to 36; the beech at 7.

| Tree | Side | Prepare ms | Drawn ms | Peak RSS MB | Vegetation p50 ms | Shadow p50 ms | Wood surfacing p50 ms | Frame p50 ms |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| Spruce 80 s1 (4 runs a side) | before | 8,396 to 10,437 | 8,963 to 11,193 | 4,763 to 4,788 | 38.1 to 38.6 | 8.1 | 9.3 to 9.6 | 56.3 to 57.2 |
| | after | 1,453 to 2,349 | 1,592 to 2,570 | 766 to 800 | 48.5 to 48.9 | 10.7 | 9.2 to 10.2 | 69.2 to 70.7 |
| Beech 80 s1 | before | 1,731 | 2,003 | 1,145 | 9.39 | | 3.58 | 14.00 |
| | after | 373 | 550 | 414 | 9.29 | | 3.27 | 13.59 |

- **Build latency falls 5 to 6×** on the spruce at 80 and 4.6× on the beech. The process's peak memory falls 6× on the spruce, from 4.8 GB to 0.8 GB.
- **The spruce's steady frame rises 23 %:** vegetation +10.5 ms, shadow +2.6 ms. The instances, the levels (all at level 0), the casters and the selection (0.85 ms) are the same on both sides. What differs is the leaves' order in the buffer:
  - consecutive leaves stand 0.074 m apart on average on the device path, against 0.020 m before (p99 1.51 m against 0.29 m; `coherence-spruce80.txt`);
  - the beech shows the same spread (0.141 against 0.033 m) and no cost.

  The order is the likely cause, by correlation; it is not proven. The device's fine runs keep D5's order, widest first across every fine root, so each run's needles land far from the previous run's. **For the host** (below).

### Gate (part 1)

- **Render half,** under the GPU lock, first: 164 passed, 0 failed (`gate-render-1.txt`).
- **CPU half:** 988 passed, 0 failed (`gate-cpu-1.txt`).
- `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.

### For the host, after part 1

1. **The spruce's frame cost (+23 %) from leaf order.** Options:
   - (a) the device places its fine stations in their roots' order (bearer, then run) rather than D5's widest-first run order. Leaves would follow the tree, as on the CPU path. Placement and parity code would change; the curve would not.
   - (b) a device sort of the placed leaves by a spatial key after placement. It costs one pass of about 17M keys a build.
   - (c) accept it until fn-176's level of detail reorders leaves anyway.
2. **Beech 80 s1's 30 unclassified leaves.** Trace them now, or name them with the beech's other residual classes.

## Part 2: the device path in the browser (host decisions 1 and 3)

### What was built

- **`Roots` is async** (`telperion-space/src/fine/step.rs`, `lay.rs`):
  - Each phase returns a `Step` (a boxed future). The CPU reference returns ready futures (`ready`), and `grow_fine` drives them with `now` on any target.
  - `settle`, `executor::settle` and `space::settled` are `async`.
  - The target split is three places: the spare cores' thread scope, the planner's scope beside phase 2, and `beside` in phase 4. Natively the driver blocks inside them (`wait`, park-based, no dependency). On wasm it awaits, the reach rows are read in turn, and no spare cores grow ahead.
  - `Device` sows natively only. A browser cannot wait on its device inside the CPU grower's callback, so every root grows in `grow`: the same seeds in the same order.
- **Every readback on the device path is awaited and propagates its error:** the totals, records and living, the dress's folds and sifts, the sweep's and stations' counts, the handoff's sums, and the placement's summaries. `resident::build` and `prepare_fine` are `async`.
  - A device error is reported in place of the readback it broke, and the kernels are made inside the error scopes.
  - `Instant` is gone from production code; `Clock` serves both targets.
  - `io::read` (the blocking read) is test-only again.
- **Three things the browser refuses that native Vulkan lets pass**, all found there and fixed:
  1. One buffer bound at two bindings where one is written: the shed's `no buds`, and the ring vertices' `ring none`. `Pass::bind` now asserts it in tests, so the native suite catches the next one.
  2. A constant NaN bitcast (`sweep.wgsl:142`), which Tint refuses and naga folds. `shader_tests::dump_generation_shaders` writes every generation shader to disk, and `task10-browser/tint.mjs` compiles each one in headless Chromium and creates every compute entry point's pipeline. All 20 shaders now pass.
  3. **Buffers are never freed on drop:** wgpu 30's WebGPU backend `Drop` is a no-op (`backend/webgpu.rs:2978`), so a buffer lives until the JS GC collects its wrapper. Without the fix, the spruce at 80 failed "Out of Memory" in a fresh page.
     - Now destroyed when let go: `Held`, `CurveGpu` and the curve's `Target`s, the readback staging buffers, `Dressed`, `Swept`, `Stationed`, the pools' leftovers, each spent wave's buds, the group's non-pooled buffers, and the ring positions after placement.
     - The native suite holds every destroy (164 passed): a buffer destroyed before its last use would be a validation panic.

### Measured in headless Chromium (RTX 3080, Vulkan; fn-198's driver, extended)

The owner's Chrome was not usable. The first two builds there were lost to device errors (the aliasing and NaN errors above), and Chrome then refused WebGPU adapters to both `127.0.0.1:5212` and `localhost:5212` (FRICTION.md). The figures below come from fn-198's headless driver (`task10-browser/drive.mjs`, with a timing step added), under the GPU lock. Raw files are in `raw/task10/browser/`.

**Build latency, from the parameter change to the first drawn frame** (before is the base module `d22eb8a9`, served by the same harness):

| Step | Before ms | After ms |
|---|--:|--:|
| Beech 40, opening (seed 42) | 840 | 982 |
| Beech seed change at 40 | 536 | 561 |
| Beech age 40 to 80 (seed 2) | 5,718 | 1,442 |
| Spruce 40, opening | 4,698 | 1,550 to 1,808 |
| Spruce 80, opening | refused (`WASM_KEPT`, 334 ms) | **3,248** (17,436,547 leaves) |
| Spruce 80 to 40 (frames paused, below) | n/a | 2,544 |
| Spruce 40 to 70 | 13,650 | 14,242 (settle about 8.9 s of it, one core) |

**Steady frame** (harness timing, 120 frames, every verdict valid; vegetation / total p50 ms):

| Tree | Before | After |
|---|--:|--:|
| Beech 80 s2 | 12.44 / 19.13 | 12.55 / 19.30 |
| Spruce 40 | 11.86 / 18.29 | 11.88 to 12.02 / 18.32 to 18.50 |
| Spruce 70 | 31.88 / 50.89 | 31.98 / 49.91 |
| Spruce 80 s42 | n/a | 38.94 / 57.59 |

**Memory:**
- The largest headless Chromium process peaked at 4.9 GB before (spruce 70) and at 0.75 to 0.80 GB after, through the spruce at 70 and 80. The fine nodes never reach the Wasm heap (R3). The sampler's process labels are unreliable, so these are the largest process's figures.
- The page's GPU limits: `maxBufferSize` 4,294,967,292, `maxStorageBufferBindingSize` 2,147,483,644. The spruce at 80's largest buffer is 1.23 GB (ring positions), under both. `io::buffer` refuses by name any buffer over them (`RenderError::Oversize`).

**Screenshots viewed:** beech 12 and beech 80 (before and after, indistinguishable), spruce 40, spruce 70, spruce 80 (`browser/after*/`).

### Open: two browser findings for the host (pass criterion not yet met)

1. **The frame loop starves the build.**
   - While a build awaits its readbacks, the harness keeps drawing the previous tree every animation frame. Behind a spruce at 80 (about 57 ms of GPU a frame), the queue grows faster than it drains: the GPU sits at 100 % and the build never completes.
   - Measured: spruce 80 to 40 stalled for over 150 s, and 40 to 70 took 148 s before the buffer fix.
   - With the harness skipping frames while a build is in flight (an experiment, reverted), 80 to 40 to 12 to 80 to 1 to 80 completes. Natively there is no frame loop.
   - Options:
     - (a) the harness pauses drawing during a build: the old tree freezes for the build's 0.3 to 3 s;
     - (b) the renderer paces its frames, drawing only when the last frame's work is done (`onSubmittedWorkDone`): it stays interactive;
     - (c) both.
2. **Device memory between builds.**
   - With frames paused, a second spruce at 80 in one page failed "Out of Memory"; the third succeeded after GC. Some buffers are still let go without being destroyed: the 83 owned `wgpu::Buffer` fields are not all covered.
   - The two pools also keep about 2.6 GB between builds by design (fn-209 task 8: the grower's regions at about 1.4 GB, plus the packed and laid buffers).
   - Options:
     - (a) one destroy-on-drop buffer type for every generation buffer, a mechanical sweep of `io::buffer`'s owners;
     - (b) the pools released after each build on wasm, trading retune speed for memory;
     - (c) a device-memory estimate checked before the build, refusing by name. WebGPU exposes no free-memory query, so the estimate needs a stated budget.

### Other findings

- **`telperion-render.wasm` grows from 2,246,194 to 2,698,916 bytes (+452,722, +20 %)** with the fine module in production: 209 KB of WGSL plus the executor's code. The ceiling is 2,200,000, and the base is already over it on this branch. Raising it is an owner decision.
- **`npm test`'s catalogue check fails before vitest runs:** `catalogue/silver-birch/README.md` differs from its regeneration (placement hash and bounds), because `pins.json` was re-pinned in task 6 (`5b9a07a3`) and the README was not regenerated. Vitest run directly: 143 of 143 passed.

### Gate (part 2)

- **Render half,** under the GPU lock: 164 passed, 0 failed (`gate-render-2.txt`).
- **CPU half:** 988 passed, 0 failed (`gate-cpu-2.txt`).

## Host decisions on part 2 (2026-10-08)

Recorded in CONTRACT.md: frame pacing (b); one destroy-on-drop buffer type across every owner; fine stations placed in their roots' order; the beech's 30 leaves traced; the render module's ceiling raised to the measured size plus a margin (**for the PR's Decisions section**); the birch's catalogue page regenerated.

## Part 3: the host's decisions on part 2 (2026-10-08)

### Built

- **Frame pacing (decision 1).** `WebRenderer::frame` draws only once the last frame's work is done: an atomic flag set at draw and cleared by `on_submitted_work_done`. The harness is unchanged.
- **One buffer type (decision 2).** `buffer::Owned` owns every buffer the renderer allocates: the generation path (`io::buffer` and every helper), `Held`, the curve and its targets, scene, selection, shadow, timing, and the staging and readback buffers.
  - Dropping an `Owned` releases it to the `Gpu`, and `Gpu::submit` (every submit in the crate goes through it) destroys the released buffers after that submit. A pass may bind a buffer its owner let go before the encoder is submitted: the native suite showed it ("chain cfg has been destroyed"), and the destroy now waits for the next submit.
  - The manual `Drop` impls of part 2 are gone.
  - Two raw handles remain, both by ownership:
    - the grower's `Region`, which holds handles on the grower's own pool;
    - the leaf view's identity buffers (`select/bind.rs`), owned by the bind group that outlives the function.
- **Stations in their roots' order (decision 3).** `stations.wgsl` numbered bearing runs by a global `atomicAdd`, in no fixed order, so a run's leaves landed anywhere in the buffer. Now:
  - `owned` walks each run's first node up to its root's stub, and counts the run there with its rank among the root's runs;
  - the per-stub counts are scanned in node order (`Steps::scan`);
  - `starts` writes each run at its root's first run plus its rank.

  There is no sort pass. A root's runs, and so its leaves, are contiguous, with the roots in their stubs' order. Within a root the order is still the atomic rank's, as before.
- **Render ceiling raised (decision 4, for the PR's Decisions section):** `dist/telperion-render.wasm` measures 2,732,171 bytes; the ceiling goes from 2,200,000 to 2,800,000, with the reason recorded in `scripts/artifact-budgets.json`. (`telperion-field.wasm`, 923,937 against its 632,580 ceiling, was over before this task.)
- **`catalogue/silver-birch/README.md` regenerated (decision 5):** `npm test` is green.

### Spruce 80 frame cost after the roots' order (native, one binary, alternate processes; `drawn-spruce80-order.txt`)

| | Consecutive leaves apart, mean / p99 | Vegetation p50 ms | Frame p50 ms |
|---|--:|--:|--:|
| CPU executor (whole) | 0.0197 / 0.289 m | 30.78 | 47.16 |
| Device, before (atomic order) | 0.0739 / 1.507 m | 48.5 to 48.9 | 69.2 to 70.7 |
| Device, roots' order | **0.0156 / 0.069 m** | 33.72, 33.97 | **50.02, 50.33** |

- The device's leaves are now more coherent than the CPU's, and the frame gap fell from +23 % to **+6 %** (vegetation +9.5 %).
- What remains is not traced. Candidates: the random order within a root, and the CPU block placed before the device block.
- The whole path's own frame fell from about 57 ms to 47 ms between part 1 and now, on a quiet desk. Only same-binary rows are compared.

### The beech's 30 leaves, traced (decision 3)

They are one run on beech 80 s1, near (-0.65, 8.73, -2.05):
- the device's run starts one node earlier (key 159487817, node 10706: 16 leaves);
- the CPU's starts one node later (key 413959751, node 10707: 14 leaves).

Node 10706's start radius is 8.896716697e-3. The tree's slender bound (`bearing_bounds()[1]`, stem pipe × shoot radius) is 8.896716406e-3:
- 2.9e-10 apart, under float32's step there (about 9.3e-10);
- in f64 the node does not bear (CPU), and in f32 its girth and the bound compare equal, so it bears (device).

This is the f32-against-f64 threshold, here on the slender bearing bound, and it accounts for the 2 extra leaves. TRACE output in `placed_tests`/`resident_tests` (behind `TRACE=1`) prints such runs.

For the host: the slender bound could be decided as SHORTEST was in task 8, in a form both executors compute alike (for example both in f32, or the girth in whole nanometres), so the class disappears.

### Browser (headless Chromium, under the GPU lock; `raw/task10/browser/paced/`, `owned/`)

- **Starvation is gone with pacing alone:**
  - no build hung;
  - spruce 80 to 40: 1,576 to 1,692 ms;
  - 40 to 12: 560 to 575 ms;
  - 12 to 80: 2,295 to 2,377 ms;
  - 1 to 80, after a failed 80: failed, the same OOM pattern as the next item.

  Option (a) was not needed.
- **OOM persists.** The spruce at 80 builds in a fresh page (3,234 to 3,330 ms). Another 80, by seed or age, built while an 80 or a 40 is drawn fails "Out of Memory", a stated error. After 12 or 1 it builds.
  - One failure also broke the renderer's next frame ("Invalid BindGroup measure"), and the timing session after it read "disjoint".
- **Device memory (nvidia-smi, the whole card):**
  - 1.26 GB before the page;
  - 7.9 GB held while the first spruce 80 is drawn, about 6.6 GB of it the page's;
  - 9.18 GB at the second build's peak, which failed.
- **Pool sizes kept between builds (native, `large_to_small`, which builds 80 s1, 80 s7, 80 s42, 40, 12, 80, 1, 80 in one process, all succeeding):**

  | After | Kernels' pool | Grower's regions | Submitted tree |
  |---|--:|--:|--:|
  | Spruce 80 | 1,126 to 1,740 MB | 1,319 MB | 721 to 738 MB |
  | Spruce 40 or 12 | 18 to 20 MB | 1,319 MB | 325 to 431 MB |

  The grower's regions never shrink. The kernels' pool holds the last build's packed, scratch and laid buffers.
- **For the host (the budget):** about 3 GB of pools stays on the device beside a drawn 80 (0.7 GB of tree plus its curve targets). In Chrome that leaves no room for a second build of the 80's peak on this 10 GB card. Natively the same sequence fits.

### Gate (part 3)

- **Render half,** under the GPU lock, first: 164 passed, 0 failed (`gate-render-3.txt`).
- **CPU half:** 988 passed, 0 failed (`gate-cpu-3.txt`).
- **`npm test`:** the catalogue check passes; vitest 143 of 143.

The acceptance's "large-to-small rebuilds survive" does not hold in the browser until the budget is decided. Task 10 stays open.
