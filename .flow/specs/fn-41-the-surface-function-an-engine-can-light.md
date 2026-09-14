# The surface function an engine can light

## Conversation Evidence

> user (turn 1, during fn-32's hill climb): "btw is this compatible to export the materials to various engines? how will that work?"
> user (turn 2, on the host's answer that the rows are portable, the shader functions transpile through naga, and one split is missing, the surface from our own lighting): "so should we capture that split?"
> user (turn 3, on the proposed contents: the split, a naga emission gate, crown depth and seed as exported attributes, the bake as a stated fallback, no engine-specific material): "capture it"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 50% [paraphrase], 30% [strategy] -->

The owner asked whether the bark and leaf materials fn-14, fn-26, fn-29 and fn-32 built can be exported to other engines, and how that would work. [user] The answer today is: partly. Every material term is a numeric row on the family, so the parameters already travel on the wire and blend anywhere. The shading is written as pure functions of the vertex coordinates the mesh carries and of the row uniforms, in WGSL, which the renderer's own toolchain can emit as HLSL, GLSL, Metal and SPIR-V. What does not travel is the last step: the wood and blade fragments end in a function that mixes the surface with this renderer's sun, sky hemisphere, shadow lookup and sheen. An engine lights surfaces itself and wants the surface's inputs, not our light. [paraphrase]

This spec makes that split: a surface function per surface kind that returns what any physically based pipeline consumes, and a gate that proves the function is expressible in the engine languages, without an engine in the loop. It is the first step of the Unreal proof the strategy schedules, taken now while the shader contract is fresh from fn-32. [paraphrase] The strategy asks exactly this: portable tree and rendering data separate from engine-specific drawing and material implementations, so future engines preserve the same fidelity without an external-engine proof in every rendering pass. [strategy:The core and integration]

## Architecture & Data Models
<!-- scope: technical -->
<!-- Architecture: 70% [paraphrase], 30% [inferred] -->

- **One surface function per surface kind.** `wood` and `blade` each get a function of the vertex coordinates (world position, normal, the circle-embedded arc and along coordinates and radius on wood; blade coordinates, facing and the placement seed on leaves) and the row uniforms, returning base colour, the perturbed normal, roughness and occlusion. Nothing inside it reads a light, a shadow map or the sky. The renderer's fragment stages become consumers: they call the surface function and then apply this renderer's lighting to its outputs, so every picture stays byte-identical. [paraphrase]
- **What stays on the lighting side.** The sun, the sky hemisphere, the shadow lookup, transmission through the blade, the sheen lobe, tone mapping, and sky occlusion by crown depth. Crown depth and the per-leaf seed leave the lighting pass as attributes an engine can read, so interior darkening and per-leaf identity are reproducible from the mesh contract rather than from our pass. [paraphrase]
- **The emission gate.** A test that hands the surface modules to naga and emits HLSL, GLSL and Metal, failing on any construct one target cannot express. It runs without a device and without an engine, in seconds, and is the honest form of the claim "this material runs elsewhere" until an engine proof exists. [paraphrase]
- **The binding table.** The mapping from rows to uniform slots is already generated from the wire macro for the browser; the same generation emits a table an engine binds against, so nobody reads WGSL to find which vec4 holds cavity strength. [inferred]
- **Footprint filtering travels.** The distance and resolution contract is a property of the surface function, built on screen-space derivatives every shading language has; it moves with the function unchanged. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Surface function signatures**, one per kind, taking the vertex coordinates and the uniform block and returning a struct of base colour, normal, roughness and occlusion; no other output and no light input. Existing rows are unchanged; no new row is added by this spec. [inferred]
- **Exported attributes.** Crown depth per wood vertex and the placement seed per leaf instance appear in the engine-neutral mesh and instance output alongside the coordinates already exported; the browser and native consumers may ignore them. [paraphrase]
- **Generated binding table** listing every row with its uniform slot, component and range, produced by the same build step that produces the browser preset mirror. [inferred]
- **Views and commands unchanged.** [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Byte identity.** Every pinned still, the redraw check and the clay room must be byte-identical before and after the split; the split is a refactor, and the pictures are its test. [paraphrase]
- **Cost.** The native oak frame and the browser orbit stay within run-to-run noise of their pre-split numbers; the split moves code, not work. [inferred]
- **Emission strictness.** The gate emits with the targets' strict profiles; a construct that emits only under a relaxed profile counts as a failure, since an engine will not relax it. [inferred]
- **No engine in the loop.** The gate proves expressibility, not appearance in an engine; the Unreal proof is separate work. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Wood and blade shading each pass through one surface function that returns base colour, perturbed normal, roughness and occlusion from the vertex coordinates and the row uniforms only, with no light, shadow or sky read inside it, and the renderer lights those outputs. Errors: a surface function that reads any lighting uniform fails the criterion. [paraphrase]
- **R2:** Every pinned still, the redraw check and the clay room are byte-identical before and after the split, and the native oak frame and the browser orbit stay within run-to-run noise of the pre-split numbers, recorded beside them. Errors: any changed pixel or a cost move outside noise stops the spec with the number. [paraphrase]
- **R3:** A device-free test emits the surface modules through naga to HLSL, GLSL and Metal under strict profiles and fails on any construct a target cannot express. Errors: an emission failure names the construct and the target. [paraphrase]
- **R4:** Crown depth per wood vertex and the placement seed per leaf instance are present in the engine-neutral mesh and instance output, asserted by a core test, with existing consumers unchanged. Errors: no error surface beyond the existing mesh validation. [paraphrase]
- **R5:** The row-to-uniform binding table is generated by the existing build step and lists every row with its slot, component and range, kept in sync by a test that fails when a row is added without an entry. Errors: a row missing from the table fails the test naming the row. [inferred]

## Boundaries
<!-- scope: business -->

- No engine-specific material, plugin or import path; the engine proof consumes this contract and is its own spec. [paraphrase]
- No new appearance term and no row change; the split moves code, and fn-32 owns the bark's look. [paraphrase]
- The bake, offline evaluation of the surface function to albedo, normal and occlusion maps for hosts that cannot run custom shaders, is named here as the fallback path and not built. [paraphrase]
- Lighting stays this renderer's; sky occlusion, shadow and transmission are not made portable by this spec beyond the attributes they need. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation
<!-- scope: business -->

- The owner's question was whether the materials can be exported to various engines, and the capture followed the host's answer that one split makes them so. [paraphrase]
- Taken now, while the shader contract is fresh from fn-32 and before an engine proof forces it under pressure. [paraphrase]

### Implementation Tradeoffs
<!-- scope: technical -->

- A surface function over a per-engine rewrite: the functions are pure and already transpile through the toolchain in hand, so one source serves every target, which is the strategy's no-engine-specific-material rule. [paraphrase]
- An emission gate over an engine proof as the acceptance: the gate costs seconds and no engine licence, and catches the constructs that would fail at import; appearance in an engine is judged when the engine proof runs. [paraphrase]
- Attributes exported over lighting exported: an engine will not take our sun and sky, but it can take a crown depth and a seed and reproduce interior darkening and identity its own way. [paraphrase]

## Parked unknowns

- Whether roughness alone carries the sheen lobe's information to an engine's specular model, or a second scalar is needed; the first emitted material in the engine proof decides it.
- Which engine is the first proof, Unreal per the strategy, and therefore whether HLSL strict profile is the binding target or all three are equal.

## Strategy Alignment

- Follows "The core and integration": portable tree and rendering data stay separate from engine-specific drawing and material implementations, keeping future engines able to preserve the same visual fidelity without requiring an external-engine proof in every rendering pass.
- Follows "Surface and rendering at scale": rendering techniques generalize across generated trees and leaves through shared geometry and data contracts.
