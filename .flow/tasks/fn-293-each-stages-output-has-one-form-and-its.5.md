---
satisfies: [R2, R3]
---
# fn-293-each-stages-output-has-one-form-and-its.5 The CPU delivery route and test-only GPU code leave the renderer's production build

## Description
Findings 6, 10 and 17: `Delivery::Cpu` with `prepare_whole`, `on_cpu`'s `CpuFallback` and `for_cpu_output*`, the core's `foliage/prepared.rs` it uses, the test-only WGSL entries concatenated into production modules, and production-compiled items only tests read.

**Size:** M
**Files:** `crates/telperion-render/src/generation.rs`, `generation/{preparation,positions,wood,sources}.rs`, `generation/positions.wgsl`, `generation/wood_geometry.wgsl`, `generation/cycle/*` (the dead-code sites), `generation/resident/{boots,build}.rs`, `generation/{standalone_tests,route_tests}.rs`, `resident/agree_tests.rs`, `examples/{generation_digest,generation_gpu,resident_stills}.rs`, `crates/telperion-core/src/pipeline/foliage/prepared.rs`, `pipeline/contract.rs`, `pipeline/executor.rs`
**Touches:** [crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/preparation.rs, crates/telperion-render/src/generation/positions*, crates/telperion-render/src/generation/wood*, crates/telperion-render/src/generation/sources.rs, crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/resident/boots.rs, crates/telperion-render/src/generation/resident/build.rs, crates/telperion-render/src/generation/*_tests.rs, crates/telperion-render/src/generation/resident/*_tests.rs, crates/telperion-render/src/programs.rs, crates/telperion-render/src/buffer.rs, crates/telperion-render/src/resident.rs, crates/telperion-render/examples/**, crates/telperion-render/src/web/**, crates/telperion-core/src/pipeline/foliage/prepared.rs, crates/telperion-core/src/pipeline/contract.rs, crates/telperion-core/src/pipeline/executor.rs]

### Approach
- Finding 6: confirm no production reader (`web/generation.rs:91` passes `Delivery::Resident` alone; `Generator::for_cpu_output*` at `generation.rs:186-195` has example and test callers only). Delete `Delivery::Cpu`, `prepare_whole`, `on_cpu`, `Backend::CpuFallback`, `for_cpu_output*`, `read_instances_async` and `io::read_leaves_*` from the production build; where a test needs a CPU-grown tree on the device, it reads the CPU cycle's expansion directly (`executor::grow` then `Grown::expansion`), not a delivery. Examples that compare deliveries (`generation_digest`'s `gpuCpu`, `generation_gpu`, `resident_stills`) drop the CPU delivery; `generation_digest` keeps the pipeline's CPU artifacts and the resident output. Core's `foliage/prepared.rs` (re-exported at `contract.rs:47-50`, used at `executor.rs:18,315,352`) follows its last reader: delete it or `cfg(test)`. `plan::runs`, `segment_stations`, `leaves_between` stay (the plan reads them).
- Finding 10: `sources.rs:29-35` concatenates `positions.wgsl` (with `admit`, `radii`, `finish`) and `wood_geometry.wgsl` into the production module; split so the production module holds `emit` and what it calls, and the test entries are added only under `cfg(test)`. Test-only Rust (`preparation.rs:157-346`, `positions.rs:28-297`, `wood.rs`) moves into test modules.
- Finding 17: the `cfg_attr(not(test), allow(dead_code))` sites in `cycle/{build_grow,census,growth,hier_build,hier_device,settle_device,settle_run}.rs`, `resident/{boots,build}.rs`, `programs.rs:166`, `buffer.rs:260`, `resident.rs:199`: each item `cfg(test)` or deleted; fix `cycle/state.rs:1`'s stale "test-only" doc.
- `generation_digest` must still print comparable lines: record that the `gpuCpu` field is gone and compare the remaining fields to the baseline.

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

### Acceptance
- [ ] No `Delivery::Cpu`, `CpuFallback` or CPU-delivery preparation in the renderer's non-test build; no test-only WGSL entry in a production module.
- [ ] The agreement and route tests still compare the device against the CPU cycle.
- [ ] Render digests (resident) and core digests equal the baseline.
- [ ] Gate and `npm test` green.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
