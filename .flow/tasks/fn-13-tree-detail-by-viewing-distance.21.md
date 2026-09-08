---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-13-tree-detail-by-viewing-distance.21 Final renderer acceptance and documentation

## Description
Final R1 to R6 acceptance on the normal viewer, done once, after the memory bound (.19) and close-motion fix (.20) are green. Reuse valid evidence already cited in `.flow/evidence/fn13/final/RESULT.md`; capture only what is missing:

- Qualified hero p95 for mature spruce and oak against 2 ms. Both existing runs were contaminated by competing GPU activity. Close other GPU clients (the desktop browser's GPU process counts) and let `scripts/benchmarks/gpu-activity.mjs` confirm an idle GPU before timing. A contended run is a retry, not a result.
- One 1,024-tree capture for the 4 ms forest target, reporting the miss honestly if it misses, with whole-stage timing, preparation, upload and memory.
- R1/R2/R4 inspection of representative close, crown and far views, one oak and one spruce holdout, and one material parameter change, using at most four stills per capture.
- README section on use and limits; rewrite RESULT.md as the single final mapping of requirements to evidence and remaining limits. Mark the spec's task list current.

No new harness subsystems. No repeated full suites. If a requirement fails, record the failure and stop; correction is a new task.

## Acceptance
- [ ] Hero spruce and oak vegetation GPU p95 measured on an idle RTX 3080 at 1600×1000 DPR1, both at or below 2 ms, with receipts kept under 1 MiB.
- [ ] The 1,024-tree p95 is measured once and reported against 4 ms, pass or miss, with CPU, preparation, upload and memory figures.
- [ ] R1, R2 and R4 inspections for the listed views are recorded in INSPECTION.md files with named stills; any failure is explicit.
- [ ] README and RESULT.md are final; `npm run typecheck`, `node_modules/.bin/vitest run` and `npm run test:browser` pass. Budget: 6 commits or 3 pilot ticks.


## Done summary
Superseded on 2026-09-08 by the fn-22 series; fn-13 closed as a reference prototype, not merged.
## Evidence
- Commits:
- Tests:
- PRs:
## Known defect to resolve here
Both 1,024-tree stills from `forest1024-bounded-retention` show one specimen drawn in mid-air above the ground plane at a stable screen position. Identify the specimen (identity and model matrix) from OUTCOME.json and the fixture layout, explain the cause, and fix it or record it as an explicit R4 failure. No new capture is needed to identify it.
