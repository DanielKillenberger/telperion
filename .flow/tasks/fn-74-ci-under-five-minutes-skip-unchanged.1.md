---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-74-ci-under-five-minutes-skip-unchanged.1 Implement CI under five minutes

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
CI now skips every suite whose inputs are unchanged: `scripts/ci-key.mjs` hashes a suite's inputs from the commit's git objects (crate trees, the fixtures and evidence files the tests read, Cargo.lock, rust-toolchain.toml, the cargo profile, the workflow itself), a `receipts` job looks each key up in the Actions cache, and the `rust` (per crate) and `node` jobs run in parallel with each suite skipped, and its key named in the log, when a green run left a receipt; a red suite saves nothing. Pushes and pull requests under `.flow/**`, `docs/**` or markdown start no run, except the four test inputs that live under those trees (`.flow/evidence/{fn9,fn34}/profiles.json`, `.flow/evidence/fn19/protocol.json`, `.flow/specs/fn-11-growth-over-time.md`). A `ci` cargo profile (release without fat LTO, 16 codegen units) is used by the workflow's `cargo test` only; every package script and local command stays on release, and the wasm builds inside `npm test` therefore stay on release too.

R4 on the desk: release and ci give identical results row for row (80 rows, 499 passed, 0 failed, 11 ignored), no suite stays on release; species.rs 133 s vs 135 s. R6 proxy: warm compile 114 s under release, 13 s under ci on 32 cores; the runner's four-core number decides whether test binaries are grouped, so grouping is not done here. R5 and R6 on the runner are unmeasured in this evidence: this shell cannot push, the conductor takes them from the Actions runs (`.flow/evidence/fn74/RUNS.md` beside LOCAL.md). Follow-ups noted, not built: a one-off dispatch that runs both profiles on the runner (FRICTION.md); `rust-cache` got `cache-on-failure: true` so a red suite keeps the dependency cache for the next run.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 805659b3849178de72927bb9807b1a6f2a648615, 08843239405d94b1bd72e42fc733f91c845c0e7b
- Tests: cargo test --release --workspace (80 rows, 499 passed, 0 failed, 11 ignored, 545 s), cargo test --profile ci --workspace (80 rows, 499 passed, 0 failed, 11 ignored, 479 s; identical to release row for row), cargo test --profile ci --workspace --no-run after touching core lib.rs (warm compile 13 s; release 114 s), npm test (6 files, 85 passed), npm run typecheck, node scripts/ci-key.mjs <suite> x5 (deterministic, distinct, moves with core edit, not with README edit), python3 yaml.safe_load on tests.yml and actions/suite/action.yml, baseline: none (the spec lists no Quick commands; the release suite run doubles as the pre-change baseline)
- PRs: