---
satisfies: [R1, R4, R5, R7]
---
# fn-8-lean-rust-tree-generation-core.6 Switch the browser harness to the Rust core

## Description
Switch the browser harness to the Rust core.

## Acceptance
- Viewer controls and comparison preserved; load/build errors recoverable; mesh-free fields reachable.

## Done summary
Rust/Wasm consumer API with transactional scene replacement; 100 harness tests, Chromium binding/UI checks and four full-tree browser cases pass.

## Evidence
- Commits: 2ced9cbd3ea08a2328c73d037d93ee920e2e7ea6
- Tests: typecheck, build, harness tests, cargo clippy, Chromium checks
- PRs:
