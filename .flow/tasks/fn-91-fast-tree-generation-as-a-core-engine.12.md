---
satisfies: [R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.12 Screen prepared envelope admission bounds

## Description
Screen prepared envelope admission bounds.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
Rejected: native oak 24.22/14.39% faster but browser oak only 1.31/3.92%, spruce regressions and an unexplained 41,824 KiB GPU-assisted oak RSS rise. Production and browser module restored exactly.

## Evidence
- Commits: 812df7be578b7b7e70c01066165e3e3af0c112f6
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs:
