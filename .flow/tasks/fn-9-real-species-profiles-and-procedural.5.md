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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
