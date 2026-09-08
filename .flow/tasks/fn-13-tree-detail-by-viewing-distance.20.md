---
satisfies: [R2]
---
# fn-13-tree-detail-by-viewing-distance.20 Fix close-motion regional detail on the 8-tree forest

## Description
Close the R2 close-motion defect on the 8-tree forest. The `forest8-angular-motion` inspection found distracting large supports during close motion; canonical fallback refresh, local capacity and payload keys were fixed but the regional request/cancel behaviour was left unfinished. The interrupted worker's two red tests are parked at `.flow/evidence/fn13/final/pending-regional-cancel-tests.patch`: apply them to `src/browser/render-selection.test.ts`, make them pass (cancel a coarser pending region on approach; request middle capability inside a covered region at the same cell scale), then run one forest8 capture with a cold approach, reversal and crown entry.

Inspect at most four stills plus the motion metric already produced by the harness. No 1,024-tree capture in this task.

## Acceptance
- [ ] The two parked regional cancel/middle-capability tests pass and are committed.
- [ ] One forest8 capture with cold approach, reversal and crown entry shows no large-support stepping, popping or missing regions; INSPECTION.md names the stills inspected and any residual defect explicitly.
- [ ] `npm run typecheck` and `node_modules/.bin/vitest run` pass. Budget: 6 commits or 3 pilot ticks, then stop with NEEDS_HUMAN.


## Done summary
Superseded on 2026-09-08 by the fn-22 series; fn-13 closed as a reference prototype, not merged.
## Evidence
- Commits:
- Tests:
- PRs: