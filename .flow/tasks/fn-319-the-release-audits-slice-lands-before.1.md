---
satisfies: [R5]
---
# fn-319-the-release-audits-slice-lands-before.1 Florets are boxed and counted on the device

## Description
The field's flower channel and the florets' share of the box move onto the device; the floret list is no longer read back. First in order because it is the riskiest change (layout, limits, a new binding) and the R1 and R2 tasks measure against its answers.

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/field/make.rs`, `resident/field.rs`, `resident/field/field.wgsl`, `resident/field/splat.wgsl`, `resident/florets.rs`, `resident/field_tests.rs`, `resident/field_time_tests.rs`, `web/field.rs`, `docs/pipeline.md`, `docs/field-package.md`
**Touches:** [crates/telperion-render/src/generation/resident/**, crates/telperion-render/src/generation/resident.rs, crates/telperion-render/src/web/field.rs, docs/pipeline.md, docs/field-package.md]

### Approach
- Before changing anything, extend `field_answers` (`resident/field_time_tests.rs:42-72`) to hash the flowers word and record the baseline hashes and `field_time` for every shipped preset in `.flow/evidence/fn-319-*/` (also R1's baseline).
- Keep the `Placed` buffer `Organ::place` returns (`resident/florets.rs:110-149`) on the device and bind it to the field pass; `Rec::run` (`resident/field.rs:160-185`) binds six buffers today, so a seventh changes `Pass::new` and every `bufs` array in `make.rs` together.
- Add an entry that folds the non-withered seats' min/max into the box words beside `BOX_WOOD`/`BOX_LEAVES` (`make.rs:152-155`), dispatched before the box readback (`make.rs:169`); seats are the florets `core_field::seats` keeps (`crates/telperion-core/src/pipeline/field.rs:152-195`, the withered bit in `packed.rs:28`). Placement must be submitted before the box entries. No seat leaves the box as it is.
- Add an entry after `SPLAT_LEAFLETS` that adds 1 per seat into a per-cell flowers word with an integer atomic, binning with the same rule `florets_into` uses; a seat outside the grid sets a flag in the answers tail beside the table-overflow flag (`tail[0]`), and `make.rs` turns it into the CPU's "a floret outside the field" error.
- The answers grow a fifth word per cell: `answer_words`, `within(gpu, "field answers", ...)`, `read_words`, `split_at`, the `answer()` chunking (`make.rs:263-272`) and the WGSL stride (`splat.wgsl:25`, `:367`) change together. Read the grid's 7-words-per-cell sizing (`make.rs:62`) against `cell_word`'s stride of 4 before touching it.
- Remove the CPU `seat_bounds` merge (`make.rs:178-187`) and `florets_into` call (`make.rs:228-230`) from the device path; `Organ::read` no longer copies florets for the field.
- Correct the doc comments that say only the grid is read back only if they become untrue; after this task they are true (`generation/resident.rs:69-73`, `resident/field.rs:1-12`, `docs/pipeline.md:80`, `docs/field-package.md:9`) and state the flower channel's agreement bound.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/resident/field/make.rs:54-272`
- `crates/telperion-render/src/generation/resident/florets.rs:100-149`
- `crates/telperion-render/src/generation/resident/field/splat.wgsl`
- `crates/telperion-render/src/generation/resident/field.rs:20-185`
- `crates/telperion-render/src/generation/resident/field_tests.rs:286-510`
**Optional:**
- `crates/telperion-render/src/generation/resident/place.wgsl:57-100` (the crown bounds already folded, which include withered rachis instances)

### Key context
The device's frame is f32 and the CPU's binning f64, so a floret on a cell edge can land one cell over: the bound is `flowers_agree`'s (`FLOWERS_TOTAL`, `FLOWERS_MOVED`), recorded with the measured share. Counts are integers, so the channel is order-independent. n = 256 is the limit case for the larger answers buffer.

### Acceptance
- [ ] Baseline hashes (with flowers) and times for every shipped preset recorded before the change
- [ ] `the_device_field_counts_the_flowers_the_cpu_counts` and `flowers_agree` pass; the moved share recorded
- [ ] Every non-flowering preset's answers byte-equal to the baseline
- [ ] A floret outside the field is refused by name; a flowering tree with no seats equals one without flowers
- [ ] A test on both sides of the answers limit at the largest side
- [ ] No floret readback in the field path; time before and after recorded

## Acceptance
- [ ] TBD

## Done summary
A flowering tree's field now reads back only the grid's answers. The placed florets stay on the device and are bound to the field pass as a seventh buffer. `box_seats` folds the seats that are not withered into the box words, and `splat_flowers` adds 1 per seat into a fifth per-cell word with an integer atomic. A seat outside the grid, and the placement's refusal word, go into the answers tail. `make.rs` refuses them by name: "a floret outside the field" and "GPU floret transform/reference". `Organ::read` is deleted, and the CPU `seat_bounds` and `florets_into` calls are gone from the device path.

- Baseline: green, measured before any edit: `cargo test --profile ci -p telperion-render --lib field` (11 passed). `field_answers`, extended first to hash the flowers word, was run on all eight presets, along with `field_time`.
- Answers: all 24 `FIELD_ANSWERS` hashes are byte-equal before and after, the flowering trees included (measured).
- Flowers: the most moved share is 4.01e-4 (Laurelin 5678 at 48), under `FLOWERS_MOVED` 1e-3. Totals match the CPU's except Telperion 5678, which is 6.35e-5 off, under `FLOWERS_TOTAL` 2e-4. Both are the same as fn-316 measured (measured).
- Time: medians of three alternating runs, base binary against the new one. Telperion went from 919.65 to 905.45 ms, Laurelin from 891.74 to 856.73 ms, and the oak is unchanged (252.08 against 252.13 ms). The gain sits within run-to-run spread.
- New tests in `resident/field_flowers_tests.rs`:
  - `a_floret_outside_the_field_is_refused_by_name`
  - `a_flowering_tree_with_no_seats_answers_as_one_without_flowers`
  - `the_largest_side_grows_or_refuses_as_the_device_s_limits_say`
- Limits at side 256: the grid (`2 + 8*cells` words) is always larger than the answers (`5*cells + 4` words), so the grid's limit is the one that binds. The test's device one word short refuses "field grid" by name, and a device at the exact size grows the field. No device limit can refuse the answers while the grid still fits.
- Evidence: `.flow/evidence/fn-319-the-release-audits-slice-lands-before/FLORETS.md`.
- Docs: added the flower channel and its agreement bound to `docs/pipeline.md`, `docs/field-package.md`, and the doc comments in `generation/resident.rs` and `resident/field.rs`. `web/field.rs` needed no change.

stage: impl-review - skipped(policy: host-deferred - conductor owns the gate)

Tier: session model (model: claude-opus-5-5)

Integrated as c790849b86bdd8ab1eee28e6c37220c531e1fc48 on the spec branch. Review: codex, three lenses, SHIP, no findings (rid 7d461173).
stage: plan-sync - skipped(config: planSync.enabled=false)
## Evidence
- Commits: c790849b86bdd8ab1eee28e6c37220c531e1fc48
- Tests: cargo test --profile ci -p telperion-render --lib -- field shader_tests, cargo test --profile ci -p telperion-render --lib -- --include-ignored --nocapture field_flowers_tests the_device_field_counts_the_flowers, FIELD_PRESETS=<all 8> cargo test --profile ci -p telperion-render --lib -- --ignored --nocapture resident::field_time_tests (before and after), cargo clippy --profile ci -p telperion-render --tests, cargo fmt -p telperion-render -- --check
- PRs: