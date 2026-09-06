---
satisfies: [R1, R2, R3, R4, R6]
---
# fn-13-tree-detail-by-viewing-distance.1 Pin fidelity references, ground reproduction and measurement protocol

## Description
Establish the reproducible acceptance rig before comparing rendering candidates.

**Size:** M
**Files:** harness/stage.ts, harness/stage.test.ts, tests/browser/rendering.mjs (new), tests/browser/fixtures/rendering.json (new), .flow/evidence/fn13/protocol/
**Touches:** [harness/stage.ts, harness/stage.test.ts, tests/browser/rendering.mjs, tests/browser/fixtures/rendering.json, .flow/evidence/fn13/protocol/**]

### Approach
- Reuse Stage GPU queries at harness/stage.ts:763-900 and capture provenance at tests/browser/species.mjs:350-380. Preserve existing stage/capture entrypoints.
- Pin mature spruce seed 1 plus the fn9 spruce/oak seed sets for holdout views. Define physical resolution, static/slow/fast camera paths, close anatomy, crown, distance, inside-crown, ground grazing, reversal and resize/DPR cases.
- Reproduce the reported ground flicker with ground actually present; retain exact renderer/material/camera state and distinguish geometry aliasing, depth precision and coplanarity hypotheses.
- Establish converged full-detail references offline where native samples alias, alongside current native output; record deterministic accumulation/sample procedure. Fix numeric pixel/coverage/shading and temporal thresholds plus direct sequence-inspection rubric before candidate measurement. Validate metrics with deliberate thinning, popping and ghosting controls.
- Extend measurement receipts to raw distributions, asynchronous invalid-query handling and interruption status. Count vegetation passes explicitly; report full-stage time separately rather than subtracting a noisy ground-only sample.
- Pin the deterministic forest manifest recipe, species/size distribution, structure fingerprint rule, placement and visible population along canopy and ground traversal. Pin warmup/sample protocol and resource accounting; no 1,000 expanded-array allocation in this task.

### Investigation targets
**Required**
- harness/stage.ts:111 — timing contracts and Stage API
- harness/stage.ts:419 — renderer/material/ground setup
- tests/browser/species.mjs:350 — reference capture receipts
- scripts/benchmarks/measure.mjs:16 — hardware and environment checks
- scripts/benchmarks/memory.mjs:6 — memory-domain distinctions
- .flow/evidence/fn9/final/captures.json — source specimen provenance

### Quick commands
npm run typecheck
npx vitest run harness/stage.test.ts

## Acceptance
- [ ] The committed protocol fixes reference generation, camera paths, thresholds, forest recipe and timer scope before candidate comparisons; negative visual controls fail the protocol.
- [ ] A reproducible ground-overlap clip and diagnostic state are recorded, or the missing reproduction is explicitly unresolved and prevents claiming R4 complete.
- [ ] GPU receipts distinguish p50/p95/p99/max and cold/steady CPU/memory; unavailable/disjoint/timeout/context-loss/resize samples cannot pass a budget.
- [ ] Existing captures and stage tests still work; new motion runner records uninspected evidence honestly and has a documented invocation.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
