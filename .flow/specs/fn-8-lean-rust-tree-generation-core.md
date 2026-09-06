# fn-8-lean-rust-tree-generation-core Lean Rust tree generation core

## Conversation Evidence

> user (E1): "i'd just like to see if i can push the envelope with tree generation in realtime environments with proc gen. I feel that games have had underwhelming trees in most settings. If we can have high performant trees that can even have a lifecycle with growth etc. That'd be insane step for gaming no?"
> user (E2): "well the parameters establish the trees "family" of trees and the seed will make an instance/specimen of that right?"
> user (E3): "we should probably also have presets for certain species"
> user (E4): "is it worth rewriting the fundamental engine in rust to be much faster/more efficient and we create bindings?"
> user (E5): "it would also be good if we could ouptut things in ways that are useful to different engines. For example if we made a minecraft mod we wouldn't need an actual surface mesh but only a field that we'd approach with blocks right? allowing for this kind of flexibility in integration should also be a core functionality. Does this make sense?"
> user (E6): "ok so we wait on this worktree and integrate fn-6 when it's done and test again"
> user (E7): "i mean the repo is 1 day old. We can just migrate the whole thing. I don't need backwards compatibility"
> user (E8): "i think it's fine for now we'll do another spec where we try and build all kinds of real trees as templates with visual QA which will definitely find structural issues."
> user (E9): "have to say the tapered branches look very very pokey now. should we fix it now or do the rust rewrite first"
> user (E10): "we don't need completion review let's call this done and move on to fn-7 (if we even still need to finish that with new version) and do the rust rewrite in fn-8?"
> user (E11): "alright sounds good. One requirement for the rust rewrite i want the code to be lean and elegant. It should be easy to change and grow. that should be the mantra overall for this project."
> user (E12): "Minimalist af. Efficient af and beautiful. In the end we want to get to realistic simulations of trees that are procedurally generated."

## Goal & Context

<!-- Source: [paraphrase], E1, E4, E7, E11, E12 -->
Replace the procedural tree engine with a lean Rust core that improves runtime efficiency and stays easy to change. The destination is realistic procedural tree simulation for real-time environments, including growth and lifecycle behaviour. FN-8 establishes the core on which that work can grow.

## Architecture & Data Models

- Parameters define a tree family, a seed selects a specimen, and species presets supply named parameter sets. [paraphrase] (E2, E3)
- One Rust core serves different engines through bindings, separating the tree's structure from the representation a consumer requests. [paraphrase] (E4, E5, E7)
- Small interfaces and explicit data flow keep botanical rules and output representations independently changeable. [strategy:The core and integration]

## API Contracts

- Consumers can request useful tree representations without being forced through surface-mesh generation; spatial fields must support a block-based consumer. [paraphrase] (E5)
- Bindings expose the same core to browser and native consumers. [strategy:The core and integration]

## Edge Cases & Constraints

- Existing APIs and formats impose no backwards-compatibility requirement. [paraphrase] (E7)
- Proposed common error contract for R2-R5: invalid inputs fail clearly; valid empty outputs remain usable; resource limits and binding failures cannot masquerade as a complete successful tree. Exact error and ownership mechanisms belong to the implementation plan. [inferred]

## Acceptance Criteria

- **R1:** Compare the rewrite with finished FN-6 using reproducible inputs and visual checks at ordinary and Telperion scales; diagnose differences without requiring byte-identical old outputs or preserving known structural defects. An unavailable or mismatched reference must be reported. [inferred]
- **R2:** The complete procedural generation engine runs in Rust, including family parameters, specimen seeds and species presets; repeated identical inputs reproduce a specimen. Boundary handling follows the proposed common error contract. [paraphrase] (E2, E3, E4, E7)
- **R3:** Mesh-based and mesh-free consumers can obtain suitable representations of the same generated tree, including a spatial field usable by a block-based consumer without constructing a surface mesh. Empty field regions remain valid results; invalid requests follow the common error contract. [paraphrase] (E5)
- **R4:** Browser and native bindings use the same generation core. Binding failures follow the proposed common error contract. [strategy:The core and integration]
- **R5:** The current interactive viewer continues to generate and display the Two Trees and parameter-driven specimens through the Rust core; existing useful controls and visual inspection remain available. Build failures are visible and leave the viewer usable. [inferred]
- **R6:** Measure complete generation latency and memory against finished FN-6 under comparable workloads, including binding costs; demonstrate a generation-speed improvement and report memory or rendering regressions explicitly. Missing or inconclusive measurements are reported as such. [inferred]
- **R7:** Deliver lean, elegant code whose botanical rules and outputs are easy to change: one production generator, small understandable interfaces and no unnecessary compatibility machinery. Assess this through the resulting module boundaries and representative changes to a botanical rule and an output, without imposing a line-count target; no runtime error surface beyond R2-R5. [inferred]

## Boundaries

- Backwards compatibility is unnecessary. [paraphrase] (E7)
- Broad real-species templates and their structural visual QA belong to a subsequent spec. [paraphrase] (E8)
- Full lifecycle simulation is the long-term destination; new lifecycle algorithms and a shipped Minecraft mod are outside this migration, while mesh-free field output is included. [inferred]

## Decision Context

### Motivation

- "Minimalist af. Efficient af and beautiful." [user] (E12)
- The young repository can undergo a full migration without carrying compatibility obligations. [paraphrase] (E7)
- Realistic procedural simulation gives the architecture its direction; lean, changeable code is a project-wide requirement. [paraphrase] (E11, E12)

## Requirement coverage

| Requirement | Tasks |
|-------------|-------|
| R1 | FN-8.1, FN-8.2, FN-8.3, FN-8.4, FN-8.5, FN-8.6 |
| R2 | FN-8.1, FN-8.2, FN-8.3 |
| R3 | FN-8.4, FN-8.5, FN-8.8 |
| R4 | FN-8.1, FN-8.6 |
| R5 | FN-8.6 |
| R6 | FN-8.7 |
| R7 | FN-8.1, FN-8.6, FN-8.7, FN-8.8 |

## Merged implementation closure (2026-09-07)

All eight tasks are done and their evidence commits are ancestors of master. The implementation landed through PR #1 (Lean Rust tree generation core), merged at 2fcac3540993afcd13927a30e9e5238f7bad0df3. This administrative closure reconciles the stale open spec with that merged history. It makes no new performance or review claim and preserves the original report's limits.
