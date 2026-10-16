---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-7-rust-surface-benchmark-prototype.1 Implement and measure the isolated Rust surface benchmark

## Description
Implement the isolated experiment and collect native and Chromium measurements against frozen source 019234b554a1b387e23b05dd3c7e2626f428f907, touching only `experiments/rust-surface-benchmark/**`.

## Acceptance
- [x] R1: isolated files and frozen, reproducible inputs with provenance
- [x] R2: equivalent reference TS, optimized TS, native Rust and browser Wasm
- [x] R3: geometry equivalence and determinism tests pass
- [x] R4: repeated measurements with raw samples and named environment
- [x] R5: memory measurements labelled; unavailable metrics explicit
- [x] R6: build, test and benchmark commands documented and exercised
- [x] R7: measured report with a justified recommendation and no GPU claims

## Done summary
Completed 2026-09-05 in an isolated worktree and imported. All 13 cases passed; Wasm was ~3x faster than preallocated TS on the surface stage. Completion review SHIP.

## Evidence
- Commits: 525a2a5620c8a875eb84a6cfcf8ed43449aaaec2
- Tests: cargo fmt --check, node experiments/rust-surface-benchmark/build.mjs, run.mjs test (13 cases), run.mjs bench, npm run typecheck, npx vitest run src/mesh (24 passed)
- PRs:
