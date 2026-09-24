---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-136-a-species-run-finishes-identity-blocks.1 Implement fn-136-a-species-run-finishes-identity-blocks

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
A species run now has a finish line. A missing identity capability blocks at the gate. A missing improvement is a known gap that carries the specs capturing it. A reviewer blocker or defect tied to a known gap leaves readiness. A converged revision, including a bootstrap one, goes to the owner's packet. R1 to R7 are met; the fn-80 data record for R4 is the host's.

- R1: the capability assessment's `classes` list is read by `pipeline/stages/capability_class.rs`. The gate blocks only on a missing identity capability or one nobody classified; it records `capability.known_gaps` and reruns when the assessment file changes.
- R2 and R6: reviewer findings and defects carry a typed `trait_id`. `ComparisonRequest.known_gaps` names the unexpressed traits, and the protocol is now `reference-first-comparison-v2` with a new prompt. `unexpressed::set_aside` files a defect tied to a known gap under that gap, and `state::blocking_findings` and `ready()` pass over a blocker tied to one. The core-coverage check leaves the trait out. No model text is parsed. Tested on the palm's recorded revision-3 state with a fixture answer: the date cluster no longer blocks, and the trunk still does.
- R3 and R7: a revision converges when the runaway guard stops it, or when every listed gap passes and the run stopped with no supported proposal or ended. A converged revision goes straight to the packet. The packet records `converged`, `machine_readiness` and `outstanding`, and its checklist lists every known gap. A converged bootstrap reads `unqualified reviewer`, and reviewer qualification is a known gap. Tested with the palm's revision-3 result shape.
- R4: `tests/fixtures/fn136-palm-capability.json` records `infructescence` as an improvement, captured by fn-33 and fn-111, owner 2026-09-24. The host applies the same record in fn-80.
- Requalification the next live run must pay for:
  - Mirror the v2 schema into `scripts/reference-first-claude.py` on fn-80 (this branch updated `reference-first-codex.py` and its test).
  - Rebuild the replay manifest's cases as v2 requests, with `protocol_sha256` set to the new adapter's hash.
  - Rerun the fresh complete Stage B replay: one paid comparison per case.
  - The Stage A inventory stays valid. A bootstrap run's reviewer stays unqualified until the owner accepts a tree.
- Conductor files touched: `conductor/finish.rs` (new), `conductor/tuning.rs` (`settle` and `revised` only), `plan.rs`, `packet.rs`, `state.rs` and `mod.rs`.
- Docs updated: species-pipeline, species-conductor, species-onboarding and tuning-loop. The spec now records both host decisions and R6 and R7.

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: session (owner and host settled design)
## Evidence
- Commits: 88c77fe5a8d450063cc58e14a8f14c74d7938e5f, a3a0d97e6cca11742ca78c62bca0f69da380c350
- Tests: cargo test --profile ci --workspace --no-fail-fast (142 test-result lines: 1038 passed, 0 failed, 21 ignored; log .flow/evidence/fn-136-a-species-run-finishes-identity-blocks/raw/gate.log), python3 scripts/test-reference-first-codex.py, baseline: none (the spec lists no Quick commands; the gate ran once at the end)
- PRs: