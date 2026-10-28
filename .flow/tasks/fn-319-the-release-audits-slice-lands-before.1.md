---
satisfies: [R5]
---
# fn-319-the-release-audits-slice-lands-before.1 Florets are boxed and counted on the device

## Description
The field's flower channel and the florets' share of the box move onto the device; the floret list is no longer read back. First in order because it is the riskiest change (layout, limits, a new binding) and the R1 and R2 tasks measure against its answers.

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/field/make.rs`, `resident/field.rs`, `resident/field/field.wgsl`, `resident/field/splat.wgsl`, `resident/florets.rs`, `resident/field_tests.rs`, `resident/field_time_tests.rs`, `web/field.rs`, `docs/pipeline.md`, `docs/field-package.md`
**Touches:** [crates/telperion-render/src/generation/resident/**, crates/telperion-render/src/generation/resident.rs, crates/telperion-render/src/web/field.rs, docs/pipeline.md, docs/field-package.md, .flow/evidence/fn-319-the-release-audits-slice-lands-before/**]

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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
