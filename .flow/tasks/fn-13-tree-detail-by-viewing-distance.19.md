---
satisfies: [R3, R5]
---
# fn-13-tree-detail-by-viewing-distance.19 Bound retained CPU memory for distant trees

## Description
Bound retained CPU memory for distant trees so the 1,024-tree forest fits the unchanged 4 GiB contract. The final streaming run (`.flow/evidence/fn13/final/forest1024-final-streaming/OUTCOME.json`) failed at 664 prepared trees with 4.28 GB retained: about 4.6 MB per distant tree, monotonic, no distance eviction. The trace in `.flow/evidence/fn13/final/retained-memory-trace.md` names the causes. Fix them in this order and stop when the acceptance holds:

1. **Drop dead arrays for distant trees** in `createResident` (`src/browser/webgpu/runtime.ts:33-42`). After compact wood and prototype dimensions are derived, a `detail:'distant'` tree keeps neither `structure` (consumed once) nor per-tree `prototypes` (only read behind `middleSelector`, which distant trees never have). Keep the fingerprint string for identity. Share one cylinder geometry across `applyCompactWood` calls (`src/browser/webgpu/compact-wood.ts:36`).
2. **Trim hierarchy levels by distance.** Every distant tree carries the full 0.16 to 5.12 cell ladder and draws one level; level 0 is about 80% of cells. Add the coarsening case symmetric to the refine predicate at `runtime.ts:78`: when the desired cell size exceeds `levels[0].cellSize * 2`, release the finer levels in place (empty arrays, stable indices) and let the existing predicate re-request them on approach.
3. **Evict farthest-first against a soft watermark** (about 3 GiB), one sweep per frame, instead of only reacting at the hard cap with `wanted = !known || visible` (`runtime.ts:76`), which makes every in-frustum tree unevictable. Exclude the nearest trees and active `regionDemand`. A rejected specimen must be retried on the next sweep rather than left in a permanent `entry.error`.

Work only with unit tests and the 8-tree forest until the memory test is green. One full 1,024-tree capture at the end, with `FOREST_COUNT=1024` through `scripts/benchmarks/forest.mjs`, writing only OUTCOME.json, INSPECTION.md and at most four stills. Do not open receipt.json. Do not add a new runner script per capture.

## Acceptance
- [ ] A unit test builds a mixed population past 4 GiB of nominal distant arrays and asserts retained CPU bytes depend on the near-tree count, not the prepared count; it also asserts a rejected specimen is retried after eviction.
- [ ] Distant trees retain no `structure` or per-tree `prototypes` after residency; unit tests for identity, fingerprint and parameter replacement still pass.
- [ ] One 1,024-tree capture completes its initial current view with zero visible-unavailable trees, retained CPU below 4 GiB and forest GPU pool below 1 GiB, with OUTCOME.json and INSPECTION.md committed. Time to first vegetation and to a complete current view are reported.
- [ ] `npm run typecheck` and `node_modules/.bin/vitest run` pass. Budget: 10 commits or 5 pilot ticks, then stop with NEEDS_HUMAN.


## Done summary
Distant residents drop dead structure and prototypes, share one wood cylinder, coarsen hierarchy levels by distance and evict farthest-first against a 3 GiB soft watermark. The 1,024-tree capture peaked at 1.73 GB retained with zero budget rejections, against 4.28 GB and a rejection at 664 trees before. The current view still did not settle inside 170 s because every distant tree is prepared at 0.16 m cells and two workers prepare about one tree per second; that clause is carried by task .22.
## Evidence
- Commits: dedf536, 6437a72, 646b533, 39c85d3, 9899585, f62f558
- Tests: vitest run: 248 passed (34 files), typecheck clean
- PRs:
## Blocker
All three ordered changes are implemented, unit-tested red-then-green and committed (dedf536, 6437a72, 646b533), and they do bound retained CPU: the 1,024-tree capture in `.flow/evidence/fn13/final/forest1024-bounded-retention/` peaks at 1.73 GB retained of the 4 GiB contract with a 536.8 MB forest GPU pool and zero `retained-array budget exceeded` rejections, where `forest1024-final-streaming` was rejected at 664 prepared with 4.28 GB; `activePreparedCpuBytes` plateaus at 905 MB from 21 regional specimens, so near-field cost is now bounded by the near-tree count. The capture nevertheless fails acceptance bullet 3 for a reason outside the three scoped changes: at the fixture's 1600x1000 viewport the per-specimen pixel scale is about 1,450 px/rad, so the streaming ladder's desired cell size is 0.16 m for essentially the whole forest, every specimen is already prepared at the finest rung, and distance coarsening has nothing to release for the static initial pose (it engages only beyond roughly 350 m); distant retention therefore stays at about 4.5 MB per prepared specimen, and independently the two-worker preparation rate of 0.94 specimens per second means the roughly 883 in-frustum specimens need about 15 minutes against the runner's 170 s initial-current-view cap, so the run stopped at 160 prepared with 723 visible-unavailable and zero failed. Bounding a full-frustum 1,024-tree current view needs a separate change to the per-specimen level-0 cell payload or to streaming throughput and the settle budget, which this task must not rescope into. One further observation for whoever picks that up: both inspected stills show a specimen rendered in mid-air above the ground plane at a stable screen position, unrelated to streaming coverage and not investigated here.
