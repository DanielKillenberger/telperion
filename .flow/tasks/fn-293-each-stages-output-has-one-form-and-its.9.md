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
The host's decisions on the open items (spec, 2026-10-19): `catalogue_identity.rs`'s header names the four digests and why they exist and points at `git log` for the re-pin history (test bodies and digests unchanged); the catalogue-row memory entry describes what a new row moves today (wire tests, generated tables, catalogue digests; no specimen snapshot); `scripts/benchmarks/generation.md`'s resident section and `docs/growth-path.md` lose their stale routes. End-of-spec friction: BASELINE.md "Running the digests" records each digest's wall time, coverage and VRAM declaration; FRICTION.md carries a disposition per entry (fixed, proposed, or local setup reported). fn-28 untouched.

Gate classify read FULL (an `.rs` comment), so the gate ran once: 1401 passed, 0 failed, 111 ignored. `npm test`: 170 passed.

Follow-ups: `Metrics::wood_fallback` and the "Resident expansion" paragraphs after the fixed one in `generation.md` describe the old wood expansion; `artifact_digest` could request the curve.

stage: impl-review - ran (codex, one reviewer: correctness, SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 604fe4612
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1401/0), npm test (170/0)
- PRs: