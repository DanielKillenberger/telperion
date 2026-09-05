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
Finished the single Rust production core, migrated legacy test owners and documented module boundaries, public API, build commands and representative rule/output changes. Paired headless complete builders improve 2.91× Telperion, 2.71× Laurelin and 2.81× comparison with unchanged work; a profiled squared-distance optimization removes the initial foliage regression.

Report and raw reproducible evidence: `.flow/evidence/fn8/REPORT.md`; checks: `.flow/evidence/fn8/verification.json`. Memory domains remain explicit: stable three-cycle backing storage and Wasm reuse, but 526 MiB giant Wasm highwater adds a persistent capacity domain; total browser peak/live allocator/GPU allocation unavailable. Real DPR1 GPU times remain roughly 6.28/3.72/9.63ms, above 2ms, with modest individual-preset regressions reported. No rendering win or total-memory reduction claimed.

baseline: none (task and parent define no Quick commands). All applicable gates pass: clean npm ci, native/Wasm release build, 39 release workspace tests plus explicit seven-case native growth/foliage references, 100 harness tests, typecheck, rustfmt, strict clippy, packaged embedded-Wasm smoke, actual headless binding/UI and ordinary/Telperion/Laurelin/comparison fixture checks. Four optional references were skipped by the regular workspace suite; growth and foliage were separately executed. Full current captures and numerical drift records are durable under the report. A pre-sample Vite HMR failure is recorded as inconclusive and excluded; restart resolved it without production workaround.

Final raw source lines: Rust core3564, binding518, browser adapter164; generated metadata348 and tests counted separately in loc.json. No LOC target was imposed. Legacy removal includes an explicit test-owner/retirement map; source-only archived reference exporters remain reproducible. No new botanical redesign or lifecycle/mod implementation.

stage: impl-review - skipped(policy: parallel-wave; explicit user REVIEW_MODE=none)


Conductor integrated fbb0728 and 48b66ce into fn8-rust-core; final combined-branch native, formatting, clippy, package, harness, typecheck and headless binding gates passed.
stage: wave-join - ran
stage: plan-sync - skipped(config: disabled)
## Evidence
- Commits: fbb0728ca6715129edb7a7153871019ea917ed15, 48b66ce0f9edd5839ef49d8604b82092f7fdf7bc
- Tests: npm ci (exit 0; .flow/evidence/fn8/logs/npm-ci.log), cargo test --release --workspace (exit 0; .flow/evidence/fn8/logs/cargo-test.log), npm test (exit 0; .flow/evidence/fn8/logs/npm-test.log), npm run typecheck (exit 0; .flow/evidence/fn8/logs/typecheck.log), cargo fmt --all -- --check (exit 0; .flow/evidence/fn8/logs/fmt.log), cargo clippy --workspace --all-targets -- -D warnings (exit 0; .flow/evidence/fn8/logs/clippy.log), npm run build (exit 0; .flow/evidence/fn8/logs/build.log), BROWSER_URL=http://127.0.0.1:5185 CHROMIUM_EXECUTABLE=/usr/bin/chromium npm run rust:test:wasm (exit 0; .flow/evidence/fn8/logs/binding.log), REFERENCE_DIR=/tmp/fn8-surface-reference BROWSER_URL=http://127.0.0.1:5185 CHROMIUM_EXECUTABLE=/usr/bin/chromium node tests/browser/migration.mjs (exit 0; .flow/evidence/fn8/logs/migration.log), node --input-type=module (packaged dist embedded-Wasm field smoke) (exit 0; .flow/evidence/fn8/logs/package-smoke.log), GROWTH_REFERENCE_DIR=/tmp/fn8-growth-reference cargo test --release -p telperion-core --test growth_reference -- --ignored --nocapture (exit 0; .flow/evidence/fn8/logs/native-growth-reference.log), REFERENCE_DIRECTORY=/tmp/fn8-growth-reference REFERENCE_CASES=ordinary,telperion,laurelin,empty,capped,degenerate,envelope-crossing cargo test --release -p telperion-core --test foliage_reference -- --ignored --nocapture (exit 0; .flow/evidence/fn8/logs/native-foliage-reference.log), baseline: none (task/parent define no Quick commands), flowctl gate classify --base 8a56a41c925ad4a7f926e3e2decb4aa3d0438a34 => FULL; all applicable recorded gates passed, Paired headless CPU/GPU protocol: 1 CPU warmup + 5 samples, 8 GPU warmups + 20 valid queries/DPR for both presets and comparison; unchanged work counts; .flow/evidence/fn8/{fn6,rust}-headless.json, Native full/field 1 warmup + 5 samples; browser field 32^3 queries; .flow/evidence/fn8/native and browser-field.json, Paired headless CDP memory: 3 build/dispose cycles per subject; .flow/evidence/fn8/{fn6,rust}-memory.json, Production import audit: no old TS generator; .flow/evidence/fn8/import-audit.json, Inconclusive: one optimized attempt failed before samples due Vite HMR duplicate modules after regeneration; server restart resolved; no result counted
- PRs: