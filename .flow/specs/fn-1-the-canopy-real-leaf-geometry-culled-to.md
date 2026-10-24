# The canopy: real leaf geometry, culled to a shell, in one instanced draw

## Goal

The library grew branches and stopped, so the owner's question, whether proper foliage makes the generator great, could not be answered. This spec adds a canopy: real procedural leaf geometry placed on shoots, culled to a shell and drawn in one instanced draw, judged in clay, toward "a high perf high fidelity tree generator" (owner, 2026-09-04).

## Decisions

Owner, 2026-09-04 (verbatim):
> "i'm actually impressed by this generator. Is there an issue with this that i'm not seeing? why don't trees in games look like this? magical and twisting and stuff? If we add proper foliage to these it'll be great looking?"
> "hm well i'd like to take this as far as we can to get to a high perf high fidelity tree generator. getting foliage would obviously be the next step. should we spec that out?"
> "hm no but having procedural leaf textures would fit this no?"
> "the color shouldn't stay outside we should be able to texture it also procedurally"
> "so is this a naive approach not worth doing? can't we make it in some optimized way? we keep texturing as separate spec still i guess?"
> "so we'd be going for actual leaf geometry here yea?"
> "should probably put this in a separate repo honestly"
> (standing boundary) "but the tree generator should stand on its own though. We will end up using lighting in that way but that shouldn't limit the tree generator ideally"
> (standing principle) "and it needs to be parameterizable that twist and turning ideally" / "like everything i guess"
> (carried forward) "I want it to be as efficient as possible should be snappy. Top tier engineering. Each component worthy of its own library."

- Real leaf geometry fitted to the silhouette, not cards: a fitted outline wastes far fewer fragments than a quad (vendor figure: a circle on a quad wastes 22%). Card mode stays reachable as a parameter, off by default.
- Foliage grows on shoots (terminal runs), with phyllotaxis, a tip clump and outward/upward bias; the canopy is a shell, interior leaves culled.
- The canopy is a fifth pure stage with its own result type; earlier stages never call it, so their byte comparisons hold. Element local frame: petiole at origin, axis +Y, normal +Z; the element owns its absolute size in metres, placement multiplies it.
- The library emits geometry and per-instance transforms; the consumer (harness) owns material, draw call and lights. Alpha test, never blending.
- Procedural texturing, wind, LOD ladder: out of scope (texturing its own spec; no LOD ladder, impostor only if measured necessary).
- Measurement rig first: GPU timer queries at vsync off, four-point resolution sweep, on a named RTX 3080; `devicePixelRatio` reported.

## Acceptance Criteria

- **R1:** The foliage element is real procedural 3D geometry from named shape parameters, not a card by default — met (blade covers 0.58 of its quad).
- **R2:** Elements placed on shoots with phyllotaxis, clumping and orientation bias, each a named parameter; empty canopy rather than throw on tiny skeletons — met.
- **R3:** Interior elements culled with silhouette unchanged while count falls substantially — met, but the fall was smaller than expected (see Result).
- **R4:** One instanced draw per element type, alpha test, silhouette-fitted geometry — met.
- **R5:** Panel reports triangles, draw calls, instances, dpr, build time; frame budget measured with GPU timer queries at vsync off over a four-point sweep on a named machine — met.
- **R6:** Judged in clay with a flat placeholder; the library emits no material, colour or light — met.
- **R7:** Same seed and parameters give an identical canopy, and the owner confirms the canopy reads as worth building on — determinism met; owner sign-off never given (closed historically).
- **R8:** The canopy contributes to the stage's subject bounds (fit, pivot, near/far planes) — met.

## Result

- Rig (RTX 3080, vsync off, dpr sweep 1.00/0.70/0.50/0.25, median of 20): the branch-only room (58,880 triangles) cost 0.21 ms of a 16.7 ms frame; floor ~0.12 ms is fixed cost, so the scene was not fill-bound. `logarithmicDepthBuffer` on vs off sat inside noise; the flag stays on for z-fighting at 400 m. Unsettled, not a verdict.
- Shell cull on the pre-fn-5 presets removed 14.8% (Telperion) and 6.5% (Laurelin), nearly flat across 1x/2x/4x density: placement already puts foliage near the surface. After fn-5's twig orders, shedding moved to the wood (`shedTwigs`, 8-9% of twig nodes) and the leaf culler removes under 1% at depth; at eight orders Telperion carries 136,808 leaves and Laurelin 766,435.
- At eight twig orders a 3.6 M-triangle tree with 190 k leaves cost 1.28 ms per frame, flat across resolution (vertex-bound); the CPU build (2.1 s) is the constraint. fn-5 owns that number.
- Closed 2026-09-07 as a completed historical implementation superseded by fn-6 branching and the fn-8 Rust migration. The closure does not award the missing owner sign-off (R7) or the old GPU frame-budget target. Rendering performance moved to fn-13, species and geometry to fn-9/fn-19.

## Open

- Owner sign-off in clay was never given.
- Whether a far impostor is ever needed, and the log-depth verdict under an alpha-tested canopy, were left to later measurement.
