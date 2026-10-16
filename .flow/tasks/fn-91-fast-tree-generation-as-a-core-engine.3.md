---
satisfies: [R1, R2, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.3 Measure wood construction stages before selecting the next GPU boundary

## Description
Measure wood construction stages before choosing the next GPU boundary.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
Native release wood stages measured for oak/spruce seeds 1/7 with scratch-only instrumentation; all 32 builds match surface hashes. Instrumentation overhead of 6.5-21.3% limits exact attribution. Pointed the next candidate at combined index/normal/attribute expansion and upload.

## Evidence
- Commits: 92d1c025641de2efccdce73bb2e79cc34994ed09
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs:
