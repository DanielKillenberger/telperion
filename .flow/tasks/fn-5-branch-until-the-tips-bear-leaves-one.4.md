---
satisfies: [R2, R3]
---
# fn-5-branch-until-the-tips-bear-leaves-one.4 Crossover continuity, and the test that cannot pass vacuously

## Description
Makes radius, direction and taper continuous where the method changes, and proves it (R2). A survey of open tree generators found none that transitions in place and asserts continuity at the seam, so there is no reference test to copy and the whole risk of this task is writing one that passes for the wrong reason.

**Size:** M
**Files:** `src/skeleton/twigs.ts`, `src/skeleton/continuity.test.ts`, `src/radius.ts` (the fine-order taper law lives in the thickness solve, and no other task owns it)
**Touches:** [src/skeleton/twigs.ts, src/skeleton/continuity.test.ts, src/radius.ts, src/radius.test.ts]

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
## Acceptance
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
## Done summary
The fine orders below the crossover now have their own taper law in the thickness solve, anchored to each parent's actual radius at the handoff, and the seam is proved continuous in radius, direction and taper on both presets - with the test that cannot pass vacuously.

What landed (commit range in the evidence file; workspace wt/fn5-4):
- `src/skeleton/twigs.ts`: `branchTwigs` returns `TwiggedSkeleton` (`Skeleton` + `crossover`), the index of the first node the pass appended; equal to `nodes.length` on every early return (bare skeleton, zero orders, ceiling already hit). `growSkeleton` passes the object through unchanged, so the harness gets it without edits.
- `src/radius.ts`: above the crossover the fork rule runs exactly as before - same loops, same order, so the trunk-to-limb field is byte-identical with twigs and without (asserted on both presets at 1 and 8 orders), and at zero orders the whole field equals that of a skeleton the twig pass never touched. Below it, top-down: `start_child = radius_parent * k^(-1/n) * min(1, L_child/L_parent)^twigTaper`, then the same length taper along the edge. At the handoff at rest the length ratio is 1, so a twig leaves its tip with exactly the balanced share a limb leaves a fork with; the seam has no ratio of its own. `RadiusParams.twigTaper` is optional (rails: NaN/absent -> `DEFAULT_TWIG_TAPER`, negative -> 0, ceiling 4); it is NOT in `DEFAULT_RADII` because the harness test "hands the solve's other two terms over as dials" pins that object to three keys and harness files are the sibling's this wave.
- `DEFAULT_TWIG_TAPER = 0.7`, measured (leaf length / median terminal diameter at eight orders, the R3 instrument): area alone (0) Telperion 2.1:1, Laurelin 1.2:1; 0.5 -> 12.5 / 7.4; 0.7 -> 25.5 / 15.1; 1.0 -> 74 / 44. 0.7 is where Telperion meets 25:1; Laurelin sits stouter by its own exponent 2.7.
- `src/skeleton/continuity.test.ts` (7 tests): ratio at the seam and at orders 1-4 below inside the [min,max] the ratio takes over ten fork generations above (per tree, bounded at the crown base); direction: every step in those bands within the run's turn limit; taper: drawn slope along runs (ring-to-ring, as a half-angle) averaged over the orders below vs the ten generations above on the same run; junction rings: every vertex of every child run's socket ring at the seam, at the whorls four orders below and at the forks above, inside the parent's inscribed radius as drawn (lobes, segment count, flare), with float32 slack of 4 ulps; error case: zero orders, a plain Skeleton, and a ceiling that stops the pass before its first twig are all reported `untested` with a reason, never sampled.
- Every sample is first held to the envelope's height (the dimension every radius is authored in): radii finite and above 1e-5 of height, a decade under the finest wood sampled (2.4e-4 Telperion, 5.3e-4 Laurelin) and above the trunk-floor collapse (4e-6). The crown's local half-width was tried and rejected as the scale: 11 Telperion / 3,970 Laurelin nodes near the seam sit above the crown top where the authored width is zero, and wood there is wood.
- Tolerance measured tight first (0.5 deg): Telperion 109/133 runs over, median 1.9, p90 5.6, worst 16.4 deg; Laurelin 364/375 over, median 3.4, p90 5.3, worst 39.9. Ships at 6 deg on the ninetieth percentile; the justification beside the constant names the tail as the blunt-tip defect (a limb ending as one full-radius tip), which is its own spec.
- `src/radius.test.ts` (+4 tests): byte-identity above the crossover on both presets; the law per edge to 1e-12; rails; monotonic below the seam with `internode: 4` (the clamp case).

For the conductor:
- Both presets should state `twigTaper` in `radii` (and `two-trees.test.ts` key list / harness `toRadiusParams` round-trip follow) - outside Touches this wave, flagged not done.
- AC4 as literally worded (every run within tolerance of the rate above) is not achievable with the designed steepening plus blunt tips; the test asserts the ninetieth percentile and says so. Conductor's call whether that reading of "a stated tolerance" stands.
- Not done: red-first confirmation of the radius.test additions against the previous solve (the continuity tests were red-first by construction at the tight tolerance; the byte-identity test was not run against a broken solve). Timebox.
- `src/index.ts` still does not re-export `TwiggedSkeleton` / `DEFAULT_TWIG_TAPER` (same gap .2 and .3 left).

baseline: green (npx vitest run 19 files / 302 tests; npx tsc --noEmit; npm run build) at 04e0277
verify: green (npx vitest run 20 files / 317 tests; npx tsc --noEmit; npm run build); gate classify: see evidence

stage: impl-review - skipped(policy: parallel-wave - conductor reviews after integration; REVIEW_MODE=none)
## Evidence
- Commits: 562cb70, 7118168
- Tests: npx vitest run (21 files / 325 tests on the joined target), npx tsc --noEmit clean, npm run build clean, red-first confirmed by the conductor: the four radius.test.ts additions fail against the pre-.4 solve, byte-identity at zero orders by the conductor's node hash: unchanged, conductor call on AC4: the 90th-percentile taper assertion stands; the tail is the blunt-tip defect, which is fn-4's spec, and the tight-first measurement is written beside the constant
- PRs: