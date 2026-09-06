---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-13-tree-detail-by-viewing-distance.2 Prove a coverage-preserving rendering candidate

## Description
Run a bounded current-viewer experiment and record the selected representation before production integration.

**Size:** M
**Files:** experiments/fn13-rendering/ (new), .flow/evidence/fn13/candidates/
**Touches:** [experiments/fn13-rendering/**, .flow/evidence/fn13/candidates/**]

### Approach
- Use task 1's frozen rig in an isolated experimental entrypoint; retain full source and neutral reference output. Begin with current WebGL capabilities.
- Compare baseline, conservative spatial visibility/bounds and a coverage-preserving aggregate candidate. Do not implement a general renderer framework. Identify whether geometry submission, foliage overdraw, wood or preparation dominates.
- A/B actual depth modes and supported MSAA/coverage settings; do not select alpha hashing or temporal filtering on a still-image win alone. Record effective capabilities and all vegetation passes.
- Test mature spruce and oak on fixed and holdout camera paths, including needle/twig attachment, crown gaps, shading, ground and detail reversals. Full-detail source is the identity reference; candidates cannot change generator rules/counts.
- Measure resident representation bytes, per-distinct-specimen marginal cost and cold packing on a 1/8/32-specimen varied sample. Bound expected 1,000-tree residency against actual machine memory with declared headroom; projections are feasibility evidence, not the forest result.
- Commit a small design decision: representation/error semantics, stable IDs, bounds, near-detail recovery, resource ownership and capability fallback. Explain what is transferable to other engines without adding one.
- If no candidate clears the hero budget and frozen visual tests with plausible residency, record failure and replan before task 3. Keep the old viewer functional; never mark an unsuccessful feasibility result as implemented rendering.

### Investigation targets
**Required**
- src/browser/three.ts:9 — materialization and prototype ownership
- src/browser/three.ts:47 — expensive exact instance bounds
- src/browser/core.ts:41 — TreeOutput and ownership
- harness/stage.ts:419 — graphics setup and depth/coverage controls
- tests/browser/rendering.mjs — task 1 acceptance rig (created dependency)
**Optional**
- https://dev.epicgames.com/documentation/en-us/unreal-engine/nanite-foliage
- https://threejs.org/docs/pages/WebGLRenderer.html

## Acceptance
- [ ] Comparable raw visual/motion/GPU results cover the baseline and bounded candidate set without changing task 1 tolerances.
- [ ] A selected candidate meets the 2 ms p95 hero target and visual gates on the pinned hardware, or the task explicitly fails feasibility and downstream integration remains blocked.
- [ ] Varied-specimen memory/preparation measurements support the proposed forest strategy; no duplicate whole-tree substitution or projected forest pass.
- [ ] The decision specifies data/lifetime and missing-capability behavior, and identifies any depth artifact still requiring task 4.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
