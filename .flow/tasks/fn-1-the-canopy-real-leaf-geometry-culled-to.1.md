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
## Acceptance
- [ ] `TreeStats` carries draw calls, instance count, raw `devicePixelRatio`, applied pixel ratio, and build time
- [ ] Forest aggregation sums the new counts across both trees without double counting
- [ ] The panel renders the new fields
- [ ] `logarithmicDepthBuffer` is togglable at runtime and the panel states which way it is set
- [ ] A four-point resolution sweep at 1.0 / 0.7 / 0.5 / 0.25 records GPU timer-query results with vsync off
- [ ] When the timer-query extension is unavailable the panel says so and reports no timing number, never a vsync-pinned one (R5 error case)
- [ ] A sweep interrupted by context loss or resize reports partial results marked incomplete (R5 error case)
- [ ] The done summary states the measured frame budget for the named machine and the logarithmicDepthBuffer verdict
- [ ] `npx vitest run` and `npx tsc --noEmit` stay green
## Done summary
The harness can now measure its own fill cost honestly, and it did: `TreeStats`
carries draw calls and instances beside triangles, vertices, nodes and build
time; the stage reports what the renderer did last frame and both pixel ratios,
raw and applied; `logarithmicDepthBuffer` is a construction option the panel
turns over by replacing the canvas; and a four-point sweep at applied dpr
1.00 / 0.70 / 0.50 / 0.25 times each point with
`EXT_disjoint_timer_query_webgl2` on the GPU's own clock, restoring the ratio it
found. Without the extension the panel says so and quotes no millisecond figure
at all, and a run cut short by a lost context or a resize keeps its points and
is marked incomplete - both of which are asserted in vitest, because that string
is the only part of the rig a runner with no GPU can hold to account.

### The measurement (R5), on the named machine

RTX 3080 (ANGLE / OpenGL ES 3.2), chrome with
`--disable-gpu-vsync --disable-frame-rate-limit`, fullscreen 3440x1440,
`devicePixelRatio` 2, branch-only subject: 58,880 triangles in one draw call,
three draws for the whole room. GPU timer queries, median of 20 samples per
point:

| applied dpr | drawing buffer | log depth on | log depth off |
|---|---|---|---|
| 1.00 | 1720x720 | 0.21 ms | 0.25 ms |
| 0.70 | -        | 0.16 ms | 0.15 ms |
| 0.50 | -        | 0.13 ms | 0.12 ms |
| 0.25 | 430x180  | 0.13 ms | 0.12 ms |

**The frame budget: at 60 Hz the frame is 16.7 ms and the whole branch-only room
spends 0.21 ms of it - a little over one percent.** The canopy about to land
therefore inherits essentially the entire budget, and nothing in this
measurement argues for making the branch surface cheaper. The curve flattens
onto a floor of about 0.12 ms below dpr 0.5, which is the fixed per-frame cost
(clear, ground disc, scale figure, state setup); only the ~0.08 ms above that
floor is fill, which is the honest reading of a scene that is not yet
fill-bound. Worth carrying forward: the harness's own default on this display is
dpr 2.00, four times the fragments of the sweep's top point, so what the owner
normally looks at sits above the top of the measured curve.

### The logarithmicDepthBuffer verdict (R5): not settled, and not settleable here

Both settings land inside the noise floor, with "on" measuring marginally
*cheaper* at dpr 1.00 - which is not a physical result, it is noise, and
reporting it as a verdict would be exactly the false precision this rig exists
to prevent. The flag stays on: the z-fighting it fixes at 400 m is real and its
cost is currently unmeasurable. **The question moves to task .5** - a
fragment-shader depth write defeats early-Z, and the case where that is supposed
to cost is an alpha-tested canopy discarding fragments, which does not exist
yet. Re-run this sweep with the canopy on screen; the rig is now standing there
for it. This closes the spec's parked unknown as "measured, and the branch-only
answer is that the flag is free at this scale", not as a number invented to fill
the slot.

### Notes

- The sweep was driven over CDP from a throwaway script in the scratchpad, not
  from anything committed; the panel's own `sweep` button does the same thing by
  hand.
- Verified rather than assumed: the applied ratio really reaches the drawing
  buffer (3440x1440 at 2.00, 1720x720 at 1.00, 430x180 at 0.25), and the sweep
  restores the ratio it found when it finishes.
- Tests were mutation-checked - a `describeSweep` that fell back to a frame time,
  a `countDraws` blind to `InstancedMesh`, and an aggregate that counted one tree
  twice each fail the new tests.
- Follow-up, not built (YAGNI): the sweep's ratios stop at 1.00 as the spec
  names them, so the harness's own dpr-2.00 default is outside the curve. If the
  canopy turns out to be fill-bound, a fifth point at the raw ratio would be one
  line.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after it integrates)
## Evidence
- Commits: 325da1e, dda4879
- Tests: npx vitest run (17 files / 238 tests passed on the integrated target), npx tsc --noEmit (clean on the integrated target), GPU timer-query sweep on RTX 3080, vsync off, 4 points at applied dpr 1.00/0.70/0.50/0.25, median of 20 samples: branch-only room 0.21 ms of a 16.7 ms frame
- PRs: