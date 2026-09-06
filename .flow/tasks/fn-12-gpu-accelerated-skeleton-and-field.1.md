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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
