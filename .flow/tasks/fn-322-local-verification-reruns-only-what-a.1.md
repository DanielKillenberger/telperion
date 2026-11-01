---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-322-local-verification-reruns-only-what-a.1 Implement Local verification reruns only what a change can affect

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Blocked:
The gate cannot go green on the base: two render tests fail, both reproducing alone, and fn-322's Decision Context stops the goal on that. The gate work itself is done and has codex SHIP; see `.flow/evidence/fn-322-local-verification-reruns-only-what-a/RESULT.md`. The owner's answer is asked for in fn-321's Goal log.

Blocked:
fn-322's gate work is done with codex SHIP. Its closing full run waits on fn-320, the device-field flake (owner's answer, 1 Nov 2026, in fn-321's Goal log); the stale test is fixed.
## Evidence
- Commits:
- Tests:
- PRs:
