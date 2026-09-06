---
satisfies: [R2, R4, R5]
---
# fn-19-comparative-botanical-geometry-benchmark.2 Add reproducible structural distribution measurements

## Description
Add a benchmark-only native measurement runner and analytically tested structural distributions using the frozen protocol.

Consume the declared species inventory instead of an oak/spruce-only list. Resolve implemented species through the existing core registry and report unsupported generation explicitly. Test duplicate IDs, unknown generator identity and admission into a new cohort independently of generation; adding a manifest entry must not falsely imply botanical implementation or change an old cohort.

**Size:** M
**Files:** crates/telperion-core/examples/geometry_benchmark.rs, crates/telperion-core/examples/geometry_benchmark/metrics.rs, crates/telperion-core/tests/geometry_benchmark.rs, .flow/evidence/fn19/measurement-controls.json
**Touches:** [crates/telperion-core/examples/geometry_benchmark.rs, crates/telperion-core/examples/geometry_benchmark/**, crates/telperion-core/tests/geometry_benchmark.rs, .flow/evidence/fn19/measurement-controls.json]

### Approach
- Reuse species_metrics::measure and existing growth/surface/foliage entrypoints from a new example, leaving the old metrics schema and production generator unchanged. Add only the distributions absent from the existing report.
- Implement the frozen operational axes/order definition, per-order length/diameter summaries, taper along runs, branch-origin angles and foliage-centroid crown bins. Preserve anatomical leaf-versus-needle units and distinguish geometric measurements from estimated botanical classification.
- Adapt species_measure's terminal JSONL events, no-overwrite output handling and provenance. Record full relevant source/binary identities including dirty content, parameters, protocol hash and allocation/cost domain availability. Do not require render outputs that a measurement does not consume.
- Compare ordered case manifests rather than only successful rows. Require every case in baseline/candidate comparison; retain capped, empty, interrupted, invalid and unavailable results. A matching-source rerun must reproduce measurements; a changed-source run may change geometry and counts.
- Add analytic small-tree fixtures whose expected axes, lengths, diameters, taper, angles and centroid bins are independently known. Keep mature performance runs out of this task while fn18 measures; no optimization claims.

### Investigation targets
**Required:**
- crates/telperion-core/examples/species_metrics/mod.rs:24-388
- crates/telperion-core/examples/species_measure.rs:17-200
- crates/telperion-core/tests/species_metrics.rs:57-76
- crates/telperion-core/src/tree.rs
- .flow/evidence/fn9/profiles.json
**Optional:**
- .flow/evidence/fn9/final/numeric.json

### Key context
Keep DBH ambiguity and existing order/length estimated status visible. Do not add a production branch-state or assembly API for a measurement tool. The tests can include the example metric module using the existing species_metrics test pattern.

### Quick commands
- cargo test --release -p telperion-core --test geometry_benchmark
- cargo test --release -p telperion-core --test species_metrics
- cargo run --release -p telperion-core --example geometry_benchmark -- --help

## Acceptance
- [ ] Analytic straight-axis, lateral-fork, equal-dominance and zero-length fixtures validate the frozen distributions and unavailable/ambiguous states.
- [ ] Tests cover empty foliage, individual needles, nonfinite input, capped generation, malformed protocol and absent candidate cases without coercing missing data to zero.
- [ ] A small unchanged-source rerun is numerically reproducible; a deliberately changed measurement and a dropped/duplicated case are detected by comparison controls.
- [ ] Existing species metric tests and output semantics remain unchanged; receipts separate baseline/candidate source identity and measurement/cost domains.

## Done summary
Added a native benchmark runner for frozen species/case manifests, operational axis distributions and biological-unit centroid bins. Existing species_metrics output is preserved in each native artifact; immutable receipts retain every case disposition and verify source/tool/binary identities before replay comparison.

Analytic geometry tests cover axis lengths, circular diameters, nearest-rank summaries, taper, lateral angles, tied dominance, zero-length exclusions, empty foliage, individual needles and nonfinite geometry. Six Python controls cover malformed/duplicate/missing manifests, independent inventory expansion without false implementation, unsupported generators, interruption, changed/dropped/duplicated cases and artifact containment. Six 2m artificial historical-seed fixtures reproduce all numerical measurements across two native runs; six capped cases remain failed and retained. No mature specimen, frozen holdout or qualifying performance run was performed.

baseline: green — pre-edit species_metrics 5/5 and npm run typecheck passed. New benchmark tests/help did not exist at baseline.
Verify: geometry_benchmark 4/4 (including six Python controls), species_metrics 5/5, npm run typecheck and runner --help passed. Native control replay passed; .flow/evidence/fn19/measurement-controls.json records identities and outcomes. Gate classify required full checks; no skip receipts claimed. A first Verify exposed overstrict attribution resolution for frozen self-identifying comparator records; the implementation was corrected and affected controls rerun green. Baseline remained green.

stage: impl-review - skipped(config: REVIEW_MODE=none)
stage: plan-sync - skipped(config: planSync.enabled != true)

Numeric runs do not require render outputs. Optional conditions identity is null unless explicitly supplied. Allocation/GPU/capture domains are unavailable; native generation timing is a nonexclusive observation. Independent botanical assessment remains unassessed. The runner uses Python 3 standard-library receipt handling with native Rust generation and metrics, and rebuilds the native example before collecting source/binary identities.

Raw verification runs remain in this workspace at .flow/tmp/fn19-native-controls-final-2; usage notes are in the assigned NOTES_DIR/task2-measurement-interface.md. Shared review, lifecycle completion and gate receipt mutations are deferred to the conductor.
## Evidence
- Commits: 2554948ee3650d9924efe5ce9a0e33a55e5c6948, 18cf081ae6ebc608c9952aa3d874d3db9448b5ab, 880452cf7d3aa7d3054ffef510648ee3b254181c
- Tests: baseline: green (cargo test --release -p telperion-core --test species_metrics: 5 passed; /tmp/fn19-task2-baseline-rust.log), baseline: green (npm run typecheck; /tmp/fn19-task2-baseline-ts.log), flowctl gate classify --base f0606620a8a2c6251e212a8b91363e9df1be2837: FULL (Rust code paths), cargo test --release -p telperion-core --test geometry_benchmark: exit 0, 4 Rust tests including 6 Python controls; /tmp/fn19-task2-verify-geometry-final.log, cargo test --release -p telperion-core --test species_metrics: exit 0, 5 tests; /tmp/fn19-task2-verify-species.log, npm run typecheck: exit 0; /tmp/fn19-task2-verify-typecheck.log, cargo run --release -p telperion-core --example geometry_benchmark -- --help: exit 0; /tmp/fn19-task2-verify-help.log, python3 -B crates/telperion-core/examples/geometry_benchmark/native_controls.py --binary target/release/examples/geometry_benchmark --output .flow/tmp/fn19-native-controls-final-2 --receipt .flow/evidence/fn19/measurement-controls.json: exit 0; /tmp/fn19-task2-verify-native-final.log, First Verify geometry/native controls failed on overstrict comparator attribution resolution; fixed in 18cf081 and affected controls rerun green (logs /tmp/fn19-task2-verify-geometry.log and /tmp/fn19-task2-verify-native.log preserve red observation)., Integrated cargo test --release -p telperion-core --test geometry_benchmark --test species_metrics: PASS (4+5 Rust, 6 Python controls), Integrated npm run typecheck: PASS, Integrated cargo fmt --all -- --check: PASS, Integrated cargo run --release -p telperion-core --example geometry_benchmark -- --help: PASS
- PRs: