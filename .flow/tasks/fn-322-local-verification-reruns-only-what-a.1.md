---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-322-local-verification-reruns-only-what-a.1 Implement Local verification reruns only what a change can affect

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The gate reruns only what a commit can affect, by ci-key.mjs's suite keys, with its skip proven by 27 fixture tests. A full run took 1,350 s, a `.flow/` commit verifies in 3 s. It was blocked on two base failures, now fixed (771ddee6a, fn-320), and every suite holds a real receipt at e04c51ae5. R3 deviation: a `src/` commit also runs rust-jev, whose isolation test reads `src`.

stage: impl-review - ran [0e544d803..c7aa840f0] (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e2f7bceefc03d529eff99b36f4d0251fd1f8ed89, 73865198e6657e3c93f21ab021aeb37dd0fd646f, 7e48bf239be348a0acec054425e400031dd30f06, 665943910879cb866f00b7eec39c8e272a197927, c7aa840f07b7344ed94c425ec7dc442834ccee02
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py
- PRs: