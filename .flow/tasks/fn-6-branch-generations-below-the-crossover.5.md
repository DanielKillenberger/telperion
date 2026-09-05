---
satisfies: [R4, R2]
---
# fn-6-branch-generations-below-the-crossover.5 Fill, shed, and the leaf on the twig

## Description
The volume-fill criterion becomes a measurement with thresholds taken first, the shell rule is checked as the load-bearing pruner it now is, and the canopy places its shoots on twig nodes so the leaf hangs on the twig rather than on any wood under a fraction of the trunk.

**Size:** M
**Files:** `src/skeleton/fill.test.ts` (measurement on the new trees), `src/skeleton/shed.ts`, `src/skeleton/shed.test.ts`, `src/canopy/place.ts`, `src/canopy/place.test.ts`, `.flow/specs/fn-6-branch-generations-below-the-crossover.md` (§Measured)
**Touches:** [.flow/specs/fn-6-branch-generations-below-the-crossover.md, src/skeleton/fill.test.ts, harness/skeleton-view.ts, harness/skeleton-view.test.ts, src/skeleton/shed.ts, src/skeleton/shed.test.ts, src/canopy/place.ts, src/canopy/place.test.ts, src/index.ts]

### Approach
- The metrics exist from task 1 (`src/skeleton/fill.ts`), in shedding's shell units, with the twig mark as the terminal classification now. Threshold procedure, as fn-5's R2 did: run both presets at rest on the new pass, record both numbers, set them against the tuft baseline task 1 recorded (the shipped eight-order trees), and choose the shipped thresholds at the value a clay render distinguishes; write the new numbers and the chosen thresholds into §Measured beside the baseline.
- Shed: `shedTwigs` (`src/skeleton/shed.ts:66-120`) keeps its per-node predicate and subtree removal; assert the floor-and-ceiling guard — sheds neither everything nor nothing on both presets — and that the metrics are measured after shedding.
- Canopy: `buildCanopy` (`src/canopy/place.ts:203-413`) accepts the twig mark as the shoot gate when present, falling back to `shootRadius` for skeletons without one; leaf stations follow the twig's `internode` and `stations`. Assert the leaf is a botanical multiple of the twig diameter at rest on both presets (the R3 test in `src/presets/two-trees.test.ts` already measures the ratio; retarget it to twig nodes).

### Investigation targets
**Required:**
- `src/envelope.ts:169-209` — profile and distance, the one shape description
- `src/skeleton/shed.ts:66-120`; `src/canopy/cull.ts:66-201` — the shell predicate and its guard
- `src/canopy/place.ts:203-260` — shoot gate and station walk

**Optional:**
- `src/canopy/cull.test.ts:56-70` — the filled-crown fixture pattern

### Key context
- Two thinning passes share one shell criterion (shed on wood, cull on leaves); keep the depth number identical at both call sites or wood and leaves visibly disagree.
- If the shell rule cannot meet the threshold, record it; a light term is a new spec (spec §Parked unknowns).
### Conductor amendment (2026-09-05, after the first dispatch)
- Path chosen for the twig's leaf stations: `buildCanopy` gains an optional anatomy argument (the `TwigAnatomy` from the skeleton params) implemented in `src/canopy/place.ts`; the harness build (`harness/skeleton-view.ts`, the `buildCanopy` call at ~435) passes `skeletonParams.twigs.twig`, and a harness test proves the stations parameter reaches actual placement (changing `stationsPerInternode` changes the element count). Without the argument the placement falls back to the shoot rule, so plain skeletons keep working. The alternative, carrying anatomy on `TwiggedSkeleton`, is rejected as widening the skeleton contract for a canopy concern.
- `harness/skeleton-view.ts` and `harness/skeleton-view.test.ts` join the Touches for that one call site and its test.
## Acceptance
- [ ] Both presets measured at rest on the new pass with the twig mark; thresholds chosen against task 1's tuft baseline by the stated procedure and recorded in §Measured; the shipped trees hold them
- [ ] Shell rule sheds neither everything nor nothing on both presets
- [ ] Canopy shoots key on twig nodes; the leaf is a botanical multiple of the twig diameter on both presets
- [ ] `npx vitest run src/skeleton src/canopy src/presets` green; `npx tsc --noEmit` green
## Done summary
The canopy now places leaves on marked twig edges using the authored twig anatomy. Both rest trees hold the measured fill bounds, and the existing shell rule passes its two-sided shedding guard without a production predicate change. Implementation and both canonical gates are complete. The tree is uncommitted and the task remains in_progress for the conductor, as explicitly requested.

buildCanopy accepts an optional TwigAnatomy argument. With anatomy and pass records, each marked incoming twig edge receives the stated stations per metre-based internode. Station groups share an internode position, distribute around the twig, and advance by canopy divergence between internodes. Two stations are opposite. The existing per-shoot cap remains 512. Twig stations use the edge's startRadius for the petiole offset, so the parent limb radius cannot displace a leaf from its twig. Without anatomy or records the previous shootRadius, spacing and clump rule remains. The harness passes skeletonParams.twigs.twig. Existing index exports already expose TwigAnatomy, so src/index.ts needed no edit.

R2 coverage includes the marked-edge fixture, exact station heights and count, opposite station directions, metre-based spacing independent of envelope height, zero marked-edge output, and the plain/missing-anatomy fallback. The preset botanical-ratio test now selects twig marks, checks their fixed 5 mm diameter, retains the greater-than-10 leaf/diameter assertion and checks the placement count and every petiole's distance from its twig foot. The harness changes stationsPerInternode from 1 to 4, proves more than twice the placed instances and checks identical trunk geometry.

R4 measurements retain task 1's 0.02-height world-origin voxel grid, 0.05-height Euclidean colonization-tip neighbourhood and 0.45-maximum-radius shell depth. The metrics consume growReport's surviving twig marks after shedding. Neither metric was redefined or re-baselined.

| Preset | Post-shed nodes | Twig marks | Occupied / shell cells | Occupancy | Near-tip / marks | Clustering | Appended nodes shed / grown |
|---|---|---|---|---|---|---|---|
| Telperion | 49,713 | 19,744 | 1,282 / 6,352 | 20.1826% | 12,700 / 19,744 | 64.3233% | 5,527 / 54,432 |
| Laurelin | 104,335 | 40,781 | 6,207 / 74,072 | 8.3797% | 12,759 / 40,781 | 31.2866% | 7,584 / 109,477 |

Against the tuft baseline, occupancy rises 6.64 times and 3.55 times. Telperion's clustering worsens from 53.9387% to 64.3233%; Laurelin improves from 50.9189% to 31.2866%. Occupancy is the fill criterion. The clustering ceilings hold the measured distribution and do not claim that Telperion improved on that metric. The old neighbourhood now counts laterals as well as tip-derived twigs; revisiting its distance would require a new baseline and was not done.

Tight bounds of 21% / 9% minimum occupancy and 64% / 31% maximum clustering failed first. A limbRadius sweep from 0.09 through 0.095, 0.099 and 0.1 measured nearby cases. The front and 45-degree clay pairs at 0.099 and 0.1 showed no distinguishable whole-tree crown-fill change at 480 by 600 pixels per view. Telperion's occupancy moves from 20.0724% to 20.1826%; Laurelin is geometrically identical across the measured interval. The shipped floors are 20% and 8.3%; ceilings are 65% and 32%. The lower Telperion 0.095 case fails the floor at 19.5844%. Both tuft baselines fail the floors. These are narrow bounds around the measured rest, not a guarantee that arbitrary trees with equal metric values look alike.

Visual evidence is .flow/tmp/task5-clay-comparison.png and its reproducible software rasterizer .flow/tmp/task5-cpu-clay.ts. It renders actual buildSurface triangles with a z-buffer and diffuse grey shading at twice the displayed resolution, with fixed orthographic framing. The local Vite server failed with listen EPERM and Chromium failed on setsockopt; .flow/tmp/task5-clay-browser.log records the browser restriction. The software comparison was inspected; no browser, GPU timing, or owner R7 verdict is claimed. Task 6 still owns owner approval. The spec's Measured section records the complete procedure and limitations.

The existing shed tests keep their stronger guard, more than 50% and less than 95% of appended wood survives, on both presets. Telperion keeps 89.85% and Laurelin 93.07%. The identity assertion DEFAULT_SHED === DEFAULT_CULL still holds. No edits to shed.ts, shed.test.ts or the cull predicate were necessary. The new metric assertions additionally require actual shedding, surviving appended nodes and neither node nor level capping.

Fixture migrations under the conductor's standing authorization:
- src/presets/two-trees.test.ts, the botanical-ratio fixture, now classifies the twig marks that placement consumes instead of every childless node. The original ratio threshold stays greater than 10, with fixed-diameter and actual placement checks added. Its full-preset test receives the authorized 60 s timeout.
- harness/skeleton-view.test.ts, the absent-canopy fixture, uses buildTree's foliage-off argument because shootRadius no longer suppresses marked twig foliage. The null object, one draw and zero instances assertions are unchanged.
- harness/skeleton-view.test.ts, the density-to-element-count fixture, changes the authored twig station count instead of fallback-only canopy spacing. Its greater-than-2x assertion remains, and identical trunk geometry is added. This is also the required anatomy-to-placement regression, so the initially added standalone harness test was consolidated into it. The full-build timeout is 60 s.

No existing budget or tolerance was weakened. The first full gate exposed exactly the two obsolete harness fixtures above, both fixed within the authorized test surface. Final npx tsc --noEmit passed with exit 0. Final npx vitest run passed with exit 0, 345 tests in 23 files, 27.27 seconds. The acceptance suite passed 160 tests before the final harness migrations; the final full suite includes all acceptance and harness tests. Baseline was green by handoff and the required receipt-check fallback reran all 343 tests before implementation. All commands, failures and log paths are in the evidence JSON. No gate observation was rounded up or skipped.

The modified production files are only src/canopy/place.ts and harness/skeleton-view.ts. The new anatomy path makes the harness's legacy shootRadius, spacing and clump controls fallback-only; task 6 may need to describe or remove those ineffective controls when it updates the panel. No additional panel work was included here.

Investigation read worker.md first, the amended anchor, predecessor done summaries, flow-next-work, the envelope/profile code, shedding/culling predicates, placement and the zero-width envelope memory. Similar-code search chose extending existing placement transforms, reusing resolveTwigs anatomy rails and retaining shoots for the fallback. The inherited first-attempt measurements were reproduced by the candidate sweep. No subagent or review dispatch ran.

Workspace: /home/daniel/Projects/telperion
Base commit: df872ab780bcedb3c8b28a2ea1cc31d47252b047
Commit range: df872ab780bcedb3c8b28a2ea1cc31d47252b047..HEAD (empty)
Intended conductor commit subject: feat(canopy): fill measured, the shell rule held, and the leaf on the twig
Task trailer: Task: fn-6-branch-generations-below-the-crossover.5

The user explicitly instructed that .git is read-only and the conductor commits and completes. No git add, commit, flowctl done or branch operation was attempted. Typecheck and unittest receipt writes were refused because STRATEGY.md is already dirty; no receipt was written. STRATEGY.md, .flow/prospects/, .worktrees/ and the conductor's task metadata retain their pre-existing changes. flowctl show confirmed in_progress. The conductor can commit the seven implementation/spec/test files, update evidence with the commit, and complete the task.

stage: impl-review - skipped(config: REVIEW_MODE=none)

Exact files edited or created by this worker:
- .flow/specs/fn-6-branch-generations-below-the-crossover.md
- harness/skeleton-view.ts
- harness/skeleton-view.test.ts
- src/canopy/place.ts
- src/canopy/place.test.ts
- src/presets/two-trees.test.ts
- src/skeleton/fill.test.ts
- .flow/tmp/base_commit
- .flow/tmp/task5-clay-browser.log
- .flow/tmp/task5-clay-comparison.png
- .flow/tmp/task5-clay-laurelin-0.099-0.ppm
- .flow/tmp/task5-clay-laurelin-0.099-45.ppm
- .flow/tmp/task5-clay-laurelin-0.1-0.ppm
- .flow/tmp/task5-clay-laurelin-0.1-45.ppm
- .flow/tmp/task5-clay-telperion-0.099-0.ppm
- .flow/tmp/task5-clay-telperion-0.099-45.ppm
- .flow/tmp/task5-clay-telperion-0.1-0.ppm
- .flow/tmp/task5-clay-telperion-0.1-45.ppm
- .flow/tmp/task5-clay.html
- .flow/tmp/task5-clay.ts
- .flow/tmp/task5-cpu-clay.log
- .flow/tmp/task5-cpu-clay.mjs
- .flow/tmp/task5-cpu-clay.ts
- .flow/tmp/task5-retry-acceptance.log
- .flow/tmp/task5-retry-baseline.log
- .flow/tmp/task5-retry-fixtures.log
- .flow/tmp/task5-retry-focused.log
- .flow/tmp/task5-retry-full-before-fixtures.log
- .flow/tmp/task5-retry-full-final.log
- .flow/tmp/task5-retry-red.log
- .flow/tmp/task5-retry-types-final.log
- .flow/tmp/task5-retry-types-loop.log
- .flow/tmp/task5-tight-fill.log
- .flow/tmp/task5-visual-measure.config.ts
- .flow/tmp/task5-visual-measure.log
- .flow/tmp/task5-visual-measure.test.ts
- /tmp/claude-1000/-home-daniel-Projects-telperion/447cc600-c1ea-45c4-85f4-5f3967b0fd7a/scratchpad/handover-5-summary.md
- /tmp/claude-1000/-home-daniel-Projects-telperion/447cc600-c1ea-45c4-85f4-5f3967b0fd7a/scratchpad/handover-5-evidence.json

conductor: verified typecheck and 345 tests on the handed-over tree and committed as aa6bc4c (Codex sandbox mounts .git read-only); worker model gpt-6-astra at low; two dispatches, the first ended on a scope amendment (harness build call site)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: aa6bc4c
- Tests: baseline: green via handoff (df872ab, conductor npx tsc --noEmit and npx vitest run, 343 tests), gate check --gate unittest --command npx vitest run: RUN due to dirty STRATEGY.md; no receipt reused, npx vitest run - pre-edit baseline PASS exit 0, 343 tests in 23 files, 37.15s; .flow/tmp/task5-retry-baseline.log, npx vitest run src/canopy/place.test.ts harness/skeleton-view.test.ts -t twig anatomy placement|passes twig stations - RED before implementation, marked placement count 0 instead of 4 and unchanged harness count; .flow/tmp/task5-retry-red.log, npx vitest run src/canopy/place.test.ts harness/skeleton-view.test.ts -t twig anatomy placement|passes twig stations - PASS, 3 focused tests; .flow/tmp/task5-retry-focused.log, npx vitest run src/skeleton/fill.test.ts - tight initial bounds RED, all four numerical bounds fail; .flow/tmp/task5-tight-fill.log, npx vitest run --config .flow/tmp/task5-visual-measure.config.ts --pool=threads --reporter=verbose --silent=false - measurement-only PASS, 8 candidates in 1 test; .flow/tmp/task5-visual-measure.log, node .flow/tmp/task5-cpu-clay.mjs - PASS, production triangle software clay renders; front and 45-degree .099/.1 pairs inspected in .flow/tmp/task5-clay-comparison.png; no GPU or owner verdict claimed, npx vitest run src/skeleton src/canopy src/presets - PASS exit 0, 160 tests in 12 files; .flow/tmp/task5-retry-acceptance.log, npx vitest run - first implementation full gate FAIL, 344 passed / 2 failed; obsolete harness shootRadius-empty and spacing-count expectations; .flow/tmp/task5-retry-full-before-fixtures.log, npx vitest run harness/skeleton-view.test.ts -t passes twig stations|leaves no canopy - PASS exit 0, 2 migrated fixtures; .flow/tmp/task5-retry-fixtures.log, npx tsc --noEmit - final PASS exit 0; .flow/tmp/task5-retry-types-final.log, npx vitest run - final PASS exit 0, 345 tests in 23 files, 27.27s; .flow/tmp/task5-retry-full-final.log, git diff --check - PASS, gate classify --base df872ab780bcedb3c8b28a2ea1cc31d47252b047 - FULL (unmatched .worktrees/.gitignore); typecheck and unittest gate receipts refused due to pre-existing dirty STRATEGY.md; no verification gate skipped, flowctl show fn-6-branch-generations-below-the-crossover.5 --json - verified in_progress; no staging, commit or done attempted per user sandbox instructions, conductor verify: npx tsc --noEmit + npx vitest run (345 passed, 23 files) on the handed-over tree before commit
- PRs: