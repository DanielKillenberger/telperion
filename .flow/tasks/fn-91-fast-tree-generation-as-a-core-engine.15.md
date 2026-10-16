---
satisfies: [R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.15 Use resident GPU generation in the interactive renderer

## Description
Use resident GPU generation in the interactive renderer.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
The mature harness builds through `setTreeGpu` via a per-stage serial queue that coalesces pending requests, suppresses callbacks after cancel or dispose, and frames the camera only for the current build; growth path unchanged. `harness/latest-build.test.ts` 4/4.

## Evidence
- Commits: 85f4731147a8dbafc2d1179e77291e72f17c10d5
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs: https://github.com/DanielKillenberger/telperion/pull/50
