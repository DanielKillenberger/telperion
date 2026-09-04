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

### Contracts settled during planning

These five were open questions at capture time. Planning closes them, because each one sets a function signature that several tasks would otherwise have to guess at.

- **The canopy is a fifth pure stage with its own result type**, parallel to the surface mesh rather than bolted onto it. The four existing stages never call it. That separation is what keeps their existing byte comparisons valid when the canopy lands, and it preserves the property that every stage is usable on its own.
- **The element's local frame is the seam between geometry and placement.** An element is authored with its petiole at the origin, its axis along +Y and its face normal along +Z. Placement emits transforms against that frame and never inspects the element's vertices. Stating the frame here is what lets the element and the placement be built in parallel without either one waiting on the other.
- **The element owns its absolute size; placement multiplies it.** A leaf is authored in metres and placement's size term is dimensionless, defaulting to 1. The frame contract above did not originally pin scale, and the two stages shipped contradicting conventions that nothing composed until the draw. Scale belongs to the element for the same reason the outline does: a leaf does not grow because its tree is tall, and a blade sized as a fraction of envelope height puts 1.5 m fronds on a tree at Valinor scale.
- **The library emits geometry and per-instance transforms. The consumer owns the draw.** The instanced mesh, the alpha-tested material and the draw call all live in the harness, because R6 forbids the library from emitting a material. "One instanced draw per element type" is therefore a claim about the consumer, verified in the panel's draw-call count, not a unit test.
- **A shoot is a terminal run**, meaning a branch run whose final node has no children. A tree with no laterals has exactly one shoot, its trunk tip. A skeleton too small to produce a run has none, and yields an empty canopy.
- **The canopy draws chance from its own XOR-derived sub-stream.** One existing stream uses `0x5b_f0_3d_11`; the skeleton and the noise field take the caller's seed raw. The canopy takes a constant not yet in use, so adding it leaves every existing stream's output untouched.
- **Both presets state a canopy block.** The preset convention is that every term is stated and none inherited, so a canopy parameter set that the presets do not name would break that convention on the day it lands.

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
- **The canopy widens the subject, and the stage frames the subject.** Room fit, orbit pivot and the near and far planes are tuned against branch-only spans today. Foliage spread is the first thing to change that, and a prior bug in this repo put the orbit pivot off the subject through exactly this kind of bounds change. [inferred]
- **Nothing here is verifiable in the test suite beyond CPU-side data.** Vitest runs in Node with no GPU and no canvas, so draw counts, instance counts and geometry are asserted on the arrays, and the frame budget is a harness procedure the owner runs. A task that goes looking for an automated check on real WebGL state will stall on one that cannot exist. [inferred]

## Quick commands

```bash
npx vitest run      # 193 tests green at plan time; the canopy adds to this
npx tsc --noEmit    # clean at plan time
npm run dev         # the clay harness, where R5 is measured and R7 is judged
```

## Acceptance Criteria

- **R1:** The foliage element is real 3D geometry generated procedurally, its shape a set of named parameters, not an authored asset and not a flat card by default. [user] Errors: non-finite or out-of-range shape parameters fall back to their documented defaults through the `held` rail the other stages use; card mode is reachable as a named parameter and is off by default.
- **R2:** Elements are placed on shoots rather than one per attachment frame, with phyllotaxis along the shoot, clumping at its end, and outward and upward orientation bias, each a named parameter. Errors: a skeleton with fewer than two nodes, or one with no terminal runs, yields an empty canopy rather than throwing; non-finite placement parameters fall back through the same rail; candidate iteration is array-backed, so placement never depends on `Map` or `Set` ordering.
- **R3:** Interior elements the camera cannot see are culled, with a test showing the silhouette unchanged while element count falls substantially. [paraphrase] Errors: the test asserts the count both falls and stays above zero, so a culler that removes everything fails it; the fixture is dense enough that a no-op culler fails it too; an element is classified by every one of its vertices against the shell, never by its centroid.
- **R4:** The canopy renders in one instanced draw per element type, with cutout transparency by alpha test or alpha-to-coverage rather than blending, and geometry fitted to the leaf silhouette rather than a bounding quad. [paraphrase] Errors: no error surface beyond R5's reporting. Instance count and draw-call count are asserted on the CPU-side data, because vitest runs in Node with no GPU or canvas.
- **R5:** The panel reports triangles, draw calls, instance count, `devicePixelRatio` and build time. The frame budget is stated for a named machine and measured with GPU timer queries at vsync off, across a four-point resolution sweep — never from a vsync-pinned frame time. [user] Errors: when the GPU timer-query extension is unavailable the panel says so and reports no timing number, rather than falling back to a vsync-pinned frame time; a sweep interrupted by context loss or a resize reports its partial results marked incomplete.
- **R6:** The canopy is judged in clay with a flat placeholder element, and nothing in this spec emits a material, a colour or a light. [paraphrase] Errors: no error surface beyond R4. The clay room's judging-mode defaults are unchanged and the lighting-check extra stays opt-in and off.
- **R7:** Same seed and parameters yield an identical canopy, and the owner confirms the canopy reads as worth building a scene on, judged in clay. [paraphrase] Errors: determinism is asserted by building twice from one seed and comparing the emitted typed arrays; adding the canopy stage leaves the existing skeleton and surface comparisons byte-identical, because the canopy is a separate stage the earlier ones never call.
- **R8:** The canopy contributes to the stage's subject bounds, so room fit, orbit pivot and the near and far planes account for foliage spread rather than trunk taper alone. Errors: an instanced mesh whose bounding volume was never computed must not contribute zero extent silently; a canopy with zero elements leaves framing identical to the branch-only framing.

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

### Implementation Tradeoffs
<!-- scope: technical -->

- **The measurement rig goes first, before any leaf exists.** Two of this spec's three parked unknowns were the frame budget and whether `logarithmicDepthBuffer` earns its cost, and both read as blocked on the canopy. They are not. The flag is set once on the renderer and the existing branch surface already spans four orders of magnitude, so toggling it and reading GPU timer queries settles it against the tree that is there today. Ordering the rig first turns two parked unknowns into a measured answer before the work they would otherwise gate, and gives R3 and R4 a baseline to be judged against instead of a guess.
- **Culling is judged against a silhouette the repo cannot currently draw.** Nothing in the codebase projects geometry to a screen-space outline, so R3's test needs a new helper before it can assert anything. That helper is the reason culling is its own task rather than a clause inside placement: the test is most of the work.
- **The element and the placement are split on the local-frame contract, not on convenience.** With the frame stated above, the two have no shared file and no shared type beyond it, so they run at the same time. Without it they would serialize on a convention neither one owns.
- **Rejected as overkill: a leaf-level LOD ladder.** The spec already cut it, and the measurement rig is what would justify reopening it. Rejected too: exporting a shared `held` helper while touching these files, since the idiom is deliberately local at each call site in the two stages that use it.
- **Alpha test is chosen knowing it costs early-Z on the target machine.** The spec records the benchmark, and the rig in the first task is what turns that from a quoted figure into this project's number.

## Measured

The rig in fn-1-the-canopy-real-leaf-geometry-culled-to.1 ran on the named machine: an RTX 3080 through ANGLE, Chrome with vsync and the frame-rate limit disabled, fullscreen 3440x1440 at `devicePixelRatio` 2, GPU timer queries, median of 20 samples per point. The branch-only subject is 58,880 triangles in one draw call, three draws for the whole room.

| applied dpr | drawing buffer | log depth on | log depth off |
|---|---|---|---|
| 1.00 | 1720x720 | 0.21 ms | 0.25 ms |
| 0.70 | - | 0.16 ms | 0.15 ms |
| 0.50 | - | 0.13 ms | 0.12 ms |
| 0.25 | 430x180 | 0.13 ms | 0.12 ms |

At 60 Hz the frame is 16.7 ms and the whole branch-only room spends 0.21 ms of it, a little over one percent, so the canopy inherits essentially the entire budget.

**What culling buys is smaller than this spec assumed, and the reason is worth keeping.** R3 was written expecting a large drop. Measured in fn-1-the-canopy-real-leaf-geometry-culled-to.4, the default shell removes about 15% of the preset canopies and 16% of an envelope-filling one. Placement already puts foliage on distal shoots, and distal shoots sit near the crown's surface, so a well-placed canopy is most of the way to being a shell before the culler ever sees it. The elements it does take are the ones that grew on inner wood. Density is the lever: a shallower shell removes a third but eats the outline on the sparse presets, and at four times the preset spacing the same shell removes more because there is more interior to remove. If the clay judgement in .5 raises density, this number is re-measured rather than reused. The curve flattens onto a floor near 0.12 ms below dpr 0.5, which is fixed per-frame cost rather than fill, so only about 0.08 ms of the top point is fill: this scene is not yet fill-bound. The harness's own default on that display is dpr 2.00, four times the fragments of the sweep's top point, so what the owner normally looks at sits above the top of the measured curve.

## Parked unknowns

- Whether `logarithmicDepthBuffer` earns its cost is measured but not settled. Both settings land inside the noise floor of a sub-millisecond scene, with "on" reading marginally cheaper, which is noise rather than a result. The flag stays on because the z-fighting it fixes at 400 m is real. The case where a fragment-shader depth write is supposed to cost, an alpha-tested canopy defeating early-Z, does not exist until fn-1-the-canopy-real-leaf-geometry-culled-to.5, and the rig is standing there to re-run the sweep against it. [inferred]
- Whether a far impostor is needed at all follows from that re-run. Nothing in this spec decides it. [inferred]

## Early proof point

Task fn-1-the-canopy-real-leaf-geometry-culled-to.1 validates the core approach: that this project can measure its own fill cost honestly, with GPU timer queries at vsync off across a four-point resolution sweep, and can state what `logarithmicDepthBuffer` and an uncapped `devicePixelRatio` actually cost on the named machine.

It runs first because it needs no canopy. If the timer-query path turns out to be unreachable in this harness, the frame budget in R5 cannot be stated at all, and the performance half of "high perf high fidelity" has no evidence behind it. Re-evaluate the measurement approach before building the canopy that would be judged against it.

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
|-----|-------------|---------|-------------------|
| R1 | Real parameterized leaf geometry, not a card by default | fn-1-the-canopy-real-leaf-geometry-culled-to.2 | — |
| R2 | Placement on shoots: phyllotaxis, clumping, orientation bias | fn-1-the-canopy-real-leaf-geometry-culled-to.3 | — |
| R3 | Interior culling, silhouette unchanged, count falls | fn-1-the-canopy-real-leaf-geometry-culled-to.4 | — |
| R4 | One instanced draw per element type, alpha test, fitted silhouette | fn-1-the-canopy-real-leaf-geometry-culled-to.2, fn-1-the-canopy-real-leaf-geometry-culled-to.5 | — |
| R5 | Panel metrics and a measured frame budget | fn-1-the-canopy-real-leaf-geometry-culled-to.1 | — |
| R6 | Judged in clay, no material, colour or light from the library | fn-1-the-canopy-real-leaf-geometry-culled-to.5 | — |
| R7 | Same seed and parameters yield an identical canopy | fn-1-the-canopy-real-leaf-geometry-culled-to.3 | Owner sign-off in clay is a manual gate on .5, not an automated check |
| R8 | Canopy contributes to subject bounds so framing stays on the subject | fn-1-the-canopy-real-leaf-geometry-culled-to.5 | — |

