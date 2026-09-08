---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-13-tree-detail-by-viewing-distance.10 Build interactive cross-part detail and shared residency

## Description
Make the next outcome a navigable spruce/oak scene with real detail reduction and loading, reusing the current viewer. Compile branch/crown/tree levels above individual shared-part roots so distant cost falls with screen-space detail. First deliver one useful cross-part coarse level for the demanding spruce hero view, with an inspected before/after and measured complete cost; per-part decimation is optional only if its measured cost warrants it. Preserve screen-resolved near anatomy while aggregating unresolved interior foliage. if a representation cannot scale, make the smallest measured change here rather than opening another feasibility project.

Support multiple geometry handles, bounded shared residency, view selection and atomic replacement through one wood/foliage/ground depth path. Keep already-valid coverage while detail arrives; reject stale/canceled uploads and protect resources used by another tree. Inspect a short navigation clip through the coarse level while selecting the representation; shimmer or popping must be no worse than the current viewer and cannot be left as an unknown until task4. Add only the filtering necessary to assess the actual geometry fairly; task4 finishes temporal/loading polish. Measure actual selected workload and complete vegetation cost on two different trees early. No whole-tree foliage expansion for every forest resident.

**Size:** L (deliver incrementally: hero coarse level, then two-tree selection/residency)
**Files:** src/browser/assembly-hierarchy.ts, src/browser/tree-compiler.ts, experiments/fn13-rendering/tree-compile-check.mjs, src/browser/geometry-residency.ts, experiments/fn13-rendering/webgpu-budget.js, experiments/fn13-rendering/live-assembly.js, experiments/fn13-rendering/live-assembly.html, experiments/fn13-rendering/compact-wood.ts, .flow/evidence/fn13/candidates/two-tree-compile/**
**Touches:** [src/browser/assembly-hierarchy.ts, src/browser/tree-compiler.ts, experiments/fn13-rendering/tree-compile-check.mjs, src/browser/geometry-residency.ts, experiments/fn13-rendering/webgpu-budget.js, experiments/fn13-rendering/live-assembly.js, experiments/fn13-rendering/live-assembly.html, experiments/fn13-rendering/compact-wood.ts, .flow/evidence/fn13/candidates/two-tree-compile/**]

Write compiler/hierarchy/residency in reusable src/browser TypeScript and import them from the experiment viewer; subsequent tasks wire these modules, not port a second renderer. Validate the GPU timing path once with a known-cost draw using supported timestamp queries; unsupported/zero/invalid or GPU-contended timing is unavailable, never a pass. Record per-tree coarse preparation time and memory and a simple 1,000-tree estimate to reject obviously infeasible preparation before task5; this estimate is not a forest performance claim. If triangle aggregation fails, consider placement-derived cluster points/cards with filtered coverage as a small alternative tied to the observed failure.

First milestone is spruce only; introduce oak for the subsequent two-tree milestone. Capture the existing noisy viewer baseline clip before hierarchy changes, using the same camera and parameters for the coarse-level before/after.

## Acceptance
- [ ] The existing interactive route renders spruce and oak through the same hierarchy/draw path, with useful cross-part coarse levels and recoverable plausible near detail; representative images and a navigation clip are inspected.
- [ ] The hero-view coarse level improves measured work/cost with convincing inspected appearance and a short motion clip no worse in shimmer/popping than the current viewer; remaining defects are explicit.
- [ ] Distance reduces actual selected work/residency above individual-part roots. Report total vegetation timing and preparation/memory for the two-tree scene, including any remaining misses/defects; do not extrapolate a forest pass.
- [ ] Delayed detail, replacement, cancellation, budget pressure and disposal preserve valid coverage/shared owners; scene depth remains correct. The implementation supports the third data-only topology without a renderer-specific branch.

## Quick commands
```bash
npm run typecheck
node experiments/fn13-rendering/tree-compile-check.mjs
```
Run focused checks during implementation; broader checks once on the integrated path. Reuse valid unchanged expensive evidence.

## Done summary
Completed the task10 interactive milestone: geometry-derived planar/3D component supports remove close aggregate blobs, bounded local selection keeps cross-part distant work, and previous exact-region retention fixes the inspected approach/reversal within the existing65,536-instance cap. Spruce/oak stills, oak small motion and the0.6m spruce approach/reversal are inspected; final renderer fidelity, broader navigation polish and uncontended performance acceptance remain tasks4–5.

Evidence: `.flow/evidence/fn13/candidates/two-tree-compile/shape-support/RESULT.md`, `NAVIGATION.json`, `RESULT.json`, `approach-reversal.mp4`. Baseline and final typecheck/compile checks green. Shape regressions failed before the corresponding fixes. Existing third-topology, source-cap, cancellation, shared-owner, atomic-budget and disposal checks remain green. No new exact-source cap or species branch.

stage: impl-review - skipped(owner policy: REVIEW_MODE=none)
stage: plan-sync - skipped(config false)

Runtime status restored on 2026-09-08 from this committed completion receipt; historical validation and scope above are unchanged.
## Evidence
- Commits: `af3223f086cbbab560830a6771996a0585392bb2`
- Tests: baseline: green (npm run typecheck; node experiments/fn13-rendering/tree-compile-check.mjs), npm run typecheck, node experiments/fn13-rendering/tree-compile-check.mjs, node --check experiments/fn13-rendering/live-assembly.js, node --check experiments/fn13-rendering/webgpu-budget.js, node .flow/evidence/fn13/candidates/two-tree-compile/shape-support/navigation.mjs, node .flow/evidence/fn13/candidates/two-tree-compile/shape-support/runner.mjs, Observed red-to-green disconnected support and nonplanar thickness regressions, Pending/settled reversal PNG SHA256 equality;556 retained exact instances <=65536, GPU performance acceptance unavailable: desktop contention; no pass
- PRs: