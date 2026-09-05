---
satisfies: [R1, R2, R3, R5, R8]
---
# fn-6-branch-generations-below-the-crossover.2 Branches from the tips: the pass grows under the law, reads the field, ends on radius

## Description
Rewrite the pass below the crossover so every colonization tip grows a branch under the law from task 1: a run of internodes, laterals along it at the divergence, a leader continuing, each lateral the root of the next generation, recursing until the base radius is at or below the twig's, where a twig is emitted and marked. The pass takes the colonization radius field as an argument and records, per appended node, the branch it belongs to and the base radius it was assigned. `levels` is retired. This task covers tips only; laterals along colonization limbs are task 4 so the R8 prefix property can be proven against a working tip pass first.

Re-planned on 2026-09-05 after two worker attempts: this task now also carries what was task 3, the solve below the crossover and the continuity suite, because the two cannot be gated separately. The moment the pass grows real branches, the old fine-order radius law thins the fixed twigs by geometry and eight continuity tests fail; the solve has to read the pass's records in the same commit. Task 3 is kept as a receipt that points here.

**Size:** M
**Files:** everything the former task 2 named (`src/skeleton/twigs.ts`, `src/skeleton/twigs.test.ts`, `src/skeleton/grow.ts`, `src/skeleton/grow.test.ts`, `src/skeleton/shed.ts`, `src/skeleton/shed.test.ts`, `src/presets/two-trees.ts`, `src/presets/two-trees.test.ts`, `src/index.ts`, `harness/skeleton-view.ts`, `harness/params.ts`, `harness/skeleton-view.test.ts`, `src/skeleton/law.test.ts`, `src/skeleton/fill.test.ts`, `src/radius.test.ts`), plus the former task 3's `src/radius.ts` and `src/skeleton/continuity.test.ts`, plus the three fixtures that mean colonization alone and must call it directly: `src/canopy/cull.test.ts`, `src/mesh/surface.test.ts`, `src/skeleton/persistence.test.ts`
**Touches:** [src/skeleton/twigs.ts, src/skeleton/twigs.test.ts, src/skeleton/grow.ts, src/skeleton/grow.test.ts, src/skeleton/shed.ts, src/skeleton/shed.test.ts, src/presets/two-trees.ts, src/presets/two-trees.test.ts, src/skeleton/continuity.test.ts, src/index.ts, harness/skeleton-view.ts, harness/params.ts, harness/skeleton-view.test.ts, src/skeleton/law.test.ts, src/skeleton/fill.test.ts, src/radius.ts, src/radius.test.ts, src/canopy/cull.test.ts, src/mesh/surface.test.ts, src/skeleton/persistence.test.ts]

### Approach
- Start from the preserved attempt: `git apply .flow/tmp/fn6-task2-attempt2.patch` applies cleanly on 986601b and carries the pass, the records through shedding, the thickness parameters through growth and harness, and the authorized fixture migrations; verify it with the focused tests before extending it. Its conductor-noted gaps: it leaves production `radius.ts` untouched (the former task 3 work below), presets use `lengthRatio` 0.4 (correct), and the panel's legacy orders control is left for task 6 with `toSkeletonParams` ignoring it.
- Colonization-only fixtures (`src/canopy/cull.test.ts` the filled-crown fixture, `src/mesh/surface.test.ts` default-growth vertex budgets and timing cases, `src/skeleton/persistence.test.ts` the extreme sweep) call `colonize` directly, or `growSkeleton` with a twig whose radius exceeds every handoff so the pass appends only twigs; their assertions and budgets stay as they are. Never widen a budget to admit branch generations into a test that never meant to measure them.
- New `TwigParams` shape (contract): the fixed `twig` anatomy; `lengthRatio` (child branch length over parent's, resting at `DEFAULT_BRANCH_LAW.lengthRatio` = 0.4, the value task 1 measured and sourced; the 0.6 an earlier draft named is superseded); `ratioPower` (rest from the p.126 table); `internodes` per branch (rest measured in task 1, small integer); `laterals` per internode (rest 1, alternate); `angle`; `divergence`; the level cap stays a constant. Every field stated in full by both presets (structural-key test at `src/presets/two-trees.test.ts:264-271` updated in the same commit).
- `branchTwigs(skeleton, field, config, twigs)`: signature gains the `RadiusField`. Keep the frontier/`Shoot` structure (`src/skeleton/twigs.ts:280-439`), the bias call and `limitTurn` per internode exactly as at `:402-408`, the bare-trunk guard at `:421`. Replace the per-order length with the branch's law-given length divided into `internodes`; a lateral's base radius by `childRadius`; the first generation's length by `branchLength(field.radius[tip])`.
- Collision reference: laterals are tested against the node's own arrival direction, not the accepted leader (spec §Architecture, for R8). The leader is exempt.
- Sub-internode branch collapses to a twig (R1 error case). Level cap reached before twig radius is recorded in the result, not clamped silently (R3 error case).
- `TwiggedSkeleton` gains, for appended nodes: branch id (index of the branch's first node), assigned base radius (`Float64Array`), and a twig mark. Keep `crossover`. State the shapes as typed arrays parallel to `nodes` from `crossover`.
- `growReport(params, radii = DEFAULT_RADII)` and `growSkeleton` likewise (`src/skeleton/grow.ts:332-350`): solve the fork rule on the colonized skeleton under the caller's `RadiusParams` before the pass and hand the field in; the harness build (`harness/skeleton-view.ts:~398`) and every test that solves radii pass the same `RadiusParams` to both, and a test asserts the two solves agree above the crossover. Changing trunk radius or fork exponent changes the handoff radii and the generations.
- Twig radius: when `childRadius` falls at or below the twig's radius the node is a twig and its recorded base radius is the twig's own (`TwigAnatomy.diameter / 2`), not the law's value; this is the one exception to the base-radius-equals-law assertion and the test states it. Twig dimensions are asserted equal across handoff radii, envelope heights and both presets.
- Validation: `branchTwigs` checks parent-before-child (no self, forward or out-of-range parent) and `field.radius.length === nodes.length`; on failure it returns the base unchanged with `crossover = nodes.length` and a `refused` reason on the result. Nothing throws. Negative tests for each case.
- Shedding: `shedTwigs` (`src/skeleton/shed.ts:66-120`) compacts the pass's parallel records with the surviving nodes and remaps branch ids through its index map; a test removes a subtree that shifts a surviving branch and its twig descendants and asserts the records still name the right nodes. `defaultGrowth`'s `twigHeadroom` (`:153-159`) is re-derived in task 4; here keep the ceiling and make the presets finish uncapped.
- Presets: replace `levels: 8` blocks (`src/presets/two-trees.ts:127-133, 271-277`) with the new shape and rewrite their comments; the harness round-trip (`harness/skeleton-view.ts:66-112, 273-318`) and `DEFAULT_PARAMS` (`harness/params.ts:363-371`) must compile, so map the new fields and drop `twigLevels` from `toSkeletonParams`; the dial itself is retired in task 6.
- Tests to rewrite in `twigs.test.ts`: parent-before-child over the whole tree; every appended node's base radius equals the law's; recursion ends at twig radius and every leaf node of the pass is marked twig; R8 first half: the pass is byte-identical for the same (skeleton, field, params) and a different field changes only appended nodes; colonization nodes byte-identical before and after.

- Fixture migration rule: a test that asserted the old `levels`/`internode` shape is rewritten to assert the new shape with the same intent (a dial changes node counts; zero orders equals colonization alone becomes: a pass whose every handoff is already at twig radius appends only twigs). Never delete an assertion to get green; never add a compatibility branch for `levels`.

### The solve below the crossover and the seam (former task 3)
- `solveRadii` below the crossover (`src/radius.ts:334-368`): for an appended node, `startRadius` is the recorded base radius when the node begins a branch, else the parent's radius; `radius` applies the same length-taper exponential the limbs carry (`shed[parent] - shed[i]`) on branch nodes, and on twig nodes only the twig's stated tip fraction, so a twig's drawn diameter is the anatomy's on every handoff. A test asserts drawn twig diameters equal across handoff radii, heights and presets (R2). Remove the `kids ** (-1/exponent)` share and `ratio ** twigTaper`. Above the crossover nothing changes; assert byte-identity on both presets (`src/radius.test.ts:288` already does this per preset — keep it).
- `RadiusParams.twigTaper` and `DEFAULT_TWIG_TAPER` go; the module header `:57-82` and `:105-132` rewrite to the new owner (the pass) in the house style.
- Continuity (`src/skeleton/continuity.test.ts`): `seam()` returns the list of handoff edges (every appended node whose parent is a colonization node), `untested` when empty. For each handoff sample ten fork generations above along the colonization lineage (`edgesAbove` already walks from a given tip) and the branch generations below; assert radius ratio inside the range above, direction change within the turn limit, taper rate within tolerance, and the junction ring on the surface as drawn, with the seam edge bound to the law's own child-radius share rather than the k^(-1/n) share (`SEAM_SHARE_FLOOR` becomes the law's ratio). The tolerance is measured before it is chosen, as fn-5 did: run tight, record what fails, ship the number a clay render cannot distinguish.
- Bias criterion: the existing lean-driven test (`src/skeleton/twigs.test.ts` R9 test) runs per internode over branches; every term at zero reproduces the unbiased recursion byte-identically.
- Presets drop `twigTaper`; the radii structural-key test follows; the `twigThinning` dial (`harness/params.ts:~300`) and its mapping go.

### Investigation targets
**Required:**
- `src/skeleton/twigs.ts:280-439` — the pass to rewrite
- `src/skeleton/grow.ts:153-159, 259-350` — budget, `resolveGrowth`, `growReport`
- `src/skeleton/law.ts` (task 1) — the law to call
- `src/presets/two-trees.test.ts:232-289` — structural-key contract
- `harness/skeleton-view.ts:66-112, 273-318` — round-trip mapping that must still compile

**Optional:**
- `src/skeleton/colonize.ts:160-193` — `limitTurn`; `src/torsion.ts:96-100` — `GrowthBias` signature
- `src/mesh/frames.ts` — `transportFrames`, if a per-branch frame is wanted

### Former task 3's targets
**Required:**
- `src/radius.ts:225-371` — the solve; `:306-322` the fork rule that must stay byte-identical
- `src/skeleton/continuity.test.ts:83-200` — `grown`, `seam`, `ordersOf`, `edgesAbove`, the constants
- `src/skeleton/twigs.ts` (as rewritten in task 2) — `TwiggedSkeleton` fields

**Optional:**
- `src/mesh/surface.ts` — the junction ring the continuity test reads
- `.flow/memory/` entry `interpenetrating-junctions-contain-the-2026-09-04` — containment at every vertex

### Key context
- Deterministic: flat arrays, breadth-first append, no `Map`/`Set` in the hot path (`twigs.test.ts:290-292` asserts on the source).
- The solve below the crossover is this task's now: it reads the recorded base radii, so tests assert on both the pass's records and `solveRadii` output.
- The fn-5 correctness audit found a x3.7 seam thinning slipping through a range test; bind the seam edge to the law, not to the crown's loosest fork.
- A parameter set with no handoffs reports untested, never passes.
## Acceptance
- [ ] `branchTwigs` takes the radius field; `growReport`/`growSkeleton` take `RadiusParams`, solve the fork rule under them before the pass, and the harness passes the preset's radii to both solves
- [ ] Twig nodes carry the twig's own radius regardless of handoff radius, height or preset; the base-radius-equals-law assertion names the twig as its one exception
- [ ] Invalid parent indices or a wrong-length field return the base unchanged with a named refusal; negative tests for self, forward, out-of-range parents and field length
- [ ] `shedTwigs` compacts branch ids, base radii and twig marks with the survivors; the shifting-subtree test passes
- [ ] Every tip grows a branch of `internodes` steps with laterals along it; recursion ends when base radius ≤ twig radius; twig nodes are marked; level cap reached is reported
- [ ] `TwigParams` has no `levels`; both presets state the new shape in full and the structural-key test asserts it
- [ ] Same (skeleton, field, params) → byte-identical appended nodes; colonization nodes unchanged byte for byte on both presets
- [ ] Laterals collide against the node's arrival direction; the leader is exempt
- [ ] `npx vitest run src/skeleton src/presets` green; `npx tsc --noEmit` green; harness compiles
- [ ] Below the crossover `solveRadii` reads recorded base radii and applies internode taper only; `twigTaper` is gone from params, presets, dials
- [ ] Above the crossover the field is byte-identical on both presets before and after
- [ ] Continuity asserts at every handoff (radius ratio, direction, taper, junction ring), samples generations either side, reports untested when there are none
- [ ] Bias-field test holds per internode; all terms at zero reproduce the unbiased recursion byte-identically
- [ ] `npx vitest run src/radius.test.ts src/skeleton/continuity.test.ts src/presets harness` green; `npx tsc --noEmit` green
- [ ] Colonization-only fixtures in cull, surface and persistence tests call colonization directly; no budget or timeout widened to admit branch generations
- [ ] `npx tsc --noEmit` and the full `npx vitest run` green on the conductor's checkout
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
