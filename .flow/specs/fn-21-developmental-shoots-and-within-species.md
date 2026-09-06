# Developmental shoots and within-species variation

## Conversation Evidence

> user: "i want this to become sota. i think we can do it. check current uptodate state and screenshots and open specs and evaluate what's needed to get there"
> user: "first give me a steering prompt for the agent overseeing fn-13 and fn-18 to adapt to what you found and then draft next steps from here"
> user: "ok can you $flow-next-capture all these?"

## Goal & Context

Improve crown and shoot plausibility through related developmental causes and controlled variation. Begin with oak and spruce, connecting shoot age, bud fate, vigor and a minimal light response to branch and foliage geometry. [paraphrase]

## Architecture & Data Models

Retain persistent shoot/branch identity and a bounded developmental state. Reuse a small family of foliage shapes, with botanical context influencing shape and placement. Keep botanical state independent of the renderer's chosen representation. [paraphrase]

## Acceptance Criteria

- **R1:** Define and demonstrate a bounded shoot model with age, terminal/lateral bud identity, vigor and dominance that affects extension, branching and survival. Repeated inputs reproduce the supported development; invalid inputs or exhausted resources leave an explicit failure and usable prior state. [inferred]
- **R2:** Compare controlled open-grown and crowded-light scenarios with identical starting specimens, using the benchmark's species references. Differences must arise through subsequent development and remain species-plausible; missing references and unsupported conditions cannot count as validation. [inferred]
- **R3:** Generate reusable foliage-shape variation in proportions and curvature, with context-related orientation and attachment rather than independent jitter alone. Validate connected shoots and preserve species anatomy; detached, degenerate or incorrectly grouped organs fail. [inferred]
- **R4:** Demonstrate coherent variation across declared ages, contexts and held-out seeds while retaining oak/spruce identity. Measure branch/foliage distributions, update costs and memory; retain counterexamples and distinguish structural improvement from rendering changes. [inferred]

## Boundaries

This work supplies the developmental botanical component and a fixed-scenario light-response proof. Fn11 owns whole-specimen timeline controls and replay; fn16 owns broader environmental competition, pruning, breakage and their event histories. These consumers reuse this component rather than implementing separate growth rules. [inferred]

A full ecosystem, soil simulation, broad species catalogue, materials and wind animation are outside this bounded proof. [paraphrase]

## Decision Context

The comparative benchmark supplies species/context acceptance evidence. This work can follow the local woody-anatomy proof without depending on its mesh representation. [paraphrase]

The proposed ownership boundary brings minimal light response into the first developmental proof while leaving wider environment and damage behavior in fn16. Fn11's existing exclusion of environmental competition needs clarification to permit this shared component. [inferred]

## Strategy Alignment

Use one persistent botanical structure from trunk to leaf-bearing twig, with growth and surroundings shaping its development. [strategy:Growth and botanical fidelity]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R4 | TBD during planning |
