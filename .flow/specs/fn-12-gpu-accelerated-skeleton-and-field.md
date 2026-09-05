# GPU-accelerated skeleton and field generation

## Conversation Evidence

> user: "those specs are excellent can you $flow-next-capture them?"

This approves the preceding seven-item roadmap, including this outcome. [paraphrase]

## Goal & Context

Investigate GPU acceleration of skeleton generation and field construction/querying, then implement the workloads that demonstrate worthwhile end-to-end gains. [paraphrase]

## Architecture & Data Models

Keep one lean, engine-independent core with small boundaries between botanical state and requested representations. [strategy:The core and integration]

## Acceptance Criteria

- **R1:** Profile representative ordinary and giant workloads and compare CPU and GPU candidates with equivalent inputs and requested outputs, including transfer, synchronization and memory costs. Unsupported hardware and inconclusive results are reported explicitly. [paraphrase]
- **R2:** Integrate accelerated paths only where measurements justify them; compare geometry, field behavior and repeatability against the core reference. A negative benchmark is a documented result, not a requirement to ship a slower GPU path. [inferred]
- **R3:** Define supported hardware, precision and determinism guarantees. Unavailable devices, failed execution and resource exhaustion have a documented fallback or explicit recoverable failure; no silent partial success. [inferred]

## Boundaries

Rendering optimization is a separate spec. This work does not assume every generation stage belongs on the GPU or select a GPU API in advance. [inferred]

## Decision Context

Experiments can start against the current core. Production integration must account for the evolving growth-state contract; completion of the growth or legendary-tree specs is not required for profiling. [paraphrase]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R3 | TBD during planning |
