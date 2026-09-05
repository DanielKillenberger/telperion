---
satisfies: [R6, R7]
---

# fn-8-lean-rust-tree-generation-core.7 Measure the migration and remove the superseded production core

## Description
Prove the full-build improvement, then finish the single-core package and documentation.

**Size:** M
**Files:** migration benchmark runner/report, production cleanup, README and test/build configuration
**Touches:** [src/**, harness/**, tests/migration/**, scripts/**, README.md, package.json, package-lock.json, vitest.config.ts, vite.config.ts, tsconfig.json, Cargo.toml, Cargo.lock, crates/**, .flow/evidence/fn8/**, experiments/rust-surface-benchmark/REPORT.md]

## Approach
- Run contemporaneous final-FN6 and Rust release measurements with identical seeds, presets, requested outputs, browser viewport, DPR and named hardware; use task-1 equivalence and preserve output counts. Reuse the FN6 timer-query rig.
- Record stage/full-builder/scene-replacement latency and transfer costs with at least the FN6 warmup/sample protocol; increase CPU samples if variation obscures improvement. Report native timings separately. Measure memory by available domain and repeated build/dispose stability without claiming an unavailable total.
- Report GPU timer medians separately at native DPR and the existing sweep; do not reinterpret CPU improvement as a frame-budget win. Investigate CPU regressions before closing R6.
- Remove production TS generation once all behavioural tests have Rust or integration owners; retain only the thin adapter/UI tests and historical reference artifacts. An import/build audit proves no production path still reaches old algorithms.
- Update README/API/build commands and add a dated FN8 decision note to the historical FN7 report without changing its measured results. Fold CI/test wiring into this finalization task.

## Investigation targets
**Required:**
- `.flow/evidence/fn6-task7/measure.mjs`
- `.flow/evidence/fn6-task7/results.json`
- `harness/stage.ts:847`
- `README.md`
- `package.json`
- `vitest.config.ts`
- `experiments/rust-surface-benchmark/REPORT.md`

## Approved capture alignment
The rewritten parent capture is authoritative. Baselines diagnose drift; exact old topology or bytes are not a compatibility requirement, and known structural defects need not be reproduced. Preserve meaningful botanical and geometric invariants and report visual/numeric differences. Keep the core lean and simple.

Demonstrate lean extensibility with a representative botanical-rule change and an output change through their owning modules (a documented example or focused development exercise), with no duplicate production generator. Include field build/query costs and successful browser/native mesh-free consumer checks.


## Acceptance
- [ ] R6 whole-build CPU improvement is demonstrated with raw samples and unchanged work for both presets and comparison.
- [ ] Memory domains, transfer costs, native timings and real GPU results have honest availability/limit labels.
- [ ] Production import audit finds one Rust generator, and every removed behavioural test has retained coverage.
- [ ] Clean native/Wasm/browser build and relevant full suites pass using documented commands.
- [ ] README and historical benchmark note describe the final architecture and measured limits.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
