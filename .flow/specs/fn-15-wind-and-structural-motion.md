# Wind and structural motion

## Conversation Evidence

> user: "those specs are excellent can you $flow-next-capture them?"

This approves the preceding seven-item roadmap, including this outcome. [paraphrase]

## Goal & Context

Produce coherent movement from trunk through branches to leaves, using the tree's structural hierarchy. [paraphrase]

## Architecture & Data Models

Keep one lean, engine-independent core with small boundaries between botanical state and requested representations. [strategy:The core and integration]

## Acceptance Criteria

- **R1:** Wind drives connected motion through the hierarchy while foliage stays attached; no error surface beyond R2. [paraphrase]
- **R2:** Zero wind preserves the rest shape. Changing wind and advancing time remain stable; invalid inputs and unsupported motion ranges are handled explicitly. Validate continuity, extreme settings and repeated playback. [inferred]
- **R3:** Demonstrate motion on contrasting species and measure update, GPU and memory costs without changing the underlying specimen identity. [inferred]

## Boundaries

Wind-induced breakage and growth adaptation belong to environment/damage work; full physical simulation is not a prerequisite for convincing first-pass motion. [inferred]

## Decision Context

Depends on Real-species profiles and procedural templates for the structural hierarchy and validation subjects. Coordinate with growth-state changes; aging need not be complete before static-tree motion works. [inferred]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R3 | TBD during planning |
