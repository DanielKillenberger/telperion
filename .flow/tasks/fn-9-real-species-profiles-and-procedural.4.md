---
satisfies: [R4, R6]
---
# fn-9-real-species-profiles-and-procedural.4 Generate species-specific leaf and needle anatomy

## Description
Generate species-specific leaf and needle anatomy. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `crates/telperion-core/examples/species_metrics/mod.rs`, `crates/telperion-core/tests/species_metrics.rs`, `crates/telperion-core/src/foliage/element.rs`, `crates/telperion-core/src/foliage/placement.rs`, `crates/telperion-core/src/foliage.rs`, `crates/telperion-core/tests/foliage.rs`, `crates/telperion-core/tests/field.rs`
**Touches:** [crates/telperion-core/examples/species_metrics/mod.rs, crates/telperion-core/tests/species_metrics.rs, crates/telperion-core/src/foliage/element.rs, crates/telperion-core/src/foliage/placement.rs, crates/telperion-core/src/foliage.rs, crates/telperion-core/tests/foliage.rs, crates/telperion-core/tests/field.rs]

### Approach
- Integrate task 2 metrics with the actual blade/needle geometry subset, excluding petiole/peg connectors. Update analytic tests so new anatomy has exact measurable dimensions; do not promote whole-prototype estimates without evidence. Task 2 is scheduled first because it owns Cargo files.
- Extend element geometry and placement for the two researched anatomies. Needle count/grouping/orientation must follow the selected species; a narrow broadleaf card alone is insufficient.
- Prefer one reusable procedural element per selected family where it faithfully expresses the anatomy, including a compound element if appropriate. Add multiple element buckets only when the profile demonstrates the need.
- Keep biological unit metadata explicit for measurement. Coordinate any output-shape change with task 2 at integration; task 7 owns Wasm and adapter propagation.
- Validate transformed bounds, attachment, culling and conservative field coverage against actual geometry. Do not require wood surface generation to create or query foliage.
- Keep tests small and geometric: known attachment sites, needle/blade dimensions, orientation/grouping, empty results and invalid parameters.

### Investigation targets
**Required:**
- `crates/telperion-core/src/foliage/element.rs:4-134` — procedural prototype
- `crates/telperion-core/src/foliage/placement.rs:10-88` — twig placement
- `crates/telperion-core/src/foliage.rs` — composition and bounds
- `crates/telperion-core/src/field.rs` — consumes foliage geometry
- `crates/telperion-core/tests/foliage.rs:97-276` — validation patterns

### Quick commands
```bash
cargo test --release -p telperion-core --test foliage --test field
```

## Acceptance
- [ ] Both selected foliage anatomies have procedural geometry and attachment tests tied to profile traits.
- [ ] Counts distinguish leaves/needles from grouped render instances and remain usable by task 2.
- [ ] Bounds include transformed instances; field coverage and empty/invalid cases pass without requiring a surface mesh.
- [ ] Focused foliage/field regressions pass; any remaining unsupported anatomy is recorded.

## Done summary
Verified integrated foliage implementation at `fc2add4a01efdbbe45c63e6084e2606e99f7a747` after task 3 was marked done. Growth/foliage/field/species_metrics suites passed all 32 tests, including lobed blades and petioles, four-sided needles and pegs, local attachment, continuous station phase across subdivided twigs, connector-excluded transformed measurements, conservative bounds/field coverage, and explicit empty/invalid handling. Workspace compilation, formatting and whitespace checks passed again. Combined release workspace suite and typecheck were already green on this same Rust tree. No Rust edits were needed.

Each instance represents one biological leaf or needle; geometry metadata excludes connectors from unit metrics. Core runtime remains dependency-free, with serde_json dev-only. Named templates/calibration, Wasm/browser exposure and inspected cross-seed visual fidelity remain downstream. No anatomy substitution, gate weakening or browser fixture changes were made.

REVIEW_MODE=none: no reviews invoked. Original source-worker provenance remains in HANDOFF.md; completion evidence commits now reference the integrated handoff SHA.

Runtime state reconciled on 2026-09-06 from this committed completion receipt after fetching remote through 1297516. The preceding tests describe the original completion, not a fresh fidelity verdict.
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: fc2add4a01efdbbe45c63e6084e2606e99f7a747
- Tests: baseline: green (cargo test --release -p telperion-core --test growth --test foliage; npm run typecheck), red-to-green: foliage geometry/attachment/invalid connector tests (.flow/tmp/foliage-red.log), red-to-green: transformed species metrics (.flow/tmp/foliage-metrics-red.log), red-to-green: multi-edge same-twig station continuity (.flow/tmp/foliage-continuity-red.log), cargo test --release -p telperion-core --test growth --test foliage (18 passed; .flow/tmp/foliage-verify-parent.log), cargo test --release -p telperion-core --test foliage --test field (14 passed; .flow/tmp/foliage-verify-task.log), cargo test --release -p telperion-core --test species_metrics (5 passed; .flow/tmp/foliage-verify-metrics.log), npm run typecheck (exit 0; .flow/tmp/foliage-verify-typecheck.log), cargo check --workspace (exit 0; .flow/tmp/foliage-verify-workspace.log), rustfmt --edition 2021 (seven touched Rust files), git diff --check (exit 0), Integrated HEAD fc2add4a01efdbbe45c63e6084e2606e99f7a747, after task 3 done: cargo test --release -p telperion-core --test growth --test foliage --test field --test species_metrics — 32 passed; .flow/tmp/fn9-integrated-foliage.log, Integrated: cargo check --workspace; cargo fmt --all --check; git diff --check — passed after task 3 done, Combined tree: cargo test --release --workspace — 54 passed, 4 existing external-reference tests ignored; npm run typecheck — passed
- PRs: