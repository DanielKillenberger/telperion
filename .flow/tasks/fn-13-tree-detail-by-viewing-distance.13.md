---
satisfies: [R1, R2, R3, R4, R6]
---
# fn-13-tree-detail-by-viewing-distance.13 Freeze representation protocol and bounded source-union references

## Description
Implement A2: freeze the census-derived protocol and a bounded canonical source-union reference shared by the triangle, aggregate and filtering probes. This is reference preparation, not a tree hierarchy/compiler.

**Size:** M
**Files:** experiments/fn13-rendering/representation-protocol.mjs (new), experiments/fn13-rendering/union-reference.js (new), experiments/fn13-rendering/union-reference-check.mjs (new), experiments/fn13-rendering/run-union-reference.mjs (new), .flow/evidence/fn13/candidates/representation-protocol/**
**Touches:** [experiments/fn13-rendering/representation-protocol.mjs, experiments/fn13-rendering/union-reference.js, experiments/fn13-rendering/union-reference-check.mjs, experiments/fn13-rendering/run-union-reference.mjs, .flow/evidence/fn13/candidates/representation-protocol/**]

### Approach
- Consume task12's measured distributions. Freeze geometry/version hashes, fit region and a distinct held-out tree/region, camera bands and motions, sparse/gap/inside/grazing cases, at least26 unseen viewing directions, source-only union selection, and at most2 triangle levels /3 voxel widths before any candidate scoring. The second specimen is a held-out validation input, not another region search.
- Declare local coverage/shading and temporal diagnostics calibrated on visibly thinned/inflated and temporal-defect controls, output resolution/DPR, sample budgets and convergence, reference memory/work limits, and cost-comparison screens. Ratios and averages cannot hide local visible damage. Never reuse task7's old-noise mismatch as a shipping gate or infer owner approval of converged motion.
- Build only the bounded source-union reference: shared part acceleration plus instance-aware queries over transformed canonical geometry. Sample visibility of the union, including overlap, touching surfaces, gaps and inside rays. No scalar alpha/normal addition or child-level surrogate reference. Small expanded reference fixtures are permitted only with declared bounds/bytes; production per-tree needle expansion is excluded.
- Reuse existing renderer/capture seams without modifying prior protocols or production Wasm. Capture converged static references and named short native/converged sequences needed by later probes. Bound large supersampled targets or tile the reference; respect actual adapter limits and preserve the owner viewer. Hash reference geometry independently of render resolution so paired data proves identical geometry.
- Publish an immutable executable manifest/API contract with optional per-cell axis-fraction diagnostics, never an isotropy gate. If adequate reference convergence or calibration fails, record unavailable and stop; later probes cannot score against it. Stop after the declared fixtures; change a source-selection rule only as an explicit pre-score protocol correction.

### Investigation targets
**Required**
- experiments/fn13-rendering/run-voxel-bake.mjs:31 — paired capture and convergence seams
- experiments/fn13-rendering/voxel-bake.js — bounded source observation and fit limits
- experiments/fn13-rendering/canonical-library.js — shared geometry and instances
- tests/browser/rendering.mjs:11 — fixture hashes and local metrics
- .flow/evidence/fn13/candidates/voxel-bake/PROTOCOL-v1.md — immutable predecessor
- .flow/evidence/fn13/advisory/fable-reassessment-2026-09-07/CONSENSUS.md — comparison contract

### Quick commands
```bash
npm run typecheck
node_modules/.bin/vitest run
node experiments/fn13-rendering/union-reference-check.mjs
node experiments/fn13-rendering/run-union-reference.mjs
```

## Acceptance
- [ ] A versioned pre-score protocol fixes census-derived fixtures, unseen/held-out cases, negative controls, candidate-set limits, local quality bars, reference convergence and bounded cost domains.
- [ ] Bounded union queries match direct canonical geometry on disjoint, overlapping, touching, empty and inside-ray fixtures without production per-tree expansion; all required references converge at the frozen bar.
- [ ] Native references and controls are inspected and hash-linked to geometry/camera manifests; unavailable or mismatched captures are excluded, with no candidate quality claim.
- [ ] Nonfinite/unsupported transforms, missing geometry, partial/canceled reference work, memory/work/adapter limits and device loss cannot publish a qualified partial reference or leak owned resources.

## Done summary
Qualified bounded source-union references and froze the census-derived representation protocol. V3 supplies59 manifest-required cases, dual-channel coverage/shading convergence, both trees' native/converged motion, unseen directions, defect calibration and hash-linked inspection. The runner derives readiness from per-case evidence; missing channel/frame/control, over-budget resources or mismatched visual hashes cannot qualify. All required Quick commands pass on the integrated target. Original v1/v2 failures remain byte-identical. No renderer representation, noise or performance acceptance is inferred.

Implementation:1947f9d,68ceb18,03c56fe via Cursor Grok4.6HighFast; current successful continuation explicitly authorized by owner. V3 result: .flow/evidence/fn13/candidates/representation-protocol/v3/RESULT.md; freeze/receipt/INSPECTION.json and tests are adjacent. Conductor verified all59image hashes,137historical files unchanged, actual59image-reader calls, representative images, and integrated software gates.

stage: implementation - ran (model: cursor-grok-4.6-high-fast)
stage: impl-review - skipped(config: owner requested none)
stage: plan-sync - skipped(config: disabled)
stage: wave-join - ran (worker commits fast-forward integrated; target checks passed)

Runtime status restored on 2026-09-08 from this committed completion receipt; historical validation and scope above are unchanged.
## Evidence
- Commits: 1947f9df6652bac46393c2b7b87c1dcaa8c56d81, 68ceb18ba5e6ff4908d859b35ea2712eb475c24b, 03c56fe466a42f8a275a4fde33bd5f13df361541
- Tests: Integrated npm run typecheck: exit0, Integrated node_modules/.bin/vitest run:119 passed, Integrated node experiments/fn13-rendering/union-reference-check.mjs: exit0, Integrated node experiments/fn13-rendering/run-union-reference.mjs: exit0,59 required artifacts qualified via validated replay, Original v3 live capture:59 cases, both channels/every required motion frame; no numeric errors;766537728 bytes observed RSS;52.3 seconds, Integrity:137 historical files unchanged;59 PNG hashes match;59 direct image reads confirmed in Cursor session log, Actual implementation model: cursor-grok-4.6-high-fast; Cursor session fa4c4dc6-2b84-4e85-8360-0894ae64bb23
- PRs: