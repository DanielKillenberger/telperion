---
satisfies: [R1, R8]
---
# fn-6-branch-generations-below-the-crossover.4 Laterals along the limbs, the budget that survives them, and the prefix proof

## Description
Colonization nodes whose wood is under a stated radius bear laterals under the same law, from their own arrival direction and their own phase, so a node bears the same laterals whether it is currently a tip or a limb. The node budget is re-derived for a pass that no longer branches only from tips. R8's prefix test is written here, because it only means something once interior nodes bear wood.

**Size:** M
**Files:** `src/skeleton/twigs.ts`, `src/skeleton/twigs.test.ts`, `src/skeleton/grow.ts` (`defaultGrowth`, `twigHeadroom`, `GrowthReport`), `src/skeleton/grow.test.ts`, `src/presets/two-trees.ts` (the new `limbRadius` term), `src/presets/two-trees.test.ts`
**Touches:** [src/skeleton/twigs.ts, src/skeleton/twigs.test.ts, src/skeleton/grow.ts, src/skeleton/grow.test.ts, src/presets/two-trees.ts, src/presets/two-trees.test.ts, harness/skeleton-view.ts, harness/params.ts, src/skeleton/continuity.test.ts, harness/skeleton-view.test.ts]

### Approach
- Harness round-trip fixture (`harness/skeleton-view.test.ts`, the exact `mapped.twigs` object in "hands branch anatomy and the law over under the library's own names", ~159-169): `limbRadius` joins that enumerated object, so the panel cannot drop it; the assertion stays exact.
- Timing: the per-vertex socket test in `continuity.test.ts` builds Laurelin in full and sits on the 5 s default; give it the 60 s the other full-build tests carry. That is a test cost of the tree's size, not a regression, and it does not license weakening any assertion. The `limbRadius` rest value is chosen by measurement (record counts and build time at the candidates) and written into the preset comments and the spec's §Measured; 0.1 is the provisional starting point.
- Continuity suite (`src/skeleton/continuity.test.ts`, the radius-law test at ~141-144): a colonization tip now hands off a leader AND laterals. The suite distinguishes them: the leader's start radius equals the parent's radius, a lateral's equals `childRadius(parent, lengthRatio, ratioPower)`, both exact; the classification comes from the pass's records (a lateral is a branch whose base radius is below its parent's, or carry an explicit role if the records need one), never from suppressing tip laterals to keep the old assertion true. No tolerance widens.
- New param `limbRadius` (fraction of trunk radius, rail 0..1, rest measured): every colonization node past the root with `field.radius[i]` under it seeds a lateral whorl of `laterals` at the divergence, phase seeded per node from a deterministic function of the node index, frame from the node's arrival direction (`arrival` in `src/skeleton/colonize.ts:~316` shows the derivation). No leader for interior nodes. Colliding against the arrival direction is already the rule from task 2.
- Bare-trunk guard applies to interior origins (memory: `a-trunk-region-guard-on-the-parent-node`, test both endpoints).
- Budget: replace `twigHeadroom` (`src/skeleton/grow.ts:153-159`) with an estimate from the law: handoff count × nodes-per-branch summed over the derived generations, using task 1's `generationsUntilTwig` on the field's median handoff radius; keep `NODE_CEILING`. `GrowthReport` gains a `levelCapped` count distinct from `capped` (node ceiling), both reported.
- R8 prefix test: grow a preset's colonization; take a prefix at a round boundary (add a `rounds` read-out to `colonize`'s result, or reproduce the boundary by counting nodes per round in the test with the same config — pick the one that does not change colonization's output); run the pass on the prefix under the MATURE tree's field values for the nodes present; assert every node present in both bears byte-identical laterals, tips differing only by the leader continuation.
- Measure and record: nodes, tips, handoffs, level-capped handoffs on both presets at rest; both finish uncapped.

### Investigation targets
**Required:**
- `src/skeleton/twigs.ts` (as of task 2) — frontier seeding, collision rule
- `src/skeleton/colonize.ts:439-575` — the round loop; nodes are pushed inside it, so the array at a round boundary is a prefix
- `src/skeleton/grow.ts:140-160, 259-283, 315-345` — ceiling, headroom, report

**Optional:**
- `src/canopy/place.ts:221-224` — `shootRadius` as a fraction of trunk radius; the same idiom for `limbRadius`

### Key context
- The memory entry `zero-width-is-not-no-constraint` applies: below the crown base the envelope has no width, a lateral there is outside the silhouette.
- Build time is the constraint; if Laurelin's interior laterals push the build past what task 7 can measure interactively, the rest value of `limbRadius` is where the trade is made, and it is recorded.
## Acceptance
- [ ] Colonization nodes under `limbRadius` bear laterals from their own arrival direction and per-node phase; no lateral below the bare-trunk line
- [ ] The node budget derives from the law; both presets finish uncapped at rest; `GrowthReport` reports level-capped handoffs separately from the node ceiling
- [ ] R8: on a round-boundary prefix under the mature field, every node present in both bears byte-identical laterals; a field of the wrong length is reported
- [ ] Presets state `limbRadius`; structural-key test follows; harness round-trip compiles
- [ ] `npx vitest run src/skeleton src/presets harness` green; `npx tsc --noEmit` green
## Done summary
BLOCKED: SCOPE_EXCEEDED
Task: fn-6-branch-generations-below-the-crossover.4
Summary: The full suite times out in the out-of-scope shedding identity fixture at src/skeleton/shed.test.ts:54, and in the harness solid-build fixture at harness/skeleton-view.test.ts:252.
Impact: Task 4 and downstream tasks 5-7 remain pending.
Suggested resolution: Add src/skeleton/shed.test.ts to Touches and authorize either a linear identity lookup with the same exact assertions or a 60 s timeout for its full-preset identity test. Authorize the same full-build timeout treatment for harness/skeleton-view.test.ts:252. Redispatch with .flow/tmp/task4-attempt3-implementation.patch, which applies cleanly on the restored checkout.

The completed implementation and measurements are preserved in .flow/tmp/task4-attempt3-implementation.patch. The patch is NOT full-gate-passed. This attempt applied the previous patch, added limbRadius to the exact harness mapping fixture, applied the authorized 60 s socket-test timeout, measured and documented the rest value, and ran all gates. The full suite passed 341 tests and timed out in two tests. Per the user's stop-on-scope instruction, the worker stopped implementation immediately, preserved all eleven changed files in the patch, and reverted only those changes. The restored whole tree passes both canonical gates. The task remains in_progress with no commits.

The out-of-scope test is “keeps every colonization node as it was and re-indexes every parent below itself” at src/skeleton/shed.test.ts:54. It took 7,388 ms against 5,000 ms. Its assertions check exact colonization-node identity, parent-before-child ordering, and preservation of surviving-node order. For every surviving node it calls find and indexOf over the unshed array, giving the identity lookup quadratic cost. Telperion at the measured rest now has 55,240 nodes before shedding and 49,713 after it. The fixture must preserve all those assertions. No edit to the file was attempted.

The other timeout was “produces a solid the stage can draw, in clay” at harness/skeleton-view.test.ts:252, taking 6,245 ms against 5,000 ms in the full suite. The focused harness run had passed. Although this file is in Touches for the exact mapped.twigs fixture, this redispatch only expressly authorized a timeout change in continuity.test.ts. No harness timeout was changed. The log is .flow/tmp/task4-final-full.log. These are timeout failures; no structural assertion failure is claimed.

The preserved implementation extends the existing Shoot/frontier rather than adding another branching pass. Every eligible colonization node seeds laterals from its arrival direction and node-index phase, tips also carry a leader, and interior nodes do not. Both origin and candidate obey the bare-trunk guard. twigHeadroom counts potential handoffs, derives their generations with generationsUntilTwig, uses the median-radius topology estimate, and adds the upper-tail excess. The 250,000-node ceiling stays. GrowthReport.capped means the node stop and GrowthReport.levelCapped retains task 2's boolean shape for the generation stop. The preset gate checks both separately. The R8 round-prefix test and both endpoint guard tests are in the patch. Existing rail coverage now includes limbRadius's bounds and non-finite fallback. Continuity keeps exact leader and lateral radius-law assertions classified from the pass records.

The measurement chose limbRadius 0.1 for both presets. At 0.075 Laurelin has no colonization laterals because the threshold is below its tip radius. At 0.1 both trees gain laterals, every candidate finishes with zero level-capped handoffs, and both the direct pass and growReport finish without hitting the node ceiling.

| Preset | limbRadius | Handoffs before / after shedding | Nodes before / after shedding | Median full CPU build ms |
|---|---|---|---|---|
| Telperion | 0.075 | 513 / 485 | 45,277 / 42,006 | 720 |
| Telperion | 0.1 | 654 / 604 | 55,240 / 49,713 | 798 |
| Telperion | 0.15 | 755 / 691 | 64,331 / 56,378 | 896 |
| Laurelin | 0.075 | 371 / 362 | 46,517 / 43,199 | 598 |
| Laurelin | 0.1 | 1,398 / 1,336 | 111,919 / 104,335 | 1,432 |
| Laurelin | 0.15 | 1,975 / 1,890 | 159,314 / 149,826 | 2,079 |

Colonization stays at 808 nodes / 133 tips for Telperion and 2,442 nodes / 376 tips for Laurelin. Full build timing uses buildPreset.stats.buildMs with foliage, one warm build per preset and three serial builds per candidate. It includes geometry, vertex normals, canopy and Three.js object construction. The 0.1 choice saves 31% of Laurelin's 0.15 CPU time. GPU timing, shell fill thresholds and clay approval remain downstream work. Preset comments and a new spec Measured section carry the method, ranges and counts in the preserved patch. The temporary measurement test/config and raw log remain in .flow/tmp; apply the patch before reproducing them.

Investigation read worker.md, the flow-next-work skill, the amended anchor, task 2's done summary, the prior attempt's handover, the required twigs/grow/colonization files and both trunk-guard memory entries. Similar-code search selected extending the frontier and reusing the law helpers. The prior attempt's recorded red test evidence remains .flow/tmp/task4-retry-red.log. No new red run is claimed here. The baseline full suite ran because the receipt check refused reuse on dirty STRATEGY.md.

Validation on the implementation passed the 61-test focused skeleton suite, the 139-test preset/radius/harness suite, the six-candidate measurement and TypeScript. The full suite failed only the two timeouts described above. After reverting the worker's changes, npx tsc --noEmit passed with exit 0 and npx vitest run passed with exit 0, 338 tests in 23 files, 15.17 seconds. Evidence JSON names every log and distinguishes implementation from restored-tree results. No green full-suite receipt was written. The TypeScript receipt attempt returned NO_RECEIPT because STRATEGY.md is dirty. No review was requested (REVIEW_MODE=none).

Workspace: /home/daniel/Projects/telperion
Base commit: 019234b554a1b387e23b05dd3c7e2626f428f907
Commit range: 019234b554a1b387e23b05dd3c7e2626f428f907..HEAD (empty).
Intended conductor commit subject: feat(skeleton): laterals along the limbs, the budget from the law, and the prefix proof

No staging, commit, branch change or flowctl done was attempted, following the sandbox-specific user instruction. STRATEGY.md, the conductor's task metadata, .flow/prospects/ and .worktrees/ were left untouched. The spec Measured edit was explicitly requested by the redispatch and is preserved in the patch alongside the ten files from Touches.

Exact files edited or created by this worker (the first eleven were subsequently restored; their edits survive in the patch):
- .flow/specs/fn-6-branch-generations-below-the-crossover.md (restored)
- harness/params.ts (restored)
- harness/skeleton-view.ts (restored)
- harness/skeleton-view.test.ts (restored)
- src/presets/two-trees.ts (restored)
- src/presets/two-trees.test.ts (restored)
- src/skeleton/continuity.test.ts (restored)
- src/skeleton/grow.ts (restored)
- src/skeleton/grow.test.ts (restored)
- src/skeleton/twigs.ts (restored)
- src/skeleton/twigs.test.ts (restored)
- .flow/tmp/base_commit
- .flow/tmp/task4-final-baseline.log
- .flow/tmp/task4-final-focused.log
- .flow/tmp/task4-measure.config.ts
- .flow/tmp/task4-measure.test.ts
- .flow/tmp/task4-measure.log
- .flow/tmp/task4-final-presets.log
- .flow/tmp/task4-final-types.log
- .flow/tmp/task4-final-full.log
- .flow/tmp/task4-attempt3-implementation.patch
- .flow/tmp/task4-attempt3-restored-types.log
- .flow/tmp/task4-attempt3-restored-full.log
- /tmp/claude-1000/-home-daniel-Projects-telperion/447cc600-c1ea-45c4-85f4-5f3967b0fd7a/scratchpad/handover-4-summary.md
- /tmp/claude-1000/-home-daniel-Projects-telperion/447cc600-c1ea-45c4-85f4-5f3967b0fd7a/scratchpad/handover-4-evidence.json

conductor: the BLOCKED lines above are the worker's history; the conductor applied the preserved attempt-3 patch, widened two full-build test timeouts to 60 s, verified typecheck and 343 tests, and committed as c5e9121 (Codex sandbox mounts .git read-only). Worker model gpt-6-astra at low; three dispatches, two ended on scope amendments.
stage: plan-sync - skipped(config: planSync.enabled != true)
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: c5e9121
- Tests: baseline: green via handoff (019234b, npx tsc --noEmit and npx vitest run, 338 tests), gate check --gate unittest --command npx vitest run: RUN, dirty STRATEGY.md; no receipt reused, npx vitest run - baseline PASS exit 0, 338 tests in 23 files, 14.72s; .flow/tmp/task4-final-baseline.log, Inherited red proof from attempt 2: missing lateral origins and empty R8 shared-lateral selection; .flow/tmp/task4-retry-red.log; not rerun in this attempt, npx vitest run src/skeleton/twigs.test.ts src/skeleton/continuity.test.ts src/skeleton/grow.test.ts - PASS exit 0, 61 tests, 21.77s; .flow/tmp/task4-final-focused.log, npx vitest run --config .flow/tmp/task4-measure.config.ts --pool=threads --reporter=verbose --silent=false - PASS exit 0, 1 measurement test, 24.43s; .flow/tmp/task4-measure.log, npx vitest run src/presets src/radius.test.ts harness - PASS exit 0, 139 tests, 33.00s; .flow/tmp/task4-final-presets.log, npx tsc --noEmit - implementation PASS exit 0; .flow/tmp/task4-final-types.log, gate classify --base 019234b554a1b387e23b05dd3c7e2626f428f907: FULL (unmatched .worktrees/.gitignore), gate receipt --gate typecheck --command npx tsc --noEmit: NO_RECEIPT, dirty STRATEGY.md, npx vitest run - implementation FAIL exit 1, 341 passed and 2 timed out, 36.61s; .flow/tmp/task4-final-full.log; shed.test.ts:54 at 7388ms and harness/skeleton-view.test.ts:252 at 6245ms, each against 5000ms, git apply --check .flow/tmp/task4-attempt3-implementation.patch - PASS after reverting only worker changes, npx tsc --noEmit - restored tree PASS exit 0; .flow/tmp/task4-attempt3-restored-types.log, npx vitest run - restored tree PASS exit 0, 338 tests in 23 files, 15.17s; .flow/tmp/task4-attempt3-restored-full.log, git diff --check - PASS, flowctl show fn-6-branch-generations-below-the-crossover.4 --json - verified in_progress, conductor: applied .flow/tmp/task4-attempt3-implementation.patch, widened shed.test.ts:54 and skeleton-view.test.ts:252 to 60 s, then npx tsc --noEmit + npx vitest run (343 passed, 23 files)
- PRs: