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

- [ ] Interior elements are culled, conservative at the silhouette (R3)
- [ ] The test asserts element count both falls and stays above zero, so a culler removing everything fails it (R3 error case)
- [ ] The fixture is dense enough that a no-op culler also fails the test (R3 error case)
- [ ] Classification tests every vertex under the instance transform, never the centroid (R3 error case)
- [ ] Silhouette compared before and after and unchanged within a stated tolerance, with the tolerance's basis written down
- [ ] `npx vitest run` and `npx tsc --noEmit` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
