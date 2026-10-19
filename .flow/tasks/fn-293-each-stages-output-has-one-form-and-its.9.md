---
satisfies: [R4]
---
# fn-293-each-stages-output-has-one-form-and-its.9 The open items' docs and comments, and the end-of-spec friction fixes

## Description
The host decisions on the open items (spec, 2026-10-19): K7's header, the stale memory entry, two stale doc mentions; plus the obvious repo-side fixes from the spec's FRICTION.md entries (docs only).

**Size:** S
**Files:** `crates/telperion-core/tests/catalogue_identity.rs` (header only), `.flow/memory/bug/data/a-new-habitparams-row-changes-the-2026-09-26.md`, `scripts/benchmarks/generation.md`, `docs/growth-path.md`, `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/{BASELINE.md,FRICTION.md}`
**Touches:** [crates/telperion-core/tests/catalogue_identity.rs, .flow/memory/**, scripts/benchmarks/generation.md, docs/growth-path.md, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Acceptance
- [ ] The header names the four digests and why; test bodies and digests unchanged; `cargo test --profile ci -p telperion-core --test catalogue_identity` green.
- [ ] The memory entry describes what holds now.
- [ ] No stale `Delivery::Resident`, CPU wood fallback or live `branching::Specimen` claim in the two docs.
- [ ] Each FRICTION.md entry has a disposition; codex review SHIP.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
