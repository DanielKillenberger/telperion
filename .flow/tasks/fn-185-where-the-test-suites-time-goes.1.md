---
satisfies: [R1, R2, R3, R4]
---
# fn-185-where-the-test-suites-time-goes.1 Implement Where the test suite's time goes

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Recon report at .flow/evidence/fn-185/REPORT.md.

- R1: the local gate split into compile and test: cold build 47 s, incremental 34 s, gate 427 s with fresh specimens and 342 s with cached ones, nextest 193 s. Also the CI job split, the slowest 30 tests, and time per binary and module.
- R2: verdicts for the slowest 30 tests and the ten pin families.
- R3: the CI rise is the no-geometry job rerunning the whole core suite since #126. The self dev-dependency unifies geometry back on.
- R4: ten ranked follow-ups with savings, obvious fixes marked.
- A CI run at the same commit was not available: the `Tests` path filter excludes `.flow/**`, so a branch that changes only `.flow/` starts no CI run.

stage: impl-review - skipped(policy: risk - report only, no code, test or workflow change)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 07f9a72e3d6f185430b8b565eb6f8ae675d11947
- Tests: cargo test --profile ci --workspace --no-fail-fast (green, twice), cargo nextest run --cargo-profile ci --workspace (green, 1103 passed)
- PRs: