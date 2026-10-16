---
satisfies: [R2, R3, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.9 Attribute remaining shared CPU preparation costs

## Description
Attribute remaining shared CPU preparation costs.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
Coarse phase timers in a scratch checkout: native oak local advance 50.56/56.16 ms dominates; repeated early radius solves 0.19/0.25 ms; sampled planner 23.14/22.60 ms. Production unchanged; all 32 canonical hashes match. Next screen: radius-only ordering (.10).

## Evidence
- Commits: 84d33bd73c20d99aa0a98b05157a04ef134fc337
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs:
