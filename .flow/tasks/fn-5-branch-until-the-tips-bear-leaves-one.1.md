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
## Acceptance
- [ ] The search radius accounts for attractor spacing as well as step size (R4)
- [ ] At today's step and attractor density the derived radius equals today's value, and every existing test plus both presets produce byte-identical trees (R7)
- [ ] The measured collapse at 0.44 m / 1,600 attractors grows a whole tree, asserted as a regression test (R4 error case)
- [ ] No reachable combination of step and attractor count in the panel's range starves the growth, asserted across a swept range rather than at one point
- [ ] The `defaultGrowth` doc comment describes the derivation that now exists
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green
## Done summary
The attractor search radius is now the wider of nine growth steps and 1.2 attractor spacings, the spacing being the cube root of the crown's volume per attractor with the volume integrated from the envelope's own profile (`influenceRadiusFor` in `src/skeleton/grow.ts`; `defaultGrowth` takes the attractor count and `growSkeleton` passes the count it scattered). At today's step the floor sits under nine steps at both presets and every fixture the suite grows, asserted by signature equality against the nine-step radius (R7); Telperion at a 0.44 m step with 1,600 attractors grows a 14,000-node, 1,200-tip tree where nine steps grew a 169-node, 3-tip stump, both halves asserted (R4). The `defaultGrowth` comment is rewritten around the derivation, and `influenceRadiusFor` carries the argument and the reason the floor is not larger.

Finding the conductor asked for, with the numbers: R4 with margin and R7 at the letter cannot both hold with a floor under the nine-step multiple. R7 pins the floor under 1.229 spacings (the suite's sparsest fixture: 50 m tall, spread 1.4, shoulder 4, 900 attractors - off-panel; the spread-1.2 fixture is next at 1.463; Laurelin at the panel's 250 attractors is 1.432). Starvation is seed-dependent right up to that ceiling: over 12 seeds x 9 panel-range cells, 1.2 spacings starves 3 of 108 draws (default envelope / 1,600 attractors / step 0.0055h seed 3 -> 74 nodes; Telperion / 1,600 / 0.003h seed 11 -> 155; Telperion / 700 / 0.003h seed 11 -> 169), 1.229 and above starve 0 of 108, and 1.0 starves Telperion at 250 attractors on every seed. Every starved tree died at the crown base with 2-4 attractors killed and nothing living within reach of any tip, because `colonize` ends the tree the moment no node sees an attractor. Shipped at 1.2 because the task names R7 as the hard constraint; the sweep test runs at the presets' own seeds and states the tail in its comment rather than asserting it away. The real fix is in `colonize.ts` (outside this task's Touches): a tip with nothing in reach should not end the tree while living attractors remain. Full table in the run note `fn5-t1-search-radius-floor.md`.

Tests: `src/skeleton/grow.test.ts` - "resolves to nine steps at today's step, on every tree there is" (R7), "grows a whole tree where nine steps grew a stump" (R4 error case), "does not starve anywhere on the panel as the step shrinks" (R4 sweep), "holds the radius at nine steps until the spacing overtakes it", and the scale test extended to the floor. `colonize.test.ts` needed no change. New tests confirmed red against the old derivation (189 nodes vs 5,000; 94 vs 437) before the fix.

baseline: green (npx vitest run 268/268, npx tsc --noEmit, npm run build at 97684f0)

stage: impl-review - skipped(policy: parallel-wave - conductor owns the review after integration)
## Evidence
- Commits: 24aa80d8b2307df4b702f6e43e598859a18bffe7
- Tests: npx vitest run (18 files / 272 tests on the integrated target; the 268 pre-existing pass unchanged, which is the byte-identity proof), npx tsc --noEmit clean, npm run build clean, R4 regression: Telperion 0.44 m / 1,600 attractors grows ~14,000 nodes / ~1,200 tips where nine steps grew 169 / 3, Residual: at k=1.2 spacings, 3 of 108 seed x panel-cell draws still starve (seed-dependent); root cause is colonize ending the tree when no tip sees an attractor while living attractors remain - outside this task's Touches, carried forward
- PRs: