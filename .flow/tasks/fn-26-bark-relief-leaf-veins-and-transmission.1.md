---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-26-bark-relief-leaf-veins-and-transmission.1 Implement Bark relief, leaf veins and transmission

## Description
Implement bark relief, leaf veins and two-sided transmission in the shader along fn-14's coordinates (R1-R5).

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Bark relief as a footprint-filtered normal perturbation (grain, furrows, plates, flake); veins and margin tone, absent at section roundness 1; shadow-gated two-sided transmission; eight new material-row fields. R5: native oak 3.6879 / 4.0284 ms p50 / p95 against 3.8 ms; browser orbit at 60 fps. R4 accepted by the owner on 2026-09-13. Six bridged runs.

## Evidence
- Commits: dcb2669, 674e2f7, f9721ee, 7eef918, 04d67fd, f1bdd8c
- Tests: cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm run wasm:build && npm test, npm run typecheck
- PRs:
