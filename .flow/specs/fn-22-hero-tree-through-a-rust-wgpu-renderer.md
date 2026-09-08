# Hero tree through a Rust wgpu renderer

## Conversation Evidence

> user (turn 1): "I'd like to instead of "make renderer super fast and efficient specs" take more manageable specs that can be reliably completed."
> user (turn 2): "I want those specs to compose to a generator and renderer that's fast and responsive and has the fidelity of basically real trees."
> user (turn 2): "And as i said no special code for tree templates it should all be parameterized and certain trees should just be configurations of the generator."
> user (turn 2): "I want to eventually have all major tree species as a template that through change of parameters proves the generator can handle those trees. That's the idea."
> user (turn 3): "each new species shouldn't necessarily be values only. If the generator isn't capable to produce a palm tree yet with parameters we need to extend the generator to allow for palms to be generated. Once that's in we the palm template is just a list of numbers yes."
> user (turn 4): "are we sure we want to merge this code? i want the code to be lean as possible. Elegant."
> user (turn 5): "should we also while we're at it port the renderer to wgpu in rust?"
> user (turn 6): "but we can still have trees in the browser also yea?"
> user (turn 7): "ok so map out the next spec for us before we capture it"
> user (turn 8): "i don't think we need pixel perfect parity from the old renderer. We'll have to iterate on the generator and renderer anyway."
> user (turn 9): "well we should have the panel though? I do want the renderer in browser with generator parameters being changeable in browser?"
> user (turn 10): "that sounds fine"
> user (earlier): "we have to be able to render everything"
> user (earlier): "it should be a preset of parameter numbers. not different way of building/rendering. The generator should be able to generate the trees if you set the right values. The presets should only store those values."
> user (earlier): "the forest is just a bunch of grey and it's super laggy.. will this be addressed?"
> user (earlier): "ok now it loads but it's super blurry while it load more detail and then it stutters when rotating"
> user (earlier): "that no color will arrive i know. I just thought you'd see some texture anyway"
> user (earlier): "and shading"
> user (earlier): "otherwise it's mad ugly"
> user (edit cycle 1): "but i still want a renderer for a website though"
> user (edit cycle 1): "The nice thing would be is if you could import telperion lib and have trees generated in your engine that look good and perform well."
> user (edit cycle 1): "I want each tree to have slightly different parameters set in telperion and another seed. To make each tree unique"
> user (edit cycle 1): "so i don't want to have to preexport trees"

## Goal & Context
<!-- Goal & Context: 50% [user], 40% [paraphrase], 10% [inferred] -->

The fn-13 renderer branch proved the hero path in TypeScript and WebGPU but grew into a 230-commit prototype: 46 modules where master has 4, untyped JavaScript in production, and a forest that is slow, flat and unstable. The owner wants the opposite: small specs that can be reliably completed, composing toward a generator and renderer that is fast and responsive with the fidelity of real trees. [paraphrase]

This spec is the first of that series. It puts a lean Rust renderer on master, built on wgpu, that draws one hero tree from generator output in the browser with the existing parameter panel driving it, and in a headless native target for clean timing. [paraphrase] The renderer is written from scratch in Rust; the fn-13 branch stays as a reference prototype and is not merged. [user]

The owner's standing rule governs everything downstream: a template is only a row of parameter values, the generator produces any tree from values through one code path, and the renderer never knows the species. [user] Where the generator cannot yet express a species, the fix is a new general parameter, never a species branch. [user]

## Architecture & Data Models
<!-- Architecture & Data Models: 60% [paraphrase], 40% [inferred] -->

- **New Rust renderer crate** in the workspace, alongside the existing generation core. It compiles to wasm for the browser and to a native headless target. [paraphrase]
- **Two build targets, one renderer.** Browser: the crate draws into the page canvas through the browser's WebGPU. Native: an offscreen surface renders the same scene from a preset id and seed, writes a screenshot, and reports GPU timing. No native window in this spec. [user]
- **Data path.** The library exposes one engine-neutral mesh output per tree, wood surface and foliage instances at a requested detail budget, produced at runtime from seed and parameters with no exported assets. [user] The web renderer is the reference consumer of that output: it reads the buffers in place from the memory the library wrote and uploads directly to GPU buffers, with no JavaScript-side geometry copies. [paraphrase] The same output is what a game-engine adapter such as Unreal or a Unity mod consumes, so nothing in it is web-specific. [paraphrase] Generation runs off the render thread and hands over one buffer per tree. [inferred]
- **Scene ownership.** The Rust renderer owns its whole scene: ground, sky, clay hemisphere lighting, camera. It shares no depth buffer with the existing Three.js stage. [paraphrase]
- **The panel stays.** The React control panel with every generator parameter as a dial is retained unchanged in purpose; the canvas beside it is drawn by the Rust renderer. A dial move regenerates and redraws. [user]
- **Exact mode.** The existing Three.js exact mode remains as an inspection tool during this spec. It is not extended. [paraphrase]
- **Ported ideas, rewritten.** Three ideas from the prototype are carried over as rewritten Rust: the compact tapered wood representation, the clay hemisphere lighting, and the GPU timing protocol with conditioning frames and contention detection. Hierarchy, aggregates and streaming are not ported here. [inferred]

## API Contracts
<!-- API Contracts: 30% [paraphrase], 70% [inferred] -->

- **Renderer creation.** Given a canvas (browser) or an offscreen size (native), returns a renderer or a typed error naming the failing condition: WebGPU absent, no hardware adapter, device request refused with the limit that failed. [paraphrase]
- **Mesh output.** The library returns, for a seed, a parameter set and a requested detail budget, one tree as engine-neutral buffers: wood surface, foliage instances, bounds. The same call serves every consumer; this spec exercises only the full-detail budget. [paraphrase]
- **Tree submission.** The renderer accepts that output plus a rigid transform. Returns wood segment and foliage instance counts and bounds matching the library's report. Rejects an oversize submission with an explicit error rather than truncating. [inferred]
- **Camera.** Accepts position, target and vertical field of view per frame. [inferred]
- **Frame.** Draws one frame and returns statistics: draw calls, triangles, instances, and when timing is enabled the vegetation GPU time for that frame with a validity flag. [paraphrase]
- **Timing session.** Runs the fixed conditioning frames, warmup and measured samples, and returns the p50 and p95 with a validity verdict; a contended or disjoint measurement is reported as invalid, never as a number that could be mistaken for a pass. [paraphrase]

## Edge Cases & Constraints
<!-- Edge Cases & Constraints: 40% [user], 30% [paraphrase], 30% [inferred] -->

- **Lean code.** The owner wants the code as lean as possible and elegant. [user] Typed Rust and TypeScript only; no code copied from the prototype without a rewrite and a test. [paraphrase] Files stay under about 400 lines and functions do one thing. [inferred]
- **No species knowledge in the renderer.** Any preset or parameter set renders through the same path. Unsupported input is an explicit, named error, never silent omission or a blank canvas. [user] [strategy:Surface and rendering at scale]
- **Browser threads.** Wasm threads need cross-origin isolation and a shared-memory build; the design assumes one render thread and generation in a worker with its own wasm instance. No thread pool in the browser. [inferred]
- **Shared GPU.** Timing on the desktop is contended by the compositor and other browsers; the native headless target exists so that a clean measurement is possible. Contended browser measurements are reported as invalid. [paraphrase]
- **Performance is reported, not gated.** The 2 ms hero target belongs to a later spec. This spec measures and records. [paraphrase]
- **Token and evidence budget.** No full-forest captures; evidence is screenshots and small JSON. The per-task budget rules in the project instructions apply. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The browser page renders a hero oak or spruce from the generator through the Rust renderer, with every generator parameter changeable from the existing panel; a dial move regenerates and redraws. [user] Errors: generator rejection or GPU failure shows a visible message in the panel with the reason, never a blank canvas; missing WebGPU or a fallback-only adapter shows a message naming the condition.
- **R2:** The native headless target renders the same scene from a preset id and seed, writes a screenshot, and exits zero. [paraphrase] Errors: no adapter or device refusal exits non-zero with the reason on stderr.
- **R3:** The renderer draws the library's engine-neutral mesh output in place from library memory with no JavaScript-side geometry copies, and a test asserts the submitted wood segment count, foliage instance count and bounds equal the library's report for oak and spruce. [paraphrase] The output is produced at runtime from seed and parameters; no tree asset is exported or stored. [user] Errors: a count or bounds mismatch fails the test; an oversize tree is rejected with an explicit error, not truncated.
- **R4:** Every shipped preset, and a random sample of at least twenty parameter sets, renders through the identical renderer path with no species or template branch in renderer code. [user] [strategy:Surface and rendering at scale] Errors: an input the generator rejects surfaces the generator's message naming the parameter; no silent omission.
- **R5:** GPU timing runs in the browser and in the native target with conditioning, warmup and measured samples, and the hero vegetation p50 and p95 for oak and spruce are recorded in the spec's evidence with their validity verdict. [paraphrase] Errors: a contended, disjoint or unavailable measurement is recorded as invalid and cannot read as a pass.
- **R6:** A five minute idle soak in the browser keeps one canvas and one GPU device with no self-initiated rebuild, and camera orbit responds throughout. [paraphrase] Errors: no error surface beyond R1.
- **R7:** One still per species at the hero pose is reviewed by the owner and the verdict is recorded in the spec, in the owner's eye sense of the strategy. [user] [strategy:Growth and botanical fidelity] Errors: no error surface beyond R1.

## Boundaries
<!-- scope: business -->

- No pixel or threshold parity with the Three.js exact renderer. Correctness is by construction against the generator's own output and by the owner's eye. [user]
- No colour, bark or foliage materials. The scene is neutral clay. [user]
- No forest, aggregates, streaming, detail hierarchy or progressive generation. Those are later specs in the series. [paraphrase]
- No native window. The native target is headless offscreen only, for timing and screenshots. [user]
- No merge of the fn-13 branch. It remains a reference prototype; ideas are ported by rewriting. [user]
- The Three.js exact mode is neither extended nor removed in this spec. [paraphrase]
- No species-specific code anywhere, in renderer or generator. [user]
- No detail budgets below full detail, no LOD chain, no engine adapter. The mesh output contract is shaped for them but only the full budget is exercised here; game-engine integration is fn-17 and the Valheim proof follows the coarse-first spec. [paraphrase]

## Decision Context
<!-- scope: both — substructured -->

### Motivation
<!-- scope: business -->

- Manageable specs that complete reliably are preferred over one large performance spec; fn-13 burned a full weekly quota on twenty-two full-forest captures without a pass. [user]
- Success for the series is a generator and renderer that is fast and responsive with the fidelity of real trees; success for this spec is a hero tree that renders on master through lean Rust with live parameters. [paraphrase]
- Lean and elegant code outranks reusing the prototype's working but dense code. [user]
- The library must be importable into a game engine so that every tree is generated at runtime, unique per position and seed, never pre-exported; the web renderer is the reference consumer of that same output. [user]
- The user experience the owner reacted to, blurry loading and stutter on rotation, is the target of later specs; this spec establishes the renderer they build on. [paraphrase]

### Implementation Tradeoffs
<!-- scope: technical -->

- Rust with wgpu over continuing in TypeScript: the wasm boundary was where most of fn-13's cost lived, from transfer-list collection to retained JavaScript copies; one crate removes the copies and gives a native timing path for free. [paraphrase]
- Rewrite over merge: 10,700 dense lines with untyped modules would become the base for every later spec; porting three ideas is cheaper than maintaining that. [paraphrase]
- Headless native over a window now: timing and screenshots are all the native target is for in this spec; a window can come with the engine integration track. [paraphrase]
- Owner's-eye sign-off over image parity: parity would break with every generator change and would tie the new renderer to the old look. [paraphrase]

### Successor specs (agreed 2026-09-08, not yet captured)

Captured one at a time as each becomes next. Order: one foliage placement path with templates as pure value tables (every preset and random parameter sets render, spruce and oak hashes unchanged); fast hero (2 ms on an idle GPU, smooth rotation); coarse-first generation with detail budgets and identical refinement, the enabler for unique runtime trees in a seeded game; a third species reached by generalizing the generator then writing values; Unreal integration through fn-17 with Nanite foliage; a Valheim mod as the Unity proof of unique per-position runtime trees; a modest web forest. Shaded aggregates fold into the web forest. Viewer stability is R6 here.

## Parked unknowns

- Whether generation stays in a worker with its own wasm instance or moves inline once progressive generation makes it cheap; resolved by measurement in the progressive-generation spec.
- When the Three.js exact mode is removed; resolved once the Rust renderer has its own single-leaf inspection.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1 | TBD — populate via /flow-next:plan |
| R2 | TBD — populate via /flow-next:plan |
| R3 | TBD — populate via /flow-next:plan |
| R4 | TBD — populate via /flow-next:plan |
| R5 | TBD — populate via /flow-next:plan |
| R6 | TBD — populate via /flow-next:plan |
| R7 | TBD — populate via /flow-next:plan |
