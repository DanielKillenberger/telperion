---
satisfies: [R2, R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.4 Expand wood into resident GPU buffers from canonical positions

## Description
Expand wood into resident GPU buffers from canonical positions.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
Resident wood expansion implemented for native and browser GPU-resident rendering; CPU-owned output stays on the CPU builder. The owner accepted the lit exterior CPU/GPU wood comparison as visually identical. Report: `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/resident-wood/REPORT.md`.

## Evidence
- Commits: 8f025f806251768a684d424742dec4565c930ef9, 990aa2df4dbddb5450c58f6a1e46464ec10f6459, 43bdb21a
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs:
