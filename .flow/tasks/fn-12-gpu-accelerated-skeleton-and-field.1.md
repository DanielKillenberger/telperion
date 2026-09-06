---
satisfies: [R1, R2, R3]
---
# fn-12-gpu-accelerated-skeleton-and-field.1 Establish paired generation baselines and portable field snapshots

## Description
Establish R1/R2/R3 experiment inputs and reference invariants (spec Approach).

**Size:** M
**Files:** `crates/telperion-core/src/field.rs`, `crates/telperion-core/tests/field.rs`, `crates/telperion-wasm/src/lib.rs`, `src/browser/core.ts`, `scripts/benchmarks/generation.mjs`, `tests/browser/integration.mjs`
**Touches:** [crates/telperion-core/src/field.rs, crates/telperion-core/tests/field.rs, crates/telperion-core/examples/**, crates/telperion-wasm/src/lib.rs, src/browser/core.ts, scripts/benchmarks/generation*, tests/browser/integration.mjs]

### Approach
Reuse Field::new/query at field.rs:56-135 and typed-array ownership at core.ts:87-117. Add the minimum optional portable snapshot needed for exact indexed wood+foliage GPU queries; preserve f64 reference and default no-copy behavior. Prefer benchmark-specific exports when they avoid widening the production contract. Reuse scripts/benchmarks/field.mjs:6-17 for same-browser Ordinary/Telperion paired baseline, stage times and queries; record schema, hashes and all preparation costs. Profile skeleton dependency/transfer opportunities against colonization.rs:156-296 without changing botanical generation.

### Quick commands
`cargo test --release -p telperion-core --test field`; `npm run typecheck`; focused Wasm/browser binding tests if changed.

## Acceptance
- [ ] Reproducible full Ordinary and Telperion CPU stage profiles and owned canonical snapshot inputs/metadata, with counts/completeness and warmup/raw samples; no render mesh generation.
- [ ] Snapshot query semantics and ownership agree with current indexed CPU field; empty, outside, point/contact, invalid and disposed/stale inputs covered; ordinary consumers do not pay snapshot copies.
- [ ] Benchmark runner supports fixed grid/query matrix and exposes construction/extraction/packing/memory costs; skeleton feasibility findings name actual parallel substeps and sequential dependencies.


## Done summary
Implemented optional owned f64 field snapshots preserving the CPU wood/foliage BVHs, with revision-checked extraction and immediate native staging release. Added the paired mesh-free browser baseline, saved raw profiles and hashes in `scripts/benchmarks/generation-baseline.json`, and documented the transferable schema and skeleton dependency findings in `scripts/benchmarks/generation.md`.

Baseline: green (four existing Rust field tests and typecheck). The new snapshot test failed first for the missing API, then passed; final field tests (five), Wasm unit tests (two), typecheck, formatting, and CPU browser binding checks passed. Browser checks cover exact indexed snapshot flags, empty/outside/point/contact/large cells, malformed/nonfinite/negative/overflow requests, ownership, and stale/released/disposed extraction. Existing consumers retain zero snapshot buffers; native release and failed stale extraction clear all staging slots.

All twelve full-preset CPU observations (one warmup and five paired measured samples per subject) completed without caps and repeated exact source/query/result hashes. Both warmup grids and every contact batch matched the independent f64 snapshot traversal. Full Telperion has 177,543 wood primitives and 1,360,279 retained foliage boxes. Median CPU 32³/64³ query times were 3.9/20.3 ms Ordinary and 6.9/25.8 ms Telperion. The latter costs 44.1 ms snapshot extraction/copy/release plus 19.9 ms exploratory packing; these costs must be charged by the GPU experiment. No GPU workloads ran.

Canonical little-endian input bundles and all metadata are in `/tmp/fn12-generation-v2`; every disk array SHA256 was checked. `sample=0` artifact manifests define offsets/types/lengths for task 2. Raw results are also committed; binaries stay outside the repository. Initial multi-file export was inconclusive due to Chromium download suppression, then replaced by one bundle per subject and rerun successfully. An optional Wasm test wrapper initially lacked the Playwright module; its rerun with the installed module and CPU binding mode passed.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 539e656b7b1b22a2c0bc83cfd0065806f1903760
- Tests: baseline: green (cargo test --release -p telperion-core --test field; npm run typecheck), cargo test --release -p telperion-core --test field portable_snapshot: red before implementation, E0599 missing snapshot method (/tmp/fn12-snapshot-red.log), cargo test --release -p telperion-core --test field: passed 5 tests (/tmp/fn12-field-verify.log), npm run typecheck: passed (/tmp/fn12-typecheck-verify.log), npm run wasm:build: passed (/tmp/fn12-wasm-build.log), cargo test --release -p telperion-wasm: passed 2 tests (/tmp/fn12-wasm-test.log), BINDINGS_ONLY=1 BROWSER_URL=http://127.0.0.1:5188 BROWSER_EVIDENCE=/tmp/fn12-bindings PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs CHROMIUM_EXECUTABLE=/usr/bin/chromium node tests/browser/integration.mjs: passed (/tmp/fn12-bindings-final.log), node scripts/test-wasm.mjs: inconclusive initial missing Playwright installation; rerun with explicit installed module and CPU bindings-only mode passed (/tmp/fn12-wasm-node-verify.log), GENERATION_OUTPUT=/tmp/fn12-generation-v2 BROWSER_URL=http://127.0.0.1:5188 PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs CHROMIUM_EXECUTABLE=/usr/bin/chromium node scripts/benchmarks/generation.mjs: passed 12 complete repeatable samples (/tmp/fn12-generation-v2.log), initial benchmark export: inconclusive, Chromium multiple-download suppression; stopped and changed to one bundle per subject before successful full rerun, All 22 source/query/result arrays in completed disk bundles independently verified against recorded SHA256s, rustfmt --edition 2021 --check crates/telperion-core/src/field.rs crates/telperion-core/tests/field.rs crates/telperion-wasm/src/lib.rs: passed, git diff --check: passed
- PRs: