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
Selected evidence-ready Oregon white oak and Norway spruce profiles with frozen numeric gates, explicit contextual/unknown quantities, anatomy rubrics and fixed/fresh seed protocol. Attributed reference retrieval and inspected Ordinary baseline discrepancies are committed under `.flow/evidence/fn9/`.

Baseline: green; verification: 14 native tests, typecheck, measure and JSON validation passed. The docs-only classifier returned tier B; native tests/measure were additionally executed rather than claiming skip receipts. No shared gate receipts or tracker mutations were written by the parallel worker.

stage: impl-review - skipped(policy: parallel-wave - conductor owns the gate)

Task remains in_progress for conductor integration. Only the three allowed evidence files changed. No review verdict is claimed. Reference images and baseline captures are local in `/home/daniel/Projects/telperion/.worktrees/fn9-research/.refs/fn9/`; capture script and raw gate logs are in the same worktree `.flow/tmp/`. Baseline attached-twig and junction close-ups are explicitly unassessed, not a fidelity pass; whole/bare and isolated element captures were inspected. Profile readiness is independent of final generated-species validation.

Downstream note: dimensions exclude petiole/peg connectors. Foliage geometry needs a lightweight blade/needle subset or connector metadata to measure actual transformed dimensions. No need to add JSON serialization to the core just to read research profiles. All selected species use individual foliage units, no fascicles.

stage: impl-review - skipped(policy: owner requested no implementation review)
stage: plan-sync - skipped(config: planSync.enabled != true)
stage: wave-join - ran (research commit integrated and native measure verified)
## Evidence
- Commits: 6e7be26
- Tests: baseline: green (14 native tests, typecheck, native measure; each exit 0), cargo test --release -p telperion-core --test growth --test foliage (verify: 14 passed, exit 0), npm run typecheck (verify: exit 0), cargo run --release -p telperion-core --example measure (verify: exit 0, six nonempty samples), npm run wasm:build (exit 0, generated tracked metadata unchanged), node .flow/tmp/capture.mjs (exit 0; headless neutral whole/bare/element images inspected), python3 -m json.tool .flow/evidence/fn9/profiles.json (exit 0), gate classify: TIER_B docs-only; native suites were additionally run, not skipped; no fabricated skip receipt, Integrated target: cargo run --release -p telperion-core --example measure passed with six nonempty samples, 13264 nodes/63029 retained; JSON profile validated
- PRs: