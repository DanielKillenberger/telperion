---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-117-a-species-run-carries-no-spending-cap.1 Implement A species run carries no spending cap

## Description
Make every conductor and tuning cap optional and add a no-progress runaway guard.

## Acceptance
Spec R1-R5.

## Done summary
All conductor and tuning caps optional (absent = no cap); resume and pilot authority compare only set caps (`tuning/caps.rs`); spend recorded as before; `tuning/runaway.rs` pauses after `runaway_rounds` (default 5) rounds with no kept adoption. Tests per R-ID in `tests/conductor.rs`, `tests/tuning.rs`, `tuning/caps.rs`, `tests/tuning_engine.rs`. Gate green.

## Evidence
- Commits: e0e157f3b2c73e7ca3996592522dec5408862879
- Tests: cargo test --profile ci --workspace --no-fail-fast (658 passed); tuning_engine runaway (red without the guard)
- PRs:
