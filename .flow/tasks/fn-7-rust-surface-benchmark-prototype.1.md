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
Implemented and measured the isolated surface experiment; all R1–R7 are covered under experiments/rust-surface-benchmark/. Frozen provenance and reproducible preset/synthetic inputs support equivalent reference TS, preallocated TS, and shared native/Wasm Rust, with actual Chromium correctness over 13 cases and ten repeated samples per representative path.

The measured report supports a bounded surface follow-up, not broader migration yet: final Wasm caller medians 27.08/45.73 ms versus optimized TS 74.10/126.29 ms (Telperion/Laurelin). Native measurements remain separate; JS heap observations, Wasm capacity, payload bytes and native RSS are explicitly distinct. Comparable total-browser peak memory, GPU and full-lifecycle gains remain unmeasured. Tail outliers are retained and disclosed.

R1: provenance.json, frozen snapshot and regenerated fixture hashes. R2: optimized.ts and shared rust/lib.rs. R3: browser.ts correctness plus native repeat comparison, 13 passing cases. R4–R5: results/measurements.json with raw samples, machine/build hashes and labeled memory. R6: README.md commands exercised; build, Rust formatting/warnings, typecheck and 24 focused tests passed. R7: REPORT.md migration decision and limits.

Baseline: green from the interrupted worker; no baseline rerun. Final verification passed once after source formatting; a benchmark overlapping final tests was superseded by the committed run without competing task test processes. Final gate classification: FULL. Green receipts recorded for typecheck, unittest and experiment at implementation commit 525a2a56.

Logs: /tmp/rust-benchmark-build-final.log, /tmp/rust-benchmark-correctness-final.log, /tmp/rust-benchmark-measurements-final.log, /tmp/rust-benchmark-final-typecheck.log, /tmp/rust-benchmark-final-tests.log. Runtime used PATH=/tmp/telperion-cargo/bin:/home/daniel/Projects/telperion/node_modules/.bin:$PATH, RUSTUP_HOME=/tmp/telperion-rustup, CARGO_HOME=/tmp/telperion-cargo. Production files and package manifests/locks unchanged.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 525a2a5620c8a875eb84a6cfcf8ed43449aaaec2
- Tests: baseline: green via interrupted-worker handoff; /tmp/rust-benchmark-baseline-typecheck.log and /tmp/rust-benchmark-baseline-tests.log (24 tests), cargo fmt --manifest-path experiments/rust-surface-benchmark/Cargo.toml -- --check, node experiments/rust-surface-benchmark/build.mjs (release native + wasm; RUSTFLAGS=-D warnings), node experiments/rust-surface-benchmark/run.mjs test (13 cases, actual Chromium + native), node experiments/rust-surface-benchmark/run.mjs bench (10 measured samples per representative case/path), npm run typecheck, npx vitest run src/mesh/surface.test.ts src/mesh/frames.test.ts src/mesh/paths.test.ts --maxWorkers=1 --testTimeout=60000 (24 passed), sha256sum -c /tmp/rust-benchmark-fixtures-before.sha256 (deterministic rebuilt fixture manifest), 23 frozen source SHA-256 values verified against git revision 019234b554a1b387e23b05dd3c7e2626f428f907, git diff --cached --check
- PRs: