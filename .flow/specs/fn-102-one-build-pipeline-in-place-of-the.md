# One build pipeline in place of the copied chains

## Conversation Evidence

> user: "should we think about how to simplify this design in core and telperion over all facets? Make it really a clean building pipeline with set stages that all coherently make sense as part of the whole and also what consumes them."
> user: "Can you illustrate current structure of the code base with every relevant branch and show the new structure that we'll be working towards with a clean pipeline design"
> user: "this pipeline restructure's performance must be faster or at worst equal to current performance. Is there a way this streamlining can actually improve performance?"
> user: "do we need the cpu chain for leaves? beacuse if you're going to render millions of leaves you'll need a gpu anyway to not stutter like crazy..?"
> assistant: "fn-102: shrink it to pipeline consolidation only. One function replaces the three copied chains, output stays byte-identical, and no algorithm changes."
> user: "ok makes sense so we just simplify cut the engine in 102?"

## Goal & Context

<!-- Goal & Context: 40% [paraphrase], 60% [inferred] code reading -->

The core's mature build recipe is written out more than once. The direct mesh build and the Wasm binding each spell out growth, wood surface, leaf placement and cull in the same order, and a test exists only to hold the two copies equal. The binding chooses stages with a row of flags, which is how a field request came to run leaf placement. The renderer's GPU preparation and the specimen view each call some of the same stages on their own.

The owner wants one pipeline with set stages, each coherent as part of the whole, with consumers reading what stages produce. This spec does the structural half of that and nothing else: one place that names the stages, their artifacts and their order. Every algorithm, every random draw and every output byte stays as it is. The ideas that change algorithms were reviewed on 2026-09-21 and found to need their own work; they are listed under Decision Context and are not built here.

## Architecture & Data Models

The pipeline names five stages, each producing one artifact. [paraphrase]

1. Family: the parameter table from a preset, a blend or a request.
2. Skeleton: the solved tree from the direct build's growth.
3. Plan: what is prepared from the skeleton before any triangle or leaf exists, as far as today's code already prepares it.
4. Outputs, each optional: the wood surface, the leaves, the occupancy field and the structure export.
5. Tree mesh: the wood surface and leaves with their union bounds.

One pipeline function in the core takes the set of wanted artifacts and runs each needed stage once, in dependency order, calling the stage functions that exist today. The direct mesh build and the Wasm binding both become callers of it. An artifact nobody asked for is never built and never held. [paraphrase]

The specimen view is not a copy of the mature recipe: it reads recorded leaves and replays the growth path's own placement. It and the renderer's GPU preparation call the pipeline's stage functions only where they already run the identical stage. [inferred]

## Edge Cases & Constraints

- A request for an output whose stage fails returns the same error, in the same order relative to other stages' errors, as today. [inferred]
- The binding's timing metadata keeps its per-stage fields and meanings. [inferred]
- The growth path stays buildable and pinned and is not routed through anything new. [inferred]

## Acceptance Criteria

- **R1:** The core has one pipeline function that names the stages and their order. The direct mesh build and the Wasm binding reach growth, surface, leaf placement, cull, field and structure export only through it, and neither spells the recipe out. [paraphrase]
- **R2:** The test that pins the binding's chain equal to the mesh build is deleted because one chain remains, and nothing replaces it. [inferred]
- **R3:** Output is byte-identical to the base commit for all six shipped presets at seeds 1 and 7: the mesh build's wood and foliage, and every combination of the binding's outputs. The existing pinned hashes pass without edits. [paraphrase]
- **R4:** Stage and total timings from the stage-measurement example, and the binding's own stage timings, are recorded on base and candidate as the median of five runs for the six presets. No median is more than 3% slower on the candidate; a larger gap stops the task with `NEEDS_HUMAN` and the numbers. Peak Wasm linear memory for each output combination is not higher than the base's. [user]
- **R5:** The binding's exports, request shape and metadata fields are unchanged, and every existing binding and browser test passes without edits. [inferred]
- **R6:** The specimen view and the renderer's GPU preparation build and pass their existing tests without edits. Where either was changed to call a shared stage function, its output is byte-identical to the base. [inferred]

## Boundaries

- No algorithm changes. Leaf placement, the cull, station preparation, the GPU path and the growth path work exactly as they do now. [user]
- No output changes. A byte difference anywhere is a defect of this spec. [paraphrase]
- The renderer's default tree call stays on the CPU mesh build; making the GPU path the default is not part of this spec. [inferred]
- The field's new definition and the slim homepage package are sibling specs and are not built here. [inferred]
- No full-forest capture. Byte comparison on single trees and the 8-tree forest is the proof. [inferred]

## Decision Context

- An earlier draft of this spec also deleted per-leaf CPU placement, moved the cull before expansion and made the GPU path the default. An outside review on 2026-09-21 found three blockers, each checked against the code. The crown-shell cull reads every transformed vertex of every leaf and station preparation takes no shell depth, so culling before expansion is a new algorithm. The growth path's timeline calls the same run-walker as mature placement, so deleting it breaks a path the project rules keep pinned. Short shoots, families with no twig layer and a numeric fallback have no station form, so removing the CPU path removes supported behaviour. [inferred]
- Ideas deferred, each to be its own spec if the owner wants it, none written yet: a CPU reference expander over station segments; a cull that rejects whole segments before expansion, with a prototype first because the birch cull costs 3.1 s of a 3.3 s build; the GPU station path as the renderer's default; a station form for short shoots. [inferred]
- The owner asked whether a CPU leaf chain is needed at all, since drawing millions of leaves needs a GPU anyway. For drawing it is not. It stays for now because the growth path, tests without an adapter and the unsupported parameter cases still stand on it. [paraphrase]
- Depends on "The field reads the plan, not placed leaves", which changes what the field stage reads; consolidating after it avoids doing the field's wiring twice. Sibling: "A slim growth-and-field package for the homepage".

## Parked unknowns

- Which stages the renderer's GPU preparation and the specimen view share exactly with the mature recipe. Reading both against the stage functions before the spec is marked ready settles what R6 covers.
- Whether the renderer crate's 35 direct imports of foliage and surface items should move behind the pipeline's artifacts. They were counted, not read; this spec leaves them alone.
