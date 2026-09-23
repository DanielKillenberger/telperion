# One build pipeline in place of the copied chains

## Conversation Evidence

> user: "should we think about how to simplify this design in core and telperion over all facets? Make it really a clean building pipeline with set stages that all coherently make sense as part of the whole and also what consumes them."
> user: "Can you illustrate current structure of the code base with every relevant branch and show the new structure that we'll be working towards with a clean pipeline design"
> user: "this pipeline restructure's performance must be faster or at worst equal to current performance. Is there a way this streamlining can actually improve performance?"
> user: "do we need the cpu chain for leaves? beacuse if you're going to render millions of leaves you'll need a gpu anyway to not stutter like crazy..?"
> assistant: "fn-102: shrink it to pipeline consolidation only. One function replaces the three copied chains, output stays byte-identical, and no algorithm changes."
> user: "ok makes sense so we just simplify cut the engine in 102?"

Review of 2026-09-23 (notes in `.flow/evidence/fn-102-one-build-pipeline-in-place-of-the/REVIEW-2026-09-23.md`):

> user: "In the end this needs to be a high performing pipeline at every stage of it to be able to achieve the performance goals we have."
> user: "well yes the pipeline should create output needed for other stages exactly once that's the whole point"
> user: "yes and that should also be a principle of the pipeline. We build stages that later stages depend on only once. But if we have depending stages that are independent of eachother we run them in parallel."

## Goal & Context

<!-- Goal & Context: 40% [paraphrase], 60% [inferred] code reading -->

The core's mature build recipe is written out more than once. The direct mesh build (`mesh::build`) and the Wasm binding (`telperion-wasm/src/generate.rs`) each spell out growth, wood surface, leaf placement and cull in the same order, and a test exists only to hold the two copies equal. The binding chooses stages with a row of flags. The renderer's GPU preparation and the specimen view each call some of the same stages on their own.

The copies have already diverged. `mesh::grow` runs `branching::clear_apical_twigs` and `branching::clothe_leaf_bases` after growth; the binding calls `branching::generate` and skips both, so a family with rosette fronds or leaf bases builds differently in the browser. The equality test covers only the oak and the spruce and did not see it. The date palm is `IN_WORK` and not served by name today, so the defect is latent until fn-82 ships it. [inferred, read on origin/master `6aab6192`, 2026-09-23]

The owner wants one pipeline with set stages, each coherent as part of the whole, with consumers reading what stages produce, and the pipeline itself efficient: each artifact a later stage needs is built once, and stages independent of each other run in parallel. This spec does the structural half: one place that names the stages, their artifacts, their order and their scheduling. Every algorithm and every random draw stays as it is. [user]

## Architecture & Data Models

The pipeline names five stages, each producing one artifact. [paraphrase]

1. Family: the parameter table from a preset, a blend or a request.
2. Skeleton: the solved tree from the direct build's growth, including the post-growth steps `mesh::grow` runs today (apical twig clearing and leaf-base clothing).
3. Plan: what is prepared from the skeleton before any triangle or leaf exists, as far as today's code already prepares it, plus the leaf element and, for a family with surface contact, the wood's ring sweep.
4. Outputs, each optional: the wood surface, the leaves, the occupancy field and the structure export.
5. Tree mesh: the wood surface and leaves with their union bounds.

One pipeline function in the core takes the set of wanted artifacts and runs each needed stage, calling the stage functions that exist today. The direct mesh build and the Wasm binding both become callers of it. [paraphrase]

Three scheduling principles bind the function (owner, 2026-09-23): [user]

- **Once.** Every artifact a later stage needs is produced exactly once per request and read by every stage that needs it. An artifact nobody asked for, directly or through a dependent, is never built and never held.
- **Parallel where independent.** The stages form a dependency graph. A stage runs when its inputs are ready, and stages that do not depend on each other run concurrently where the target has threads. On native targets the wood surface and the leaves run concurrently when both are wanted.
- **Correct serially.** The same graph runs one stage at a time where there are no threads (Wasm today). Output never depends on scheduling: each stage keeps its own random stream, results join in a fixed order, and when several stages fail the earliest stage's error is returned.

The shared ring sweep: `foliage::place_on_surface` builds a `surface::attachment::AttachmentSurface` when `surface_contact > 0`, which repeats `surface::build`'s sweep (`sample_path`, `frames`, the angular profile) and rounds each ring point to the float32 vertex `build` submits. Among shipped presets only the Norway spruce sets contact (`presets.rs:243`, contact 1.0; the default is 0). The rings become a Plan artifact that the wood surface and the leaf placement both read, so the wood and the leaves can still run concurrently. How the sweep is split, and whether the Linux worker path in `surface/parallel.rs` computes the ring artifact or only the emission, is the implementer's call under R4. [inferred, checked against the code 2026-09-23]

The specimen view is not a copy of the mature recipe: it grows through the timeline and shares only `surface::build`, `foliage::build_element`, `foliage::cull` and `mesh::union`. The renderer's `prepare_async` already calls `mesh::grow` for the Skeleton stage and `mesh::assemble` for its full CPU fallback; its GPU path places and culls leaves in shaders and calls neither `place_on_surface` nor `cull`. Both change only where they already run the identical stage. [inferred, checked 2026-09-23]

## Edge Cases & Constraints

- A request for an output whose stage fails returns the same error as today; when two concurrent stages fail, the earlier stage's error in the order Skeleton, Plan, wood, leaves, field, structure is returned. [inferred]
- The binding's timing metadata keeps its per-stage fields and meanings; under concurrency each stage's time is its own wall time and `coreMs` the whole request. [inferred]
- The growth path stays buildable and pinned and is not routed through anything new. [inferred]
- A family with zero surface contact builds no ring sweep for its leaves, as today. [inferred]

## Acceptance Criteria

- **R1:** The core has one pipeline function that names the stages and their order. The direct mesh build and the Wasm binding reach growth (including apical twig clearing and leaf-base clothing), surface, leaf placement, cull, field and structure export only through it, and neither spells the recipe out. [paraphrase]
- **R2:** The test that pins the binding's chain equal to the mesh build is deleted because one chain remains, and nothing replaces it. [inferred]
- **R3:** Output is byte-identical to the base commit for every preset in the catalogue and in `IN_WORK`, at seeds 1 and 7: the mesh build's wood and foliage, and every combination of the binding's outputs. One exception (owner, 2026-09-23): the binding's output changes for a family with rosette fronds or leaf bases, and there it equals the mesh build's, because that change is the fix. The existing pinned hashes pass without edits. [user]
- **R4:** Stage and total timings from `examples/generation_stages.rs`, extended to time placement and cull separately, and the binding's own stage timings, are recorded on base and candidate as the median of five runs for the oak, spruce and birch and every other catalogue preset. No median is more than 3% slower on the candidate; a larger gap stops the task with `NEEDS_HUMAN` and the numbers. Peak Wasm linear memory for each output combination is not higher than the base's; native peak RSS is recorded for base and candidate and any rise is reported beside the time it bought. [user]
- **R5:** The binding's exports, request shape and metadata fields are unchanged, and every existing binding and browser test passes without edits. [inferred]
- **R6:** The specimen view and the renderer's GPU preparation build and pass their existing tests without edits. Where either was changed to call a shared stage function, its output is byte-identical to the base. [inferred]
- **R7:** For a family with surface contact, one request performs one ring sweep, read by both the wood surface and the leaf placement; the element is built once per request. A test counts the sweeps through the pipeline's stage instrumentation. [user]
- **R8:** On a native target with threads, the wood surface and the leaves run concurrently when both are wanted; a test builds every catalogue preset with scheduling forced serial and forced concurrent and finds the outputs byte-identical, and a test with two failing stages finds the earlier stage's error. [user]

## Boundaries

- No algorithm changes. Leaf placement, the cull, station preparation, the GPU path and the growth path work exactly as they do now. [user]
- No output changes beyond R3's exception. A byte difference anywhere else is a defect of this spec. [paraphrase]
- The renderer's two browser calls keep their behaviour: `setTree` builds through `mesh::build` on the CPU, and `setTreeGpu`, which the harness already uses (`harness/GrowerDev.tsx:79`), prepares through `Generator::prepare_async` with GPU leaves where the family is supported. Making the GPU expansion the generator's own contract and the only drawn path is the follow-up below, not this spec. [inferred, checked 2026-09-23]
- No Wasm threads; the binding stays serial. [inferred]
- No full-forest capture. Byte comparison on single trees and the 8-tree forest is the proof. [inferred]

## Decision Context

- An earlier draft also deleted per-leaf CPU placement, moved the cull before expansion and made the GPU path the default. An outside review on 2026-09-21 found three blockers, each checked against the code: the crown-shell cull reads every transformed vertex of every leaf and station preparation takes no shell depth; the growth path's timeline calls the same run-walker as mature placement; and short shoots, families with no twig layer and a numeric fallback have no station form. [inferred]
- Direction (owner, 2026-09-23): consumers are GPU applications and receive the full-fidelity tree to draw. The generator owns the expansion of the plan into leaves and wood as a contract (the plan's layout, portable shaders, and a CPU reference expander that defines correct), and the GPU becomes the drawn default. That is its own spec after this one, with a gap spec per family that lacks a station form. This spec's Plan stage is where that contract will live, which is why it goes first. [user]
- Ideas deferred to their own specs, listed with evidence in the review notes: a per-leaf early exit in the cull, culling while placing, a forward cursor in station placement, the attractor grid in growth, a parallel cull, Wasm SIMD, and overlaps in the renderer's GPU preparation. [inferred]
- The earlier line "the birch cull costs 3.1 s of a 3.3 s build" had no committed measurement. The nearest is `.flow/evidence/fn76/LOCAL.md`: birch generate 0.28 s, surface 0.37 s, placement and cull together 4.21 s at seed 1. R4's split timing supplies the real figure. [inferred]
- "The field reads the plan, not placed leaves" (fn-100) has landed; the field stage reads the plan. The slim package (fn-101) has landed and is untouched here. [inferred]

## Resolved before ready (host, 2026-09-23)

- Which stages the renderer and the specimen view share: answered under Architecture, read on origin/master `6aab6192`.
- Whether the attachment rings are the same computation as the wood sweep: they are, down to the float32 rounding (`surface/attachment.rs:63-77`, "Query exactly the float32 vertices submitted by build()"). Sharing them is a reuse, not a derivation.
- The renderer crate's direct imports of foliage and surface items stay; moving them behind the pipeline's artifacts belongs to the GPU contract spec.
