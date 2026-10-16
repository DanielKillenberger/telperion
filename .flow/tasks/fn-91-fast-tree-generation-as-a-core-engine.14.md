---
satisfies: [R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.14 Streamline shared foliage station preparation toward the oak milestone

## Description
Streamline shared foliage station preparation toward the oak milestone.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
Retained shared station streaming and zero-contact position-admission overlap. Browser 158.1/180.8 ms oak (9.93x/10.30x), 178.9/164.2 ms spruce; oak seed 1 misses strict 10x by 1.1 ms and the host stops there. Station records match exactly. Report: `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/stations/REPORT.md`.

## Evidence
- Commits: 1c34c6b14bcdd27e859ca7292352de7d41587664
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs:
