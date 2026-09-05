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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
