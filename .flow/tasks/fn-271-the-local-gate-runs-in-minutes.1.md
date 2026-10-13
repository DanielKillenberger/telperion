# fn-271-the-local-gate-runs-in-minutes.1 Measure and shrink the gate's slowest tests

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The gate's slow tests now grow their independent trees at once. Assertions, ages, seeds, step counts and presets are unchanged. The gate went from 1,525 s to 1,128 s (25:25 to 18:48) with 1,357 tests run and 89 ignored both times. Every changed test was shown red on a temporary broken-generator patch and green on the engine (.flow/evidence/fn-271-the-local-gate-runs-in-minutes/GATE.md).

Per binary: walks.rs 481.6 to 310.2 s, core lib 217.4 to 190.4 s (bound by the repeat test's full-size one-core builds), field lib 100.9 to 54.0 s, catalogue_identity 62.8 to 10.1 s, render lib 101.8 to 98.4 s (GPU, unchanged). The walks binary's peak memory rose from 2.7 to 6.9 GB.

OPEN_ITEM: R2 per-walk minimum sizes. Probes show 200 steps is the branch test's premise and that dropping seeds moves its asserted crossing count. Age was not probed because several walks are written for age 12. Options are in GATE.md (narrow the explosive trunk-lateral walk, two seeds with the count restated, or the two big walks behind --ignored with CI running them).
OPEN_ITEM: R4 nextest time. Every width tried (32, 8, 4, 16 with groups, 4 with one heavy test at a time) ran available memory under 4 GB and was stopped at 986 to 1,275 of 1,357 tests. The capped run under telperion-heavy was never admitted. The core lib alone peaks at 17 to 18 GB under cargo test.
OVERRIDDEN: 2 declined findings (codex re-review kept R2 and R4 as partial; both are the open items above).

stage: impl-review - ran (codex, 3-draw panel then 1 re-review; NEEDS_WORK on R2/R4 partial, overridden as open items)
Tier: actual_model claude-opus-5-5

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 4b39c53d01b2fe4be598041e81a30ef6c3e6956a, d3766e68512f9e80a70b373572cf9a27e8dbf7fd, 0675ccde45704560920ab26c6154396846c87d8f, c15255d42a2c6662f3cc6a3e9dfb384363500387
- Tests: cargo test --profile ci --workspace --no-fail-fast (rc=0, 1128 s, raw/gate-after.log), npm test (rc=0, 162 passed)
- PRs: