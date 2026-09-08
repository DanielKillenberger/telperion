---
satisfies: [R1, R3, R6]
---
# fn-13-tree-detail-by-viewing-distance.12 Measure projected needle detail and draft the probe protocol

## Description
Produce the bounded A1 projected-detail census and draft protocol. This task observes the task-8 library; it selects no representation and captures no new GPU benchmark.

**Size:** M
**Files:** experiments/fn13-rendering/projected-needle-census.mjs (new), experiments/fn13-rendering/projected-needle-census-check.mjs (new), .flow/evidence/fn13/candidates/projected-census/**
**Touches:** [experiments/fn13-rendering/projected-needle-census.mjs, experiments/fn13-rendering/projected-needle-census-check.mjs, .flow/evidence/fn13/candidates/projected-census/**]

### Approach
- Freeze seed1/seed42 fingerprints, existing near/whole/far cameras, units and a source-only natural-region rule before collecting results: largest non-trunk branch by bearing-span count and the median-height eligible mid-crown branch, with stable identity tie-breaks. Record empty/ambiguous selections explicitly. These are finite fixtures, not an invitation to search for favorable regions.
- Evaluate actual per-needle geometry/bounds from each shared part through its rigid instance transform. Report population-weighted projected needle-width and spacing distributions, triangle/instance counts and natural-region extents. Part maximum extent or unweighted library prototypes cannot substitute for needle detail. Declare the spacing estimator, perspective/near-plane handling and visibility scope; in-frustum counts are not unoccluded counts or measured GPU work.
- Reuse canonical-library ownership without retaining full per-tree expanded needle matrices. Before scanning, declare iteration, resident-memory and wall-time caps. A bounded deterministic stratified sample is permitted if its weights, strata, sample-doubling convergence and uncertainty are recorded; unavailable precision stays explicit. Do not raise caps after observing favorable candidate results.
- Produce a draft protocol with candidate bands, at most two triangle tessellation levels and three union-cell widths suggested by measured detail, plus fixture and reference requirements for task13. Draft suggestions are not frozen quality thresholds or passes. Label prior task7/8 timings as historical samples with their original scope.
- Stop when the valid census and draft exist. If source geometry, projection or sampling cannot support them within the declared limits, retain an unavailable receipt and leave this prerequisite incomplete.

### Investigation targets
**Required**
- experiments/fn13-rendering/canonical-library.js:33 — shared part and tree-instance data
- crates/telperion-core/examples/species_metrics/mod.rs:176 — transformed geometry/count metrics
- tests/browser/fixtures/rendering.json — existing camera and native-output identities
- .flow/evidence/fn13/candidates/canonical-library/DECISION.md — fit, ownership and memory limits
- .flow/evidence/fn13/advisory/fable-reassessment-2026-09-07/CONSENSUS.md — A1 boundary

### Quick commands
```bash
npm run typecheck
node_modules/.bin/vitest run
node experiments/fn13-rendering/projected-needle-census-check.mjs
node experiments/fn13-rendering/projected-needle-census.mjs
```

## Acceptance
- [ ] Two fingerprint-distinct trees and all six camera cases have reproducible weighted width/spacing/count receipts with units, region identities, visibility limitations and declared sampling uncertainty.
- [ ] Actual instanced needle geometry drives measurements without full per-tree needle-matrix storage; tiny analytic fixtures check projection, population weights, clipping, near-plane and empty cases.
- [ ] Invalid/nonfinite geometry or cameras, unsupported transforms, missing parts, cancellation and budget exhaustion yield explicit invalid/unavailable results without partial accepted census data or leaked shared ownership.
- [ ] A draft protocol and finite follow-up candidate suggestions are recorded; no representation, noise, performance or task9 pass is inferred from this census.

## Done summary
The prospective v3 depth/quadrant census supports all six fixed camera cases under the unchanged sample-doubling bars and caps. The single run and supported draft are in `.flow/evidence/fn13/candidates/projected-census/RESULT.md`; old freezes, runs, regions and historical documents remain unchanged. No representation, visual or performance qualification is inferred.

Baseline: typecheck,119 Vitest and analytic green; expected unavailable v2 measurement reused from prior immutable receipt. Final task12 Quick commands all pass, with full validation and integrity evidence in `validation/SPATIAL-FINAL.json`. Previous parent browser evidence remains historical; no GPU/renderer change. Gate receipt writes declined the dirty precommit tree (nonblocking); logs record successful actual exits. Done is verified before the implementation commit as requested; the handover evidence receives the resulting commit ID afterward.

stage: impl-review - skipped(config: owner requested none)
stage: plan-sync - skipped(config: disabled)

Runtime status restored on 2026-09-08 from this committed completion receipt; historical validation and scope above are unchanged.
## Evidence
- Commits:
- Tests: npm run typecheck, node_modules/.bin/vitest run (119 passed), node experiments/fn13-rendering/projected-needle-census-check.mjs, node experiments/fn13-rendering/projected-needle-census.mjs (valid six views; single frozen v3 run), integrity: old evidence unchanged, population conserved, unchanged convergence bars all pass
- PRs: