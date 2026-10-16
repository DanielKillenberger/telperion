---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-74-ci-under-five-minutes-skip-unchanged.1 Implement CI under five minutes

## Description
Receipts keyed on suite inputs, path filters, two parallel jobs and a `ci` cargo profile.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
`scripts/ci-key.mjs` hashes suite inputs (crate trees, test fixtures and evidence inputs, Cargo.lock, toolchain, profile, workflow); a `receipts` job looks them up; `rust` and `node` jobs run in parallel and skip suites with a receipt. Docs and `.flow` changes start no run except the four test inputs there. `ci` profile used by the workflow's `cargo test` only. Desk: release and ci identical row for row (499 passed); warm compile 114 s -> 13 s. `rust-cache` has `cache-on-failure: true`.

## Evidence
- Commits: 805659b3849178de72927bb9807b1a6f2a648615, 08843239405d94b1bd72e42fc733f91c845c0e7b
- Tests: cargo test --release --workspace, cargo test --profile ci --workspace, npm test, npm run typecheck
- PRs: #35
