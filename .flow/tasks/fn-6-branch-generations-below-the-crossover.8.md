---
satisfies: [R1, R2, R7]
---
# fn-6-branch-generations-below-the-crossover.8 The twig has a length, and the branch its own resolution

## Description
The owner's R7 pass on task 6's build returned "not yet" with two findings, and both trace to the pass's anatomy rather than to a preset value. First, a twig is emitted as a single 20 mm internode and placement puts one leaf per internode, so leaves equal twigs exactly (19,744 on Telperion, 40,781 on Laurelin), below the tuft's 137,000 and far below the botanical range; the twig anatomy has a diameter and an internode but no length, so the pass cannot grow a shoot. Second, a branch is cut into three internodes whatever its length, so Telperion's first generation leaves its limbs as 3.9 m straight rods with the bias field sampled three times along 21 m; raising `internodes` explodes the tree because laterals are counted per internode (6 internodes puts both presets at the 250,000 ceiling). This task gives the twig a length, and separates a branch's geometric resolution from how many laterals it bears.

**Size:** M
**Files:** `src/skeleton/law.ts` (twig length in the anatomy; an internode-length law), `src/skeleton/twigs.ts` (the twig grown as a shoot; laterals per branch spread along its internodes; internode count from the law), `src/skeleton/twigs.test.ts`, `src/skeleton/grow.ts` (budget follows the new topology), `src/skeleton/grow.test.ts`, `src/canopy/place.ts` and `src/canopy/place.test.ts` (stations along the whole twig), `src/presets/two-trees.ts` and `src/presets/two-trees.test.ts` (the new terms stated in full; rest values by measurement), `harness/params.ts`, `harness/skeleton-view.ts`, `harness/skeleton-view.test.ts`, `harness/GrowerDev.tsx` (dials and read-outs follow), `src/skeleton/continuity.test.ts`, `src/skeleton/fill.test.ts`, `src/skeleton/law.test.ts` (fixtures follow), `.flow/specs/fn-6-branch-generations-below-the-crossover.md` (§Measured)
**Touches:** [src/skeleton/law.ts, src/skeleton/law.test.ts, src/skeleton/twigs.ts, src/skeleton/twigs.test.ts, src/skeleton/grow.ts, src/skeleton/grow.test.ts, src/canopy/place.ts, src/canopy/place.test.ts, src/presets/two-trees.ts, src/presets/two-trees.test.ts, harness/params.ts, harness/skeleton-view.ts, harness/skeleton-view.test.ts, harness/GrowerDev.tsx, harness/params.test.ts, src/skeleton/continuity.test.ts, src/skeleton/fill.test.ts, src/radius.ts, src/radius.test.ts, .flow/specs/fn-6-branch-generations-below-the-crossover.md]

### Approach
- **The twig has a length.** `TwigAnatomy` gains `length` (metres, rest about 0.25 m, sourced as a current-year broadleaf shoot; the same selected-anatomy caveat as the other terms), and the pass grows a twig as a shoot of `round(length / internodeLength)` internodes at the twig's fixed radius, one node per internode, so the canopy's station walk (already per internode) puts a leaf at every station. Expect leaves to multiply by about twelve: on the order of 240,000 and 490,000. Record the counts against fn-5's R5 range (10^5 to 10^7). The twig's internodes are still twig-marked and carry the fixed radius at both ends.
- **Laterals per branch, not per internode.** `laterals` becomes the count of side branches a branch bears along its length (rest measured; 2 is the starting point, matching the topology task 1 estimated), placed at evenly spaced internode stations with the phyllotactic rotation, never at the branch's first internode. The pass's node estimate in `grow.ts` follows: N = internodes + laterals × N_child + twig internodes.
- **Internode length follows the wood.** Replace the fixed `internodes` count with an internode-length law in `law.ts`: `internodeLength(radius) = internodeFactor × 2 × radius`, floored at the twig's internode and capped so a branch has at most 32 internodes; `internodeFactor` (a multiple of the branch's diameter, rest measured; 2.5 is the starting point, giving about 2 m on 79 cm wood) is a stated dial. A branch's internode count is `max(1, round(branchLength / internodeLength))`. The bias field and turn limit are consulted per internode as before, so a finer internode is a smoother branch.
- **Rest values by measurement, against the ceiling and the build.** Sweep `internodeFactor` (say 1.5, 2.5, 4) and `laterals` (2, 3) on both presets: nodes before and after shedding, twigs, leaves, capped flags, growth time and the harness build time. Choose the rest that keeps both presets uncapped under the 250,000 ceiling with a build the dial can still drag (task 4's 0.8 s and 1.4 s are the reference), and record the whole table in §Measured with the reason. If no value satisfies both, report it rather than raising the ceiling silently; raising the ceiling is a decision for the spec.
- **Re-measure R4 and the continuity suite** on the chosen rest; thresholds from task 5 stay unless the metric moves, in which case record the new values and why.
- Harness: the `internodes` dial becomes `internodeFactor`; `laterals` keeps its name with the new meaning and comment; the read-out gains leaves placed.
- Fixture migrations under the standing authorization: any test asserting the old `internodes` semantics or one-leaf-per-twig is rewritten to the new intent; never a widened budget.

### Investigation targets
**Required:**
- `src/skeleton/twigs.ts` — `branchTwigs`: the `Shoot` frontier, `completed`/`internodes` bookkeeping (~190-230), the twig emission at `isTwig`, the lateral placement per internode
- `src/skeleton/law.ts` — `TwigAnatomy`, `DEFAULT_BRANCH_LAW`, `generationsUntilTwig`
- `src/canopy/place.ts:296-372` — the anatomy station walk (already per internode; the twig just needs internodes)
- `src/skeleton/grow.ts` — the budget estimate task 4 wrote
- `.flow/tasks/fn-6-branch-generations-below-the-crossover.6.md` — the R7 findings as recorded

**Optional:**
- `.flow/specs/fn-6-branch-generations-below-the-crossover.md` §Measured, the task 4 and task 5 tables — the numbers to extend

### Key context
- The owner's words: "for telperion at original size the resolution of the growth doesn't seem fine enough. Laurelin seems okish. In general there's not even close to enough leaves on any of them."
- Measured before this task on the shipped presets: internodes 3 → 6 takes Telperion from 49,713 to 214,658 nodes (capped) and Laurelin from 104,335 to 232,127 (capped), with mean first-generation internodes of 3.9 m and 5.1 m. That is the coupling this task removes.
- R7 is judged again on this task's build; the conductor and owner take it. Leave that box unchecked and say so.

## Acceptance
- [ ] `TwigAnatomy` has a `length`; a twig is grown as a shoot of internodes at the fixed radius and the canopy places a leaf at every station; leaf counts on both presets recorded and inside 10^5 to 10^7
- [ ] `laterals` counts side branches per branch, spread along its internodes; `internodes` is replaced by an internode-length law with a stated `internodeFactor` dial; the node estimate follows
- [ ] Rest values chosen by the recorded sweep: both presets uncapped under the ceiling, build time recorded against task 4's reference, the table and the reason in §Measured
- [ ] Telperion's mean first-generation internode is under 2.5 m at rest, or the measured reason it cannot be is recorded
- [ ] R4 metrics and the continuity suite re-measured on the chosen rest; any threshold change recorded with its reason
- [ ] Harness dials and read-outs follow; round-trip and slider-range tests exact; fixtures migrated with intent preserved and listed
- [ ] `npx tsc --noEmit` and `npx vitest run` green on the whole tree
- [ ] R7 judged again by the owner on this build (conductor's box; leave unchecked)


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
