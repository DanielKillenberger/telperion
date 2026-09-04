---
satisfies: [R4, R6, R8]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.5 The instanced draw, the clay judgment, and framing that includes the canopy

## Description
Puts the canopy on screen: one instanced draw per element type with alpha test (R4), inside the clay room unchanged (R6), framed so the stage accounts for foliage spread (R8). Carries R7's manual sign-off gate, which cannot be automated.

**Size:** M
**Files:** `harness/skeleton-view.ts`, `harness/stage.ts`, `harness/params.ts`
**Touches:** [harness/skeleton-view.ts, harness/stage.ts, harness/params.ts]

### Approach

- Build one `THREE.InstancedMesh` per element type from the culled instances. This is the repo's first instanced mesh; a repo-wide grep for `InstancedMesh`, `alphaTest` and alpha-to-coverage returns zero hits, so there is no local pattern to mirror and no existing code to conflict with.
- Alpha test or alpha-to-coverage, never blending.
- The placeholder material is the harness's, not the library's. Do not touch the judging-mode defaults: `CLAY` at `harness/stage.ts:30`, the single `HemisphereLight` at `:238`, `NoToneMapping` at `:219`, and `setLightingCheck` staying opt-in and off at `:454-465`.
- R8 is the integration risk. Compute the element geometry's bounding volume before the instanced mesh joins the tree group, or `box.setFromObject` contributes zero extent for it and framing silently ignores the canopy. Then verify room fit, orbit pivot and the near and far planes against a canopy-widened subject.
- Confirm the dispose path releases instance attribute buffers on regenerate. `InstancedMesh` extends `Mesh`, so an existing `instanceof THREE.Mesh` check catches it, but confirm rather than assume.
- Draw-call and instance numbers land in the panel fields the measurement task added.

### Investigation targets

**Required** (read before coding):
- `harness/skeleton-view.ts:193-231` - the build path the canopy joins
- `harness/stage.ts:150-200` - room fit and pivot maths tuned on branch-only spans
- `harness/stage.ts:317-333` - `subjectBox` and `box.setFromObject(tree)`
- `harness/stage.ts:205-225` - renderer settings
- `harness/stage.ts:440-470` - lighting check and dispose

**Optional** (reference as needed):
- `harness/params.ts` - dials for canopy density if judging needs them

### Key context

- Project memory, slack band around an orbit pivot: a tolerance sized by the thing being protected rather than the failure being corrected left the pivot off the subject. R8 is the same surface.
- Project memory, a second subject on stage makes every "what is on screen" read a call site. Two trees each grow a canopy, so grep every read of the tree group rather than patching the one that looks wrong.
- Project memory, an AC that enumerates the default state is the test and a comment is not a licence. R6 enumerates what the clay room emits. A helpful extra such as ambient occlusion or a quiet LOD is not licensed by a comment explaining it.
- Vitest cannot observe WebGL state. Assert instance counts and draw counts on the CPU-side data.

### Acceptance

- [ ] One instanced draw per element type, confirmed by the panel's draw-call count (R4)
- [ ] Cutout transparency by alpha test or alpha-to-coverage, never blending (R4)
- [ ] Clay judging-mode defaults unchanged and the lighting check still opt-in and off (R6)
- [ ] Nothing in `src/` emits a material, a colour or a light (R6)
- [ ] The canopy contributes to subject bounds, so room fit, orbit pivot and near and far account for foliage spread (R8)
- [ ] A canopy with zero elements leaves framing identical to the branch-only framing (R8 error case)
- [ ] An instanced mesh with an uncomputed bounding volume cannot silently contribute zero extent (R8 error case)
- [ ] Regenerating the tree releases instance attribute buffers
- [ ] Owner sign-off recorded: the canopy reads as worth building a scene on, judged in clay (R7 manual gate)
- [ ] `npx vitest run` and `npx tsc --noEmit` green

## Acceptance
- [ ] TBD

## Done summary
The canopy is on screen: each tree is a group of two renderables - the swept
trunk and the whole crown as one `THREE.InstancedMesh`, drawn in the room's own
clay, alpha tested rather than blended and double sided because a leaf is an
open sheet. The framing guard R8 names is in the stage rather than in the
builder, the regenerate path now releases instance transforms as well as
geometries, and the ten canopy terms are on the panel under the library's own
names so the clay judgement can steer density.

### What landed

- `harness/skeleton-view.ts`: `toCanopyParams` (a rename, ten terms, none
  spread); `buildCanopyMesh(canopy, element, material)`, which returns `null`
  for a canopy with nothing in it and computes the mesh's bounding volume LAST,
  after the transforms are in; `build` now runs `buildCanopy` ->  `cullCanopy`
  -> the instanced mesh, and returns a `THREE.Group` ("grower-tree") holding
  "grower-trunk" and "grower-canopy". `presetToParams` carries the preset's
  canopy.
- `harness/stage.ts`: `Clay` gains `element` - same CLAY colour, `DoubleSide`,
  `alphaTest: 0.5`, `transparent: false`, flat shaded. Two new pure exported
  seams: `measureSubject` (recomputes any instanced mesh's bounds before
  `setFromObject`) and `disposeSubject` (calls `InstancedMesh.dispose()` as
  well as `geometry.dispose()`).
- `harness/params.ts`: ten canopy dials, `shootRadius` through `sizeVariation`.

### Acceptance

- One instanced draw per element type - stats report 2 draws per tree, 4 for
  the two-preset comparison, asserted against the crown's own `count`.
- Alpha test, never blending - asserted at source (`alphaTest` present, no
  `transparent: true`, no `blending:`).
- Clay defaults unchanged, lighting check still opt-in and off - now written
  down as a test rather than trusted: one light in the judging scene, key and
  fill reachable only from inside `setLightingCheck`, `NoToneMapping`, no fog.
- Nothing in `src/` emits a material, a colour or a light - a scan over every
  non-test file under `src/`, asserted.
- The canopy contributes to subject bounds - `measureSubject`, with the stale
  box measured alongside it in the test so the guard's reason is visible.
- Zero elements leaves framing identical - no instanced mesh is built at all;
  `bare.stats.drawCalls === 1`, `instances === 0`, and `measureSubject` on a
  canopy-free subject returns `setFromObject`'s box float for float.
- Uncomputed bounds cannot contribute zero extent - two tests: bounds never
  computed, and bounds computed too early.
- Regenerate releases instance attribute buffers - `disposeSubject` dispatches
  the mesh's own `dispose`, which is the only thing that frees them.
- `npx vitest run` and `npx tsc --noEmit` green (266 tests, up from 251).

### Two acceptance items I could not close, and they are the owner's

- **R7's manual gate.** "The canopy reads as worth building a scene on, judged
  in clay" is a sign-off, not a check. Not recorded.
- **The `logarithmicDepthBuffer` re-run.** This task was supposed to close the
  parked unknown by re-running the four-point sweep with the alpha-tested
  canopy on screen. It needs a GPU, a browser and vsync off; this worker has
  none of the three, and a number invented here would be worse than the parked
  unknown. The rig is standing (panel toggle plus sweep button, both settings),
  the canopy now compiles a discard into the fragment shader either way, and
  the run is a five-minute procedure the owner can do in `npm run dev`. Until
  it is run, the flag stays on and the unknown stays parked.

### Measured on the CPU side (what the numbers actually came out at)

| subject | placed | after cull | removed | draws | canopy tris |
|---|---|---|---|---|---|
| Telperion preset | 3,407 | 2,904 | 14.8% | 2 | 46,464 |
| Laurelin preset | 8,679 | 8,115 | **6.5%** | 2 | 129,840 |
| dials, default spacing 0.006 | 4,830 | 4,072 | 15.7% | 2 | 65,152 |
| dials, spacing 0.003 | 8,650 | 7,269 | 16.0% | 2 | 116,304 |
| dials, spacing 0.0015 | 16,240 | 13,638 | 16.0% | 2 | 218,208 |

Element: 16 triangles, 14 vertices. Build time at the default dials 43 ms,
100 ms at four times the density.

Two things worth carrying forward. **Laurelin's cull removes 6.5%, not the
~15% the spec's Measured section quotes** - the spec's number is Telperion's
and an envelope-filling fixture's, and Laurelin's broad domed crown puts even
more of its foliage near the surface. Culling on the presets as authored buys
less than the spec says. And **density does not change the fraction**: at four
times the preset density the shell still removes 16%, because placement, not
volume, is what puts foliage near the surface.

**The canopy barely widens the default subject**: 6.86 m to 6.92 m of
half-width on a 24 m tree, since foliage grows on distal shoots that are
already inside the branch envelope. R8's guard still matters, and not
marginally - it matters because the failure is SILENT (a cached box reporting
one leaf at the origin), because `size` and `outward` are dials that push
foliage past the wood, and because the crown does raise the top of the box.
But the visible reframing on the default tree is small, and a reviewer looking
for a dramatic before-and-after will not find one.

### Judgement, offered rather than claimed

I have not seen this render. What the CPU-side numbers support: the default
dials put roughly 4,000 leaves on a 24 m tree, which against 2,900 for
Telperion at its own spacing is the same order, and both are a canopy rather
than a scattering. If the owner's clay judgement wants denser, `leaf spacing`
is the dial and 0.003 doubles it for 60 ms of build; the cull fraction does not
move, so nothing about the culling story needs re-measuring at that density
(re-read it only if `shootRadius` or `outward` move, which change WHERE the
foliage sits rather than how much of it there is).

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after integration)
## Evidence
- Commits: 922749e, 9e8d7ff
- Tests: npx vitest run (18 files / 266 tests passed on the integrated target), npx tsc --noEmit (clean on the integrated target), npm run build (clean, dist/telperion.js 37.25 kB), cull fractions re-measured by the conductor on both presets at x1/x2/x4 density: Telperion 14.8/15.0/15.2%, Laurelin 6.5/6.2/5.9%
- PRs: