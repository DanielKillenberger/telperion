---
satisfies: [R3]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.4 Culling to a shell, and the silhouette test that proves it

## Description
Culls the interior elements the camera never sees, and builds the screen-space silhouette helper R3's test needs. Separate from placement because the helper is most of the work: nothing in the repo projects geometry to an outline today.

**Size:** M
**Files:** `src/canopy/cull.ts`, `src/canopy/silhouette.ts`, `src/canopy/cull.test.ts`
**Touches:** [src/canopy/cull.ts, src/canopy/silhouette.ts, src/canopy/cull.test.ts]

### Approach

- Consume the placed instances from the placement task and return a culled subset. Stay conservative at the silhouette, since the outline is what the eye reads.
- The silhouette helper is new. A repo-wide grep for silhouette, projection and screen-bound helpers returns nothing, so there is nothing to reuse. Project element extents under their instance transforms from a fixed set of camera directions and compare the outline before and after culling.
- Classification tests every vertex of the element under its instance transform. A centroid test misclassifies any leaf straddling the shell.
- `envelopeRadiusAt` at `src/envelope.ts:70` is the repo's existing notion of a shell radius at a height. Read it before inventing a second one.

### Investigation targets

**Required** (read before coding):
- `src/canopy/place.ts` - the instance shape this consumes (from the placement task)
- `src/canopy/element.ts` - element vertex data to transform (from the element task)
- `src/envelope.ts:70-125` - `envelopeRadiusAt`, the existing shell notion

**Optional** (reference as needed):
- `src/radius.test.ts` - the shape of an invariant asserted over every node

### Key context

- Project memory, interpenetrating junctions: contain the whole ring against the surface as drawn, and assert on every vertex rather than the centre. This entry is the direct reason R3 classifies by vertex.
- Project memory, zero width is not no constraint: a vacuous predicate let the tree fork below its own envelope. R3's floor guard exists for the same reason. A test that only checks the count fell passes on a culler that removes everything.
- Tolerance on the silhouette comparison is sized by the failure being corrected, not by the thing being protected. A prior slack-band bug in this repo came from the other choice.

### Acceptance
## Acceptance
- [ ] Interior elements are culled, conservative at the silhouette (R3)
- [ ] The test asserts element count both falls and stays above zero, so a culler removing everything fails it (R3 error case)
- [ ] The fixture is dense enough that a no-op culler also fails the test (R3 error case)
- [ ] Classification tests every vertex under the instance transform, never the centroid (R3 error case)
- [ ] Silhouette compared before and after and unchanged within a stated tolerance, with the tolerance's basis written down
- [ ] `npx vitest run` and `npx tsc --noEmit` green
## Done summary
Culls the canopy to a shell beneath the authored envelope and ships the
screen-space silhouette helper that holds the culling to account - depth
measured as distance to the envelope's own profile curve, classified by
every vertex under its own instance transform, and judged against an
outline drawn twice from six fixed directions with the raster's own pixel
resolution as the tolerance.

stage: impl-review - skipped(policy: parallel wave - conductor reviews after integration)

### What landed

- `src/canopy/silhouette.ts` - projects instanced element geometry to a
  screen-space outline in Node, no GPU and no canvas. `viewAlong` /
  `SILHOUETTE_VIEWS` (four horizontal quarter turns and two from thirty
  degrees up), `screenFor` (one fixed frame, built from the before-canopy
  and reused for the after-canopy - a rescaled frame would hide the
  shrinkage), `silhouetteOf` (triangle edges walked in pixel space,
  per-column topmost and bottommost pixel, filled between), and
  `silhouetteChange` / `silhouettePixelFloor`.
- `src/canopy/cull.ts` - `cullCanopy(canopy, element, envelope, params)`,
  `CullParams { shellDepth }`, `DEFAULT_CULL`. Pure, order-preserving,
  float-for-float.
- `src/canopy/cull.test.ts` - 13 tests.

### Acceptance

| AC | Where |
|---|---|
| Interior culled, conservative at the silhouette (R3) | `cullCanopy`; tests "removes a substantial part of a canopy that fills its envelope" and "leaves the silhouette where it was, from every judged direction" |
| Count both falls and stays above zero | two separately named tests - "removes a substantial part..." (kills a no-op culler) and "leaves a canopy behind" (kills an everything culler); the count alone would not have caught the second, which is why they are not one test |
| Fixture dense enough that a no-op fails | the fixture is Telperion grown with `shootRadius: 1` and half the preset's spacing - every limb bearing foliage down to the trunk, 8741 elements; culling removes 16% of it, so a no-op fails the `< 0.9x` assertion |
| Every vertex, never the centroid | test "classifies by every vertex, never by the centroid": a 4.8 m blade whose petiole and centroid are deep and whose tip reaches the shell is kept, while the same element shrunk to a speck at that blade's own centroid is culled |
| Silhouette unchanged within a stated tolerance, basis written down | test "leaves the silhouette where it was...", six views, `fraction <= silhouettePixelFloor(before)`; the basis is in `silhouettePixelFloor`'s own doc comment |
| `npx vitest run` and `npx tsc --noEmit` green | 18 files / 251 tests, up from 17 / 238; tsc clean |

### The tolerance, and its basis

`silhouettePixelFloor` is one pixel at each end of every column the
silhouette reaches, as a fraction of its filled area - about 1.8% on
these fixtures at 256 px. It is the smallest change the raster can
distinguish from where it happened to put its pixel boundaries. The
culler's one dial was then set against that fixed floor; the floor was
never widened to fit the culler. That is the direction this repo's
slack-band bug went the wrong way round.

The comparison is also guarded against being vacuous: the test asserts
the before-silhouette has real extent (>10,000 filled px, >100 columns)
before asserting it is unchanged, and `silhouetteChange` reports a
fraction of 1, not 0, when there was no silhouette to lose.

### Judgement calls the conductor should see

- **Depth is distance to the envelope's profile curve, not slack in the
  radius.** The first implementation read depth radially and it ate the
  crown's underside: a leaf hanging under the middle of a wide crown is
  metres from the nearest wood but centimetres from the envelope's belly,
  and it is outline. Measured on Telperion at one shell depth, switching
  to the profile distance halved what culling cost the silhouette (15.5%
  to 8.0%). Pinned by the test "keeps foliage hanging under the crown's
  own belly", confirmed red against the radial reading. The shape is
  sampled from `envelopeRadiusAt`, so there is still exactly one
  description of the envelope in the library.
- **`shellDepth` defaults to 0.45 of the crown's half-width, and that is
  measured rather than chosen.** It is the shallowest shell that keeps
  every one of the six views inside the pixel floor on both presets, at
  their own density and at twice it. The next step down, 0.4, comes
  within 2% of the floor on one view of Telperion - a margin any change
  to placement would erase.
- **What culling actually buys here is smaller than the spec assumed, and
  the reason is interesting.** Placement puts foliage on distal shoots,
  and distal shoots are near the crown's surface, so a well-placed canopy
  is most of the way to being a shell before culling sees it. On the
  presets as they ship, the default removes about 15%. On a canopy that
  fills its envelope - every limb bearing foliage, the botanically wrong
  one - it removes 16%, and at a shallower shell it removes a third. The
  elements it takes are the ones that grew on inner wood. Density is the
  lever: at four times the preset spacing the same shell removes more,
  because there is more interior to remove. If .5's clay judgement raises
  density, re-read this number rather than assuming it.
- **The predicate is written `!(depth > shell)` rather than
  `depth <= shell`.** Every NaN - a non-finite transform, a degenerate
  envelope - therefore keeps its element instead of silently deleting it,
  and where the envelope has no width the shell culls nothing. Two tests
  hold that.
- **Nothing outside the three declared files was touched.** `place.ts`
  does not call the culler and `src/index.ts` is untouched: wiring the
  cull into the stage and exporting it are .5's and .6's.

### For task .5 (the draw) and .6 (the public surface)

- `cullCanopy` takes and returns a `Canopy`, so it drops in between
  `buildCanopy` and the `InstancedMesh` with no change to either.
- It costs about 50 ms on 8,700 elements and 250 ms on 20,000, single
  pass, dominated by the profile distance. If that shows up in build
  time, the radial slack already short-circuits the common case and the
  remaining loop is the obvious place to bound.
- The silhouette helper is a test instrument, not a runtime one, but it
  is pure library code with no test-only dependency, so .6 can decide
  whether it belongs in the barrel. It emits no material and no colour.
## Evidence
- Commits: 23909d3
- Tests: npx vitest run (18 files / 251 tests passed on the integrated target), npx tsc --noEmit (clean on the integrated target), silhouette held across six views within the raster's own pixel floor (~1.8% at 256px); default shell removes ~15% of the preset canopies, ~16% of an envelope-filling one
- PRs: