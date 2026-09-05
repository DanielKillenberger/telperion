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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
