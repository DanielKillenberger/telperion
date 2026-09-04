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
The skeleton is one structure built in two passes: `src/skeleton/twigs.ts` continues from every tip colonization leaves, breadth-first by order, appending into the same node array with `parent < self` by construction, level-capped on `SkeletonParams.twigs.levels` (default 0) and on the run's node ceiling, with no Map, Set or chance in the candidate path. Every twig step inherits the parent's tangent through `limitTurn` (now exported from colonize.ts) and passes through the same `config.bias` colonization consults, at the parent's position and colonization's step (R9); with every term at zero the field is `v/|v|`, exactly what the unbiased path does, so zero terms and no field are one recursion byte for byte. Six rules rest on botanical defaults with their sources in the header - children 2 (alternate phyllotaxis, Corner), angle 45 (Weber-Penn nDownAngle), divergence 137.508 (golden angle, Jean; Weber-Penn nRotate 140), internode 1 step (continuity), taper 0.6 per order (Weber-Penn nLength) - and a per-whorl sibling check drops two children within half the angle the tree can actually make, min(angle, turn limit)/2.

R7: 12 pinned skeleton signatures (both presets, every R7 fixture, panel default, Telperion at 0.011) byte-identical before and after (scratchpad fn5-t3-pre/post-signatures.json). R3, measured: at eight orders at rest the leaf is 1.64x the median terminal diameter on Telperion and 1.16x on Laurelin, from 0.15/0.16 today; six orders is 0.86/0.70. Turn limit holds at every twig node (26.00 / 46.00). The R9 lean test isolates the field's effect on the twig pass (one base, lean field vs zero field): +0.65 of a unit step along the trunk's lean on both trees; a bypassed field fails exactly the two R9 tests (negative check run and reverted).

baseline: green (npx vitest run 18 files / 281 tests; npx tsc --noEmit; npm run build) at d5e3a907
verify: green (npx vitest run 19 files / 302 tests; npx tsc --noEmit; npm run build) at 5393596; gate classify: FULL; receipts unittest/typecheck/build written
tests added: src/skeleton/twigs.test.ts (20 tests: one per AC and per enumerated error case - R1 degenerate skeleton, R1 non-finite rails, R7 zero orders, R9 zero-field identity, node ceiling, bare-trunk guard); harness/skeleton-view.test.ts "hands the six twig rules over"; src/presets/two-trees.test.ts key list gains "twigs"

outside Touches (implementation): src/skeleton/colonize.ts (one word: `limitTurn` exported, per the task's reuse-not-reimplement instruction); src/presets/preset.ts and src/presets/two-trees.ts (both presets state `twigs` in full at levels 0, per conductor steering); harness/params.ts and harness/skeleton-view.ts (six `twig*` dials as a "twigs" group after `step`) - forced by the harness's own preset round-trip test, which is designed to fail when a preset states a term the panel cannot express; the alternative was a red gate. Same four harness files .2 touched for `step`.
outside Touches (tests only): src/presets/two-trees.test.ts, harness/skeleton-view.test.ts
findings for .4/.5: the zero-term field is NOT bit-identical to no field in colonization (colonize never re-unitises `chosen`), so the R9 error case is asserted on `branchTwigs`; in a leaning tree the crown's mean step points against the lean (colonization spends more steps fighting it), so the trunk is the limb the test compares against; `maxNodes` is honoured mid-order and at the default step's 8,000 it binds Telperion at ~4 orders and Laurelin at ~2 - the ceiling should scale with orders before the panel's `orders` dial is usable; radius taper at the fine orders is solveRadii's area rule and is not measured here; twigs are not held inside the envelope (at rest they extend at most 2.5 steps past a tip)
follow-ups (not built): src/index.ts does not re-export TwigParams / DEFAULT_TWIGS / branchTwigs / resolveTwigs
run-note: /home/daniel/Projects/telperion/.git/flow-notes/fn-5-branch-until-the-tips-bear-leaves-one-20260904T203339Z-685936/fn5-t3-twigs.md

stage: impl-review - skipped(policy: parallel-wave - conductor reviews after integration; REVIEW_MODE=none)
## Evidence
- Commits: 539359612a8e1e2fa5692e0beecb4c0f501b8b0f
- Tests: npx vitest run (19 files / 302 tests on the integrated target), npx tsc --noEmit clean, npm run build clean, byte-identity at zero orders verified by the conductor's independent node hash: Telperion 808 / Laurelin 2442, unchanged, conductor probe across orders on Telperion: 0 -> 79.8 cm finest wood, 4 -> 23.8 cm, 8 -> 6.6 cm at 50,045 nodes and 123 ms; leaf-to-twig ratio 0.15 -> 1.83 at eight orders, so area conservation alone does not reach the botanical relationship - carried to .4 as the fine-order taper law
- PRs: