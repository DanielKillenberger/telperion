# Growth in half the time

## Conversation Evidence

> user: "In the end this needs to be a high performing pipeline at every stage of it to be able to achieve the performance goals we have."
> user: "Faster growth matters for everything. We need to get these generation speeds down that is our mission."
> assistant: "So growth is now the biggest single piece of the drawn tree, roughly half. [...] So the spec should start with a profile of growth, then fix what it shows."
> user: "ok go"

## Goal & Context

<!-- Goal & Context: 30% [user], 70% [inferred] from fn-91 evidence -->

Since fn-91, the browser draws a mature tree through `setTreeGpu` in 158 to 181 ms for the oak and 164 to 179 ms for the spruce (fn-91 final result, seeds 1 and 7). Leaves now expand on the GPU in milliseconds. What is left is CPU work before the GPU, and the largest part is growth: browser skeleton medians of 73.3 and 81.7 ms for the oak and 57.9 and 53.4 ms for the spruce (fn-91 `cpu-profile/REPORT.md`). Growth also bounds every other consumer: the slim homepage package (289 to 649 ms, fn-101) and the CPU build. [inferred]

fn-91's native profile on a Ryzen 9 5950X (same report) splits oak growth as follows: local advance 50.6 and 56.2 ms, of which whole `Planner::run` calls are an estimated 23 ms (heading and turn limiting, envelope admission, run storage, and a 40-step bisection clip of about 4.6 ms); scaffold advance 3.8 and 4.7 ms; radius solves and remap under 5 ms together. About half of local advance, some 28 ms on the oak, sits outside the planner and is not attributed. Three growth candidates in fn-91 (radius-only ordering, inactive-bias specialisation, prepared envelope bounds) were screened and reverted for missing their targets. [inferred]

The owner states the mission as generation speed at every stage. This spec makes growth, stage 2 of fn-102's pipeline, substantially cheaper. [user]

## Architecture & Data Models

The work is profile first, then fix in ranked order. [inferred]

1. **Attribute the whole of local advance.** Extend fn-91's instrumentation (`cpu-profile/`) to cover `branching/local/advance.rs` outside `Planner::run`, and split the planner's heading, admission and storage. No candidate is chosen before the remaining 28 ms is named.
2. **Candidates the code already suggests, each screened against the profile:**
   - Planned runs in parallel on native. `Planner::run` takes `&self` and one `Axis` and draws from `Rng::new(seed ^ key)`, keyed by axis. If a run reads no state another run in the same step writes, the step's runs can be planned concurrently and joined in axis order with identical output. Whether that holds is unchecked.
   - The attractor grid in scaffold advance. `scaffold.rs` `pull` and `consume` scan every attractor per step; `colonization/grid.rs` `AttractorGrid` exists and is used only by `colonize`. Scaffold advance is 4 to 9 ms, so this is a small candidate. Byte identity requires visiting candidates in index order, because the heading is a sum.
   - Wasm SIMD. The browser build sets no `simd128` target feature (`scripts/build-wasm.mjs`, no `.cargo/config.toml`). One build with it enabled and one measurement of the whole `setTreeGpu` path decide whether it stays.
3. Byte-identical output is preferred. A candidate that changes bytes is allowed under the 2026-09-20 policy when it is measured faster and shows no perceptible visual regression. [user, CLAUDE.md]

## Edge Cases & Constraints

- The growth path (fn-11, fn-30) shares the local advance code. It stays buildable and pinned; a change to shared code keeps its tests green. [inferred]
- Parallel planning runs only on native targets with threads, and the same code runs serially in Wasm with identical output (fn-102's "correct serially" rule). [user]
- Wasm SIMD must not change output bytes unless the change passes the visual and correctness checks above. [inferred]

## Acceptance Criteria

- **R1:** A committed profile attributes at least 90% of oak and spruce skeleton time at seeds 1 and 7, native and browser, including local advance outside the planner. [inferred]
- **R2:** Browser skeleton medians of five runs, for the oak and spruce at seeds 1 and 7, are recorded on base and candidate beside `setTreeGpu` completed-frame medians. The target is half of the base skeleton median. It is a feasibility gate: if the ranked candidates miss it, the worker records the profile and stops with `NEEDS_HUMAN`. [inferred]
- **R3:** Each shipped candidate states whether output is byte-identical. A byte change carries the visual comparison and correctness evidence the 2026-09-20 policy requires. [user]
- **R4:** Native CPU build time and peak memory are recorded for base and candidate; no stage of `examples/generation_stages.rs` gets slower by more than 3%. [inferred]
- **R5:** Wasm SIMD is measured once on the full `setTreeGpu` path and either kept with its gain or dropped with its number. [inferred]

## Boundaries

- Stage 3 preparation (compact surface, stations, descriptors; 36 to 44 ms and 20 to 25 ms in the browser) belongs to the GPU expansion contract spec, which reshapes it. [inferred]
- No change to what the tree looks like beyond what R3 allows. [user]
- No full-forest capture. [CLAUDE.md]

## Decision Context

- Depends on fn-102, so growth has one place to change and one caller shape. [inferred]
- fn-91's rejected candidates stay rejected unless the new profile shows why they would now pass. [inferred]
- Captured 2026-09-23 from the fn-102 review (items D and F). [inferred]

## Resolved before ready (host, 2026-09-24)

- The parallel-planning premise, read on origin/master `123c4261`: `Planner::run` (`branching/local/planner.rs:56`) is pure. It takes `&self` and one `Axis`, reads no `Tree`, and draws only from `Rng::new(seed ^ key)`; its other inputs (`GrowthConfig`, `TwigParams`, `GrowthBias::apply(&self, ..)`, `Curtain::admits`/`sagged` by value) are immutable. The loop that calls it is not: `Frontier::advance` (`branching/local/advance.rs`) pops shoots in queue order, assigns node ids in birth order, spends a shared budget, stops at `max_nodes`, and can visit a shoot born earlier in the same call when `visits` exceeds the starting queue. So runs can only be planned in parallel speculatively: compute the `Axis` of every shoot in a queue snapshot that will start a run, plan those runs concurrently, then let the unchanged sequential loop consume them in order, planning inline any run the snapshot did not foresee. Output stays byte-identical because each run is a pure function of its axis. Two mechanical constraints: `Planner` holds `WidthQuery = Option<&dyn Fn(&Tree, usize) -> [f64; 3]>`, which is not `Sync`, and runs are cached as `Rc<Run>`, which is not `Send`. Speculative work is wasted only where the loop stops early (budget, cap, separation rejection after planning). The planner is an estimated 23 of the oak's 51 to 56 ms local advance, so this is worth at most about 20 ms native before R1 names the other 28 ms. [inferred, read 2026-09-24]
