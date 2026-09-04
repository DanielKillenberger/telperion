# The canopy: real leaf geometry, culled to a shell, in one instanced draw

## Conversation Evidence

> user: "i'm actually impressed by this generator. Is there an issue with this that i'm not seeing? why don't trees in games look like this? magical and twisting and stuff? If we add proper foliage to these it'll be great looking?"
> user: "hm well i'd like to take this as far as we can to get to a high perf high fidelity tree generator. getting foliage would obviously be the next step. should we spec that out?"
> user: "hm no but having procedural leaf textures would fit this no?"
> user: "the color shouldn't stay outside we should be able to texture it also procedurally"
> user: "so is this a naive approach not worth doing? can't we make it in some optimized way? we keep texturing as separate spec still i guess?"
> user: "so we'd be going for actual leaf geometry here yea?"
> user: "should probably put this in a separate repo honestly"
> user (standing boundary): "but the tree generator should stand on its own though. We will end up using lighting in that way but that shouldn't limit the tree generator ideally"
> user (standing principle): "and it needs to be parameterizable that twist and turning ideally" / "like everything i guess"
> user (carried forward): "I want it to be as efficient as possible should be snappy. Top tier engineering. Each component worthy of its own library."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 55% [user], 30% [paraphrase], 15% [inferred] -->

The library grows branches and stops. It emits the attachment frames foliage would hang on and nothing more, so the question the owner actually asked — whether proper foliage makes this great — cannot be answered by anything already built [user]. This spec answers it.

The ask is "a high perf high fidelity tree generator" [user] and both halves are load-bearing. Fidelity wants real geometry under a close camera. Performance wants tens of thousands of elements. Instancing and culling therefore belong in the design from the first line rather than in a later pass, because retrofitting instancing onto a mesh-per-leaf design means writing it twice.

**A research pass corrected the cost model this spec was first drafted on, and two of its corrections invert earlier reasoning.** The first draft claimed a leaf covers the same pixels however it is built, so cards saved nothing. That is false: a tightly-fitted silhouette processes far fewer wasted fragments than a quad, and the vendor recommendation is explicitly to spend triangles on the fit. Real geometry is still right, on a better argument. The second is that overdraw genuinely dominates here, but the largest single fill term is one nobody had named — `devicePixelRatio`, which quadruples fragment count on a HiDPI display and is currently invisible in the harness.

**Scope was narrowed twice by the owner.** Procedural leaf texturing belongs in this library in principle and ships as its own spec [user], so this one proves canopy structure with a flat placeholder element. And the LOD ladder was cut: the target is one hero tree in one composed shot, not a forest, so three tiers is machinery for a problem that does not exist yet [paraphrase].

## Architecture & Data Models
<!-- scope: technical -->

- **Real leaf geometry, not cards, and the vendor data settles it** [user]. Imagination's published figure: rendering a circular sprite on a best-fit quad wastes 22% of processed fragments, a dodecagon wastes 3%, and their recommendation is that added silhouette complexity is outweighed by rendering less transparency. A leaf silhouette fills far less of its quad than a circle does, so the waste is worse than 22%. Cards cost *more* fill, not the same. [paraphrase]
- **Games use cards for a reason that does not apply here**: a game leaf card is one quad carrying a dozen painted leaves, seen from a distance, across hundreds of trees. One tree with a close camera inverts every term. [paraphrase]
- **The consuming scene settles it too.** The eventual consumer is a Two Trees scene where the elements are luminous blossom; a flat card that emits light reads as a flat glowing card. [inferred]
- **The element stays parameterized so cards remain reachable.** Real geometry is the default, not a law. [user]
- **Foliage grows on shoots, not one element per attachment frame.** Sprays off a shoot, phyllotaxis along it, a clump at the end. One element per frame is the uniform-random failure the skeleton work already fought, one level down. [inferred]
- **The canopy is a shell, and this is the performance work.** Real trees shed interior leaves that never see light. Filling the envelope uniformly is botanically wrong and the most expensive available mistake, because the wasted elements are the ones the camera never sees. [paraphrase]
- **One instanced draw per element type**, per-instance transform plus variation. [paraphrase]
- **Alpha test, never blending** — but knowing what it costs. A shader that discards defeats early-Z on an immediate-mode GPU, and measured benchmarks put the loss at more than half the frame rate in a fill-heavy case. Front-to-back ordering still buys roughly 10% even after discard has compromised early-Z. [paraphrase]
- **The library generates what the tree is; the consumer brings only lights.** Geometry now, and venation, masks, albedo, roughness and translucency when the texturing spec lands. Lights, shading model, exposure and post stay outside. [user]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The target machine is named: an RTX 3080, immediate-mode with Hi-Z, no hidden-surface removal.** [inferred] This is not a detail. On an immediate-mode GPU a depth prepass is the right mitigation and alpha test costs early depth *write*; on a tile-based deferred GPU a prepass is wasted bandwidth and alpha test drops the canopy out of hidden-surface removal entirely. The two answer the same question in opposite directions, so a budget that does not name its machine means nothing.
- **`devicePixelRatio` is the largest fill term and belongs in the budget.** The harness caps at `min(dpr, 2)`, so a HiDPI display rasterizes 4x the fragments — larger than overdraw, larger than triangle size. [inferred]
- **A vsync-pinned frame time proves nothing.** Any measurement anchoring the budget needs GPU timer queries with vsync disabled, plus a four-point resolution sweep (1.0 / 0.7 / 0.5 / 0.25) read for shape. Two points under-report quad overshading, because covered-area work scales with triangle area while helper-lane waste scales with perimeter. [inferred]
- **Interior culling stays conservative at the silhouette.** Thinning is free performance up to the moment it eats the outline, and the outline is what the eye reads. [inferred]
- **Judged in clay**, flat grey, neutral sky, no bloom. A green texture would hide bad placement, and placement is what this spec proves. [paraphrase]
- **The harness's `logarithmicDepthBuffer` is now suspect and must be measured.** It was added to fix z-fighting at 400 m, and writing depth from the fragment shader defeats early-Z; published benchmarks show a 5x frame-time regression from exactly that, recoverable only by a `depth_unchanged` qualifier GLSL ES 3.00 does not have. It may be paid for out of this spec's budget. [inferred]
- **Wind and motion are out of scope.** Separate system, separate budget, and a canopy that cannot stand still convincingly will not be rescued by moving. [inferred]
- **Procedural texturing is out of scope**, its own spec: venation by space colonization at leaf scale (the algorithm's original 2005 application), albedo, translucency, masks. This spec ships a flat placeholder the texturing spec replaces. [user]
- **No LOD ladder** — a stated budget, and a far impostor only if the measurement demands one. [paraphrase]

## Acceptance Criteria

- **R1:** The foliage element is real 3D geometry generated procedurally, its shape a set of named parameters, not an authored asset and not a flat card by default. [user]
- **R2:** Elements are placed on shoots rather than one per attachment frame, with phyllotaxis along the shoot, clumping at its end, and outward and upward orientation bias, each a named parameter. [inferred]
- **R3:** Interior elements the camera cannot see are culled, with a test showing the silhouette unchanged while element count falls substantially. [paraphrase]
- **R4:** The canopy renders in one instanced draw per element type, with cutout transparency by alpha test or alpha-to-coverage rather than blending, and geometry fitted to the leaf silhouette rather than a bounding quad. [paraphrase]
- **R5:** The panel reports triangles, draw calls, instance count, `devicePixelRatio` and build time. The frame budget is stated for a named machine and measured with GPU timer queries at vsync off, across a four-point resolution sweep — never from a vsync-pinned frame time. [user]
- **R6:** The canopy is judged in clay with a flat placeholder element, and nothing in this spec emits a material, a colour or a light. [paraphrase]
- **R7:** Same seed and parameters yield an identical canopy, and the owner confirms the canopy reads as worth building a scene on, judged in clay. [paraphrase]

## Boundaries

- Procedural leaf texturing is a separate spec; this one ships a flat placeholder element. [user]
- Wind, sway and any foliage animation are out. [inferred]
- No mid-tier LOD; a far impostor only if the measurement demands one. [paraphrase]
- No materials, colours, lights, bloom or post. [user]
- No change to branch geometry. [inferred]

## Decision Context

### Motivation

- **"High perf high fidelity" is the owner's framing and both words bind** [user]. Fidelity is why the element is real geometry; performance is why culling and instancing are designed in rather than added.
- **Triangle count is the intuitive worry and close to the wrong one.** A leaf at 8 to 20 triangles across 50,000 instances stays under a million for one object, against 59,000 for the branch surface. Fill is the bill, which is why culling elements beats simplifying them — and why the fitted silhouette wins despite costing more triangles. [paraphrase]
- **Splitting texturing out is what makes this shippable.** Canopy structure is judgeable in clay with no texture, and clay is where density and distribution should be judged. Texturing is judgeable on a flat quad. Neither needs the other. [user]
- **WebGPU is strictly better than WebGL2 for this canopy** and worth recording even though the harness is WebGL today: the spec normatively permits skipping a fragment invocation when the depth test fails, and `discard` is not excluded from that, so alpha-tested leaves keep early-Z by guarantee. WGSL also has `sample_mask`, which WebGL2 lacks entirely, so per-sample hashed alpha is only reachable there. [inferred]
- **The risk is stated rather than hidden.** Foliage is where procedural trees usually fail, and it fails differently from branches: fill explodes, the interior fills with leaves a real tree would have shed, and leaves read as plastic unless the material does what diffuse cannot. Done badly it wastes the branches rather than crowning them. [inferred]

## Parked unknowns

- The frame budget in R5 is not yet a number. It resolves on the first honest measurement at a realistic element count, and nothing before that is better than a guess. [inferred]
- Whether `logarithmicDepthBuffer` survives is the same measurement. It may turn out to cost more than the z-fighting it fixes. [inferred]
- Whether a far impostor is needed at all follows from both. [inferred]

## Requirement coverage

| R-ID | Task |
|------|------|
| R1 | TBD — populate via /flow-next:plan |
| R2 | TBD — populate via /flow-next:plan |
| R3 | TBD — populate via /flow-next:plan |
| R4 | TBD — populate via /flow-next:plan |
| R5 | TBD — populate via /flow-next:plan |
| R6 | TBD — populate via /flow-next:plan |
| R7 | TBD — populate via /flow-next:plan |
