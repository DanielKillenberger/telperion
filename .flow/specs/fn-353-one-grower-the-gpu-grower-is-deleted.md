# One grower: the GPU grower is deleted

## Goal

Growth runs on one grower, on the CPU. The GPU grower, the device expansion it alone feeds, their bindings and the tests that hold the CPU to them are deleted, and every consumer reaches the screen through the path that is already production: `mesh::build` then `Renderer::submit`.

## Context

- Child of fn-351, Phase 1 (criterion P5). Owner: "we only want one" (1 Nov 2026); "i thought we'd remove the gpu grower?" (11 Nov 2026).
- The survey this spec rests on, with paths and line numbers: `.flow/evidence/fn-351-020-the-release-candidate/PHASE1-SURVEY.md`. Read it first.
- Work happens on branch `fn-353-one-grower` in worktree `.worktrees/fn-353`, because the scoreboard (fn-352) is measuring the integration branch's build. The host merges it.

## The host's design decisions (11 Nov 2026)

1. **All of `crates/telperion-render/src/generation/` goes** (about 51,500 of the render crate's 70,000 lines), not only the grower. The device expansion (`resident/`) reads one buffer only the grower writes; keeping it would mean writing a second path from a CPU tree to the screen beside `mesh::build`, which the principles forbid. Curve surfacing, selection, levels, foliage, wood, shadows and scene stay: they already draw an uploaded CPU tree.
2. **In the browser, Phase 1 grows on the main thread through the existing `setTree`.** A Worker is Phase 6's, with the wire it needs. (The plan's Phase 1 line said "in a worker"; the goal spec's Phase 6 line owns it.)
3. **`telperion/field` grows through the CPU now,** by `TreeEngine.build(family, { field })` in `telperion.wasm`, and so works without WebGPU. Its device-only errors and entries (`webgpu-unavailable`, `prepareField`, `compileField` where they exist only for the device) are removed, not stubbed. 0.2.0 is unreleased, so this is not a break of a shipped API; the migration note says what changed.
4. **Grow-in and play-the-year stay,** on `telperion_core::pipeline::growth` (`begin`, `grow_to`, `settle`, `colouring`, `mesh`) through a new Wasm binding with a test. Showing the years is a product feature (AGENTS.md).
5. **The page budget, admission and "makeRoom" go.** A tree that does not fit the device at upload is refused by name with the amount it needed (`submit::fits`), and nothing else caps it.
6. **The f32 decision contract and `Transcendental` stay unchanged.** Native and Wasm equality rests on them.
7. **`pipeline::executor` keeps only what production or a surviving test reads;** the device half goes. The `boundary.rs` tests are updated to hold the boundary that remains.
8. **The cost is stated, not hidden:** the harness's build per dial move goes from the device's 0.2 to 1.3 s to the CPU's full growth (the survey's table: oak 3.6 s, spruce 11.9 s in Wasm). No tree's own growth cost changes. That gap is what Phases 4 and 5 remove.

## Acceptance Criteria

- **R1 (the bindings):** CPU growth by year is bound for the browser (begin, grow to, settle, draw at) over `pipeline::growth`, with a test that grows one preset a year at a time and checks the tree at year y equals the tree asked for at age y.
- **R2 (the harness):** the harness builds, grows in and plays the year through the CPU path; no control or message refers to the device growing, admission or "makeRoom".
- **R3 (the field entry):** `telperion/field` returns its `FieldGrid` from the CPU with and without WebGPU; its tests, `tests/browser/field.mjs`, `scripts/test-dist.mjs` and the probe worker follow.
- **R4 (the native tools):** `examples/film.rs`, `film/bounds.rs`, `grow_in.rs` and `frames.rs` build through `pipeline::growth::mesh` or `mesh::build` and `submit`. Device-only tools and benchmarks are deleted.
- **R5 (the deletion):** `crates/telperion-render/src/generation/` and `generation.rs`, `src/web/generation.rs`, `growing.rs` (device form) and `field.rs` (device form), `submit_prepared` and the resident submit paths, `examples/gpucycle/`, and the device members of the browser renderer are gone. `grep -r "setTreeGpu\|Generator::prepare\|hier_sweep\|cycle::tree" crates src harness scripts tests` finds nothing.
- **R6 (the CPU side trimmed):** `grow/cycle/capture*.rs`, `agree.rs`, and whatever of `fine/tables.rs` and `pipeline::executor` nothing reads are gone. What the CPU grower itself reads stays, and the report says which was which.
- **R7 (paperwork):** `docs/pipeline.md`, `docs/field-package.md`, `docs/gpu-tests.md`, the README's sections, `docs/migration-0.2-parameters.md` and `package.json`'s description say what is true; `.config/nextest.toml`, `.config/gate-budgets.toml`, `scripts/artifact-budgets.json` follow, baselines set from measured sizes. `cargo fmt --all` is applied and `cargo fmt --all --check` is clean.
- **R8 (measured):** lines of Rust, WGSL and TypeScript before and after; the render and core Wasm sizes raw and Brotli before and after; the gate's test count and test-run seconds before and after; the harness's CPU build time for the oak and the palm, native.
- **R9 (green):** the two guards, the full gate (`--full`) from a clean tree, and `npm test`. A codex review of this spec's diff inside the queue.

## Boundaries

- No change to how a tree grows or is drawn on the CPU path: the still runner's oak at 40, seed 1001, has the same node and leaf counts before and after, and that is checked.
- No fallback, no feature flag, no stub left where the device grower was.
- No new Worker, no `OffscreenCanvas`, no reading of the grower's lattice: those are Phase 6.
- Nothing is pushed.

## Evidence

`RESULT.md` with R8's tables and what was kept and why; `FRICTION.md` as it happens.
