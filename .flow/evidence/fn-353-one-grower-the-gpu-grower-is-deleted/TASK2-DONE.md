Task 2 deleted the device grower, its expansion and their bindings, moved the native tools onto `pipeline::growth` and `mesh::build`, and trimmed the CPU code that existed only for the device (R4 to R6). The still runner's oak at 40, seed 1001, is unchanged: 272,095 nodes, 47,279 leaves, 6,009,038 wood vertices, before and after.

Tier: product code changes, mostly deletion. Narrow tests in this task; the full gate, the format pass and the review come at task 3 (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; the host reviews the spec's diff after task 3)

## Commits

- `710a8e2ab` (R4): `film`, `film/bounds` and `grow_in` grow through `pipeline::growth` (`begin`, `grow_to`, `mesh`, `colouring`) and put the tree up with `Renderer::submit`. `frames` builds with `mesh::build` and `submit`. Deleted: `resident_stills`, `prepare_timing`, `generation_gpu`, `examples/gpucycle/`, `generation-gpu-smoke.mjs`, the device half of `mature-generation.mjs`, and the device probe page (`probe.html`, `src/probe/{main,gpu,measure,kernels}.ts`). The probe imported the gpucycle WGSL; the field probe stays.
- `9f7643e81` (R5): `src/generation/`, `generation.rs`, `web/generation.rs`, `web/field.rs`, `submit_prepared` and the resident submit paths in wood, foliage, select, curve and buffer. The device budget goes with them: the ledger keeps only the deferred destroy behind open encoders, and `free_device_memory` and the `ash` dependency are gone. `render.ts` loses `setTreeGpu`, `admission`, `GpuSubmitted` and `GenerationStages`. `shader_tests` and `compile_size` read the drawing sources only. The core's 32-bit wood refusals now say "build it with fewer radialSegments". `nextest.toml` and `gate-budgets.toml` drop the deleted tests. The grep `setTreeGpu|Generator::prepare|hier_sweep|cycle::tree|submit_prepared` over `crates src harness scripts tests` finds nothing.
- `afb879895` (R6): see "Kept and why" below.

## Lines (tracked files under crates, src and harness; measured)

| | Base `0daf5e02d` | Head `afb879895` |
|---|--:|--:|
| Rust | 202,944 | 164,249 |
| WGSL | 23,756 | 3,657 |
| TypeScript | 12,413 | 11,863 |
| TSX | 766 | 766 |

The render crate's `src` went from 69,936 lines of Rust and WGSL to 17,480. The diff is 329 files, 287 insertions and 59,768 deletions.

## Kept and why

- **`preparePrograms` (Rust binding) and the `programs` member of the browser `Renderer`: kept, against the task's list.** The deleted `Generator::prepare_programs` also compiled the wood's surfacing passes (`curve::passes`) and settled them through `programs::ready`. In a browser those compute pipelines are filled only when `ready` is awaited, so without it the CPU tree's wood would never draw. The binding now compiles only the curve passes. `programs` still rejects by name when the browser refuses one. **The host should decide whether `programs` stays public or the compile becomes internal to `createRenderer`.**
- **`grow_capture_light_at` and the `Capture` type** (`grow/cycle/capture.rs`, trimmed to the type, and `capture_light.rs`): kept because `telperion-core/tests/light_rows.rs` (fn-264 R2a) reads the buds' light through them. All the other captures are deleted.
- **The device tables were never read by the CPU grower.** `fine/tables.rs` (`Units`, `UnitZone`), `fine/surface.rs` (`ChainTables`, `PaRow`) and `grow/alone.rs` were read only by the device and the deleted captures, so all three are gone. `closed_form` has its own `Units` (`closed_form/mass.rs`), and core's `PaRow` is its own.
- **`agree.rs` is deleted.** The two geometry tests bounded scales by `agree::PATHS.scale` (6e-4). They now read the same number as `geometry::SCALE_APART`, with its fn-254 measurement. The ignored `agreement_with_todays_tree` measurement went with it.
- **Executor:** keeps `grow`, `expand`, `element`, `growth` (now `species`, `request` and `year()`) and the expansion's `tree`, `element`, `reference`, `wood_box`, `stations`, `curve`, `wood` and `mesh`, each read by a surviving test, example or the still runner. Deleted: `resident`, `Summaries`, `checked`, `bases`, `stations` (fn), `reach_terms`, `ConvertRow`, the folds re-exports and the rest of `Expansion`. With them went the compact and prepared surfaces (`surface/compact.rs`, all of `surface/prepared.rs` but `Run`), the shared and compact stations, `Reach::terms`, `Establishing::words`, the suite file `surface_prepared.rs` (six tests of those forms) and `flowers::both_executors_draw_the_same_flowers`. `docs/generation-limits-inventory.json` loses `compact.rs`'s two sites, which the limit guard requires.
- **The boundary tests** (`tests/boundary.rs`) hold unchanged: the interface builds through `grow().expansion().wood()`, and a stage called around it still fails to compile.
- **`telperion-render`** takes `telperion-space` as a dev-dependency only (the examples read it). `scripts/ci-key.test.mjs` counts dev-dependencies, so it needs no edit.

## Green (measured)

- `telperion-heavy 12 --gpu 2 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-render --lib --tests --test-threads 2` after R5: 217 passed, 14 skipped.
- `telperion-heavy 12 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --test-threads 4` after R6: 766 of 767 passed in 1,084 s. The one failure was `generation_limits_are_classified`, from the two stale inventory sites; after they were removed, `cargo test --profile ci -p telperion-core --test generation_limit_guard` passed (2 tests). `cargo test --profile ci -p telperion-render --lib compile_size` also passed (2 tests).
- After R6: `cargo nextest run --cargo-profile ci -p telperion-render --test boundary --test curve --test submit`, 12 passed. `node scripts/build-wasm.mjs && node scripts/build-render.mjs` built both modules.
- After R5: `npx tsc --noEmit` was clean. `npx vitest run src/browser src/field harness src/probe` passed 18 files and 142 tests. `node scripts/build-wasm.mjs && node scripts/build-render.mjs && npx tsc -p tsconfig.build.json && npx vite build && node scripts/test-dist.mjs` passed: 24 cases, and the field grew without WebGPU.
- `cargo check --profile ci --workspace --lib --tests --examples`, the wasm32 check of `telperion-render` and `telperion-wasm`, and `cargo test --no-run --lib -p telperion-core --no-default-features` all compiled with no new warning.
- The film tests (`film_placing`, `film_map`, `film_path`) passed after R4: 7 tests.
- The still runner, `target/ci/examples/space_oak <dir> 40 --seeds 1001 --shots none`: 272,095 nodes and 47,279 leaves, the same as `BEFORE.md`.

## Red or unverified

- Nothing is red.
- Warnings that predate this task (not caused by it): `occupancy_audit` has an unused `math` import. With no default features, `input.rs` has an unused `BloomParams` import and `branching.rs` an unused `room` import. On wasm32, `chunks.rs`'s `extent` is unused and `telperion-wasm/src/live.rs:49` has an unneeded `unsafe` block.
- Not run, because they need a GPU window, a browser or a video encoder: `film`, `grow_in` and `frames` were built and type-checked but not run end to end, and `preparePrograms` has not been driven in a browser.

## For task 3

- Docs still describe the device: `docs/pipeline.md`, `README.md`, `docs/migration-0.2-parameters.md` and `docs/field-package.md` (R7). Also `scripts/benchmarks/generation.md`, whose GPU sections from "Bounded GPU CPU-output readback" on were left; only the opt-in paragraph and "Resident wood expansion" were removed.
- The `lobes` catalogue row's note in `pipeline/surface.rs` says "the GPU executor draws only unlobed wood itself". It feeds the generated parameter docs, so changing it moves the wire tests and the generated tables together.
- About 25 doc comments in `telperion-core` and `telperion-space` that this task did not touch still name the device or "both executors" as history, for example `girth.rs:112` and `pipeline/space.rs:42,235`.
- `cargo fmt` was not run. The rewritten `executor.rs`, `buffer.rs` and `trace.rs` were written by hand in rustfmt's style.
- The harness `programs` question above is the host's to decide.
- Raw logs are in `raw/t2-*.log` and `raw/still-oak-40-1001-task2.log`.
