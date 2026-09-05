---
satisfies: [R1, R2, R3, R8]
---
# fn-6-branch-generations-below-the-crossover.2 Branches from the tips: the pass grows under the law, reads the field, ends on radius

## Description
Rewrite the pass below the crossover so every colonization tip grows a branch under the law from task 1: a run of internodes, laterals along it at the divergence, a leader continuing, each lateral the root of the next generation, recursing until the base radius is at or below the twig's, where a twig is emitted and marked. The pass takes the colonization radius field as an argument and records, per appended node, the branch it belongs to and the base radius it was assigned. `levels` is retired. This task covers tips only; laterals along colonization limbs are task 4 so the R8 prefix property can be proven against a working tip pass first.

**Size:** M
**Files:** `src/skeleton/twigs.ts` (rewrite of `branchTwigs`, `TwigParams`, `DEFAULT_TWIGS`, `TwiggedSkeleton`), `src/skeleton/twigs.test.ts` (rewrite), `src/skeleton/grow.ts` (order: colonize → solve fork rule → pass → shed; `growReport`/`growSkeleton` take `RadiusParams`), `src/skeleton/shed.ts` and `src/skeleton/shed.test.ts` (compact the pass's records through shedding), `src/skeleton/grow.test.ts` (fixtures), `src/presets/two-trees.ts` and `src/presets/two-trees.test.ts` (twigs block shape; structural-key test), `src/skeleton/continuity.test.ts` (only the `grown()` helper's `levels` override), `src/index.ts` (exports)
**Touches:** [src/skeleton/twigs.ts, src/skeleton/twigs.test.ts, src/skeleton/grow.ts, src/skeleton/grow.test.ts, src/skeleton/shed.ts, src/skeleton/shed.test.ts, src/presets/two-trees.ts, src/presets/two-trees.test.ts, src/skeleton/continuity.test.ts, src/index.ts, harness/skeleton-view.ts, harness/params.ts]

### Approach
- New `TwigParams` shape (contract): the fixed `twig` anatomy; `lengthRatio` (child branch length over parent's, Weber & Penn nLength, rest 0.6); `ratioPower` (rest from the p.126 table); `internodes` per branch (rest measured in task 1, small integer); `laterals` per internode (rest 1, alternate); `angle`; `divergence`; the level cap stays a constant. Every field stated in full by both presets (structural-key test at `src/presets/two-trees.test.ts:264-271` updated in the same commit).
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

### Key context
- Deterministic: flat arrays, breadth-first append, no `Map`/`Set` in the hot path (`twigs.test.ts:290-292` asserts on the source).
- Keep `radius.ts` untouched here; task 3 makes the solve read the recorded bases. Until then the fine-order law still runs and the tree is thicker than it will be; tests in this task assert on the pass's own records, not on `solveRadii` output below the crossover.
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
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
