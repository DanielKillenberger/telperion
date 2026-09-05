---
satisfies: [R2, R3, R4, R6]
---
# fn-9-real-species-profiles-and-procedural.5 Calibrate and register the broadleaf template

## Description
Calibrate and register the broadleaf template. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `crates/telperion-core/src/presets.rs`, `crates/telperion-core/tests/species.rs`, `crates/telperion-core/examples/species_measure.rs`, `.flow/evidence/fn9/broadleaf.md`
**Touches:** [crates/telperion-core/src/presets.rs, crates/telperion-core/tests/species.rs, crates/telperion-core/examples/species_measure.rs, .flow/evidence/fn9/broadleaf.md]

### Approach
- Use the selected broadleaf profile and existing Family composition. Register an explicit species ID; keep seed independent from family anatomy.
- Extend the measurement runner's preset selection and calibrate against all fixed seeds, reporting per-trait mismatches. Do not relax source ranges to match generated output.
- Capture and inspect temporary neutral views using the current harness/adapter where compatible; record pending browser-dependent checks for task 9. This task does not claim final visual acceptance.
- Keep changes focused on the template. Shared-rule gaps require a reproducible counterexample and a targeted follow-up to tasks 3/4, with all affected templates rechecked.

### Investigation targets
**Required:**
- `crates/telperion-core/src/presets.rs:10-123` — family registry
- `crates/telperion-core/tests/growth.rs` — deterministic preset checks
- `crates/telperion-core/examples/measure.rs` — existing preset measurement dispatch
- `harness/stage.ts:471-505` — neutral view conventions

### Quick commands
```bash
cargo test --release -p telperion-core --test species --test species_metrics
```

## Acceptance
- [ ] Broadleaf identity is selectable natively and resolves to its recorded profile.
- [ ] Same-seed repeatability and meaningful cross-seed variation are tested without duplicating golden buffers.
- [ ] All fixed seeds meet gating numeric/geometry checks; estimates and visual checks still pending are explicit.
- [ ] Calibration notes record parameters, measurements, observed failures and any shared-rule repairs.

## Done summary
Registered native Oregon white oak with explicit profile identity and calibrated spreading/lobed/alternate Family. All 12 fixed seeds pass numeric and geometry gates; deterministic full outputs and meaningful cross-seed variation tested. Template-only density calibration, no shared-rule changes. Final visual acceptance remains pending task 9; failed software captures and initial sparse crown recorded in broadleaf.md. Calibration/evidence SHA: d068391a9d043decc81ff1b556b8622f420618aa. REVIEW_MODE=none; no reviews or merge.
## Evidence
- Commits: d068391a9d043decc81ff1b556b8622f420618aa
- Tests: cargo test --release -p telperion-core --test species --test species_metrics: 7 passed; all 12 fixed seeds checked, cargo test --release -p telperion-core --test growth --test foliage --test surface: 28 passed, cargo check --workspace: passed, cargo fmt --all --check: passed, git diff --check: passed, species_measure: all 12 frozen oak cases pass on committed calibration, CLI unknown preset and mismatched species/profile: explicit failures, independent continuation verified
- PRs: