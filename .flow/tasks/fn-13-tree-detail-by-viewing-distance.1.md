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
Implemented the full-source rendering protocol, deterministic ground-visible capture/reference runner and strict GPU timing validity. Durable evidence and limitations are in `.flow/evidence/fn13/protocol/README.md` and `OBSERVATIONS.md`; final whole/ground references converge, actual-source/image negative controls fail, source hashes match fn9, and the unoptimized native whole-stage GPU baseline is about53.4ms p95.

Baseline: green (typecheck,106 unit tests, existing browser integration). Final: typecheck,119 unit tests, focused stage tests, metric controls and browser integration pass. One intermediate browser regression hit Vite module duplication after Wasm rebuild; restarting only the owned5190 server resolved it, with failure and final-pass logs preserved.

Task1 establishes the protocol; it does not claim the final R1–R6 acceptance. Ground depth-fault cause remains unresolved, most planned views/holdouts remain unassessed, vegetation-only timing remains unavailable, and no rendering optimization has started. Current evidence never awards a visual or2ms/forest performance pass. Final references were regenerated after fixing linear mask encoding and preserving source face-culling semantics.

The owner’s fn9 server5184 is untouched. The fn13 Vite server5190 remains available for the next worker; no GPU job remains running. All camera/seed/forest/threshold contracts are in `tests/browser/fixtures/rendering.json`.

stage: impl-review - skipped(config: REVIEW_MODE=none)

Runtime status restored on 2026-09-08 from this committed completion receipt; historical validation and scope above are unchanged.
## Evidence
- Commits: 590d812732e49f45bcbf35d5fd2a230a042cb51f, 317ed2742d71d730e4a0e25378a3a31b13e065e3, b2e5cc6880439a2ff1a1eafbf360d6b3e68b8271, 1693540f4d4bc2c49109f70440972714b493883b
- Tests: baseline: green (npm run typecheck; npm test:106 tests; npm run test:browser), npm run typecheck — pass; .flow/evidence/fn13/protocol/gates/typecheck.log, npm test — pass119 tests; .flow/evidence/fn13/protocol/gates/unit.log, npx vitest run harness/stage.test.ts — pass45 tests; .flow/evidence/fn13/protocol/gates/focused.log, node tests/browser/rendering.mjs --self-test — pass synthetic and transfer-function controls, BROWSER_URL=http://127.0.0.1:5190 PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs CHROMIUM_EXECUTABLE=$PWD/scripts/species-chromium-gpu.sh npm run test:browser — pass; .flow/evidence/fn13/protocol/gates/browser.log, node tests/browser/rendering.mjs --output /tmp/fn13-final-reference --view whole --reference --controls --measure — capture complete; converged64/128 reference, source thinning/imagepop/ghost rejected; visual unassessed, node tests/browser/rendering.mjs --output /tmp/fn13-final-ground-reference --path ground-grazing --frame0 --reference — capture complete (actual CLI --frame 0); converged64/128; visual unassessed, node tests/browser/rendering.mjs --output /tmp/fn13-ground-log --path ground-grazing —24full-source frames complete, node tests/browser/rendering.mjs --output /tmp/fn13-ground-conventional --path ground-grazing --log-depth false —24full-source frames complete, losslessFFV1 decodedRGB24 frame hashes match all24sourceframes for both depth-modeclips, node tests/browser/rendering.mjs --output /tmp/fn13-oak-holdout --case oregon-white-oak-1 --view whole — capture complete; visual unassessed, flowctl validate --spec fn-13-tree-detail-by-viewing-distance --json — valid6tasks, git diff --check — pass
- PRs: