---
satisfies: [R1, R3, R5]
---
# fn-13-tree-detail-by-viewing-distance.22 Settle the full-frustum 1,024-tree view: cell target and preparation throughput

## Description
Make a full-frustum 1,024-tree current view settle. Task .19 bounded retained memory (1.73 GB peak, zero rejections) but the view still stopped at 160 prepared trees inside the runner's 170 s cap. See `.flow/evidence/fn13/final/forest1024-bounded-retention/INSPECTION.md` and the blocker at the end of task .19. Two causes, both measured, neither fixed:

1. **Every distant tree is prepared at the finest 0.16 m cell.** `desiredCellSize` in `src/browser/webgpu/runtime.ts` admits a cell only when it projects to at most about 1.3 px, so 0.16 m holds out to about 175 m and 0.32 m to 350 m. At about 4.5 MB per tree, 883 in-frustum trees would exceed 4 GiB again, and the .19 coarsening never engages. Raise the projected-cell target for distant trees to the coarsest value that still passes the pinned R1 whole-view statistic (worst 16 px tile absolute signed mean below .03, as in `production-angular-r1/refinement128/metrics.json`, reproduced on the 8-tree forest). Try 2, 3 and 4 px; keep the coarsest that passes. Alternatively or additionally, shrink the per-cell payload (48 B/cell today: 5 f32 points plus 7 f32 angular) if a packed encoding keeps the same statistic. Hero and regional near detail must not change.
2. **Preparation throughput.** Two concurrent workers prepare about one tree per second. Raise the worker bound to a value derived from `navigator.hardwareConcurrency` (leave two cores free), keep the visible/near priority, and report trees per second in the runner output. Then set the runner's initial-view settle budget honestly from the measured rate rather than the fixed 170 s.

Also add hysteresis to the .19 eviction sweep: a specimen evicted for retained bytes is not re-requested until the camera moves closer or retained bytes fall below the watermark by one ladder, so a frustum above the watermark cannot churn.

Work with unit tests and the 8-tree forest until the tile statistic passes at the chosen cell target and the throughput number is measured. Then one 1,024-tree capture through `scripts/benchmarks/forest.mjs` with `FOREST_COUNT=1024`, writing only OUTCOME.json, INSPECTION.md and at most four stills. Do not open receipt.json. Do not add a new runner script.

## Acceptance
- [x] The chosen distant projected-cell target (or packed cell payload) passes the pinned R1 tile statistic on the 8-tree forest, with the metrics file and the rejected coarser settings recorded.
- [x] A unit test asserts a budget-evicted specimen is not re-requested while retained bytes stay above the watermark and the camera has not approached.
- [x] Measured preparation throughput with the raised worker bound is reported in trees per second on this machine, and the runner's settle budget derives from it.
- [x] One 1,024-tree capture completes its initial current view with zero visible-unavailable trees, retained CPU below 4 GiB and forest GPU pool below 1 GiB; time to first vegetation and to a complete current view are reported in INSPECTION.md with at most four stills.
- [x] `npm run typecheck` and `node_modules/.bin/vitest run` pass. Budget: 10 commits or 5 pilot ticks, then stop with NEEDS_HUMAN.


## Done summary
Superseded on 2026-09-08 by the fn-22 series; fn-13 closed as a reference prototype, not merged.
## Evidence
- Commits:
- Tests:
- PRs: