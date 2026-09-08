---
satisfies: [R2, R3, R4, R6]
---
# fn-13-tree-detail-by-viewing-distance.7 Correct GPU measurement and minimal batched submission

## Description
Implement consensus stage 0 before judging the hybrid candidate. Keep this task limited to measurement, submission and the depth behavior necessary for valid comparisons.

**Size:** M
**Files:** experiments/fn13-rendering/webgpu-budget.js, experiments/fn13-rendering/measure.js, experiments/fn13-rendering/webgpu-scene.ts, experiments/fn13-rendering/run-pipeline-baseline.mjs (new), experiments/fn13-rendering/pipeline-check.mjs (new), experiments/fn13-rendering/budget-cache-check.mjs, experiments/fn13-rendering/live-budget.js, experiments/fn13-rendering/live-assembly.js, experiments/fn13-rendering/live-assembly.html, experiments/fn13-rendering/live-selection.js, experiments/fn13-rendering/live-shoot.js
**Touches:** [experiments/fn13-rendering/webgpu-budget.js, experiments/fn13-rendering/measure.js, experiments/fn13-rendering/webgpu-scene.ts, experiments/fn13-rendering/run-pipeline-baseline.mjs, experiments/fn13-rendering/pipeline-check.mjs, experiments/fn13-rendering/budget-cache-check.mjs, experiments/fn13-rendering/live-budget.js, experiments/fn13-rendering/live-assembly.js, experiments/fn13-rendering/live-assembly.html, experiments/fn13-rendering/live-selection.js, experiments/fn13-rendering/live-shoot.js, .flow/evidence/fn13/candidates/pipeline/**]

### Owner inspection and resumed scope — 2026-09-07
The owner inspected the retained playback and the interactive reversed-depth assembly viewer. Feedback: foliage looks convincing but noise is objectionable even in stills and worsens in motion; interactive rendering feels fast; visible detail chunks still load in. The owner agreed to finish task7's measurement/batching/depth checks and address foliage filtering in tasks8–10 and visible transitions in task11, then explicitly resumed work. This is scoped feedback, not a noise-free, source-equivalent-motion or GPU-budget acceptance verdict.

Resolve task7 qualification using the preserved source/candidate geometry and camera hashes, inspected matched stills, hardware depth/material/occlusion fixtures, delayed timing/resource tests and explicit owner observations. Keep the0.015 historical paired diagnostic as failed and preserve the isolated MSAA evaluation difference; reproducing the old subpixel noise pattern is not a task7 release condition under the existing perceptual contract. Demonstrate the selected explicit reversed-depth path is functionally correct without unconditional fragment-depth output; document unresolved static noise/shimmer and chunk loading for downstream qualification. Source depth remains available for comparison. Do not claim that task7 satisfies finalR1/R2/R4 quality or task2 feasibility. The new live depth selector is part of inspection support and must preserve camera/seed across comparison changes.

### Approach
- Migrate the four existing budget-renderer live callers to delayed sample results, matching original frame IDs with bounded history. Pending/unavailable GPU timing stays explicit; label first submission separately from actual GPU completion/readiness. Preserve camera/selection/content behavior and archived measurement receipts; verify these callers remain usable under the new timing contract.
- Maintain budget-cache-check.mjs as an active regression: update draw-count/firstInstance assertions for representation batching and serialized-timing assertions for delayed completion, while preserving cache, payload, invalid admission, selected-content, canvas/offscreen and disposal checks.
- In webgpu-budget.js, first separate presentation from measurement. Use bounded delayed timestamp readback with explicit unavailable samples; remove mandatory completion/readback waits from the interactive submission path while retaining an explicit offline capture path.
- Add minimal representation-binned instance indirection so page count does not require a draw per page. Core WebGPU indirect operations are available; do not assume mesh shaders, 64-bit atomics or multi-draw. Preserve selected contents and instance order where needed for comparison; do not change sampling or page thresholds.
- Qualify a conventional or supported reversed-depth shader path without unconditional fragment-depth output. Keep source lighting/material semantics and validate ground/wood/foliage depth before timing. Do not remove ground or override foliage occlusion.
- Pin equivalence with matched source/camera/lighting captures from the old and corrected paths at close/whole/far and grazing ground. Run one corrected exact baseline and preserved sampled candidate; report differences and capture overhead separately, never import old timing as new evidence.
- Record complete vegetation and whole-stage spans, CPU selection/update/submission, uploads, preparation and submission-to-presentation behavior. Empty work, query exhaustion/map failure, device loss, timeout and fallback adapters are explicit unavailable evidence. Close owned browsers only; preserve other GPU sessions and label contention.

### Investigation targets
**Required**
- experiments/fn13-rendering/webgpu-budget.js:138 — pass encoding and per-page draws
- experiments/fn13-rendering/webgpu-budget.js:161 — frame queries and serialized completion
- experiments/fn13-rendering/measure.js:3 — existing valid/rejected timing receipts
- experiments/fn13-rendering/webgpu-scene.ts — numeric camera/material packet
- tests/browser/fixtures/rendering.json — matched fixtures
- .flow/evidence/fn13/candidates/selection/resumed/DECISION.md — previous timing scope and failed candidate

### Quick commands
```bash
npm run typecheck
node_modules/.bin/vitest run
npm run test:browser
node experiments/fn13-rendering/pipeline-check.mjs
node experiments/fn13-rendering/budget-cache-check.mjs
```
Run the new baseline driver on the recorded hardware after software checks.

## Acceptance
- [x] Matched source/camera fixtures demonstrate source-equivalent appearance and correct shared depth, including grazing ground; no sampling/cap tuning is mixed into the plumbing change.
- [x] Presentation does not await every query/frame completion; query storage and outstanding work stay bounded with explicit failure/unavailable states and safe release.
- [x] Batched submission preserves selected content while avoiding a draw per page; empty/overflow/invalid work is tested without silently dropping visible content.
- [x] One source/candidate baseline records actual adapter, hashes, all vegetation work, whole-stage costs, CPU/upload/presentation and cold preparation separately; unavailable or nonexclusive results cannot pass hardware targets.

Acceptance is assessed under the Owner inspection and resumed scope above; the detailed mapping and unresolved final-quality obligations are in `.flow/evidence/fn13/candidates/pipeline/COMPLETION.md`.

## Done summary
Task7 measurement, batching and functional shared depth are complete under the explicit owner-inspection/resumed scope. The live selector preserves source/candidate mode, seed, geometry identity and camera through all three depth paths; the explicit reversed path passes current hardware stage/depth/material checks without unconditional fragment-depth output. Durable interpretation and AC mapping: `.flow/evidence/fn13/candidates/pipeline/COMPLETION.md`.

Baseline: green. Typecheck,119 unit tests, software browser integration, pipeline/cache checks, both hardware selector modes with six rebuilds and nine hardware depth fixtures pass. Final application sources were unchanged after the complete software gates; retained renderer, fixture and baseline evidence hashes are in `RESUMED-PROVENANCE.json`. Actual observations and logs are in `RESUMED-VALIDATION.json`. Prior implementation commits423ee95 and0191af0 remain the measurement/batching foundation; this resumed commit contains the authorized selector and qualification evidence.

The historical0.015 diagnostic remains failed. Static noise, shimmer and chunk loading remain open for tasks8–11; no finalR1/R2/R4, source-equivalent-motion, task2 feasibility,2ms or4ms pass is claimed. Existing baseline timing is nonexclusive and reused explicitly; no new full benchmark was run. The owner browser and Vite5190 service were preserved; the two additional owned hardware browsers closed.

stage: impl-review - skipped(config: REVIEW_MODE=none; explicit owner disabled automatic reviews)
stage: plan-sync - skipped(config: planSync.enabled=false)

Runtime status restored on 2026-09-08 from this committed completion receipt; historical validation and scope above are unchanged.
## Evidence
- Commits: d8491716ce84ce7c80949a387578b16f6adf95ff
- Tests: npm run typecheck (exit 0), node_modules/.bin/vitest run (exit 0), PLAYWRIGHT_MODULE=/tmp/fn13-browser/node_modules/playwright/index.mjs BROWSER_URL=http://127.0.0.1:5190 CHROMIUM_EXECUTABLE=/usr/lib/chromium/chromium npm run test:browser (exit 0), node experiments/fn13-rendering/pipeline-check.mjs (exit 0), node experiments/fn13-rendering/budget-cache-check.mjs (exit 0), node .flow/evidence/fn13/candidates/pipeline/check-live-depth-selector.mjs (exit 0), PLAYWRIGHT_MODULE=/tmp/fn13-browser/node_modules/playwright/index.mjs node experiments/fn13-rendering/run-pipeline-baseline.mjs .flow/evidence/fn13/candidates/pipeline/depth-resumed --depth-only (exit 0), baseline: green; full software gates ran on final application sources; 119 unit tests, Reused baseline-final/receipt.json and matched native captures; unchanged-code SHA256 provenance: .flow/evidence/fn13/candidates/pipeline/RESUMED-PROVENANCE.json
- PRs: