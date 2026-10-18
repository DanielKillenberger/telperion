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
The space crate's experimental growth passes (oracle, coupling, partition, subtree, realized, stepped, the coarse tree and its expansion, the bounded build) and their examples are deleted; the Grower's oracle/couple/fine/handed hooks, the chain-grown-alone hooks, run_beside, the bounded records and the unread leaf-mean pass are gone, each branch keeping its live arm. The kept recursion, the boundary rows, the closed form's event positions (Stand, Along, step start/length), the unquantised light field (Field::new/on), sketch/run/whole and agree compile only into tests; bud_light, field_reads and the unread capture entries (units, layout, settled, settle_sag_turn) are deleted. Cross-crate test readers kept public and named: invariants (core repeat_tests.rs), grow_capture_light_at (core light_rows.rs now reads the cycle's lattice reads instead of bud_light), and the render tests' capture entries.

Digests: core artifact_digest (24 lines), generation_digest (byte-identical, sha256 fb4ce911...) and the resident digest (telperion-heavy 10 --gpu 6) all equal the baseline; core digests re-checked equal after each review fix. Non-test lines of the space crate 27197 -> 21643 (MEASURE.md; about 20080 less the whole modules gated by cfg(test)). Gate (before review fixes): 1404 passed, 0 failed; npm test 170/0. Review fixes got focused tests: space crate 204/0, core grown_total + units_bound, light tests.

Follow-ups (not in this task): the whole-tree grower's non-`one` step path (rows/balance/allot) still compiles into production, reached only by the test-only `run`; the fine-root parameters left in girth/geometry/sag/shed (thicken_given's given, place_from's fixed, disused_below's stub, sag::gather's given, Structure's stub, Error::Executor) are always neutral in production; capture_units' Before is now always `nothing()`.

stage: impl-review - ran (codex: round 1 NEEDS_WORK 2 findings, round 2 NEEDS_WORK 1, both fixed in 08f23f9e0 and 067d5ea6a; cap 2/2 reached while converging, host reset the task counter with recorded reason (FRICTION.md), re-review SHIP)
Tier: not reported (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8b19fbb968d801a6b77655099682727edc352c10, 08f23f9e075a631d48ff539996a47fa59fc70afd, 067d5ea6a5c7b10dddb2e940ef79ddec782ac583
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1404 passed, 0 failed; before review fixes), npm test (170/0), cargo test --profile ci -p telperion-space --no-fail-fast (204/0, after fixes), cargo test --profile ci -p telperion-core --test light_rows --test grown_total --test units_bound, artifact_digest x 24, generation_digest, resident digest_tests::digest: equal to BASELINE.md
- PRs: