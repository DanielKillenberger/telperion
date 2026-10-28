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
The device field and the device tree build now refuse a family the CPU's build refuses, with the CPU's message. They do it through `telperion_core::pipeline::executor::checked`, which is not a copy of the CPU's checks.

`checked` first runs `growth(family)`, the shared growth conversion, which includes the whole-year age check in `space::asked`. It then runs Stage 3's checks that need no tree, now factored into `executor::planned`: the leaf element and its own check, the twig placement, the shell depth, then `canopy_rows`. `prepared` calls the same `planned`, so both executors' Stage 3 now refuses an invalid canopy row at the point the CPU reaches it. The CPU's later `canopy::validate` in station preparation stays.

The calls sit in three places. `Generator::field` and `Generator::prepare_resident` call `checked` before any device work. On the wasm side, `WebField.check`, `WebField.grow` and `setTreeGpu` call it through the `checked` helper in `web/field.rs`. `growField` loads the module and calls `WebField.check` before `device()`. The field also counts its stations against `canopy.maxInstances` with `build::instances`, as the build does.

Fix pass (codex NEEDS_WORK): the first pass used `Family::validate`, which still ran old scaffold checks and missed the age check. All three of its call sites now use `executor::checked`.

- Rust, measured: `field_tests::the_device_refuses_a_family_the_cpu_refuses`. The cases are Ordinary at seed 1, n 12:
  - `maxInstances=1`, `scatter=-1`, `age=1.5`, and `age=1.5` with `scatter=-1`. Each is refused through `grow_field` and `gen.prepare` with a message containing `pipeline::build`'s, and the combined case gives the age message.
  - `maxNodes=0` gets the same outcome from `checked` as from the CPU build: the CPU grows it, and `checked` does not refuse it.
  - Every catalogue preset passes `checked`.
- Node, measured: in `src/field/field.test.ts`, without WebGPU, `scatter=-1`, `age=1.5` and both together reject with the core's error, not `webgpu-unavailable`. The CPU entry gives `Tree core: <reason>`; the field gives `the generator rejected the tree: invalid input: <reason>`. That prefix difference predates this task, and the test pins both texts.
- Accepted trade (host, 28 Oct 2026): a family invalid both in a way only growth finds (such as the engine's phytomer budget) and in a canopy row is refused on the device by the canopy row, before it grows. The CPU grows first and names the growth's error. This is recorded in `docs/field-package.md` and `docs/pipeline.md`.
- `maxInstances=1` is refused only once the tree has grown, because the budget counts the grown stations. The CPU also refuses it only after placing the leaves.
- Outside the declared Touches:
  - `crates/telperion-core/src/pipeline/executor.rs`, as the host's design asked.
  - From the first pass, a one-line `check` stub in the `WebField` mock in `src/field/device.test.ts`. Without it, five existing tests broke.
- The first `field.test.ts` case now catches its three refusals together. A growth now awaits the module load before it refuses.
- fn-320 flake, seen: in the final field run, `every_preset_grows_its_field_from_its_parameters_as_from_its_id` failed on silver-birch 5678 ("the parameters grew another field"). It passed in the first pass's run. Per the host rule it was not chased.
- Friction entry added in the first commit.

baseline: green (`cargo test --profile ci -p telperion-render --lib the_device_field_refuses_what_the_core_refuses`; `npx vitest run src/field`, 21 passed, after building the generated modules)
Gates:
- `cargo test --profile ci -p telperion-render --lib refuse`: 27 passed.
- `cargo test --profile ci -p telperion-core -- executor canopy refuse scatter`: 48 passed.
- `cargo test --profile ci -p telperion-render field`: 16 passed, 1 failed (the fn-320 flake), 9 ignored.
- vitest Quick files: 37 passed.
- `tsc --noEmit`: clean.
- `cargo fmt --check`: clean.
- `render:build` (wasm32): built.

stage: impl-review - skipped(policy: parallel-wave - conductor owns the gate)

Tier: session model (model: claude-opus-5-5)

Integrated on the spec branch (94f56ca8e, 40cefe7a9). Review: codex three lenses NEEDS_WORK (Family::validate is not the CPU build's check: fractional age order, maxNodes=0), host-designed fix (executor::checked over the shared Stage 3 checks), one-reviewer re-review SHIP. Known fn-320 flake seen once in the field suite, not this task's.
stage: plan-sync - skipped(config: planSync.enabled=false)
## Evidence
- Commits: 94f56ca8e6d1c75add98274778d1ebe78d8f9073, 40cefe7a9c47ae423361f6d07457ae6804a1783a
- Tests: cargo test --profile ci -p telperion-render --lib -j 2 refuse (via telperion-heavy; 27 passed), cargo test --profile ci -p telperion-core -j 2 -- executor canopy refuse scatter (48 passed), cargo test --profile ci -p telperion-render -j 2 field (16 passed, 1 failed: every_preset_grows_its_field_from_its_parameters_as_from_its_id, silver-birch 5678, the known fn-320 flake; passed on the first-pass run), npx vitest run src/field src/browser/render.test.ts harness/grow-in.test.ts (37 passed), npx tsc --noEmit -p tsconfig.json, cargo fmt -p telperion-render -p telperion-core -- --check
- PRs: