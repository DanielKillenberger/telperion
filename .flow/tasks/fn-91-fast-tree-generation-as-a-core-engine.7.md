---
satisfies: [R2, R3, R4]
---
# fn-91-fast-tree-generation-as-a-core-engine.7 Probe GPU wood positions from compact ring inputs

## Description
Probe GPU wood positions from compact ring inputs.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
Feasibility probe retained: all four fixtures pass numeric, topology, bounds and repeatability screens; oak position preparation 2.70x/2.83x faster with lower counted allocation. Far-origin 100 km geometry unsupported (11.05 mm max error). Host approved integration in .8.

## Evidence
- Commits: 64eba9563205a30f3663fa17c8a35b283d5b64b7
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs:
