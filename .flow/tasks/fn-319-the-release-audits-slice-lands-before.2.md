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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
