---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-89-autonomous-species-conductor.1 Implement Autonomous species conductor

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The species conductor (`crates/telperion-jev/src/conductor`, binary `species-conductor`, runbook `docs/species-conductor.md`) carries one species run from the admitted manifest to the owner's packet: it runs the pipeline's stages by their own keys, hands a halt to the gap loop, runs the tuning loop and answers the reachable/covered/new check on every gap its `result.json` lists, drives a gap spec through design and implementation on separately judged tiers under the shared continuation check, and assembles a packet gated on numeric validation, visual readiness, matched views and documentation. A new gap is packaged in the fn-95 shape and escalated; the run pauses for a scoped human decision and resumes with its spend retained; obsolete and interrupted results never advance. One policy table (`data/conductor-policy.json`) maps Jev's judgments to routine, investigate, design, implement_cheap, implement_strong_low, implement_strong_medium or human, held to labelled and held-out cases in `data/cases/conductor.json` (routing and continuation scored apart, `species-conductor cases`). The report measures dispatches by role, tier and effort with unknown costs explicit and claims no saving.

Not proven here: no live species has run under the conductor, and the four conductor question sets are unvalidated against the live model (fn-80 owns the live proof); `continuation_validated` in the config asserts what the tuning loop's calibration establishes. Follow-ups noted, not built: a live executor exercise against the real binaries; a labelled set for the reach/cover/design/implementation questions.

Tier: session model (Fable 5.1), no bridge.
baseline: not run pre-edit (owner rule: the gate runs once at the end; no green receipt for HEAD)
gate: env -u TYPESAFE_API_KEY cargo test --profile ci --workspace --no-fail-fast -> suite_rc=0 (118 green result lines, none failed)

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: d6856737639d32e8be57f18b9ba6e31700ac004e, c3878a716f0229c44638011559792598039f59d2
- Tests: cargo test -p telperion-jev --profile ci --test conductor --lib, env -u TYPESAFE_API_KEY cargo test --profile ci --workspace --no-fail-fast, baseline: not run pre-edit (owner rule, CLAUDE.md Gates: the gate runs once at the end of a task; no green receipt for HEAD)
- PRs: