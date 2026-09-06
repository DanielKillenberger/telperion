---
satisfies: [R3, R4, R5]
---
# fn-9-real-species-profiles-and-procedural.8 Show species and anatomy in the neutral viewer

## Description
Show species and anatomy in the neutral viewer. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `src/browser/three.ts`, `harness/GrowerDev.tsx`, `harness/skeleton-view.ts`, `harness/stage.ts`, `tests/browser/integration.mjs`
**Touches:** [src/browser/three.ts, harness/GrowerDev.tsx, harness/skeleton-view.ts, harness/stage.ts, tests/browser/integration.mjs]

### Approach
- Add species selection using the exported catalogue and retain independent seed controls. Expose applicable botanical and supernatural settings as distinct groups without inventing research-dashboard UI.
- Adapt Three.js instancing only to the finalized foliage representation. Recompute aggregate/per-bucket bounds from transformed instances; do not use prototype bounds for culling.
- Make whole-tree, bare-branch and foliage-detail states accessible for the capture runner. Frame the currently displayed specimen and preserve the existing Two Trees comparison behavior.
- Extend headless integration coverage for selection, seed changes, output visibility and valid bounds. Do not launch visible browser windows or use consent-bypass flags.

### Investigation targets
**Required:**
- `src/browser/three.ts:9-32` — instancing/bounds assumptions
- `harness/GrowerDev.tsx` — existing controls
- `harness/skeleton-view.ts` — view selection
- `harness/stage.ts:471-505` — neutral materials/framing
- `tests/browser/integration.mjs:10-121` — existing browser gate

### Quick commands
```bash
npm run typecheck
npm test -- harness/stage.test.ts harness/skeleton-view.test.ts
node tests/browser/integration.mjs
```

## Acceptance
- [x] Both species can be selected and varied by seed; botanical and supernatural controls are distinct.
- [x] All three required view types frame the displayed tree and expose actual foliage anatomy.
- [x] Instances remain visible across camera changes and empty foliage does not break rendering.
- [x] Headless integration and affected harness tests pass; CPU rendering backend availability/failures are explicit.

## Done summary
Implemented independent catalogue/seed selection, distinct botanical and supernatural controls, and whole/bare/actual-foliage-detail framing. Recomputed transformed instance bounds and preserved Two Trees comparison. Typecheck, 82 affected tests, and the full headless integration gate pass. Durable inspected oak/spruce whole, bare and detail PNGs plus exact inputs/backend metadata are committed under .flow/evidence/fn9/viewer/. SwiftShader succeeds on complete 4 m fixtures; full-size mature captures and reference fidelity remain task 9 work. See .flow/evidence/fn9/neutral-viewer.md. No reviews, PR or merge. Next job: fn-9.9.

Runtime state reconciled on 2026-09-06 from this committed completion receipt after fetching remote through 1297516. The preceding tests describe the original completion, not a fresh fidelity verdict.
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b038e44
- Tests: npm run typecheck: passed, npm test -- harness/stage.test.ts harness/skeleton-view.test.ts: 82 passed, exit 0, BROWSER_EVIDENCE=.flow/tmp/fn98-browser node tests/browser/integration.mjs: passed, exit 0; six durable species PNGs and SwiftShader metadata, git diff --check: passed
- PRs: