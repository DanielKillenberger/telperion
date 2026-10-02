---
satisfies: [R1, R2, R3]
---
# fn-186-the-no-geometry-job-tests-the-build.1 Implement The no-geometry job tests the build without geometry

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Blocked:
Waiting on fn-181 (owner, 2026-10-02). Without geometry, the core's lib tests do not compile: 196 errors, 190 of them in growth-path tests that fn-181 deletes.

The partial gates are in a git stash in the main checkout: "fn-186 partial: no-geometry test gates (resume after fn-181)".
## Evidence
- Commits:
- Tests:
- PRs:
