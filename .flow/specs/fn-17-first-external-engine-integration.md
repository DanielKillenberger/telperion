# First external engine integration

## Conversation Evidence

> user: "those specs are excellent can you $flow-next-capture them?"

This approves the preceding seven-item roadmap, including this outcome. [paraphrase]

> user: "could we export a high fidelity tree and render it in unreal with nanite and it'll handle it well?"
>
> user: "that should be another requirement we still want our own fast renderer"

## Goal & Context

Export high-fidelity Telperion trees for Unreal Engine and render them through Nanite, using the representations that consumer needs. This is an additional capability alongside Telperion's own fast renderer, which remains the separate fn13 goal. [paraphrase]

## Architecture & Data Models

Keep one lean, engine-independent core with small boundaries between botanical state and requested representations. [strategy:The core and integration]

## Acceptance Criteria

- **R1:** Use Unreal Engine with Nanite as the first concrete target. Demonstrate a reproducible path from a generated Telperion tree through export/import to rendering in Unreal; document the supported engine/features and unsupported capabilities. [paraphrase]
- **R2:** Export the structure, wood and foliage representations the Unreal consumer needs without constructing unrelated representations. Preserve specimen identity and useful reusable structure so engine-specific preparation can organize detailed parts efficiently. [strategy:The core and integration]
- **R3:** Demonstrate ownership, updates and disposal, with explicit handling of generation failures, invalid requests and resource limits. Measure complete integration latency and memory on a reproducible scene. [inferred]

- **R4:** The exported tree retains the source botanical character and human-perceived high fidelity from close inspection to a distant crown under Nanite's view-dependent representation. Verify appearance and report rendering performance and memory on declared hardware; enabling Nanite alone is not evidence that the result performs well. Unreal export supplements rather than replaces Telperion's own fast renderer. [paraphrase]

## Boundaries

One concrete integration first; a universal adapter framework and simultaneous support for multiple engines are outside this pass. [inferred]

## Decision Context

Unreal/Nanite is the selected target; the first bounded investigation chooses a suitable export and asset-preparation workflow. Preserve reusable parts where suitable and evaluate current Nanite foliage capabilities rather than assuming one expanded mesh will scale. This uses the shared core and does not inherently depend on legendary trees, materials or GPU acceleration. Coordinate required growth/detail support with the chosen consumer. [inferred]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R4 | TBD during planning |
