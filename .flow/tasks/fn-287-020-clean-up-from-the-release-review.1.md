---
satisfies: [R1]
---
# fn-287-020-clean-up-from-the-release-review.1 Production creates only the position programme's emit pipeline; the oracle route is test-only

## Description
R1. The resident position programme's `admit`, `radii` and `finish` entries and the comparison route that runs them leave the production build; `emit`, which the resident build's ring seating runs, stays.

**Size:** M
**Files:** `crates/telperion-render/src/generation.rs`, `crates/telperion-render/src/generation/positions.rs`, `crates/telperion-render/src/generation/preparation.rs`, `crates/telperion-render/src/generation/sources.rs`, `crates/telperion-render/src/generation/wood.rs` (the oracle upload `upload_wood`/`UploadedWood` and helpers), possibly `crates/telperion-render/src/generation/io.rs`
**Touches:** [crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/positions.rs, crates/telperion-render/src/generation/preparation.rs, crates/telperion-render/src/generation/sources.rs, crates/telperion-render/src/generation/wood.rs, crates/telperion-render/src/generation/io.rs, crates/telperion-render/src/buffer.rs, crates/telperion-render/src/generation/compute.rs, crates/telperion-render/src/generation/*tests.rs, crates/telperion-render/src/generation/resident/*tests.rs]

### Approach
- `Generator::create` (generation.rs, the `positions` `io::Pass::new` with `["emit", "admit", "radii", "finish"]`): production names `emit` only; the three oracle entries are added only under `cfg(test)`, from one entry table (a const for the production entries and a `cfg(test)` extension), never a runtime switch and never a second constructor.
- `ring_vertices` (positions.rs) runs entry 0 and stays production. `begin_positions`, `complete_positions`, `emit_positions` and their helpers move under `cfg(test)`; so does `prepare_whole`'s `Delivery::Resident` branch (preparation.rs), leaving production `prepare_whole` the CPU delivery. Keep the oracle bytes identical: the comparison tests are the check.
- Anything that becomes unused in the non-test build only because the oracle left (helpers, uploads, `sources::positions` callers) follows it under `cfg(test)` or is removed; no `allow(dead_code)`.
- Proof test: a unit test (no GPU needed) that fails if the production entry table names anything but `emit`; compile proof: the non-test build of `telperion-render` succeeds natively and with `cargo build --profile ci --target wasm32-unknown-unknown -p telperion-render` (the browser renderer, built by `scripts/build-render.mjs`; `telperion-wasm` does not depend on the renderer) with the oracle absent.
- `position_*` metrics written only by the oracle branch stay as fields (out of scope; note them as a follow-up).

### Investigation targets
**Required**:
- `crates/telperion-render/src/generation.rs:150-230` - the `positions` field and `create`
- `crates/telperion-render/src/generation/positions.rs` - `emit_positions`, `begin_positions`, `complete_positions`, `ring_vertices`
- `crates/telperion-render/src/generation/preparation.rs:1-150` - `prepare_async`, `prepare_whole`'s resident branch
- `crates/telperion-render/src/generation/resident/build.rs:215-235` - the ring seating that runs `emit`
- `crates/telperion-render/src/generation/io.rs:100-185` - `Pass::new`, its `cfg(test)` `entries`
**Optional**:
- `crates/telperion-render/src/generation/resident/route_tests.rs`, `generation/lobed_tests.rs:27`, `generation/tests.rs:436` - the oracle's readers
- `crates/telperion-render/src/generation/standalone_tests.rs:27` - asserts `positions` is `None` for CPU-only generators
- `docs/gpu-tests.md` - GPU test conduct (tests skip without a GPU)

### Key context
- GPU tests return early without a device (`test_gpu`), so the entry-table test must not need one.
- fn-257 tasks 4-5 are still open on these files; cite symbols, not line numbers, in commits.
- Heavy runs only through `telperion-heavy`; the gate once at the end (`telperion-heavy 12 --threads 2`).
## Acceptance
- [ ] Production `Generator::create` builds the position pass with `emit` only; `admit`, `radii`, `finish` are named only under `cfg(test)`
- [ ] `begin_positions`, `complete_positions`, `emit_positions` and `prepare_whole`'s resident branch compile only under `cfg(test)`; `ring_vertices` and CPU delivery see the same pass as at the base (no new panic path; the pre-existing `for_cpu_output` + `begin_growth` reach of its `expect` is noted as a follow-up)
- [ ] A non-GPU test fails if the production entry table names any entry but `emit`
- [ ] `cargo build --profile ci -p telperion-render` and `cargo build --profile ci --target wasm32-unknown-unknown -p telperion-render` succeed with no new warning
- [ ] Route, lobed and generation comparison tests and the seated resident tests pass; workspace gate green
## Done summary
Production `Generator::create` makes the position pass from `positions::entries()`, which is `positions::ENTRIES = ["emit"]` alone; under `cfg(test)` `admit`, `radii`, `finish` are appended at indices 1-3. `begin_positions`, `complete_positions`, `emit_positions`, `UploadedWood`, `upload_wood` and their helpers are test-only; production `prepare_whole(family)` is the CPU delivery and the old resident branch is the test-only `prepare_whole_resident` (sharing `on_cpu`, `assemble`). `buffer::Lent` is removed (gating its `Borrowed` variant alone fails E0392); `compute_buffer_async` takes any `Deref<Target = wgpu::Buffer>` rings and the lent bytes explicitly, the values the enum gave. Test: `generation::tests::production_makes_only_the_position_emit_entry` (no GPU).

Evidence: workspace gate once at 787682963, `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, 2026-10-16T06:26:41+02:00 to 07:00:25+02:00 (2,024 s): 1,407 passed, 0 failed, 103 ignored. Native and wasm32 `cargo build --profile ci -p telperion-render` warning sets identical to the base. Baseline: green (generation filter 115/0/39 before, 116/0/39 after).

Follow-ups: `position_*` metrics are now written only by the test oracle (out of scope per spec); the pre-existing `ring_vertices` `expect` reachable through `for_cpu_output` + `begin_growth`/`settle`; `assemble` takes seven parameters (reviewer FYI). Touches widened by the conductor to `buffer.rs` and `generation/compute.rs`.

stage: impl-review - ran (codex, three draws: correctness, contracts, integration; SHIP, 0 findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 20c0f41793f82370ae3ee1003aa0d52358c4cb8c, 51e6a00cd08727643d5abe9498d0669992e4204e, 78768296357427da83f2286001a2f796bf1dd3b2
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1407 passed, 0 failed, 103 ignored), cargo build --profile ci -p telperion-render; cargo build --profile ci --target wasm32-unknown-unknown -p telperion-render (warning-neutral)
- PRs: