---
satisfies: [R2, R3, R4, R6]
---
# fn-9-real-species-profiles-and-procedural.6 Calibrate and register the conifer template

## Description
Calibrate and register the conifer template. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `crates/telperion-core/src/presets.rs`, `crates/telperion-core/tests/species.rs`, `crates/telperion-core/examples/species_measure.rs`, `.flow/evidence/fn9/conifer.md`
**Touches:** [crates/telperion-core/src/presets.rs, crates/telperion-core/tests/species.rs, crates/telperion-core/examples/species_measure.rs, .flow/evidence/fn9/conifer.md]

### Approach
- Register and calibrate the selected conifer through the same native identity/measurement path established in task 5.
- Judge leader/whorl or other researched habit and foliage-bearing shoot placement separately from overall crown dimensions. Keep needle/fascicle metrics aligned with task 1 definitions.
- Run all fixed conifer seeds and rerun the broadleaf cases after shared parameter/rule changes. Retain counterexamples rather than tuning one showcase.
- Task 5 is a real dependency because the preset registry, measurement dispatch and species tests are shared files.

### Investigation targets
**Required:**
- `crates/telperion-core/src/presets.rs:10-123` — family registry
- `crates/telperion-core/src/foliage/placement.rs:10-88` — attachment semantics
- `crates/telperion-core/tests/growth.rs` — reproducibility conventions
- `crates/telperion-core/examples/measure.rs` — baseline measurement pattern

### Quick commands
```bash
cargo test --release -p telperion-core --test species --test species_metrics --test foliage
```

## Acceptance
- [ ] Conifer identity is selectable natively and resolves to its recorded profile.
- [ ] All fixed conifer seeds meet gating metrics and structural invariants with appropriate needle accounting.
- [ ] Broadleaf checks still pass after conifer calibration; remaining visual assessment is explicit.
- [ ] Evidence records parameters, measurements and every corrected or unresolved counterexample.

## Done summary
Registered native NorwaySpruce against frozen Picea abies profile. All 12 spruce and 12 oak cases pass numeric gates and structural/geometry checks; 37 focused tests plus workspace/fmt checks green. Bare diagnostic captures 1–3 inspected; whole software captures timed out, final visual acceptance remains task 9. Contextual DBH ambiguity and targeted follow-up retained in conifer.md. No shared rule or target change; no task 7 start, review or merge.

Runtime state reconciled on 2026-09-06 from this committed completion receipt after fetching remote through 1297516. The preceding tests describe the original completion, not a fresh fidelity verdict.
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5a40d45a7988f4231cff42a33c299d01ffc9b7a8
- Tests: cargo test --release -p telperion-core --test species --test species_metrics --test foliage --test growth --test surface: 37 passed; all 12 fixed seeds per species, cargo check --workspace: passed, cargo fmt --all --check: passed, git diff --check: passed, species_measure on committed calibration: all 24 spruce/oak fixed cases pass
- PRs: