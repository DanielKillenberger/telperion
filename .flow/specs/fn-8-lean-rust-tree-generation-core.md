# fn-8-lean-rust-tree-generation-core Lean Rust tree generation core

## Goal

Replace the TypeScript tree engine with a lean Rust core that is faster and easy to change, serving browser and native consumers through bindings and offering mesh-free representations (a spatial field for block-based consumers) as well as meshes. It is the base for realistic real-time procedural simulation including growth and lifecycle (owner, 2026-09-05).

## Decisions

Owner (verbatim, 2026-09-05):
> (E1) "i'd just like to see if i can push the envelope with tree generation in realtime environments with proc gen. I feel that games have had underwhelming trees in most settings. If we can have high performant trees that can even have a lifecycle with growth etc. That'd be insane step for gaming no?"
> (E2) "well the parameters establish the trees "family" of trees and the seed will make an instance/specimen of that right?"
> (E3) "we should probably also have presets for certain species"
> (E4) "is it worth rewriting the fundamental engine in rust to be much faster/more efficient and we create bindings?"
> (E5) "it would also be good if we could ouptut things in ways that are useful to different engines. For example if we made a minecraft mod we wouldn't need an actual surface mesh but only a field that we'd approach with blocks right? allowing for this kind of flexibility in integration should also be a core functionality. Does this make sense?"
> (E6) "ok so we wait on this worktree and integrate fn-6 when it's done and test again"
> (E7) "i mean the repo is 1 day old. We can just migrate the whole thing. I don't need backwards compatibility"
> (E8) "i think it's fine for now we'll do another spec where we try and build all kinds of real trees as templates with visual QA which will definitely find structural issues."
> (E9) "have to say the tapered branches look very very pokey now. should we fix it now or do the rust rewrite first"
> (E10) "we don't need completion review let's call this done and move on to fn-7 (if we even still need to finish that with new version) and do the rust rewrite in fn-8?"
> (E11) "alright sounds good. One requirement for the rust rewrite i want the code to be lean and elegant. It should be easy to change and grow. that should be the mantra overall for this project."
> (E12) "Minimalist af. Efficient af and beautiful. In the end we want to get to realistic simulations of trees that are procedurally generated."

- Parameters define a family, a seed selects a specimen, species presets name parameter sets.
- One Rust core, structure separate from the representation a consumer requests; no backwards compatibility.
- Real-species templates with visual QA belong to a later spec (became fn-9); lifecycle algorithms and a Minecraft mod are out of scope, the mesh-free field is in.

## Acceptance Criteria

- **R1:** Compare with finished fn-6 on reproducible inputs and visual checks without requiring byte-identical outputs — met (pinned fn-6 `fdafb099`; crown drift 7.04e-11 m).
- **R2:** The whole generation engine runs in Rust with families, seeds and presets, reproducibly — met.
- **R3:** Mesh and mesh-free consumers get representations of the same tree, including a block-usable field without a surface mesh — met (wood and foliage fields with BVH queries).
- **R4:** Browser and native bindings share one core — met.
- **R5:** The viewer generates and displays the Two Trees and parameter specimens through Rust, errors visible and recoverable — met.
- **R6:** Complete generation latency and memory measured against fn-6 including binding costs, with a speed improvement — met (see Result).
- **R7:** Lean, elegant, easy-to-change code with one production generator — met; superseded TS core removed.

## Result

Paired headless Chromium runs on the RTX 3080 / Ryzen 9 5950X: the Rust builder is 2.91x faster for Telperion (4,720 -> 1,621 ms), 2.71x for Laurelin (2,190 -> 809 ms) and 2.81x for the comparison, with identical node, vertex, triangle, leaf and twig counts. An initial run was flat because software `hypot` dominated; `distance_to_profile` now minimises squared distances. GPU frame cost is unchanged or slightly worse (Telperion 6.28 ms at DPR 1) and still misses 2 ms; giant-tree Wasm high-water is 526 MiB. Full report: `.flow/evidence/fn8/REPORT.md`. Merged through PR #1 at 2fcac3540993afcd13927a30e9e5238f7bad0df3; closed 2026-09-07.

## Open

- The "very very pokey" taper (E9) and real-species structural QA went to fn-9.
- GPU rendering budget and total browser memory unresolved here.

## Evidence

- `.flow/evidence/fn8/REPORT.md`
