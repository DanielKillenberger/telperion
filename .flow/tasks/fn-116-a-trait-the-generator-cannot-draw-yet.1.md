---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-116-a-trait-the-generator-cannot-draw-yet.1 Implement A trait the generator cannot draw yet never vetoes an adoption

## Description
Let the tuning config list unexpressed traits that never veto an adoption.

## Acceptance
Spec R1-R5.

## Done summary
`unexpressed` config entries; `worsened` records a listed backwards trait as a route note instead of a rollback reason; readiness unchanged; `Config::verify` refuses blank specs, unknown traits or a listing without inventory. Tests per R-ID in `veto`, `tuning_engine`, `reference_first`, `tuning_command`; the engine test was red without the wiring. Gate green (937 passed).

## Evidence
- Commits: c1002d603c7b17e5aefed02339e883a97453d847
- Tests: cargo test --profile ci --workspace --no-fail-fast (937 passed)
- PRs:
