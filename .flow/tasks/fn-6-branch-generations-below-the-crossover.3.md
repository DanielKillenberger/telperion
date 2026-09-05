---
satisfies: [R5, R2]
---
# fn-6-branch-generations-below-the-crossover.3 Thickness below the crossover reads the pass, and the seam becomes every handoff

## Description
The solve below the crossover stops deriving twig radii from geometry and reads the base radii the pass recorded, applying only the internode length taper; `twigTaper` is retired. The continuity suite generalises from one crossover to every handoff and asserts fn-5's three criteria at each, on the surface as drawn, never vacuously; the bias-field criterion carries over per internode. Split from task 2 because it owns different files and its tests are the spec's load-bearing invariant.

**Size:** M
**Files:** `src/radius.ts`, `src/radius.test.ts`, `src/skeleton/continuity.test.ts`, `src/presets/two-trees.ts` (drop `twigTaper`), `src/presets/two-trees.test.ts` (radii keys), `harness/params.ts` and `harness/skeleton-view.ts` (`twigThinning` dial and mapping go)
**Touches:** [src/radius.ts, src/radius.test.ts, src/skeleton/continuity.test.ts, src/presets/two-trees.ts, src/presets/two-trees.test.ts, harness/params.ts, harness/skeleton-view.ts, harness/skeleton-view.test.ts]

### Approach
- `solveRadii` below the crossover (`src/radius.ts:334-368`): for an appended node, `startRadius` is the recorded base radius when the node begins a branch, else the parent's radius; `radius` applies the same length-taper exponential the limbs carry (`shed[parent] - shed[i]`) on branch nodes, and on twig nodes only the twig's stated tip fraction, so a twig's drawn diameter is the anatomy's on every handoff. A test asserts drawn twig diameters equal across handoff radii, heights and presets (R2). Remove the `kids ** (-1/exponent)` share and `ratio ** twigTaper`. Above the crossover nothing changes; assert byte-identity on both presets (`src/radius.test.ts:288` already does this per preset — keep it).
- `RadiusParams.twigTaper` and `DEFAULT_TWIG_TAPER` go; the module header `:57-82` and `:105-132` rewrite to the new owner (the pass) in the house style.
- Continuity (`src/skeleton/continuity.test.ts`): `seam()` returns the list of handoff edges (every appended node whose parent is a colonization node), `untested` when empty. For each handoff sample ten fork generations above along the colonization lineage (`edgesAbove` already walks from a given tip) and the branch generations below; assert radius ratio inside the range above, direction change within the turn limit, taper rate within tolerance, and the junction ring on the surface as drawn, with the seam edge bound to the law's own child-radius share rather than the k^(-1/n) share (`SEAM_SHARE_FLOOR` becomes the law's ratio). The tolerance is measured before it is chosen, as fn-5 did: run tight, record what fails, ship the number a clay render cannot distinguish.
- Bias criterion: the existing lean-driven test (`src/skeleton/twigs.test.ts` R9 test) runs per internode over branches; every term at zero reproduces the unbiased recursion byte-identically.
- Presets drop `twigTaper`; the radii structural-key test follows; the `twigThinning` dial (`harness/params.ts:~300`) and its mapping go.

### Investigation targets
**Required:**
- `src/radius.ts:225-371` — the solve; `:306-322` the fork rule that must stay byte-identical
- `src/skeleton/continuity.test.ts:83-200` — `grown`, `seam`, `ordersOf`, `edgesAbove`, the constants
- `src/skeleton/twigs.ts` (as rewritten in task 2) — `TwiggedSkeleton` fields

**Optional:**
- `src/mesh/surface.ts` — the junction ring the continuity test reads
- `.flow/memory/` entry `interpenetrating-junctions-contain-the-2026-09-04` — containment at every vertex

### Key context
- The fn-5 correctness audit found a x3.7 seam thinning slipping through a range test; bind the seam edge to the law, not to the crown's loosest fork.
- A parameter set with no handoffs reports untested, never passes.
## Acceptance
- [ ] Below the crossover `solveRadii` reads recorded base radii and applies internode taper only; `twigTaper` is gone from params, presets, dials
- [ ] Above the crossover the field is byte-identical on both presets before and after
- [ ] Continuity asserts at every handoff (radius ratio, direction, taper, junction ring), samples generations either side, reports untested when there are none
- [ ] Bias-field test holds per internode; all terms at zero reproduce the unbiased recursion byte-identically
- [ ] `npx vitest run src/radius.test.ts src/skeleton/continuity.test.ts src/presets harness` green; `npx tsc --noEmit` green
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
