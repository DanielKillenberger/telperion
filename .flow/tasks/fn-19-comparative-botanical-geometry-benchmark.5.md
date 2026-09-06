---
satisfies: [R5]
---
# fn-19-comparative-botanical-geometry-benchmark.5 Provide reusable species onboarding and parallel-agent workflow

## Description
Provide the reusable species-onboarding template and parallel-agent handoff workflow against task 1's frozen admission contract.

**Size:** M
**Files:** docs/species-onboarding.md, templates/species-profile.md, .flow/evidence/fn19/onboarding-examples/**
**Touches:** [docs/species-onboarding.md, templates/species-profile.md, .flow/evidence/fn19/onboarding-examples/**]

### Approach
- Turn the fn9 profile/reference/seed discipline into one fillable onboarding packet with scientific/stable identity, target age/context, references/confidence, dimensions and their definitions, structural/foliage traits, required shared capabilities, parameters, specimen seeds and per-trait validation evidence. Clearly separate a species template from its generated specimens.
- Document research/profile, capability assessment, isolated template implementation, specimen generation, numeric/visual validation and coordinated catalogue integration. Identify the input, output, evidence gate and handoff for each stage. Use existing Flow specs/tasks, not a second task/status database.
- Specify one worktree and species-owned artifacts per concurrent workstream. Independent research/profiles/templates can proceed together; shared missing core capabilities must have one owner and explicit dependent species. Registry/binding changes and integration are coordinated; expensive measurements obey the fn13/fn18 resource-window contract.
- Demonstrate two independently populated onboarding packets, reusing oak/spruce evidence where useful. Add a third illustrative species-manifest fixture to a later cohort and exercise task 1's admission rules while preserving the original cohort/hash. Label nonimplemented species as unsupported for generation, and incomplete packet anatomy as unmet. This proves workflow/format extensibility, not another species' fidelity.
- Include accepted/rejected examples for duplicate species IDs, absent references, unsupported anatomy, reused holdouts, competing shared changes and unavailable expert feedback. Describe resuming a failed species workstream without altering another's evidence or silently substituting anatomy.

### Investigation targets
**Required:**
- .flow/evidence/fn9/profiles.json
- .flow/evidence/fn9/REFERENCES.md
- .flow/evidence/fn9/seeds.json
- crates/telperion-core/src/presets.rs:11-65
- tests/migration/README.md:89-180
**Optional:**
- README.md:90-125

### Key context
The current core registers species centrally. A workflow document must acknowledge this shared integration point rather than promise that arbitrary simultaneous edits to the registry are independent. No runtime species plugin system or new agent scheduler is required.

### Quick commands
- Parse and validate onboarding fixtures against task 1's contract.
- Run task 2/3/4 admission/comparison checks at the integration task once available.

## Acceptance
- [ ] Fillable template separates species-level botanical evidence and capability requirements from specimen-level seeds and receipts.
- [ ] Workflow names stage inputs/outputs, quality gates, independent file ownership and coordinated core/registry/measurement handoffs using existing Flow execution.
- [ ] Two onboarding packets and a later-cohort additional-species admission fixture show expansion without changing earlier cohort identity; unsupported generation is reported honestly.
- [ ] Duplicate IDs, missing evidence and shared capability conflicts have explicit handling, and the guide never equates successful generation with botanical approval.


## Done summary
Reusable dispatch template and stage/ownership guide accompany separate oak and spruce packets containing attributed profiles, complete parameters and specimen lists. A later illustrative cohort preserves original protocol/reference hashes and twelve cases; Scots pine parses but actual admission reports unsupported-anatomy with implemented=false, never runnable admission or botanical approval.

R5: verify.py uses actual task-2 manifest/admission/schema/native support and parameter parsing. Its 22 checks cover old identity, expansion, duplicate IDs, missing references/profile hash, reused holdouts, unsupported anatomy, coordinator ownership/resource examples and unavailable expert input. Workflow controls are explicitly coordinator examples, not a native scheduler. Pine profile is incomplete and parameter payload is parser-shape-only; no generation or production changes.

baseline: green (Rust 5/5, TypeScript after npm ci). Verify: parent Quick commands passed; onboarding 22, admission controls 6/6, visual controls 9/9 passed. Initial fixture check rejected an incorrect seed-role spelling; fixture corrected to frozen holdout-at-freeze. Logs: workspace .flow/tmp/task5-*. Classify FULL; no gate skips. Precommit receipt declined staged docs; no receipt claimed.

stage: impl-review - skipped(config: REVIEW_MODE=none; parallel-wave conductor owns review)
stage: plan-sync - skipped(config: planSync.enabled=false; conductor owns lifecycle)

Working tree clean; conductor owns integration, review and completion.
## Evidence
- Commits: 7550dd7fb66220f14cd65992babe1cd4331270da
- Tests: baseline: green (species_metrics 5/5 and TypeScript after npm ci), cargo test --release -p telperion-core --test species_metrics: 5/5 passed, npm run typecheck: passed, cargo build --release -p telperion-core --example geometry_benchmark: passed, python3 -B .flow/evidence/fn19/onboarding-examples/verify.py: 22 checks passed, python3 -B -m unittest discover -s crates/telperion-core/examples/geometry_benchmark -p test_controls.py: 6/6 passed, node --test tests/browser/geometry-benchmark.test.mjs: 9/9 passed, git diff --check: passed, gate classify: FULL; parent Quick commands executed, gate receipt: NO_RECEIPT due to staged docs before commit; no receipt claimed, Integrated python3 -B .flow/evidence/fn19/onboarding-examples/verify.py: PASS 22 checks, Integrated npm run typecheck: PASS
- PRs: