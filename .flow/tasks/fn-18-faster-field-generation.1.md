---
satisfies: [R1, R2, R4]
---
# fn-18-faster-field-generation.1 Pin complete-build savings budget and exact candidate probes

## Description
Freeze the complete-build baseline and rank measured opportunities before production optimization.

**Size:** M
**Files:** scripts/benchmarks/field-generation.mjs (new), scripts/benchmarks/field-generation.md (new), tests/browser/field-generation.mjs (new), crates/telperion-core/examples/field_generation.rs (new), benchmark instrumentation in core/Wasm.
**Touches:** [scripts/benchmarks/field-generation*, tests/browser/field-generation.mjs, crates/telperion-core/examples/field_generation.rs, crates/telperion-core/src/**, crates/telperion-wasm/src/lib.rs, scripts/build-wasm.mjs, experiments/fn18-generation/**, .flow/evidence/fn18/**]

## Approach
- Base the implementation checkout on completed fn12 commit 6f3adb2, then bring this updated fn18 plan. Pin the clean integrated commit/binaries; do not use stale fn12 state in the authoring worktree as evidence of unfinished implementation.
- Reuse generation.mjs and generation-inputs.mjs. Extend stage accounting to envelope sampling, colonization/habit growth, local shoot planning, radius solves, shedding, foliage placement/cull, bounds conversion and both indices. Keep instrumentation benchmark-only, verify overhead and report exclusive/nested timings without double counting.
- Freeze full Ordinary/Telperion inputs and spruce/oak seeds 1 and 2, source/tree/report/matrix hashes and query cells. Compare reference/candidate field-only and combined output, exact primitive multiplicity and bounds, flags and same-version deterministic snapshots; BVH permutation alone may differ.
- Use the parent cold-build denominator, named allocation lifetimes, paired sampling and baseline-versus-baseline noise rule. Freeze all targets before candidate code. Save error/slow/interrupted samples.
- Allocate the savings necessary for 3× and 10× by stage. Run bounded, disposable probes of one high-share growth kernel and one foliage/index kernel; measure real work, not extrapolated zero-cost stages. If unavailable during fn13 timing, prepare the runner first and schedule exclusive measurements.
- Record ranked opportunities, measured-versus-projected ceilings, browser synchronous limitations and native parallel scope. No candidate gets a production pass here.

## Investigation targets
**Required:**
- scripts/benchmarks/generation.mjs and generation-inputs.mjs — runner/oracle.
- crates/telperion-core/src/branching.rs:252 — integrated growth pipeline.
- crates/telperion-core/src/colonization.rs:121 — incremental attraction settle.
- crates/telperion-core/src/branching/local.rs:259 — ordered frontiers.
- crates/telperion-core/src/field.rs:99 — retained bounds/index stages.
- crates/telperion-wasm/src/lib.rs:210 — build boundary.
- .flow/evidence/fn12/REPORT.md — historical evidence.

## Acceptance
- [ ] Frozen baseline, full-source oracle and noise/measurement rules reproduce, including negative controls for dropped/duplicated primitives and changed contact flags.
- [ ] Growth substages, foliage, bounds/index and named temporary lifetimes are accounted; unavailable telemetry cannot qualify.
- [ ] Bounded probes and the stage savings budget identify concrete opportunities and limitations for 3×/10× without claiming projected speedups.
- [ ] Browser/native cold, warm and throughput domains are explicitly separated; protocol and raw receipts are durable.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

