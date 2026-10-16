---
satisfies: [R5]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.1 The measurement rig: panel metrics, DPR truth, and the logarithmicDepthBuffer verdict

## Description
Build the measurement rig before any leaf exists: panel metrics, true devicePixelRatio, GPU timer-query sweep, and the logarithmicDepthBuffer measurement.

## Acceptance
- Panel reports triangles, draw calls, instances, raw and applied dpr, build time.
- Four-point dpr sweep timed with `EXT_disjoint_timer_query_webgl2` at vsync off; no timing number when the extension is missing; an interrupted sweep is marked incomplete.

## Done summary
Rig landed and ran on the RTX 3080 (vsync off, 3440x1440, dpr 2): the branch-only room (58,880 triangles, one draw) costs 0.21 ms at dpr 1.00, flattening to a ~0.12 ms fixed floor below dpr 0.5, so the scene is not fill-bound and the canopy inherits nearly the whole 16.7 ms frame. Log depth on vs off is inside noise; not settled, flag kept on.

## Evidence
- Commits: 325da1e, dda4879
- Tests: npx vitest run (238 passed), npx tsc --noEmit, GPU timer-query sweep on RTX 3080
- PRs:
