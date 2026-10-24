---
satisfies: [R2, R3]
---
# fn-293-each-stages-output-has-one-form-and-its.5 The CPU delivery route and test-only GPU code leave the renderer's production build

## Description
Findings 6, 10 and 17: `Delivery::Cpu` with `prepare_whole`, `on_cpu`'s `CpuFallback` and `for_cpu_output*`, the core's `foliage/prepared.rs` it uses, the test-only WGSL entries concatenated into production modules, and production-compiled items only tests read.

**Size:** M
**Files:** `crates/telperion-render/src/generation.rs`, `generation/{preparation,positions,wood,sources}.rs`, `generation/positions.wgsl`, `generation/wood_geometry.wgsl`, `generation/cycle/*` (the dead-code sites), `generation/resident/{boots,build}.rs`, `generation/{standalone_tests,route_tests}.rs`, `resident/agree_tests.rs`, `examples/{generation_digest,generation_gpu,resident_stills}.rs`, `crates/telperion-core/src/pipeline/foliage/prepared.rs`, `pipeline/contract.rs`, `pipeline/executor.rs`
**Touches:** [crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/preparation.rs, crates/telperion-render/src/generation/positions*, crates/telperion-render/src/generation/wood*, crates/telperion-render/src/generation/sources.rs, crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/resident/boots.rs, crates/telperion-render/src/generation/resident/build.rs, crates/telperion-render/src/generation/*_tests.rs, crates/telperion-render/src/generation/resident/*_tests.rs, crates/telperion-render/src/programs.rs, crates/telperion-render/src/buffer.rs, crates/telperion-render/src/resident.rs, crates/telperion-render/examples/**, crates/telperion-render/src/web/**, crates/telperion-core/src/pipeline/foliage/prepared.rs, crates/telperion-core/src/pipeline/contract.rs, crates/telperion-core/src/pipeline/executor.rs, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- Finding 6: confirm no production reader (`web/generation.rs:91` passes `Delivery::Resident` alone; `Generator::for_cpu_output*` at `generation.rs:186-195` has example and test callers only). Delete `Delivery::Cpu`, `prepare_whole`, `on_cpu`, `Backend::CpuFallback`, `for_cpu_output*`, `read_instances_async` and `io::read_leaves_*` from the production build; where a test needs a CPU-grown tree on the device, it reads the CPU cycle's expansion directly (`executor::grow` then `Grown::expansion`), not a delivery. Examples that compare deliveries (`generation_digest`'s `gpuCpu` and `gpuResident`, `generation_gpu`, `resident_stills`) drop the delivery they can no longer reach from the non-test library; the resident output is the digest test's (task 1). Core's station preparation (`foliage/prepared.rs`, reached through the executor interface's `compact_stations`, `shared_stations`, `prepared_with_contacts`, `executor.rs:334-375`) stays public: the renderer's agreement tests (`resident/agree_tests.rs`) read it across the crate boundary as their CPU reference, where a dependency's `cfg(test)` items do not exist, and `docs/pipeline.md` lists it in the executor interface. Its reason is named in the summary. Readback the resident digest test reads (`read_instances_async`, `io::read_leaves_*`) moves into the renderer's test modules with it (task 1 moved the digest into the test binary). `plan::runs`, `segment_stations`, `leaves_between` stay (the plan reads them).
- Finding 10: `sources.rs:29-35` concatenates `positions.wgsl` (with `admit`, `radii`, `finish`) and `wood_geometry.wgsl` into the production module; split so the production module holds `emit` and what it calls, and the test entries are added only under `cfg(test)`. Test-only Rust (`preparation.rs:157-346`, `positions.rs:28-297`, `wood.rs`) moves into test modules.
- Finding 17: the `cfg_attr(not(test), allow(dead_code))` sites in `cycle/{build_grow,census,growth,hier_build,hier_device,settle_device,settle_run}.rs`, `resident/{boots,build}.rs`, `programs.rs:166`, `buffer.rs:260`, `resident.rs:199`: each item `cfg(test)` or deleted; fix `cycle/state.rs:1`'s stale "test-only" doc.
- The digests must still compare: the core and the example's pipeline lines against the baseline, and the resident digest test's lines against the baseline's.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/preparation.rs`
- `crates/telperion-render/src/generation.rs:40-60,150-270`
- `crates/telperion-render/src/generation/sources.rs`
- `crates/telperion-render/src/generation/route_tests.rs:40-60,300-320`
**Optional:**
- `docs/pipeline.md` "The one cycle and its executors"

### Key context
GPU tests run natively here; never run WebGPU in a browser. The agreement tests' CPU reference is the CPU cycle (fn-237: not touched); only the render-side delivery that assembled it goes.
## Acceptance
- [ ] No `Delivery::Cpu`, `CpuFallback` or CPU-delivery preparation in the renderer's non-test build; no test-only WGSL entry in a production module.
- [ ] The agreement and route tests still compare the device against the CPU cycle.
- [ ] Render digests (resident) and core digests equal the baseline.
- [ ] Gate and `npm test` green.
## Done summary
The renderer's production build no longer has the CPU delivery. `Delivery::Cpu`, `prepare_whole`, `on_cpu`, `Backend::{Cpu, CpuFallback}`, `for_cpu_output*`, `cpu_mesh`, the standalone generator (`resident_allowed`) and its test are deleted, and `prepare_async` builds resident only. The comparison route the tests use (`prepare_whole_resident`), with its placement (`compute_async`, `compute_buffer_async`, `place_leaves`, `place_into`, `mass`, `empty`), its readback (`read_instances`, `io::read_leaves_*`) and its ring-position admission, now compiles only into tests: `preparation.rs` and `wood.rs` are `cfg(test)` modules. `positions.wgsl` holds only `emit`. `admit`, `radii`, `finish` and `wood_geometry.wgsl` are added under `cfg(test)` from the new `positions_admit.wgsl`, and `production_makes_only_the_position_emit_entry` parses the production module and checks that `emit` is its one entry.

- Finding 17: every `cfg_attr(allow(dead_code))` site is gone. Items read only by tests are now `cfg(test)`: `Compact::Every` (with a `due` method), `Start::Sizes`, `REUSED`/`Reused`, the step fields of `Hierarchy` and `Sums`, `Table.units`, `Settled.freed`, `Hung.bases/stems` and `Built.device`. Items read by tests and the browser are `cfg(any(test, wasm32))`: `programs::settle` and seven items in `budget.rs`. `Ledger::live` has native production readers, so its allow was stale and is removed. `cycle/state.rs` no longer calls itself test-only.
- Agreement and route tests still compare the device with the CPU cycle (`executor::grow` then `expansion`, or `prepare_whole_resident`). In `a_flowering_tree_draws_every_floret_on_either_path`, the CPU-delivery comparison is removed.
- Core station preparation (`foliage/prepared.rs` and the executor's `compact_stations`, `shared_stations`, `prepared_with_contacts`) stays public and unchanged. The renderer's tests read it across the crate boundary, where `cfg(test)` items of a dependency do not exist, and `docs/pipeline.md` lists it in the executor interface.
- Examples: `generation_gpu` loses `gpu-output`, and in device mode `GENERATION_VERIFY` no longer hashes leaves (the resident digest test covers them). `resident_stills` loses `STILLS_CPU`.
- Digests equal the baseline (BASELINE.md "After task 5"): core (24 lines), render pipeline (byte-identical, `fb4ce911...`) and resident (24 identical lines).
- Outside Touches, edited because the deletion left them dead: `generation/compute.rs` (the test-only placement grouped under `cfg(test)`), `generation/data.rs` (`segments` and its key helpers), `generation/io.rs` (`read_leaves_*`, `reserve_leaves`) and `generation/resident/budget.rs` (the finding-17 site at `resident.rs`).
- Gate: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` gave 1401 passed, 0 failed and 111 ignored. Task 4 had 1403; two CPU-delivery tests were deleted. The first run failed to link because `target/ci` was emptied outside this worker (FRICTION.md), so it was re-run. `npm test`: 170 passed. Baseline: green (`cargo check --profile ci -p telperion-render --all-targets` and the wasm32 lib).
- Follow-ups:
  - `docs/pipeline.md:64` still names `Generator::prepare_whole` as a CPU delivery (task 7, docs).
  - `src/browser/render.ts` still types `woodBackend` as `"Cpu" | "Gpu" | "CpuFallback"`, and `scripts/benchmarks/generation-gpu-smoke.mjs` asserts a `CpuFallback` backend.
  - `resident/growing.rs`'s `DRAW_MARGIN` and `expected_draw` are dead in native non-test builds, as they were on the base.
- OPEN_ITEM: `Delivery` and `Backend` now each have one variant (`Resident`, `Gpu`). They are kept for the `prepare` signature and the browser's stage report (`"backend":"Gpu"`). Should both be deleted, and the report's `backend` and `woodBackend` fields with them?

Tier: not reported (model: claude-opus-5-5)

stage: impl-review - ran (codex, three draws: correctness, contracts and integration, all SHIP with no findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 425241fbdfaea31abc1b907c01f1d8d36dae9dc7, 29a56bf488935baa9a16e16f6e43e06c6be9a3c0, bf9a105c5494177526ccddb2c8178fd5be54c995
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test, DIGEST_SEEDS=1,7,42 cargo test --profile ci -p telperion-render --lib generation::resident::digest_tests::digest -- --exact --ignored --nocapture
- PRs: