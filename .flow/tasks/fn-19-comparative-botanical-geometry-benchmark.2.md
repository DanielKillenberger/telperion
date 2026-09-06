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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
