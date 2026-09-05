---
satisfies: [R3, R7]
---
# fn-6-branch-generations-below-the-crossover.6 The harness and the two trees: the dial retired, the read-out, no depth stated

## Description
The panel stops offering a depth and starts reporting one: the orders slider goes, a read-out shows the derived generation range and how many handoffs hit the cap, the ceiling notice stops telling the owner to lower the orders, and both presets are stated without a depth so the owner can judge R7 in clay. Frontend task: no DESIGN.md exists, follow the panel's own conventions.

**Size:** M
**Files:** `harness/params.ts`, `harness/skeleton-view.ts`, `harness/skeleton-view.test.ts`, `harness/GrowerDev.tsx`, `harness/params.test.ts`, `src/presets/two-trees.ts` (comments and any rest values the clay pass settles)
**Touches:** [.flow/specs/fn-6-branch-generations-below-the-crossover.md, harness/params.ts, harness/skeleton-view.ts, harness/skeleton-view.test.ts, harness/GrowerDev.tsx, harness/params.test.ts, src/presets/two-trees.ts]

### Approach
- Remove the `twigLevels` slider (`harness/params.ts:271`) and its block comment (`:258-270`); add dials for the new terms with rails and measured-comment justification in the same style (`lengthRatio`, `ratioPower`, `internodes`, `laterals`, `limbRadius`; the twig anatomy is a preset term with no dial unless the clay pass wants one).
- `TreeStats` (`harness/skeleton-view.ts:196`) gains generations min/median/max, handoffs, level-capped handoffs, twig count; `GrowerDev.tsx` renders them beside nodes/triangles; the capped notice at `GrowerDev.tsx:472` is reworded to the dials that now exist (step, `limbRadius`).
- Round-trip tests (`harness/skeleton-view.test.ts:549-568`) follow the new param set term for term; slider-range test follows.
- Presets: comments on the twigs blocks rewritten to the new model (no R8 depth judgement; the read-out is the number); the R7 clay pass with the owner happens on this task's build, after task 5 has put the foliage on the twig, and any rest value the owner moves is recorded in the preset comment with the reason and re-measured against task 5's R4 thresholds before it ships.
- Build-time honesty: keep the settle-after-last-notch mechanism; the build timer already in `TreeStats.buildMs` is shown.

### Investigation targets
**Required:**
- `harness/params.ts:100-140, 223-301, 360-372` — field docs, sliders, defaults
- `harness/skeleton-view.ts:66-130, 190-215, 273-318, 377-470` — mappings, stats, build
- `harness/GrowerDev.tsx:440-480` — stats and notices rendering

**Optional:**
- `harness/skeleton-view.test.ts:540-600, 705-730` — the tests that build both presets in full (60 s timeouts)

### Key context
- Memory `a-second-subject-on-stage-makes-every`: every "what is on screen" query must handle the comparison view with two presets.
- R7 is a manual gate: this task ends with the owner's verdict recorded in the spec, not with a test.
## Acceptance
- [ ] No orders slider; a read-out shows generation range, handoffs and level-capped handoffs; ceiling notice names existing dials
- [ ] New dials with rails and measured comments; round-trip and slider-range tests green
- [ ] Both presets state the twig and the ratios, no depth; comments rewritten
- [ ] Any preset value moved by the verdict still holds task 5's R4 thresholds
- [ ] Owner's R7 verdict recorded in the spec (fills the volume; reads as its size; Telperion and Laurelin read as themselves)
- [ ] `npx vitest run harness src/presets` green; `npx tsc --noEmit` green; `npm run dev` builds both presets
## Done summary
The panel now offers branch-law controls and reports derived generations, surviving handoffs, law-capped handoffs and twig marks for one tree or the comparison. Implementation and both full-tree gates are green. The tree is uncommitted and fn-6-branch-generations-below-the-crossover.6 remains in_progress for the conductor, as requested.

The orders slider, twigLevels state and orphaned twig-thinning comment are removed. The panel names lengthRatio, ratioPower, internodes and laterals directly, eliminating the children-minus-one conversion. Those dials and limbRadius use the library rails, with integer counts and measured comments. shootRadius, spacing, clump and clumpSpan remain exact preset values but have no sliders because marked twig anatomy supplies foliage stations. All four argument objects still round-trip term for term. The ceiling notice names growth step and limbRadius. The build timer and settle-after-last-notch mechanism remain intact.

TreeStats reads surviving appended edges whose parents belong to colonization. It reuses generationsUntilTwig on each recorded base radius, including the lateral reduction and fixed-twig clamp. The read-out counts required lateral reductions and excludes the terminal twig. Telperion and Laurelin each report 4 / 4 / 5 generations, with 604 / 1,336 surviving handoffs, 19,744 / 40,781 twig marks and zero law-capped handoffs. Comparison sums counts and pools the generation histogram before taking the median, giving 1,940 handoffs and 60,525 twigs.

The level-capped handoff count is a radius-law prediction over surviving handoffs. GrowthReport exposes actual generation stops only as a boolean and carries no per-handoff stop provenance. The panel therefore labels the count "level-capped by radius law" and separately displays the actual levelCapped warning, which survives shedding. Collisions, short runs and shedding can reduce visible depth below the law prediction. This distinction is documented in the code and spec; no actual per-handoff stop count is fabricated.

Both preset twig comments now state the shared 5 mm diameter, 20 mm internode, one station, three branch internodes, one lateral, length ratio 0.4 and radius power 1.3. No preset rest value changed. Task 5's R4 assertions passed in the final full suite without changes. Twelve growReport measurements varied lengthRatio, ratioPower, internodes and laterals independently on both presets. All completed without node or generation caps. The spec table and slider comments carry the counts; the temporary measurement source, config and raw log are preserved.

R7 remains unchecked. The worker did not run Vite or Chromium under the supplied sandbox restriction and claims no browser, GPU or owner visual verdict. The conductor and owner run the clay pass on this build. If that pass changes a preset rest value, record the reason and re-measure R4 before shipping.

Tests cover exact law rails and retired controls; known-radius handoffs with an even median, downstream exclusion, zero handoffs and nonconvergent law capping; exact preset round trips; preset values inside slider ranges; and comparison counts and pooled median. The new controls and stats tests were observed red before implementation. Final npx tsc --noEmit passed with exit 0. Final npx vitest run passed with exit 0, 347 tests in 23 files, 26.62 seconds. No tolerance or budget was weakened.

Fixture migrations under the supplied authorization affect only harness/skeleton-view.test.ts. The branch-law mapping fixture changes twigChildren 3 / 1 to laterals 2 / 0, preserving exactly the library inputs and the greater-than-2x node-count assertion. Its ignored twigLevels input is removed because that state no longer exists. The expected law object follows the renamed fields with the same values. Existing round-trip and slider-range assertions are unchanged. The comparison test gains exact handoff and twig counts and pooled statistics assertions. No timeout migrations were needed, and no other test file required a fixture migration.

Investigation followed worker.md, the anchor, task 2/4/5 done summaries, the flow-next-work skill, required panel files, law and pass records, and the comparison-view memory. Similar-code search selected extending existing mappings and build/comparison statistics, reusing generationsUntilTwig and resolveTwigs. No subagent or review dispatch ran.

Baseline was green via the supplied handoff. The receipt check refused reuse because STRATEGY.md was already dirty, so the pre-edit full suite also ran and passed 345 tests. Final gate classify requested full verification. Both receipt writes returned NO_RECEIPT for the same pre-existing STRATEGY.md change. No verification gate was skipped. All test commands, exit observations and log paths are in the evidence JSON.

Workspace: /home/daniel/Projects/telperion
Base commit: ffa448a6f8289e955b1577a11c22acfef30d37be
Commit range: ffa448a6f8289e955b1577a11c22acfef30d37be..HEAD (empty)
Intended conductor commit subject: feat(harness): the orders dial retired for a read-out, and the two trees stated without a depth
Task trailer: Task: fn-6-branch-generations-below-the-crossover.6

The user explicitly assigned committing and completion to the conductor because .git is read-only. No git add, commit, flowctl done or branch operation was attempted. STRATEGY.md, .flow/prospects/, .worktrees/ and task lifecycle records retain their pre-existing state. flowctl show confirmed in_progress. The conductor can commit the seven scoped implementation/spec/test files and complete the lifecycle after its required owner gate.

stage: impl-review - skipped(config: REVIEW_MODE=none)

Exact files edited or created by this worker:
- .flow/specs/fn-6-branch-generations-below-the-crossover.md
- harness/params.ts
- harness/skeleton-view.ts
- harness/skeleton-view.test.ts
- harness/GrowerDev.tsx
- harness/params.test.ts
- src/presets/two-trees.ts
- .flow/tmp/base_commit
- .flow/tmp/task6-baseline.log
- .flow/tmp/task6-red.log
- .flow/tmp/task6-stats-red.log
- .flow/tmp/task6-focused.log
- .flow/tmp/task6-types-loop.log
- .flow/tmp/task6-measure.config.ts
- .flow/tmp/task6-measure.test.ts
- .flow/tmp/task6-measure.log
- .flow/tmp/task6-types-final.log
- .flow/tmp/task6-full-final.log
- /tmp/claude-1000/-home-daniel-Projects-telperion/447cc600-c1ea-45c4-85f4-5f3967b0fd7a/scratchpad/handover-6-summary.md
- /tmp/claude-1000/-home-daniel-Projects-telperion/447cc600-c1ea-45c4-85f4-5f3967b0fd7a/scratchpad/handover-6-evidence.json

conductor: verified typecheck and 347 tests and committed as 6dd6081 (Codex sandbox mounts .git read-only); worker model gpt-6-astra at low; one dispatch.

R7, taken by the owner in the harness on this build: NOT YET. "for telperion at original size the resolution of the growth doesn't seem fine enough. Laurelin seems okish. In general there's not even close to enough leaves on any of them." Diagnosed by the conductor: leaves equal twigs (one 20 mm internode, one leaf) at 19,744 / 40,781; first-generation internodes 3.9 m / 5.1 m with laterals coupled to internodes. Routed to task 8, which re-takes R7. The verdict is recorded here and in the spec's §Measured; the panel work this task built stands.
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 6dd6081
- Tests: baseline: green via handoff (ffa448a, conductor npx tsc --noEmit and npx vitest run, 345 tests; only .flow changed since aa6bc4c), gate check --gate unittest --command npx vitest run: RUN due to pre-existing dirty STRATEGY.md; no receipt reused, npx vitest run - pre-edit baseline PASS exit 0, 345 tests in 23 files, 27.63s; .flow/tmp/task6-baseline.log, npx vitest run harness/params.test.ts - RED exit 1, new law-control assertion found no matching dials; .flow/tmp/task6-red.log, npx vitest run harness/skeleton-view.test.ts -t derived branch read-out - RED exit 1, branchStats is not a function; .flow/tmp/task6-stats-red.log, npx vitest run harness/params.test.ts harness/skeleton-view.test.ts - PASS exit 0, 69 tests, 24.57s; .flow/tmp/task6-focused.log, npx tsc --noEmit - focused implementation PASS exit 0; .flow/tmp/task6-types-loop.log, npx vitest run --config .flow/tmp/task6-measure.config.ts --pool=threads --reporter=verbose --silent=false - measurement PASS exit 0, twelve configurations in one test; .flow/tmp/task6-measure.log, gate classify --base ffa448a6f8289e955b1577a11c22acfef30d37be: FULL (unmatched .worktrees/.gitignore), npx tsc --noEmit - final PASS exit 0; .flow/tmp/task6-types-final.log, npx vitest run - final PASS exit 0, 347 tests in 23 files, 26.62s; .flow/tmp/task6-full-final.log, git diff --check - PASS, gate receipt for typecheck and unittest: NO_RECEIPT due to pre-existing dirty STRATEGY.md; no gate skipped, flowctl show fn-6-branch-generations-below-the-crossover.6 --json - verified in_progress; no staging, commit or done attempted per user instruction, Vite and Chromium not run per supplied sandbox facts; R7 owner verdict remains unchecked, conductor verify: npx tsc --noEmit + npx vitest run (347 passed, 23 files) on the handed-over tree before commit
- PRs: