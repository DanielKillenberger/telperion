---
satisfies: [R1, R2, R4, R7]
---
# fn-8-lean-rust-tree-generation-core.1 Pin the final reference and establish the lean native/Wasm foundation

## Description
Pin the final fn-6 reference and build the lean native/Wasm foundation.

## Acceptance
- Dependency-free core with owned storage and deterministic primitives; native/browser ownership proof; reproducible fixtures and comparison policy.

## Done summary
Rust core and thin Wasm layer with deterministic math/RNG/envelope/noise/bias; browser proof compares 192 owned values (max error 0). Provenance and seven fixtures in `tests/migration/README.md`.

## Evidence
- Commits: b410f3ba1fb304ed0810712262cc1534ea122442
- Tests: cargo test (5 passed), browser ownership proof
- PRs:
