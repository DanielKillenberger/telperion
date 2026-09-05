---
satisfies: [R1, R2, R4]
---
# fn-6-branch-generations-below-the-crossover.1 The branch law, stated and measured before anything grows

## Description
A pure module that states the law the pass will grow under, and a measurement of what that law would do to both presets' colonization skeletons as they stand. Nothing in the growth pipeline changes in this task. It exists so the recursion-by-radius design is proven on the shipped trees before the pass is rewritten (spec §Early proof point), and so the biological review R1 asks for is recorded once, with sources, where every later task reads it.

**Size:** M
**Files:** `src/skeleton/law.ts` (new), `src/skeleton/law.test.ts` (new), `src/skeleton/fill.ts` (new: the two R4 metrics), `src/skeleton/fill.test.ts` (new), `.flow/specs/fn-6-branch-generations-below-the-crossover.md` (§Measured: the review and the numbers)
**Touches:** [src/skeleton/law.ts, src/skeleton/law.test.ts, src/skeleton/fill.ts, src/skeleton/fill.test.ts, .flow/specs/fn-6-branch-generations-below-the-crossover.md]

### Approach
- Contracts to state (signatures, not bodies): the fixed twig anatomy as a `TwigAnatomy` value with diameter, internode length and stations per internode at botanical defaults with sources; `branchLength(radius, params)` by elastic similarity, length proportional to radius to the two thirds, anchored by a stated constant so the shipped handoff wood (median 79 cm on Telperion, ~116 cm on Laurelin) carries the length its diameter implies; `childRadius(parentRadius, lengthRatio, ratioPower)` as Weber & Penn's radius-from-length-ratio rule; `generationsUntilTwig(radius, params)` by repeated application until the twig's radius, capped at `MAX_TWIG_LEVELS` (`src/skeleton/twigs.ts:187`) and reporting whether the cap bound.
- Resting values and sources go in the module header in the house style (`src/skeleton/twigs.ts:48-102` is the template: one named term per rule, source cited inline). Weber & Penn 1995 §4.3 and the p.126 table (Ratio/RatioPower 0.015/1.2 Aspen, 0.015/1.3 Tupelo; nLength 0.6/0.4 fine levels), Niklas & Spatz 2004 (elastic similarity 2/3 near the trunk, flow similarity toward the twig), McMahon 1975, Shinozaki 1964 pipe model, the PMC3979699 daughter/mother area ratio 1.04-1.3, Corner's rules. Note the Niklas & Spatz finding that one exponent will not fit both ends; state the choice and its rail.
- Rails via the file-local `held`/`pinned` idiom (`src/skeleton/twigs.ts:209-213`), never imported.
- Measurement: a test that grows both presets at zero orders with `bare()` (see `src/skeleton/grow.test.ts:90`), solves radii (`solveRadii`, `src/radius.ts:225`), and for every colonization tip reports branch length, generations until twig, and the summed node estimate under the law; asserts the spec's three proof-point conditions and writes the table into §Measured of the spec by hand from the test's output.
- Fill metrics (contracts), written here so the tuft can be measured before task 2 removes the dial that produces it: `shellOccupancy(skeleton, terminal: Uint8Array, envelope, cellSize)` voxelises the crown's shell at a cell size stated as a fraction of height and returns the fraction of shell cells holding a terminal node; `tipClustering(skeleton, crossover, terminal, distance)` returns the fraction of terminal nodes within `distance` of a colonization tip. Shell membership uses shedding's own conversion and predicate (`src/skeleton/shed.ts:66-120` converts `shellDepth` from a fraction of the envelope's maximum radius to metres before comparing with `distanceToProfile`), never the raw fraction. `terminal` is a caller-supplied classification: for the baseline it is the pass's leaf nodes (appended nodes with no children); after task 2 it is the twig mark. Empty terminal set → the metric reports untested, never 0 or NaN silently.
- Tuft baseline: run both presets as shipped today (`levels: 8`) and at zero orders, record both metrics for each, and write them into §Measured as the baseline R4 is judged against.
- Record the biological review in §Measured as a short list: each rule, its resting value, its source, and the one line on why it is right for a tree this size.

### Investigation targets
**Required** (read before coding):
- `src/skeleton/twigs.ts:11-108` — the current rule doc and rail style to mirror
- `src/radius.ts:57-143` — the fine-order law and `twigTaper` this law replaces; `225-371` for `solveRadii`
- `src/skeleton/grow.test.ts:85-100` — `bare()` and `signature()` fixtures
- `.flow/specs/fn-6-branch-generations-below-the-crossover.md` §Measured — the table format to extend

**Optional:**
- `src/canopy/element.ts:124` — `DEFAULT_ELEMENT` leaf length the twig is measured against

### Key context
- Do not touch `branchTwigs` or any preset in this task; the point is a law measured on unchanged trees.
- Apical dominance: the leader keeps most of the cross-section, so generations along the leader path exceed a balanced-fork estimate. The measurement must follow lateral radii, not a log of the area ratio.
## Acceptance
- [ ] `src/skeleton/law.ts` exports the twig anatomy and the three law functions with resting values, rails and sources in the header
- [ ] Every rule's source is cited; the review list in the spec's §Measured names each rule, its value, its source
- [ ] Measured on both presets at zero orders: median branching carried by a handoff is on the order of 20 m for Telperion's 79 cm wood, every handoff reaches twig radius within the level cap, and the summed node estimate is under `NODE_CEILING` (250,000) on both
- [ ] `src/skeleton/fill.ts` exports `shellOccupancy` and `tipClustering` in shedding's shell units, tested on synthetic skeletons with known answers and reporting untested on an empty terminal set
- [ ] The tuft baseline (both presets at eight orders and at zero orders) is recorded in §Measured
- [ ] The numbers are written into the spec's §Measured
- [ ] `npx vitest run src/skeleton/law.test.ts` and `npx tsc --noEmit` pass; the full suite is unchanged
## Done summary
Implemented the pure branch law and fixed twig anatomy, the two crown-fill metrics, synthetic checks, and measurements on the unchanged Telperion and Laurelin presets. The spec's Measured section records the biological sources and their caveats, the per-handoff proof, its explicit tip-only topology, and the four tuft baselines.

Task: fn-6-branch-generations-below-the-crossover.1
Workspace: /home/daniel/Projects/telperion
baseline: green (all four canonical Quick commands passed before implementation on redispatch)
stage: impl-review - skipped(config: REVIEW_MODE=none)

### Implementation and scope

- src/skeleton/law.ts exports TwigAnatomy, DEFAULT_TWIG_ANATOMY, BranchLawParams, DEFAULT_BRANCH_LAW, branchLength, childRadius and generationsUntilTwig. It states numerical rails, the 12-level cap semantics and the sources. The header explicitly limits the 2/3 exponent to large diameters, identifies 1.04 as a model example, and identifies the twig dimensions as selected anatomy.
- src/skeleton/law.test.ts checks the allometric calibration, radius rule, fixed anatomy, exact stopping boundary, cap reporting, non-finite fallbacks and rails. Its two preset censuses assert every handoff reaches twig radius within the cap and the explicit topology stays under the 250,000-node ceiling. Telperion has 133 handoffs at 20.996890 m first branch length, five lateral reductions, and 21,556 estimated total nodes. Laurelin has 376 at 27.124593 m, five reductions, and 61,098 estimated nodes.
- The estimate assumes three internodes per leader, two lateral subtrees and one terminal twig per branch. It is a tip-only model before shedding or collisions. Task 2 must remeasure if it chooses a different topology; task 4 still owns the budget for laterals along colonization limbs. This does not assert that the future pass has already been built or its full budget proven.
- src/skeleton/fill.ts accepts caller-supplied terminal marks. Shell occupancy uses world-origin voxels, finite crown containment and shedding's own shell-unit conversion and predicate. Clustering classifies reference tips only within the colonization prefix. Empty or invalid measurements explicitly report untested.
- src/skeleton/fill.test.ts checks known synthetic fractions, duplicate occupancy, scale-independent voxel resolution, inside/outside shell membership, the inclusive clustering boundary, prefix-only tip classification and untested cases. It measures both presets at zero and eight orders.
- .flow/specs/fn-6-branch-generations-below-the-crossover.md records both tables and sources in Measured, including correcting the old handoff-wood label from radius to diameter. The R4 baseline uses cell edge 0.02H and clustering distance 0.05H. Eight-order shell occupancies are 193/6352 and 1747/74072; clustering fractions are 9860/18280 and 38180/74982. Zero orders is untested because it has no appended terminals.
- No implementation edit falls outside the five declared Touches. No preset or production growth pipeline changed. Pipeline error handling under R1/R4 belongs to the later pass/shedding tasks; this task supplies the pure law, measurement contracts and their errors.

### Validation

Pre-edit baseline passed the two focused Quick commands (57 and 139 tests), npx tsc --noEmit, and npx vitest run (327 tests). Logs are /tmp/fn6-retry-baseline-{skeleton,presets,types,full}.log.

The introduced tests first failed because the law and fill modules were absent (/tmp/fn6-law-fill-red.log). The first implementation run exposed a hand-calculated expected length error in the new test; the expected value was corrected to 20.9816014211964 m for radius 0.395 m without widening its tolerance. The final focused law/fill run passed all 11 tests (/tmp/fn6-law-fill-final.log).

Final canonical checks passed:
- npx vitest run src/skeleton/twigs.test.ts src/skeleton/continuity.test.ts src/skeleton/grow.test.ts (57 tests; /tmp/fn6-retry-verify-skeleton.log).
- npx vitest run src/presets src/radius.test.ts harness (139 tests; /tmp/fn6-retry-verify-presets.log).
- npx tsc --noEmit (/tmp/fn6-retry-verify-types.log).
- npx vitest run (338 tests in 23 files; /tmp/fn6-retry-verify-full-retry.log).

The first final full-suite attempt timed out two existing five-second tests, src/canopy/place.test.ts:371 and src/presets/two-trees.test.ts:95, with 336 passing (/tmp/fn6-retry-verify-full.log). The unchanged canonical command passed on retry in 42.37 s. No timeout, test assertion or runner setting was changed to obtain that pass. The first run took 68.81 s; the host reported load average 30.01 on 32 CPUs when inspected afterward. Timing remains an observed tooling concern for the conductor.

The default fork pool suppressed worker stdout in this environment, including direct stdout writes. The measurement-only command npx vitest run src/skeleton/law.test.ts src/skeleton/fill.test.ts --pool=threads --reporter=verbose --silent=false exposed all 509 per-handoff rows and all four fill records (/tmp/fn6-law-fill-measure-threads.log). The spec gives this reproduction command. Canonical gates used the unchanged commands.

Gate classification returned FULL. Typecheck and unittest receipt attempts returned NO_RECEIPT because the worktree is dirty. No green receipt was written or reused, and no gate was skipped. Whitespace and measurement-row arithmetic checks passed.

### Sandbox restriction and conductor action

Git staging failed with exit 128: fatal: Unable to create '/home/daniel/Projects/telperion/.git/index.lock': Read-only file system. The attempted command began with git add -A and would have committed the task with the required co-author trailer. No implementation commit exists; all five files remain ready to stage. The standard worker sandbox-blocked-commit exception permits flowctl done after passing validation, with this restriction recorded. The conductor must commit the implementation and task receipt; this worker cannot stage either and will not repeat the denied staging operation.

Intended implementation subject: feat(skeleton): state and measure the branch law and crown fill
Task trailer: Task: fn-6-branch-generations-below-the-crossover.1
Required final trailer: Co-Authored-By: Codex gpt-6-astra <noreply@openai.com>
## Evidence
- Commits:
- Tests: baseline: green; 57 skeleton tests, 139 preset/radius/harness tests, typecheck, and 327 full-suite tests passed pre-edit, red introduction: npx vitest run src/skeleton/law.test.ts src/skeleton/fill.test.ts failed because both new modules were absent; /tmp/fn6-law-fill-red.log, npx vitest run src/skeleton/law.test.ts src/skeleton/fill.test.ts (11 passed; /tmp/fn6-law-fill-final.log), npx vitest run src/skeleton/law.test.ts src/skeleton/fill.test.ts --pool=threads --reporter=verbose --silent=false (11 passed; measurements in /tmp/fn6-law-fill-measure-threads.log), npx vitest run src/skeleton/twigs.test.ts src/skeleton/continuity.test.ts src/skeleton/grow.test.ts (57 passed; /tmp/fn6-retry-verify-skeleton.log), npx vitest run src/presets src/radius.test.ts harness (139 passed; /tmp/fn6-retry-verify-presets.log), npx tsc --noEmit (passed; /tmp/fn6-retry-verify-types.log), npx vitest run (first verify attempt failed two existing 5 s timeouts, 336 passed; /tmp/fn6-retry-verify-full.log), npx vitest run (unchanged retry passed all 338 tests in 23 files; /tmp/fn6-retry-verify-full-retry.log), gate classify: FULL; typecheck and unittest receipt creation refused because sandbox-blocked changes remain uncommitted, git diff --check and five-file whitespace check passed
- PRs: