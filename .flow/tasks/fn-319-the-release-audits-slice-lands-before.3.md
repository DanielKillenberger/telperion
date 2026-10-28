---
satisfies: [R2]
---
# fn-319-the-release-audits-slice-lands-before.3 The device refuses what the CPU refuses

## Description
The device field and the device tree build call `Family::validate` and the counted instance budget, so `canopy.maxInstances=1` and `canopy.scatter=-1` (Ordinary, seed 1, n 12) are refused with the CPU's messages, and before the device is requested.

**Size:** S
**Files:** `crates/telperion-render/src/generation/resident/field/make.rs`, `generation/resident/build.rs`, `web/field.rs`, `generation/resident/field_tests.rs`, `docs/field-package.md`, `docs/pipeline.md`
**Touches:** [crates/telperion-render/src/generation/resident/**, crates/telperion-render/src/generation/resident.rs, crates/telperion-render/src/web/field.rs, crates/telperion-render/src/web/generation.rs, src/field/index.ts, src/field/field.test.ts, docs/field-package.md, docs/pipeline.md]

### Approach
- Write the failing test first beside `the_device_field_refuses_what_the_core_refuses` (`resident/field_tests.rs:286-312`), comparing with `pipeline::build`'s message for both inputs, through the field and through the device build.
- `Family::validate` (`crates/telperion-core/src/family.rs:90-134`) runs the canopy rows; the device never calls it (`executor.rs:142-148` returns early without leaf bases). Call it where the family enters the device paths (`web/field.rs` parses at `:101` before `fields()`; `web/generation.rs` for the build). Check it rejects no shipped preset.
- `growField` (`src/field/index.ts:139-145`) calls `device()` before it loads the module, so a Rust check alone never runs without WebGPU. Load the module first and call a synchronous wasm export on `WebField` that parses and validates the parameters (the same `params::parse` and `Family::validate`), then `device()`, then `grow`. The module loads without a GPU. Test in `src/field/field.test.ts` (Node has no WebGPU): an invalid family rejects with the family's message, a valid one with `webgpu-unavailable`.
- After `stationer.stations` in `make.rs:95`, apply `instances(device.stations, max_instances)` (`build.rs:124-129`) as `placed` does (`build.rs:225`). Check whether `maxInstances=1` is already refused by `validate`'s canopy row (`canopy/params.rs:540-545`) and which message the CPU gives first; match it.
- Update the refusal paragraphs in `docs/field-package.md` and `docs/pipeline.md:79`.

### Investigation targets
**Required:**
- `crates/telperion-core/src/family.rs:85-135`
- `crates/telperion-render/src/generation/resident/build.rs:120-230`
- `crates/telperion-render/src/generation/resident/field_tests.rs:286-312`
**Optional:**
- `crates/telperion-core/src/pipeline/foliage/canopy/params.rs:110-120,535-550`

### Acceptance
- [ ] Both inputs refused through the field and the device build with the CPU's message (ordinary tests)
- [ ] Without WebGPU, `growField` rejects an invalid family with its message and a valid one with `webgpu-unavailable` (Node test)
- [ ] A family's error is reported before a device is requested
- [ ] Every shipped preset still grows on the device

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
