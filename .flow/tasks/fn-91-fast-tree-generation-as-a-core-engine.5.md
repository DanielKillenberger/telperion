---
satisfies: [R2, R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.5 Reduce division-heavy wood admission traversal

## Description
Reduce division-heavy wood admission traversal.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
Retained an allocation-free nested ring/segment admission traversal with exact prepared fields and CPU hashes: preparation 9.69-11.97% faster, native completed-frame 3.51/4.15% (below the 5% aim).

## Evidence
- Commits: ef52e093347544a39640ced5306339b5e69d5cc9
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs:
