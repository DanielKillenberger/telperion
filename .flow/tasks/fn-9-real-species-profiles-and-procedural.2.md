---
satisfies: [R2]
---
# fn-9-real-species-profiles-and-procedural.2 Measure specimens against the botanical profiles

## Description
Measure specimens against the botanical profiles. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `crates/telperion-core/Cargo.toml`, `Cargo.lock`, `crates/telperion-core/examples/species_measure.rs`, `crates/telperion-core/examples/species_metrics/mod.rs`, `crates/telperion-core/tests/species_metrics.rs`
**Touches:** [crates/telperion-core/Cargo.toml, Cargo.lock, crates/telperion-core/examples/species_measure.rs, crates/telperion-core/examples/species_metrics/mod.rs, crates/telperion-core/tests/species_metrics.rs]

### Approach
- Use a tooling-only serde_json dev dependency if needed to read the profile manifest; do not write a JSON parser or add a runtime core dependency. Cargo files make this task serial at admission. Keep the metrics helper in a subdirectory so Cargo does not discover it as a standalone example binary.
- Add a native CPU-only runner by reusing the measure example and public structure/foliage outputs. Keep metric helpers in example/test support, not a new public runtime subsystem.
- Compute actual height/DBH/crown extents, branch runs/order/length and foliage counts/area using task 1 definitions. An unsupported measurement must report unavailable; never derive anatomy from conservative field occupancy.
- Accept explicit case/profile IDs, seeds and output location. Before presets land, support measuring existing families so this task remains independently verifiable.
- Emit per-case summaries incrementally with completion state, timing, counts, profile revision, seed, machine/git metadata and mismatch reasons. Specify timeouts in the documented invocation and continue independent failed cases; aggregate failure is nonzero.
- Use tiny hand-constructed structures and foliage with analytically known dimensions to test metrics independently of generator output; avoid tautological tests against input height parameters.

### Investigation targets
**Required:**
- `crates/telperion-core/examples/measure.rs` — output/timing pattern
- `crates/telperion-core/src/tree.rs:11-53` — canonical topology
- `crates/telperion-core/src/foliage.rs` — actual foliage outputs
- `crates/telperion-core/src/field.rs:1-9` — conservative occupancy caveat
- `crates/telperion-core/tests/foliage.rs` — fixtures and failure conventions

### Quick commands
```bash
cargo test --release -p telperion-core --test species_metrics
cargo run --release -p telperion-core --example species_measure -- --help
```

## Acceptance
- [ ] Known small fixtures verify units, DBH interpolation, branch-vs-node counting and retained foliage accounting.
- [ ] Missing/estimated quantities, truncated growth and non-finite outputs have distinct statuses; required failures make the run fail.
- [ ] Fixed-seed runs reproduce metric values; interrupted or failed cases preserve prior completed summaries.
- [ ] Runner works without display/browser/GPU and measures existing families before the new species land.

## Done summary
Implemented a CPU-only profile comparison runner with actual wood/foliage geometry, DBH interpolation, operational branch axes/order/length, retained foliage accounting and transformed triangle area. Durable JSONL preserves per-case failures and interruption state, records frozen profile data plus machine/git metadata, and returns nonzero for unmet numeric gates.

Baseline: green (14 growth/foliage tests and typecheck). Verification: four analytic metric tests, 14 existing native tests, typecheck, --help, and real runner integration checks passed. Overflow regression was observed red with a null serialized branch length, then green after explicit derived-metric rejection. Initial interrupted-log observer encountered an unterminated final line; writes now serialize each event together, help documents complete-line recovery, and the corrected interruption check passed. Logs and compact generated case output are in the assigned workspace .flow/tmp/ (measure-integration.log points to final case artifacts).

Similar code search: reused measure.rs generation/output stages, Tree/NodeKind/run metadata and actual foliage/surface outputs. New example-private metrics helper because no botanical DBH or operational axis implementation existed. Runtime core remains dependency-free; serde_json is dev-only. Only declared Touches changed.

R2 tests: analytic_units_dbh_axes_and_retained_area independently verifies metres, 0.74 m DBH, 4 m branch run, one axis versus five nodes, transformed triangle area 6 m2 and pre/retained/discarded counts. missing_ambiguous_truncated_and_nonfinite_are_distinct covers absent required geometry, multistem DBH, growth cap and NaN. gates_do_not_pass_estimates_missing_values_or_outliers covers required estimates/unavailable values and out-of-range extrema. overflow_and_degenerate_geometry_fail_explicitly covers arithmetic overflow and zero-area triangles. Integration checks cover unknown identities, continuation, repeatable seed values and durable evidence.

Current generic prototypes have no blade/connector boundary. Whole-prototype length/width and area remain explicitly estimated, so required foliage gates are unassessed, never placeholder passes. Branch axes are operational estimates and DBH is labelled measured_proxy as the frozen definition requires. Crown and height use actual retained vertices and actual wood mesh, never field occupancy. Task4/calibration must wire connector-excluded blade/needle subsets into this helper before botanical dimension gates can pass; closed-needle surface/projected area must be distinguished when that geometry lands. Native preset dispatch currently accepts ordinary, telperion and laurelin; species registration tasks extend its explicit match.

CLI: --case ID:PROFILE:PRESET:SEED (repeat) --output NEW.jsonl [--profiles FILE]. --help documents compile then timeout 120s invocation. Timeout bounds the entire run; errors continue independent cases. Ignore an unterminated final line after interruption. Output files must be new, preventing silent evidence overwrites. Visual status stays unassessed.

stage: impl-review - skipped(policy: owner requested no implementation review)

Conductor integrated and verified this task. No review, tracker mutations, gate receipts, plan-sync or integration were performed. Gate classification was FULL due Cargo.lock; actual parent gates ran successfully. Conductor owns lifecycle completion and any shared receipts.

stage: plan-sync - skipped(config: planSync.enabled != true)
stage: wave-join - ran (integrated metric tests and CLI help passed)
## Evidence
- Commits: df0119d
- Tests: baseline: green — cargo test --release -p telperion-core --test growth --test foliage (14 passed); npm run typecheck (exit 0), cargo test --release -p telperion-core --test species_metrics (4 passed; analytic dimensions/DBH/axes/accounting, status/gates, overflow and degeneracy), red-to-green: overflow_and_degenerate_geometry_fail_explicitly rejected previous serialized-null branch length; .flow/tmp/measure-overflow-red.log, cargo run --release -p telperion-core --example species_measure -- --help (exit 0), cargo test --release -p telperion-core --test growth --test foliage (14 passed post-edit), npm run typecheck (exit 0 post-edit), native subprocess integration: fixed-seed metrics equal, unknown profile and preset errors preserve subsequent cases, required dimensions unassessed, output overwrite rejected, interruption preserves newline-terminated completed events (.flow/tmp/measure-integration.log), initial interruption observer: inconclusive due unterminated final JSONL line; event serialization tightened and documented newline recovery verified on rerun, git diff --cached --check (exit 0), Integrated target: species_metrics 4/4 pass; species_measure --help exit 0
- PRs: