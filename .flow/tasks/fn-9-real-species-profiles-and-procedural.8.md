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
- [ ] Both species can be selected and varied by seed; botanical and supernatural controls are distinct.
- [ ] All three required view types frame the displayed tree and expose actual foliage anatomy.
- [ ] Instances remain visible across camera changes and empty foliage does not break rendering.
- [ ] Headless integration and affected harness tests pass; CPU rendering backend availability/failures are explicit.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
