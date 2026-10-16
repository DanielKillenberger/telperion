---
satisfies: [R1, R2, R3]
---
# fn-122-a-tuning-result-says-what-each-attempt.1 Implement fn-122-a-tuning-result-says-what-each-attempt

## Description
Record each tuning attempt's moves, adoption and whether it stood.

## Acceptance
Spec R1-R3.

## Done summary
Attempts carry `moves`, `adopted`, `stood`, `rolled_back`, recorded at the source (single-dial round, bundle keep); legacy records read via defaults; RESULT.md and `docs/tuning-loop.md` updated. Tests in `end_result.rs` and `tuning_engine.rs`. R3 needed a four-line test-helper edit in `conductor/gapcheck.rs` (forbidden to the worker, NEEDS_HUMAN); with it the gate passed (972).

## Evidence
- Commits: 6a75db61
- Tests: cargo test --profile ci --workspace --no-fail-fast (972 passed)
- PRs:
