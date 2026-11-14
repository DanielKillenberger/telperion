---
satisfies: [R1]
---
# fn-319-the-release-audits-slice-lands-before.2 The field makes only the programs it dispatches

## Description
The device field (`Generator::for_fields_async`, prepared by `web/field.rs fields()`) stops making the curve's surfacing passes, the sweeper and the ringer, and holds a tool set narrower by type.

**Size:** M
**Files:** `crates/telperion-render/src/generation.rs`, `generation/resident.rs`, `generation/resident/build.rs`, `generation/resident/field/make.rs`, `web/field.rs`, `generation/resident/field_time_tests.rs`, `generation/resident/field_tests.rs`, `docs/pipeline.md`, `docs/field-package.md`
**Touches:** [crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/resident.rs, crates/telperion-render/src/generation/resident/**, crates/telperion-render/src/web/field.rs, docs/pipeline.md, docs/field-package.md]

### Approach
- `prepare_programs` (`generation/resident.rs:46-54`) makes `Kit::new` beside `curve::passes`; `kit()` (`resident.rs:122`) builds the whole `Kit` lazily, so dropping the call alone saves nothing: the field needs its own kit shape.
- Split `Tools` (`resident/build.rs:37-67`) into the part the shared `folded` path and the field use (hanger, dresser, stationer, fielder) and the renderer's remainder (sweeper, ringer), following the `io::pair` pattern; `field::grow` (`make.rs:54-56`) takes the narrower type, so a field generator cannot reach `placed`/`readied` (`build.rs:225`, `:378`).
- Check whether a one-shot `tree::drawn` uses `tree::Kernels.present` (only known non-test user `cycle/growth.rs:278`) and whether `Generator::create` (`generation.rs:164-195`) must build `mass` and `positions` for a field; drop what the field does not dispatch, keep `place` for flowering trees.
- The field's preparation (`web/field.rs:38-65`) calls a field-scoped preparation; the renderer's `preparePrograms` (`web/generation.rs:42-51`) is unchanged.
- Measure natively: cold preparation and first tree before (from task 1's baseline commit) and after, three runs each, in `.flow/evidence/fn-319-*/`; answers hashes equal to task 1's.
- Update `docs/pipeline.md:77-82` and the `prepareField` description in `docs/field-package.md`.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/resident.rs:15-130`
- `crates/telperion-render/src/generation/resident/build.rs:30-130`
- `crates/telperion-render/src/web/field.rs:30-90`
- `crates/telperion-render/src/generation.rs:150-200`
**Optional:**
- `crates/telperion-render/src/generation/cycle/compile_tests.rs:82-143` (measurement only, asserts nothing)

### Acceptance
- [ ] A field generator holds no curve pass, sweeper or ringer (a test), and the narrower type cannot be passed where they are needed (it does not compile)
- [ ] Field answers hashes equal task 1's for every shipped preset
- [ ] Paired cold preparation and first-tree times recorded
- [ ] The renderer's preparation and its tests unchanged

## Acceptance
- [ ] TBD

## Done summary
A field's generator now prepares only the cycle's kernels, a field's own tools and the generator's passes. The kernels still include `present`, which a field never dispatches. It never makes the wood's 11 surfacing passes, the sweeper (11 programs) or the ringer. `build::Tools` was split in two. A field's `Tools` holds the hanger, dresser, stationer and fielder. A build's `Drawing` holds the sweeper and ringer. `readied` and `placed` take `(&Tools, &Drawing)`, so passing a field's tools alone fails to compile (E0308, checked with a scratch edit that was then reverted). `Generator::prepare_field_programs` is the field's preparation, and `web/field.rs` calls it. The renderer's `prepare_programs` / `preparePrograms` still makes every program. Its tests pass, with call sites updated to `kit.full(gpu)`.

- Baseline: green (`cargo test --profile ci -p telperion-render --lib field`, 14 passed, before any edit).
- Test: `field_tests::a_field_generator_holds_no_curve_pass_sweeper_or_ringer`.
- Focused tests: 23 passed on the final code (field, `a_build_makes_no_program`, growing, repeat, organ). The wasm target check and `cargo fmt --check` are clean.
- Answers: 24 of 24 `FIELD_ANSWERS` hashes equal task 1's (measured).
- Open item: in 3 of 14 all-preset runs of the final binary, `silver-birch 36` hashed differently, once with 48 as well. The base binary had no mismatch in 12 runs and the measured binary none in 5. Birch run alone was always equal. All three mismatches came within 5 minutes, and the 15 runs after them were equal. The field code path is the same in the measured and final binaries. The likely cause is latent ordering or timing nondeterminism in the device field, but that is inferred, not shown. See `FIELD-PROGRAMS.md`.
- Cold measurement (medians of 3 alternated runs, driver shader cache off, measured):
  - Preparation is unchanged within spread: oak 5765 ms before, 5880 ms after; Telperion 6076 ms before, 5994 ms after. Natively the modules are made on parallel threads, so the dropped programs were not on the critical path (inferred).
  - The first field is faster: oak 536 to 462 ms, Telperion 1207 to 1172 ms.
  - The browser gain (23 fewer programs compiled) was not measured.
- Follow-ups, blocked by Touches: the field still makes `mass` and `positions` in `Generator::create` and `tree::Kernels.present`, and dispatches none of them. Dropping them needs edits to `generation/compute.rs`, `positions.rs` and `cycle/tree.rs`.
- Evidence: `.flow/evidence/fn-319-the-release-audits-slice-lands-before/FIELD-PROGRAMS.md`. A friction entry was added for the birch flake.

stage: impl-review - skipped(policy: host-deferred - conductor owns the gate)

Tier: session model (model: claude-opus-5-5)

Integrated on the spec branch (b38980cc6, 78b1704cd). Review: codex, three lenses, SHIP, no findings (rid 2ce32c8e). Host re-ran the birch hashes: 15 + 15 clean runs on base and final binaries; the two mismatches came from a binary run while its crate was rebuilt (FIELD-PROGRAMS.md). mass, positions and present carried to fn-242.
stage: plan-sync - skipped(config: planSync.enabled=false)
## Evidence
- Commits: b38980cc6, 78b1704cd
- Tests: cargo test --profile ci -p telperion-render --lib -- field a_build_makes_no_program growing_tests repeat_tests organ_tests (23 passed), cargo check --release --target wasm32-unknown-unknown -p telperion-render, cargo fmt -p telperion-render -- --check, field_time_tests::field_answers --ignored (all presets; 24 hashes equal FLORETS.md; birch-36 intermittent, see FIELD-PROGRAMS.md), field_time_tests::field_cold --ignored (paired, 3 runs each, oak and telperion), host: t2-bin-base x15 and t2-bin-final x15 field_answers, all hashes equal
- PRs: