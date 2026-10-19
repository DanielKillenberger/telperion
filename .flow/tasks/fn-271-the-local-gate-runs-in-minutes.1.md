# fn-271-the-local-gate-runs-in-minutes.1 Measure and shrink the gate's slowest tests

## Description
fn-271 R1-R4: measure the gate, shrink its slow tests without weakening assertions, report nextest.

## Acceptance
- [x] The spec's R1-R4 (R2 and R4 settled by host decision).

## Done summary
The gate's slow tests grow their independent trees at once; assertions, ages, seeds, steps and presets unchanged. Gate 1,525 → 1,128 s (25:25 → 18:48), 1,357 run and 89 ignored both times; each changed test red on a temporary broken-generator patch and green on the engine. walks.rs 481.6 → 310.2 s, core lib 217.4 → 190.4 s, field lib 100.9 → 54.0 s, `catalogue_identity` 62.8 → 10.1 s. The walks' premise needs full size; nextest ran the desk out of memory at every width tried. Codex NEEDS_WORK on R2/R4 partial, overridden as the host's decisions.

## Evidence
- Commits: 4b39c53d01b2fe4be598041e81a30ef6c3e6956a, d3766e68512f9e80a70b373572cf9a27e8dbf7fd, 0675ccde45704560920ab26c6154396846c87d8f, c15255d42a2c6662f3cc6a3e9dfb384363500387
- Tests: cargo test --profile ci --workspace --no-fail-fast (rc=0, 1128 s), npm test (rc=0, 162 passed)
- PRs:
