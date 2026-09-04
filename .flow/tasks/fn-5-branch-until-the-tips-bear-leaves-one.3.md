---
satisfies: [R1, R3, R9]
---
# fn-5-branch-until-the-tips-bear-leaves-one.3 Local-rule branching below the crossover, down to leaf scale

## Description
Carries the recursion from where colonization stops down to leaf-bearing twigs (R1, R3), by a level-capped local pass that inherits colonization's exit state. This is the task that makes the tree a tree at every scale rather than a coarse tree with foliage attached.

**Size:** M
**Files:** `src/skeleton/twigs.ts`, `src/skeleton/twigs.test.ts`, `src/skeleton/grow.ts`
**Touches:** [src/skeleton/twigs.ts, src/skeleton/twigs.test.ts, src/skeleton/grow.ts]

### Approach

- **Structurally this is a second pass, not a branch inside the colonize loop.** `colonize` is a climb loop then a grow loop with per-round `Map`/`Set` structures keyed by parent index; nodes append in exactly two places, always with `parent < self`. Drive colonization to completion, then walk its tips and keep appending into the same `SkeletonNode[]`. One skeleton, two passes.
- **The parent-before-child invariant is load-bearing.** `src/radius.ts`, `src/mesh/paths.ts` and `src/mesh/surface.ts` all do single forward passes that assume it. Append with strictly increasing indices and `parent < self` or every downstream stage breaks.
- **Inherit exit state as explicit boundary conditions**: position, terminal tangent, radius and remaining vigour. Do not reseed direction; reuse `limitTurn` at `src/skeleton/colonize.ts:151-192` rather than reimplementing great-circle clamping.
- **Level-capped recursion, not a scale search.** A counter sized to real twig counts, tens to low hundreds per terminal branch. Shrinking a radius toward zero is the failure mode this whole spec exists to escape.
- **The fine orders get their own taper law**, steeper than the order-0 area-conservation exponent and anchored to the parent's actual radius at handoff. An exponent authored for limbs produces visibly wrong twigs at order 8.
- The local pass has no attractors, so it needs none of colonize's per-round `Map`/`Set` machinery. Flat arrays; determinism depends on it.
- Keep a lightweight sibling-collision check per whorl. Blind local replacement with no neighbour awareness is the documented weakness of pure rule systems, and fn-4 measures exactly that defect in this repo.

### Investigation targets

**Required** (read before coding):
- `src/skeleton/colonize.ts:201-450` - both loops, the two append sites, and where the grow loop exits
- `src/skeleton/colonize.ts:93-104` - `SkeletonNode` / `Skeleton` and the parent-index invariant in its doc comment
- `src/skeleton/colonize.ts:151-192` - `limitTurn`, to reuse for direction continuity
- `src/radius.ts:156-246` - `solveRadii`, confirmed depth-agnostic, to see what the new nodes must satisfy
- `src/mesh/paths.ts:69-137` - `branchPaths`, which will now decompose far more runs

**Optional** (reference as needed):
- `src/torsion.ts:165-180` - the XOR sub-stream convention for a new source of chance

### Key context

- Switching method by scale is standard: Weber-Penn, SpeedTree and Runions all generate per branch order with different rules per order, and Runions scopes colonization to macro structure explicitly. This is not a compromise.
- Which terms matter at the fine orders, from a 1.6k-star reference implementation: per-level angle, children per node, length ratio, radius, taper, sections and segments. That is a starting list, not a specification.
- Project memory, zero width is not no constraint: a predicate that is vacuous where the envelope has no width let the tree fork below its own envelope once already. The local pass runs at the crown's edges where width is smallest.
- Determinism is array-backed iteration only. A `Map` or `Set` in the candidate path is how R7 fails intermittently on someone else's machine.

### Acceptance
## Acceptance
- [ ] The recursion continues from colonization's tips to leaf-bearing twigs, appending into one skeleton (R1)
- [ ] Every appended node satisfies `parent < self` with strictly increasing indices, asserted over the whole skeleton
- [ ] Direction inherits the terminal tangent through `limitTurn`; no reseed at the handoff
- [ ] The fine orders use their own taper law, anchored to the parent's radius at handoff
- [ ] Recursion stops on a level cap that is a named parameter, never on a shrinking search radius
- [ ] Terminal wood reaches leaf scale, with the leaf-to-twig-diameter ratio measured on both presets against today's 0.15 to 1 baseline (R3)
- [ ] A sibling-collision check runs per whorl
- [ ] No `Map` or `Set` in the candidate path; determinism asserted by building twice from one seed
- [ ] The four stages before this produce byte-identical output
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green
- [ ] Every twig node's direction passes through the same growth-bias function colonization uses, at the same position and step (R9)
- [ ] A test drives one bias term to an extreme and asserts the twigs move with the limbs (R9)
- [ ] Every bias term at zero reproduces an unbiased local recursion byte-identically (R9 error case)
- [ ] The local rules' resting values are botanical defaults, with their sources stated beside them (R1)
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
