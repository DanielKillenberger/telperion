---
satisfies: [R1, R2, R4]
---
# fn-13-tree-detail-by-viewing-distance.4 Integrate continuous detail and stable foliage-ground rendering

## Description
Use the selected representation in the live viewer with continuous motion and correct depth/coverage.

**Size:** M
**Files:** harness/stage.ts, src/browser/three.ts, src/browser/render-selection.ts (new), src/browser/render-selection.test.ts (new), tests/browser/rendering.mjs
**Touches:** [harness/stage.ts, src/browser/three.ts, src/browser/render-selection.ts, src/browser/render-selection.test.ts, tests/browser/rendering.mjs]

### Approach
- Integrate task 2's screen-space selection/error strategy and task 3's resources. Preserve full-detail inspection, current camera controls and neutral appearance.
- Keep resolved anatomy on approach and coverage/shading stable through representation boundaries. Handle fast reversals, camera inside crown, large-to-small projected detail, resize/DPR and source replacement.
- Fix the task 1 ground reproducer using the diagnosed cause. Select depth/coverage settings from measured evidence, feature-detect required capabilities, and test fallback output against the same contract.
- Avoid silent missing-detail frames: keep an already-valid representation or explicitly pause/report incomplete preparation. No lower-resolution performance shortcut.
- Extend deterministic motion tests for disocclusion, stale history, ghosting and bounds updates. Report selected representation cost in the existing diagnostics rather than exposing implementation controls as a product workflow.

### Investigation targets
**Required**
- harness/stage.ts:419 — render state
- harness/stage.ts:627 — tree replacement/disposal
- harness/skeleton-view.ts:498 — full-detail inspection
- src/browser/three.ts — task 3 adapter
- tests/browser/rendering.mjs — task 1 motion/ground runner

### Quick commands
npm run typecheck
npx vitest run harness/stage.test.ts src/browser/render-selection.test.ts

## Acceptance
- [ ] All frozen close/crown/distant, reversal, inside-crown and resize sequences satisfy task 1 appearance and temporal criteria.
- [ ] Before/after ground-overlap evidence proves the diagnosed artifact is removed while correct occlusion/contact remains.
- [ ] Unsupported capabilities, incomplete detail and invalidated data follow the explicit contract; no silent thinning, missing tree or stale-history workaround.
- [ ] Full-detail inspection and source identity remain intact, and renderer costs are captured for final budget verification.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
