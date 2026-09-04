---
satisfies: [R4, R7]
---
# fn-5-branch-until-the-tips-bear-leaves-one.1 The search radius learns how far apart the attractors are

## Description
Derives the attractor search radius from attractor spacing instead of purely from the growth step (R4), so branching more finely cannot blind the search. First because every step below today's is the regime that starves growth, and a depth dial built on the old derivation would produce stumps at exactly the settings this spec exists to reach.

**Size:** M
**Files:** `src/skeleton/grow.ts`, `src/skeleton/colonize.test.ts`, `src/skeleton/grow.test.ts`
**Touches:** [src/skeleton/grow.ts, src/skeleton/grow.test.ts, src/skeleton/colonize.test.ts]

### Approach

- `defaultGrowth` at `src/skeleton/grow.ts:66-79` derives `influenceRadius = step * 9`. That multiple is the bug: it is blind to how many attractors there are or how far apart they fall.
- The attractor positions already exist before `defaultGrowth` is called. `growSkeleton` at `src/skeleton/grow.ts:82-91` calls `sampleEnvelope(params.envelope, params.attractors, rng)` on line 84 and assembles growth config after, so spacing is available without reordering anything.
- **The hard constraint is R7.** The current ratio is one every existing skeleton test and both presets implicitly depend on. The new derivation must resolve to the same radius at today's step and attractor density, and only diverge as the step shrinks. Treat it as a floor added under the existing multiple rather than a replacement for it.
- Expected spacing in a volume goes as the cube root of volume over count. The envelope's volume is derivable from `envelopeRadiusAt`; do not approximate it with a bounding box, since the crown's profile is the whole point of the envelope.
- No repo surveyed ties these together, so there is no reference implementation to mirror. The reasoning has to stand on its own in the comment.

### Investigation targets

**Required** (read before coding):
- `src/skeleton/grow.ts:56-91` - `defaultGrowth`, the stated ratios and their rationale, and `growSkeleton`'s assembly order
- `src/skeleton/colonize.ts:201-450` - the climb loop and grow loop, and how `influenceRadius` is actually consumed
- `src/envelope.ts:70-150` - `envelopeRadiusAt` and `sampleEnvelope`, for deriving volume and spacing
- `src/skeleton/colonize.test.ts:56-166` - the existing termination tests, especially "stops at maxNodes" and "does not grow a flagpole past an unreachable crown"

**Optional** (reference as needed):
- `src/radius.ts:168-191` - the `held` rail idiom

### Key context

- The regression case is measured and exact: a 0.44 m step with 1,600 attractors yields 186 nodes, 4 runs, 540 cm finest wood and zero usable tips today. The test passes by growing a whole tree, not by reporting a failure.
- Project memory, trunk-region guard: a guard on a region must test both ends of every edge crossing it. A radius that is correct at the trunk and wrong in the crown is the same shape of mistake.
- The `defaultGrowth` doc comment states the nine-step multiple as art direction and claims it is "narrow enough that the crown does not collapse to a single mast." That claim is measurably false at small steps and the comment is rewritten with the change, not left behind it.

### Acceptance

- [ ] The search radius accounts for attractor spacing as well as step size (R4)
- [ ] At today's step and attractor density the derived radius equals today's value, and every existing test plus both presets produce byte-identical trees (R7)
- [ ] The measured collapse at 0.44 m / 1,600 attractors grows a whole tree, asserted as a regression test (R4 error case)
- [ ] No reachable combination of step and attractor count in the panel's range starves the growth, asserted across a swept range rather than at one point
- [ ] The `defaultGrowth` doc comment describes the derivation that now exists
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
