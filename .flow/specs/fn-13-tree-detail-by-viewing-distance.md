# Seamless full-fidelity tree rendering across engines

## Conversation Evidence

> user: "those specs are excellent can you $flow-next-capture them?"
>
> user: "so with all those needles there's a lot of noise on the screen and rendering clearly isn't possible at full 60 fps"
>
> user: "it looks incredibly structurally accurate but the rendering suffers"
>
> user: "there's also some flickering when the foliage overlaps with the \"Ground\""
>
> user: "so should we do a rendering improvement pass?"
>
> user: "i want this renderer to not even break a sweat at one tree with full fidelity. It should never lose fidelity at any distance and no stepping. I guess something like nanite in unreal engine? Can we include this in this spec?"
>
> user: "also we should think about how we can have these trees also work in other game engines with the same fidelity reqs"

The original roadmap already included viewing-distance representations and measured hero-tree/forest budgets. This amendment strengthens that scope with the owner's rendering, continuity and cross-engine fidelity requirements. [paraphrase]

## Goal & Context

Make one fully detailed tree inexpensive to render while preserving its perceived fidelity continuously from individual needle/leaf inspection to a distant crown. Preserve the structurally accurate botanical source; solve rendering noise, ground-overlap flicker and runtime cost in the rendering representation. A visibly thinner crown, lost tier gaps, flattened sprays or obvious detail changes do not satisfy the request. [paraphrase]

The same visual contract must apply in other supported game engines, rather than being an effect that only works in the current viewer. [paraphrase]

## Architecture & Data Models

Keep one lean, engine-independent core and an authoritative full-detail specimen. Derive rendering representations without changing species rules, seed identity, attachment geometry or source foliage to make benchmarks cheaper. [strategy:The core and integration] [paraphrase]

Investigate a Nanite-like approach to continuous, screen-space-driven detail: hierarchical reuse/culling and aggregate representations that preserve coverage, volume, shading and fine detail as it becomes resolvable. Nanite is an architectural reference, not a mandated engine, implementation or guarantee. No specific hierarchy, voxel, triangle, card or graphics API solution is selected by this capture. [paraphrase] [inferred]

Separate portable tree/representation data and fidelity semantics from engine-specific drawing, material, depth and antialiasing implementations. Account for source identity, hierarchy/bounds, coverage/material information, error descriptions, ownership and updates as part of the portability design; prove what each supported backend actually needs. [inferred]

## Acceptance Criteria

- **R1 — Full-fidelity appearance:** Render the same specimen from close anatomy views through the distant crown without perceptible loss of silhouette, volumetric foliage coverage, branch/leaf/needle identity, attachment, tier gaps or lighting response at the output resolution. Full source detail remains available and resolves when approached. Define and justify measurable screen-space and appearance error criteria against full-detail references before selecting an optimization; do not redefine fidelity after seeing a failing result. [paraphrase] [inferred]
- **R2 — Continuous motion and detail:** Slow/fast orbiting, dollying, zooming, reversals and repeated threshold crossings must show no visible stepping, popping, thinning, representation swaps, temporal shimmer or unexplained disappearance. Validate rendered sequences as well as still images; transition blending or temporal filtering is not accepted merely because it hides a switch if it creates noise, ghosting or lost detail. Missing detail must be handled explicitly without silently violating R1. [paraphrase] [inferred]
- **R3 — Measured headroom:** Demonstrate the existing 2 ms GPU hero-tree budget on the named RTX 3080 at a declared native output resolution/pixel ratio, using the mature full-fidelity spruce as a required stress case and additional species for coverage. Measure real GPU time, CPU submission/update time, frame-time distributions, memory and generation/preparation costs across close, whole-tree and distant moving-camera views; separate cold preparation from steady rendering and expose transition spikes. The 60 fps frame budget is 16.7 ms for the whole application, not permission to spend all of it on one tree. A hero-budget miss remains a failed performance target and cannot be fixed by relaxing R1/R2. Retain the thousand-tree forest measurement and existing 4 ms vegetation target, explicitly reporting misses without substituting CPU timings. [strategy:Surface and rendering at scale] [paraphrase] [inferred]
- **R4 — Stable foliage/ground rendering:** Reproduce and diagnose the observed flicker where foliage overlaps the ground, then eliminate the artifact while preserving correct depth/occlusion and contact appearance. Verify stationary and moving views, low viewing angles and relevant lighting/depth modes. Reduce subpixel needle/leaf noise while preserving aggregate foliage coverage and resolved anatomy; disabling the ground, making foliage always draw on top, or removing foliage is not a fix. [paraphrase] [inferred]
- **R5 — Cross-engine fidelity contract:** Document and demonstrate a portable representation boundary and shared visual/motion tests that can be consumed by the current viewer and external game engines. Each supported rendering backend must meet R1/R2/R4 for the same specimens and comparable camera, resolution and lighting conditions; report backend requirements, resource limits and unsupported capabilities explicitly. A weaker fallback is not an equivalent full-fidelity backend. Performance is measured on each named backend/hardware combination, not inferred from another engine's result. [paraphrase] [inferred]
- **R6 — Portable proof before architectural lock-in:** Prove the critical representation/coverage/continuous-detail assumptions in at least one actual external-engine rendering path, coordinated with fn-17, before declaring the renderer portable. Use the same full-detail references and camera sequences, retain semantic specimen identity, and report visual/performance gaps. Keep a durable reference/inspection mode and reproducible evidence so optimization cannot silently replace the botanical source or hide a rendering regression. [paraphrase] [inferred]

## Fidelity interpretation and validation

The requirement is visually preserved fidelity at every viewing distance, not an obligation to submit every original subpixel triangle on every frame. A finite-resolution image cannot resolve unlimited geometry; a representation may change internally only while preserving the visible result under the agreed error tests, including stable shading and temporal behavior. If the proposed representation cannot meet that contract, record the limitation and investigate another approach instead of substituting a visibly degraded distant tree. Literal mathematical losslessness and a performance guarantee on arbitrary hardware are not claimed. [inferred]

Use the existing full-foliage spruce, oak and source-reference evidence as the fidelity baseline, including connected needle/leaf close-ups, hanging spruce sprays, irregular crown gaps and ground overlap. Do not validate fidelity only on a sparse, reduced-count or favorable single-camera fixture. Pin hardware, backend, resolution, camera paths, seeds, measurement protocol and error tests before comparisons; preserve raw evidence and separate visual acceptance from frame-time acceptance. [paraphrase] [inferred]

## Boundaries

This spec owns rendering representations, continuous detail, temporal stability, the ground-overlap defect and the portable rendering contract. It does not change botanical generation to buy performance, mandate GPU tree generation, require Unreal, or undertake a universal adapter framework for every engine. The first external-engine proof and integration lifecycle coordinate with fn-17; engine selection remains an investigation decision. Both specs must carry the same fidelity requirements. [paraphrase] [inferred]

Full bark/seasonal appearance authoring and wind simulation remain their own roadmap work. This rendering pass must preserve the current reference appearance and define how representation changes preserve future material/animation information, without pretending those systems are already implemented. [inferred]

## Decision Context

Depends on fn-9's representative species and fidelity evidence; does not depend on legendary templates. Treat lightweight single-tree rendering as the first performance milestone, and retain the forest target without making it an excuse to defer that milestone. [paraphrase]

Epic's Nanite Foliage documentation is a research reference: its assemblies reuse detailed parts, and near-pixel aggregate voxel representations address dense foliage with seamless internal representation changes. This reinforces the direction of preserving visible detail rather than drawing all source triangles at every distance. Epic labels that foliage system experimental; evaluate the transferable principles and measure feasibility in our own renderer and external consumers. [inferred; research: https://dev.epicgames.com/documentation/en-us/unreal-engine/nanite-foliage]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R6 | TBD during planning |
