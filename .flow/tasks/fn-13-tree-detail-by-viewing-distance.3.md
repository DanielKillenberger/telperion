---
satisfies: [R2, R5, R6]
---
# fn-13-tree-detail-by-viewing-distance.3 Promote the working data pipeline into the browser build

## Description
Promote the working data pipeline into the browser build.

## Acceptance
- Worker-backed preparation and owned prepared data exported from src/browser; exact defaults unchanged.

## Done summary
Assembly transport and canonical fitting moved into src/browser with worker-backed TreePreparation, owned PreparedTree data, bounded exact-view preparation and pure middle selection; exact TreeEngine defaults and the viewer unchanged.

## Evidence
- Commits: (on the unmerged fn-13 branch)
- Tests: npm run build; npm run typecheck; vitest run
