---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-7-rust-surface-benchmark-prototype.1 Implement and measure the isolated Rust surface benchmark

## Description
Implement the isolated experiment described by the parent spec and collect actual native and Chromium measurements. Freeze committed source 019234b554a1b387e23b05dd3c7e2626f428f907 so later FN-6 edits cannot change the reference. Read production surface/frames/paths/radius types, but do not modify them. Optimize the TypeScript allocation path for a fair baseline; reuse one Rust implementation across native and Wasm. Include a report that separates evidence from unmeasured memory/GPU claims.

**Touches:** [experiments/rust-surface-benchmark/**]

**Investigation targets:** src/mesh/surface.ts, src/mesh/frames.ts, src/mesh/paths.ts, src/radius.ts, src/presets/two-trees.ts; existing surface tests; STRATEGY.md.

**Review:** none per CLAUDE.md. Completion review remains conductor-owned.

## Acceptance
- [ ] R1: isolated files and frozen, reproducible inputs with provenance
- [ ] R2: equivalent reference TS, optimized TS, native Rust and browser Wasm
- [ ] R3: meaningful geometry equivalence and determinism tests pass
- [ ] R4: repeated actual measurements with raw samples and named environment
- [ ] R5: memory measurements clearly labeled; unavailable metrics explicit
- [ ] R6: build, test and benchmark commands documented and exercised
- [ ] R7: measured report makes a justified migration recommendation without claiming GPU gains

## Done summary
Imported the completed FN-7 receipt from its isolated worktree state after integration. Original completion 2026-09-05T13:03:31Z; implementation 525a2a5 and completion review SHIP retained. Historical frozen-input results remain unchanged; final-FN-6 whole-pipeline comparison belongs to FN-8.
## Evidence
- Commits: 525a2a5620c8a875eb84a6cfcf8ed43449aaaec2
- Tests: baseline: green via interrupted-worker handoff; /tmp/rust-benchmark-baseline-typecheck.log and /tmp/rust-benchmark-baseline-tests.log (24 tests), cargo fmt --manifest-path experiments/rust-surface-benchmark/Cargo.toml -- --check, node experiments/rust-surface-benchmark/build.mjs (release native + wasm; RUSTFLAGS=-D warnings), node experiments/rust-surface-benchmark/run.mjs test (13 cases, actual Chromium + native), node experiments/rust-surface-benchmark/run.mjs bench (10 measured samples per representative case/path), npm run typecheck, npx vitest run src/mesh/surface.test.ts src/mesh/frames.test.ts src/mesh/paths.test.ts --maxWorkers=1 --testTimeout=60000 (24 passed), sha256sum -c /tmp/rust-benchmark-fixtures-before.sha256 (deterministic rebuilt fixture manifest), 23 frozen source SHA-256 values verified against git revision 019234b554a1b387e23b05dd3c7e2626f428f907, git diff --cached --check
- PRs: