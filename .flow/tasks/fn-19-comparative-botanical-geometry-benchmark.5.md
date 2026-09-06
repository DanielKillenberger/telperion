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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
