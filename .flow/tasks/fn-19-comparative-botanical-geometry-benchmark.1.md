---
satisfies: [R1, R2, R5]
---
# fn-19-comparative-botanical-geometry-benchmark.1 Freeze reference coverage and the comparative benchmark protocol

## Description
Establish the versioned reference inventory, specimen/view manifest, metric definitions and review rubric consumed by the two tooling tasks.

Design the inventory for additional species and independently owned species packets. Oak/spruce are only the initial cohort. Freeze the input contract consumed by tasks 2, 3 and 5, including stable species identity, supported anatomy, benchmark versioning and duplicate/missing/unsupported admission outcomes. The template/workflow task owns reusable documentation; this task owns protocol semantics.

**Size:** M
**Files:** .flow/evidence/fn19/PROTOCOL.md, .flow/evidence/fn19/REFERENCES.md, .flow/evidence/fn19/protocol.json, .flow/evidence/fn19/references.json
**Touches:** [.flow/evidence/fn19/PROTOCOL.md, .flow/evidence/fn19/REFERENCES.md, .flow/evidence/fn19/protocol.json, .flow/evidence/fn19/references.json]

### Approach
- Audit fn9's source IDs and historical seed usage before browsing for missing references. Verify primary botanical sources and available competitor examples; record quantitative/qualitative/unavailable matching status per species and anatomical scale. Public images with unknown dimensions cannot become matched mesh comparisons.
- Define a compact schema for protocol, reference and downstream run receipts. Freeze units, statuses, source/binary hashing, specimen parameters, camera/anatomical targeting, physical resolution, convergence settings, output paths and artifact completion rules before tasks 2/3 start. Reuse the receipt vocabulary of the existing runners.
- Draw and persist three genuinely unused holdout seeds per species before inspecting output; retain historical seeds 1/2/3 as regression/development cases. Audit prior manifests across relevant worktrees so previously inspected seeds are not relabeled held out. Record the baseline source content identity and capture-tool revision independently.
- Specify axis/order proxy semantics, run taper/angle measurements, foliage centroid binning and projected gap masks in enough detail for analytic fixtures. Define the crown ROI, exterior-opening/enclosed-hole distinction, threshold/connectivity and units. Keep candidate ROI and camera matching independent of changed candidate output.
- Include the independent reviewer rubric and what source labels may be hidden during assessment. Preserve attribution in the evidence mapping; no external contact or purchase is part of this task. Inventory age/crowded-context evidence for fn21 without expanding the initial mature-open-grown population.

### Investigation targets
**Required:**
- .flow/evidence/fn9/REFERENCES.md:5-48
- .flow/evidence/fn9/profiles.json:21-40
- .flow/evidence/fn9/seeds.json
- .flow/evidence/fn9/final/visual.json
- tests/migration/README.md:89-180
**Optional:**
- README.md:90-125

### Key context
Fn9's final report does not establish competitor superiority. Existing branch-order data are estimated botanical proxies. External comparator discovery uses the parent spec's reference pointers and current official documentation during work. Evidence records are benchmark data, not a second task tracker.

### Quick commands
- node tests/browser/species.mjs --help
- Parse and cross-check the four authored protocol/inventory artifacts before freezing them; task 2/3 add automated validators.

## Acceptance
- [ ] Every oak/spruce anatomical scale has attributed evidence or a specific missing-reference record; competitor availability is independently recorded with no superiority inferred from absence.
- [ ] Versioned protocol and seed manifest fix the 12-specimen initial population, required views, source identity, metric definitions and failure states before candidate output is inspected.
- [ ] A manual admission check demonstrates at least one usable real-reference comparison per species; remaining scale/context gaps have bounded claims and do not silently substitute species.
- [ ] Task 2 and task 3 can implement against the same frozen receipt/definition contract without editing one another's files; fn9 evidence stays unchanged.

## Done summary
Frozen fn19-v1 supplies twelve explicit mature specimens with full preset parameters, six newly drawn seeds audited against 2,637 seed-bearing files, hashed baseline source identity, and shared JSON Schema receipt/admission contracts. Precise axis/taper/angle/bin/mask definitions and fixed baseline camera conditions let downstream numeric and capture workers implement independently.

Verified primary botanical pages and opened all six attributed OSU images. Both species admit bounded qualitative comparisons; dedicated base/fork detail and bare spruce references remain explicit gaps, and no quantitative competitor match or independent botanical assessment is claimed. All four authored artifacts parse and cross-check; no fn9 or production files changed.

baseline: green — Rust species_metrics 5/5, TypeScript and runner help passed. Verify: JSON Schema/case/reference/hash/link checks passed and TypeScript passed. Gate classify: docs-only. Full check details and logs are in the evidence JSON. Cross-check program and temporary downloaded assets live in this workspace's ignored .flow/tmp; no source pixels are committed.

GATE_SKIPPED:unittest:docs-only - cumulative diff classified tier-B (no executable paths touched)
GATE_SKIPPED:smoke:docs-only - cumulative diff classified tier-B (no executable paths touched)

stage: impl-review - skipped(config: REVIEW_MODE=none; user requested)
stage: plan-sync - skipped(config: planSync.enabled != true)

Downstream workers consume protocol.json schema.$defs and PROTOCOL.md cross-field semantics; conditions are prepared once from baseline and reused unchanged across revisions. The historical source and adapted capture-tool identities are separate.
## Evidence
- Commits: 554720e77abd495f249bf2c5c7f9ff828ad21582
- Tests: baseline: green, cargo test --release -p telperion-core --test species_metrics (baseline: exit 0, 5 tests; /tmp/fn19-task1-baseline-rust.log), npm run typecheck (baseline and verify: exit 0; /tmp/fn19-task1-baseline-types.log, /tmp/fn19-task1-verify-types.log), node tests/browser/species.mjs --help (baseline: exit 0; /tmp/fn19-task1-baseline-help.log), .flow/tmp/validation-venv/bin/python .flow/tmp/check-protocol.py (exit 0; /tmp/fn19-task1-crosscheck.log), git diff --cached --check (exit 0), GATE_SKIPPED:unittest:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:smoke:docs-only - cumulative diff classified tier-B (no executable paths touched), Integrated target npm run typecheck: PASS, Integrated target protocol schema/case/reference/hash/link cross-check: PASS
- PRs: