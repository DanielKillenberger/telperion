---
satisfies: [R2, R3]
---
# fn-293-each-stages-output-has-one-form-and-its.2 The space crate's experimental passes and test-only items leave production

## Description
Findings 5, 11 (space side) and 16: the experimental growth passes with no production reader, the `Grower { oracle, couple, fine }` hooks production never sets, the kept-recursion columns production never reads, the unquantised light field and the pub test-only modules.

**Size:** M
**Files:** `crates/telperion-space/src/lib.rs`, `grow.rs`, `grow/{oracle,couple,partition,subtree,realized,stepped}*`, `grow/sketch.rs`, `grow/cycle/paused.rs`, `couple.rs`, `bounded*`, `fine/coarse.rs`, `closed_form/kept*`, `light.rs`, `light/field.rs`, `agree.rs`, `invariants.rs`, the space crate's `examples/` and `tests/`, `crates/telperion-core/tests/light_rows.rs`
**Touches:** [crates/telperion-space/**, crates/telperion-core/tests/light_rows.rs, crates/telperion-core/src/pipeline/space/repeat_tests.rs, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- For each symbol, grep its readers in `crates/*/src` excluding tests: no reader at all → delete; read only by tests or examples → `cfg(test)` (a test module) or delete with the example. Examples that exist only to drive a deleted pass go with it.
- Remove the three `Grower` fields and every arm that branches on them (`grow.rs:300-338,401-403`, `sketch.rs:209-224`, `paused.rs:326`, `draw.rs`, `balance.rs`, `shoot.rs`, `wake.rs`); the `(None, None)` arm is the live one and stays as the only code.
- Keep the production slices the audit names: `fine::{lineage_order, identities, lineage_permutation, canonical, digest, ChainTables}`, `capture_settled::settle_rows`, `grow_capture_tables`, `Capture`, `SettleTo`; and every `kept` item `predictor.rs`, `predictor/axis.rs`, `boundary.rs` read. `bounded/girth.rs` reads `kept::section::delta`: if `bounded` goes, keep only what a production reader needs.
- `light::Field::new`, `bud_light`, `field_reads`: `cfg(test)` or move to the tests that read them; `pub mod agree`, `pub mod invariants`: test-only if only tests read them (core's `repeat_tests.rs:58` reads `agree` from another crate's test; a cross-crate test reader needs the item public, so keep those public and name the reader).
- Fix `grow/stepped.rs:6`'s stale comment if the file survives.

### Investigation targets
**Required:**
- `crates/telperion-space/src/lib.rs:50-85`
- `crates/telperion-space/src/grow.rs:180-410`
- `crates/telperion-space/src/grow/sketch.rs:200-230`
- `crates/telperion-space/src/closed_form/predictor.rs:1-30`
**Optional:**
- `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/PIPELINE-AUDIT.md` findings 5, 11, 16

### Key context
The hooks are on the production grower's hot path; their removal must keep the cycle's digests (`grow/cycle/paused/tests.rs:70`, `unfold/tests.rs:123`) and the core baseline identical. Name collisions (`Measured`, `Stand`) make bare greps noisy: grep by path.
## Acceptance
- [ ] No experimental pass, `Grower` hook or unread `kept` column compiles into the space crate's non-test build.
- [ ] Digests equal the baseline (core, all presets, seeds 1, 7, 42); the cycle digest tests pass.
- [ ] Non-test line count of the space crate before and after in `MEASURE.md`.
- [ ] Gate and `npm test` green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
