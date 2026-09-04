---
satisfies: [R5]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.1 The measurement rig: panel metrics, DPR truth, and the logarithmicDepthBuffer verdict

## Description
Stands up honest fill measurement in the harness before any leaf exists (R5). Split first because it needs no canopy: the flag under suspicion is set once on the renderer and the existing branch surface already spans the scene's full scale range, so this task settles two of the spec's parked unknowns against the tree that is there today.

**Size:** M
**Files:** `harness/skeleton-view.ts`, `harness/stage.ts`, `harness/GrowerDev.tsx`, `harness/params.ts`
**Touches:** [harness/skeleton-view.ts, harness/stage.ts, harness/GrowerDev.tsx, harness/params.ts]

### Approach

- Extend the `TreeStats` interface at `harness/skeleton-view.ts:135` with draw calls, instance count, the raw uncapped `window.devicePixelRatio`, and the applied pixel ratio. Build time already lives there.
- Forest aggregation at `harness/skeleton-view.ts:308-311` sums across both trees. New counts sum the same way; do not double count a shared geometry.
- Draw calls and triangles come from `renderer.info.render`, read after a render. Mind its reset semantics.
- GPU timing uses `EXT_disjoint_timer_query_webgl2` on the WebGL2 context. Keep the extension inside `harness/`. The repo boundary is that only `OrbitControls` is imported from three's addons (`harness/stage.ts:2`), and `src/` imports none.
- `logarithmicDepthBuffer: true` at `harness/stage.ts:214` becomes a runtime toggle so both settings can be measured. It is a renderer construction flag, so toggling rebuilds the renderer. That cost is acceptable in a dev harness.
- `setPixelRatio(Math.min(window.devicePixelRatio, 2))` at `harness/stage.ts:216` is what hides the real fill cost. Report the raw ratio beside the applied one, and drive the applied ratio to 1.0, 0.7, 0.5 and 0.25 for the sweep.
- Vsync off is a browser and driver condition. The procedure documents it; no code can assert it.

### Investigation targets

**Required** (read before coding):
- `harness/skeleton-view.ts:132-143` - the `TreeStats` shape to extend
- `harness/skeleton-view.ts:193-231` - `build()`, where `performance.now()` already brackets the work
- `harness/stage.ts:205-225` - renderer construction, log depth buffer, pixel ratio, tone mapping
- `harness/GrowerDev.tsx:280-290` - the single panel line that renders stats

**Optional** (reference as needed):
- `harness/params.ts` - `SliderSpec` shape if a dial is added
- `harness/skeleton-view.ts:300-315` - forest aggregation across the two trees

### Key context

- Vitest runs in Node with no GPU and no canvas. Nothing here is unit-testable past the shape of the stats object. Assert on the object; deliver the sweep as a documented procedure with recorded numbers.
- `format()` at `harness/GrowerDev.tsx:43-46` infers decimal places from a slider's `step`, so choose `step` deliberately for any new dial.
- This task produces the number R5 needs. Record the measured budget for the named machine and the logarithmicDepthBuffer verdict in the done summary; the spec's parked unknowns are closed by what this task measures.

### Acceptance

- [ ] `TreeStats` carries draw calls, instance count, raw `devicePixelRatio`, applied pixel ratio, and build time
- [ ] Forest aggregation sums the new counts across both trees without double counting
- [ ] The panel renders the new fields
- [ ] `logarithmicDepthBuffer` is togglable at runtime and the panel states which way it is set
- [ ] A four-point resolution sweep at 1.0 / 0.7 / 0.5 / 0.25 records GPU timer-query results with vsync off
- [ ] When the timer-query extension is unavailable the panel says so and reports no timing number, never a vsync-pinned one (R5 error case)
- [ ] A sweep interrupted by context loss or resize reports partial results marked incomplete (R5 error case)
- [ ] The done summary states the measured frame budget for the named machine and the logarithmicDepthBuffer verdict
- [ ] `npx vitest run` and `npx tsc --noEmit` stay green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
