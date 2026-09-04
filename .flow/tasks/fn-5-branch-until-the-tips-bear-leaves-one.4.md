---
satisfies: [R2, R3]
---
# fn-5-branch-until-the-tips-bear-leaves-one.4 Crossover continuity, and the test that cannot pass vacuously

## Description
Makes radius, direction and taper continuous where the method changes, and proves it (R2). A survey of open tree generators found none that transitions in place and asserts continuity at the seam, so there is no reference test to copy and the whole risk of this task is writing one that passes for the wrong reason.

**Size:** M
**Files:** `src/skeleton/twigs.ts`, `src/skeleton/continuity.test.ts`
**Touches:** [src/skeleton/twigs.ts, src/skeleton/continuity.test.ts]

### Approach

- **The mechanism**: interpolate child radius from the parent's local cross-section at the fork point, not from a global taper curve. A global curve evaluated on both sides of a method change is exactly what produces a step. This is what makes R2 satisfiable rather than merely required.
- **The test shape already exists in this repo.** `src/radius.test.ts:127-147` walks every node of a real grown skeleton asserting a per-edge invariant. Copy that shape; assert radius-ratio and turn-limit continuity instead of monotonic taper.
- **Sample several generations either side**, never only the boundary edge. The crossover is a region boundary and a node just below it can take one step whose child lands above it.
- **Assert against the envelope's local dimensions**, not as a bare number comparison, so a region where both sides are zero fails rather than passes.
- **Measure the tolerance before choosing it.** Run the tight case, record what fails, then set the number that admits only what a clay render cannot distinguish. Write the justification beside the constant.
- **Assert on the surface as drawn.** Every vertex of the junction ring, against the lobed and finitely-sampled parent, not skeleton centres against analytic radii.

### Investigation targets

**Required** (read before coding):
- `src/radius.test.ts:127-147` - "never thickens from a parent toward its child", the per-edge-invariant test shape to copy
- `src/skeleton/twigs.ts` - the local pass from task .3, where the handoff happens
- `src/mesh/surface.ts:370-400` - how a child ring is contained against its parent, and the inscribed-radius reasoning
- `src/envelope.ts:70-125` - `envelopeRadiusAt`, for asserting against local dimensions

**Optional** (reference as needed):
- `src/radius.ts:119` - `MAX_TAPER_SHED`, accumulated by distance rather than per node, which is why subdividing does not change taper

### Key context

- Project memory, interpenetrating junctions: assert the geometric claim on every vertex, never on its centre, and state the bound in terms of the drawn surface including lobes and segment count. A test that checks a centre is testing placement, not containment.
- Project memory, zero width is not no constraint: a width-only predicate is vacuous where width is zero. This is the single most likely way this task ships a test that proves nothing.
- Project memory, the slack band: a tolerance sized by the thing it protects rather than the failure it corrects left the orbit pivot off the subject. Size this one by measuring first.
- Project memory, an AC that enumerates the default state is the test: R2 names radius, direction and taper. A fourth check added later with a comment for justification is a review finding, not an improvement.
- The structural argument for direction continuity: a tree is a cantilever, so an abrupt direction change is a discontinuous bending moment. It reads as a kink because it is one.

### Acceptance

- [ ] Child radius interpolates from the parent's local cross-section at the fork (R2)
- [ ] Radius ratio at the crossover falls inside the range that ratio takes over the ten generations above it
- [ ] Direction change at the crossover is within the turn limit already enforced during growth
- [ ] Taper rate is within a stated tolerance of the rate immediately above
- [ ] Continuity is sampled at several generations either side, not only at the boundary edge
- [ ] Every assertion is against the envelope's local dimensions, so a zero-width region fails rather than passes (R2 error case)
- [ ] The tolerance was measured tight first, and its justification is written beside the constant
- [ ] Junction-ring assertions are per-vertex against the drawn surface
- [ ] A parameter set whose crossover falls outside the grown range is reported as untested, not passed (R2 error case)
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
