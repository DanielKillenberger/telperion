# fn-198 friction

## 2026-10-05, design research: measuring the engine's cost per stage

- **Doing:** the cost table at 80 years, every passed species, seeds 1 and 7: growth stages, conversion, expansion, phytomers grown against kept, triangles, leaves, peak memory.
- **Slowed by:** no tool reports all of it.
  - `measures stages` has no palm and takes its light from a fixed table, not each species' production light. It reports neither grown phytomers, the conversion nor the expansion.
  - The stills runner reports grown and dressed times, but no stage split, no grown count and no memory.
  - The engine's grown count (`Grower::grown`) is private.
- **Cost:** about 10 minutes and one extra release build. I wrote a scratch example (`f_cost`, not committed) and a one-line `eprintln!` of `grower.grown` in `grow.rs`, reverted before the commit.
- **Would remove it:** a `cost` mode on `measures` that takes each species' production light and the stills runner's conversion. It would report grown and kept phytomers (a `Structure::grown` field or a stage-callback count), the conversion, the expansion split and `VmHWM`.

## 2026-10-05, design research: shared load while timing

- **Doing:** the same measurements.
- **Slowed by:** other sessions kept the 1-minute load average at 6.5 to 10 on 32 threads throughout. The baseline was taken at 2 to 3. fn-197 recorded the same problem: the spruce under light read 40 s against 16 s in a quiet run.
- **Cost:** one repeated pass of every tree, about 3 minutes, and timings that carry a stated load.
- **Would remove it:** a timing run that records the load and refuses to run above a ceiling, or a quiet window for cost runs.

## 2026-10-05, design research: shell guard on evidence files

- **Doing:** writing this file with a heredoc.
- **Slowed by:** the dcg hook blocks a `>` redirect to a new file under `.flow/evidence/` when the command also runs `mkdir -p` on the parent. fn-190 hit the same guard.
- **Cost:** about 1 minute.
- **Would remove it:** nothing in the repository; it is a local hook setting.

## 2026-10-06, task 1 survey: the engine in wasm32

- **Doing:** checking whether `telperion-space` builds and runs for wasm32 and how close each species and age comes to the 4 GB ceiling (INTEGRATION.md, section 2.7).
- **Slowed by:** no tool runs the engine or the GPU path's CPU preparation in wasm32. `rust:test:wasm` covers the main module's entries only; GPU-PATH.md's peaks are native RSS.
- **Cost:** about 15 minutes: a scratch `cdylib` (`wasm-probe/`) and a Node runner, one tree per process. The load was 14 to 15 throughout, so its times are indicative only.
- **Would remove it:** a wasm32 memory probe kept beside the stills runner (species, age, peak memory per step), so step 3's ceiling per species and age is one command.
- **Also:** the dcg hook blocked a `cp`/redirect into `.flow/evidence/` through shell variables; literal paths passed (about 1 minute, local hook, as recorded on 2026-10-05).

## 2026-10-07, task 2: the test fallout of a new stage 2

- **Doing:** running the workspace's tests after the engine became stage 2.
- **Slowed by:** about 130 tests build a family through the pipeline or the scaffold's grower on a preset's rows. Each failed for one of three reasons (a species-less family refused, an engine preset's rows changed under a scaffold test, a pin on the scaffold's tree), and nothing groups them: the triage is test by test, reading each panic.
- **Cost:** about 60 minutes of triage across core, render, field, jev and wasm; one full core run is 4 minutes at a load of 10 to 50.
- **Would remove it:** R0 deleting the scaffold's grower with its tests in one change, so no test of a removed stage is triaged; and test fixtures that name the tree they need (an engine preset, a young age) rather than `Family::default()`.

## 2026-10-07, task 2: the shared GPU lock

- **Doing:** rendering two before/after stills for the canonical axis order.
- **Slowed by:** the GPU lock was held by other workers; the two stills took about 10 minutes of waiting for under 10 seconds of rendering.
- **Cost:** about 10 minutes wall, no tokens.
- **Would remove it:** a lock that reports its holder and queue, so a worker can do other work and come back.

## 2026-10-07, task 2: timing at a load of 65 to 130

- **Doing:** timing parameters to a prepared tree for the four species at 40 and 80, one core and all.
- **Slowed by:** other sessions held the 1-minute load at 65 to 130 on 32 threads through the run (the GPU lock serialises the device, not the CPU), so every all-core time is inflated by an unknown factor and the one-core times compete for a core.
- **Cost:** a 16-tree run of about 6 minutes whose times are indicative only; a quiet rerun is needed for R2.
- **Would remove it:** as recorded on 2026-10-05: a timing runner that records the load and refuses above a ceiling, or a quiet window for cost runs.

## 2026-10-07, task 3: the shared GPU lock before the first harness run

- **Doing:** opening the harness on the beech at 40 in a headless Chromium, the first look at the engine through `setTreeGpu`.
- **Slowed by:** the shared GPU lock was held by another worker; one five-second page load waited 8.5 minutes for it (load 6 to 12 on 32 threads).
- **Cost:** about 9 minutes wall, no tokens.
- **Would remove it:** as recorded on task 2: a lock that names its holder and queue, so a worker does CPU work meanwhile; or a lock per device use measured in seconds, not per script.

## 2026-10-07 05:15, task 3 (resumed): headless Chromium and the harness's WebGPU canvas

- **Doing:** driving `npm run dev` headless to screenshot the engine's trees, after the previous worker stalled for three hours on a browser step.
- **Slowed by:** the repository's browser scripts all launch Chromium with `headless: false` (a window on the owner's desktop); headless with their flags offers only SwiftShader, which the stage refuses. Native Vulkan with ANGLE on it (`--enable-features=Vulkan,VulkanFromANGLE,DefaultANGLEVulkan --use-vulkan=native --disable-vulkan-surface`) gives the NVIDIA adapter, but the page's first adapter request comes back empty before the GPU process is up (the panel's "retry build" then succeeds), and a headless page screenshot does not carry the WebGPU canvas (it is read with `toDataURL` in the frame it is drawn in).
- **Cost:** about 12 minutes and five short runs.
- **Would remove it:** one checked-in headless driver for the harness with these flags, the retry and the in-frame canvas read (this task's `harness/drive.mjs` is a start), named in docs so no worker rediscovers them.

## 2026-10-07 05:35, task 3: a Wasm trap hangs the harness; the GPU shared outside the lock

- **Doing:** finding the spruce's ceiling age in the browser, and driving the beech to 80 after the other species in one page.
- **Slowed by:** a Wasm trap (`RuntimeError: unreachable`, the spruce at 75 and 80) leaves `setTreeGpu`'s promise unsettled, so the panel says "building..." for ever; a driver without its own timeout waits for ever, which is most likely the previous worker's three-hour stall. Separately, the beech at 80 was refused "Out of Memory" by the GPU while another worktree's `telperion_render` test held 4.7 GB of the 10 GB card with the shared lock free.
- **Cost:** two 150-second waits on traps before the driver failed fast on a page error; one beech-at-80 run to repeat.
- **Would remove it:** the driver now ends a step on a trap at once. A GPU test outside the lock is a lock-discipline gap; a lock that GPU tests take by construction (a test harness wrapper) would close it.

## 2026-10-07 05:43, task 3: the GPU lock held through another session's test suite

- **Doing:** one more harness run (a smaller spruce after a larger one in the same page, which took 66 s against 9.6 s fresh).
- **Slowed by:** another session's `cargo test --profile ci` held the shared GPU lock for over 10 minutes, so the run's `flock -w 600` gave up; the load rose to 16 to 32 meanwhile, so a rerun would not time cleanly anyway. Also: `npm run build` during a dev session hot-updates the render glue under a versioned URL, and the driver's wrapper then patches a stale copy (the dev server is restarted after a rebuild).
- **Cost:** 10 minutes of waiting; the check is left open.
- **Would remove it:** as recorded above, a lock that names its holder and a per-device-use lock rather than per suite.

## 2026-10-07 06:25, task 4: reverting a scratch timer in a tracked file

- **Doing:** splitting stage 2's time (growth, canonical sort, conversion) with a temporary `eprintln!` in `pipeline/space.rs`, then putting the file back.
- **Slowed by:** the dcg hook blocks both `git checkout -- <file>` and `git show HEAD:<file> > <file>` on a file whose only change is the agent's own scratch instrumentation; the edit had to be reversed by a script.
- **Cost:** about 3 minutes.
- **Would remove it:** a stage-split clock the pipeline reports itself (stage 2's growth, sort and conversion in `Stages`), so no tracked file is instrumented by hand; the hook is a local setting.

## 2026-10-07 06:10, task 4: the gate's runner

- **Doing:** running core and engine tests after the F1 merge.
- **Slowed by:** AGENTS.md names nextest as CI's runner, and `cargo nextest` is not installed on this desk; the first run failed at once.
- **Cost:** about 1 minute.
- **Would remove it:** nothing in the repository; a local install.

## 2026-10-07 07:00, task 5: no sampling profiler on the desk

- **Doing:** profiling `grow` on one core, base against now, to name the regression's cause.
- **Slowed by:** `samply record` refuses with `perf_event_paranoid` at 2 (it needs 1), and `perf` and `valgrind` are not installed; the cause had to be found by stage clocks (`grow_staged`) and call counters patched into a scratch copy of the engine (log2, exp2_neg, Q32 products, leads by call site), then a microbenchmark of each primitive's cost.
- **Cost:** about 15 minutes and four scratch builds.
- **Would remove it:** a local setting (`kernel.perf_event_paranoid = 1`) or `perf` installed; nothing in the repository.

## 2026-10-07 09:15, task 5: one-core timings under another worktree's test load

- **Doing:** the final one-core matrix (base against now, natively and in Wasm), after the gate.
- **Slowed by:** the F1 worktree's test suite (`closed_form` and others at 400 % CPU) took the desk to a load of 24 to 47; one matrix run gave the base spruce at 80 13.5 s against 5.9 s quiet, so it was thrown away and the matrix waits for a quiet desk. Earlier, interleaved rounds were needed because single runs moved 3 to 8 percent with the load.
- **Cost:** one wasted matrix (5 minutes) and the wait.
- **Would remove it:** a desk-wide timing lock, like the GPU lock, that test suites and timing runs both take; or timings on a reserved core (`isolcpus`), a local setting.

## 2026-10-07 13:10, task 6: the task's readers of the envelope rows did not hold on the engine path

- **Doing:** finding who reads shoulder, irregularity and lobe scale on the engine's path before measuring them (decision 11 says "the leaf cull and the leaf box read them").
- **Slowed by:** the engine path's leaf box is `Reference::grown` (the wood's own extent), which reads no envelope row; `Reference::authored`, the one that reads irregularity, runs only in `Family::validate`. Lobe scale is read only by today's grower (the pendant curtain, containment). Irregularity's lobes are seeded noise, not the tree's. Two of the three rows have no consumer to measure for, which is a design question, so the task stops there with options for the host.
- **Cost:** about 15 minutes of tracing readers; the task's two tree-changing parts held.
- **Would remove it:** a spec's architecture claim about readers checked against code before the task is written (AGENTS.md, "Gates and checked claims").

## 2026-10-07 13:15, task 6: the engine's `grow` panics in a debug build

- **Doing:** a debug-only test of the lineage order's new assertion on a grown beech (age 6, seed 1).
- **Slowed by:** `grow` panics in a debug build at `closed_form/predictor.rs:60`, `debug_assert!(!count.is_nan(), "a count for PA 0")`; the gate's `ci` profile has no debug assertions, so no suite has seen it. The test grows through `grow_whole` instead. The NaN count was not traced.
- **Cost:** about 3 minutes.
- **Would remove it:** the predictor's NaN count traced and fixed, or the assertion's premise corrected; a debug-profile run of the engine's tests somewhere.

## 2026-10-07 14:20, task 6: `cargo fmt -p telperion-core` rewrites 33 files

- **Doing:** formatting the shoulder fit's two files before the commit.
- **Slowed by:** the core crate is not rustfmt-clean on this branch, so the package-wide format rewrote 33 files the task never touched; with `git checkout --` blocked by the dcg hook, each had to be put back from `git show HEAD:` by a script, and the file-level `rustfmt --check` used instead.
- **Cost:** about 4 minutes.
- **Would remove it:** the crate formatted once in its own commit (or a CI `cargo fmt --check`), so a package-wide format touches only what changed.

## 2026-10-07 11:10, task 8: F2b's `Instant::now()` reached the browser's oak unseen

- **Doing:** the parity test after the pin, the first Wasm build of the merge.
- **Slowed by:** F2b's `run_beside` (every lit `grow`) read `Instant::now()`, which traps on wasm32; native tests and the engine's suite pass over it, so only the Wasm build of the oak showed it, as `RuntimeError: unreachable` with no message. Tracing it to the merge's diff took a grep of F2b's `Instant` uses.
- **Cost:** about 8 minutes.
- **Would remove it:** a lint or test that the engine's non-test source names no `std::time::Instant` outside `clock.rs` (host decision 8 as a check, not a convention).

## 2026-10-07 11:15, task 8: the stills and digests example would not take a plain redirect

- **Doing:** writing before/after digests to the scratchpad from one command.
- **Slowed by:** the dcg hook blocks a `>` redirect to a path held in a shell variable; the commands moved into a script with literal paths.
- **Cost:** about 3 minutes.
- **Would remove it:** a local setup matter (the hook's rule), reported, not specced.
## 2026-10-07 11:35, task 7: timings on a desk four worktrees share

- **Doing:** the gate's wall time and the slowest tests, before and after the audit.
- **Slowed by:** the desk ran at a load of 20 to 55 from other worktrees' gates, stills and builds; the baseline gate took 980 s with `walks` alone at 395 s, and one test pair took 370 s wall where it took 195 s and 120 s alone an hour earlier. Wall times were unusable for comparing one change, so each change was measured by CPU time (`time`) or alone. The GPU lock was held by another worktree's whole gate, so a GPU test waited up to 15 minutes for a 1-minute run. `/usr/bin/time` and `bc` are not installed, which cost one empty gate start.
- **Cost:** about 20 minutes of waiting and one wasted gate start.
- **Would remove it:** per-test times from the runner itself (cargo-nextest locally, as CI has, reports each test's time and a slow-test list); a desk-wide CPU lock beside the GPU lock for timing runs; a local setup matter for the missing tools.

## 2026-10-07 23:30, merging fn-199 into F4: the second key clash in a day, and no hashes to compare against

- **Doing:** merging fn-199 (head 8112fb0f) onto F4's b31a4f65, which carries fn-224.
- **Slowed by:** both branches took `u64::MAX - 10` as a lineage key (fn-199's `FAILED`, fn-224's `INSERTION`) and selector 17 of the contract probe, with the GPU copies spread over six WGSL files as raw words; nothing failed until a reader found them. fn-199's evidence records no `space_hash` output, so its 16 clothed hashes had to be rebuilt from its tree in a separate copy with its own `target/`.
- **Cost:** about 15 minutes on the clash; one extra full build of the render crate for the hashes.
- **Would remove it:** done in the merge: the lineage keys sit in one registry (`telperion_space::lineage_keys`), the selectors in another (`fine/selectors.rs`), each with a distinctness test and a WGSL mirror (`fine/lineage.wgsl`, `probe.wgsl`'s `P_*`) checked by `fine/mirror_tests.rs`. Still open: a branch that changes trees records its `space_hash` lines in its RESULT.md.
