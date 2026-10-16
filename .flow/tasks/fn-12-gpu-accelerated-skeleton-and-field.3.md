---
satisfies: [R1, R2, R3]
---
# fn-12-gpu-accelerated-skeleton-and-field.3 Apply the measured adoption decision and document support

## Description
Apply the measured adoption decision and document support, precision and failure behaviour; a negative result completes the task with CPU retained.

## Acceptance
- Evidence-based decision for skeleton, construction and querying; only qualified paths ship.
- README and report state support, precision, resources and failure contract.

## Done summary
Production GPU queries rejected; optional owned f64 snapshot API documented in README and `.flow/evidence/fn12/REPORT.md`. On 2026-09-07 the owner had the rejected candidate, runner, tests and guide removed; historical reproduction points to commit `6f3adb2`.

## Evidence
- Commits: 080b8e07ad19ecce4fc337b0d7fd4b7e0888b8c5
- Tests: cargo test --release -p telperion-core --test field; npm test; npm run build; cargo test --release --workspace; cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings
