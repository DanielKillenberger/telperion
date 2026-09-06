---
satisfies: [R1, R5]
---
# fn-13-tree-detail-by-viewing-distance.3 Build portable rendering data and explicit resource ownership

## Description
Turn the proven candidate into portable derived data and adapt the current Three materialization seam.

**Size:** M
**Files:** src/browser/render-data.ts (new), src/browser/render-data.test.ts (new), src/browser/three.ts, tests/browser/integration.mjs
**Touches:** [src/browser/render-data.ts, src/browser/render-data.test.ts, src/browser/three.ts, tests/browser/integration.mjs]

### Approach
- Implement only task 2's selected data contract. Reuse TreeOutput; keep generation rules and public source outputs intact. Any necessary native-output packing change must be scoped and coordinated before touching fn12-owned code.
- Carry source/version identity, conservative cluster/specimen bounds, coverage/material/error data and deterministic near-detail recovery. Share parts with explicit owners and per-specimen transforms; avoid full expanded arrays for all forest residents.
- Adapt materializeTree rather than creating a second competing Three ownership model. Preserve a direct full-detail path and avoid recomputeInstanceBounds's vertex-by-instance scan where proven conservative bounds exist.
- Pin equivalence by replaying task 1's source identities and full-detail image references against the new adapter; source hashes must remain identical.
- Cover empty/malformed/nonfinite data, source/version invalidation, interrupted construction, shared-part lifetime, scene replacement and dispose/rebuild loops. Do not invent public serialization/export or a universal backend API.

### Investigation targets
**Required**
- src/browser/core.ts:20 — portable output types
- src/browser/core.ts:67 — Wasm release/dispose semantics
- src/browser/three.ts:9 — materializeTree
- src/browser/three.ts:39 — disposeTreeGeometry
- harness/skeleton-view.ts:342 — copy/release/materialize lifecycle
- tests/browser/integration.mjs — existing browser contracts

### Quick commands
npm run typecheck
npx vitest run src/browser/render-data.test.ts
npm run test:browser

## Acceptance
- [ ] The selected numeric representation preserves source identity and full-detail replay; no Three objects enter the portable data contract.
- [ ] Bounds contain rendered geometry at every supported representation and placement; no stale-bound disappearance in the task 1 cases.
- [ ] Failure/replacement/disposal tests prove shared resources survive unrelated tree removal and partial allocations are released.
- [ ] Preparation and resident memory remain within task 2's measured design limits; deviations require resolving the candidate decision before continuing.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
