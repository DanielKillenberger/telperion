---
satisfies: [R1, R2, R5]
---
# fn-9-real-species-profiles-and-procedural.1 Research two species and freeze botanical profiles

## Description
Research two species and freeze botanical profiles. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `.flow/evidence/fn9/profiles.json`, `.flow/evidence/fn9/REFERENCES.md`, `.flow/evidence/fn9/BASELINE.md`
**Touches:** [.flow/evidence/fn9/profiles.json, .flow/evidence/fn9/REFERENCES.md, .flow/evidence/fn9/BASELINE.md]

### Approach
- Select one broadleaf and one needle-bearing conifer with distinguishing architecture and accessible references. Use the parent References as research leads, not preapproved targets. Record scientific names and stable kebab-case IDs; fix one mature stage/context each.
- Create compact profiles with whole/branch/foliage source attribution, access and usage notes, measurement definitions, sourced ranges, confidence, missing data, and a trait rubric. Keep downloaded images in ignored .refs/ and record reproducible retrieval instructions.
- Freeze the 12 fixed seeds, gating/contextual metric classification, and fresh-seed protocol before tuning. Distinguish biological branches from nodes, needles from fascicles, and instances from actual foliage units.
- Run a disposable baseline using current Family generation and neutral captures, inspect them, and record the precise anatomy gaps. Name required natural branching, needle attachment, and any surface defects for tasks 3/4; do not invent a universal system.
- Record profile readiness explicitly. Replace poorly documented candidates before completing this task; uncertain abundance can stay estimated if meaningful dimensional and anatomical targets exist.

### Investigation targets
**Required:**
- `crates/telperion-core/src/presets.rs:10-123` — current family composition
- `crates/telperion-core/src/tree.rs:11-53` — measurable structure
- `crates/telperion-core/src/foliage/element.rs:4-134` — existing element capability
- `crates/telperion-core/examples/measure.rs` — baseline native runner
- `tests/browser/integration.mjs:10-121` — capture pattern
- `.gitignore` — local reference policy

### Quick commands
```bash
cargo run --release -p telperion-core --example measure
```

## Acceptance
- [ ] Both named profiles meet R1 readiness and carry the agreed units/context, evidence provenance and visual rubric.
- [ ] Fixed seeds and gating ranges are recorded before any template calibration; unknown branch/leaf totals are labelled rather than invented.
- [ ] Baseline images have been inspected and a per-profile capability matrix identifies the smallest required changes, including branch habit and attachment.
- [ ] Compact evidence persists in the worktree; images can be retrieved from the manifest, with unavailable sources explicitly reported.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
