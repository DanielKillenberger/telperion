# Phase 1 survey: what removing the GPU grower touches, 11 Nov 2026

Read-only survey by a Fable agent at `ff3f3cabf`; condensed by the host. [V] read in code or a repo document, [I] inferred, [U] not determined. Nothing was built or run.

## The finding that shapes the design

Every device stage after the grower reads one device buffer the grower's last stage writes (`Dressed`, `resident/fold.rs:53`, built only at `cycle/drawn.rs:206`). No production code makes that buffer from a CPU tree. A complete CPU path to the screen already exists beside it: `telperion_core::mesh::build(&Family) -> TreeMesh`, then `Renderer::submit(&TreeMesh)` (`lib.rs:236-270`), used by the still runner, 129 render integration tests and `Renderer.setTree` in the browser. [V]

## Extent [V, `wc -l`]

The render crate's `src` is 70,030 lines; `src/generation` with `generation.rs` is 51,545 of them.

| Part | Lines | Role |
|---|--:|---|
| `generation/cycle/`, `cycle.rs` | 32,980 | The device grower: 83 Rust files (19,396 lines, 7,695 in test modules), 63 WGSL files (13,584) |
| `generation/resident/`, `resident.rs` | 12,931 | Device expansion of the drawn tree, fed only by the grower |
| `generation/*.rs`, `*.wgsl` | about 5,400 | `Generator`, placement, mass, ring positions, I/O |
| `src/web/generation.rs`, `growing.rs`, `field.rs` | 464 | Wasm bindings to the above |

Stays in every case, and does not reference `generation` except where listed: `curve.rs`, `curve/`, `shaders/`, `select.rs`, `select/`, `foliage.rs`, `wood.rs`, `mass.rs`, `shadow*`, `scene*`, `pass.rs`, `camera.rs`, `headless.rs`, `timing*`. Touch points: `shader_tests.rs:33,149`, `foliage.rs:198`, `select.rs:259`, `wood.rs:59`, `curve.rs:141`, `generation/submit.rs:3` (`submit_prepared`).

CPU-side code that exists for the device: `telperion-space/src/grow/cycle/capture*.rs` (7 files, 1,515 lines), `fine/tables.rs` and `ChainTables` (the CPU's own need is [U]: `grow/alone.rs` names it), `agree.rs`, and in `telperion-core/src/pipeline/executor.rs` (473 lines) `resident`, `Summaries`, `growth`, `reach_terms`, `stations`, `bases`, `checked`, `ConvertRow`, `Establishing::words`. `closed_form`, `contract.rs`, `contract/` and `transcendental.rs` are the CPU grower's own and stay.

## Consumers [V]

| Consumer | Calls | CPU path today |
|---|---|---|
| Harness build | `harness/grow-in.ts:113` `setTreeGpu`; `rust-stage.ts:155-157`; `admission` | Exists: `Renderer.setTree` (`src/browser/render.ts:270`) to `WebRenderer::set_tree` (`web.rs:222`). Synchronous, main thread. |
| Harness "grow in", "play the year" | `beginGrowth`, `growTo`, `draw`, `growthAdmission` (`grow-in.ts:60-126`, `web/growing.rs`) | Native only: `pipeline::growth::{begin, grow_to, settle, colouring, mesh}` (`pipeline/space/growth.rs:78-143`); `examples/unfold.rs`. No Wasm binding. |
| Package `.` | `createRenderer` with `setTree`, `setTreeGpu`, `beginGrowth`, `admission`, `programs`; `TreeEngine` | `TreeEngine.build` (`src/browser/core.ts:117`) and `setTree` exist. |
| Package `./field` | `WebField.prepare/check/grow` to `Generator::grow_field` | Exists: `TreeEngine.build(family, { field })` returns the same `FieldGrid` (`core.ts:132`, `telperion-wasm/src/generate.rs:74-145`, `pipeline/field.rs`). |
| `examples/film.rs` growth shots | `begin_growth`, `grow_to`, `settle`, `submit_prepared` (`film.rs:339-372`, `film/bounds.rs:69-73`) | `pipeline::growth::mesh`. Walks already use `mesh::build`. |
| Still runner, `species:qa` | `mesh::build` only | Already CPU. |
| `examples/frames.rs` (release scorecard) | `Generator::prepare` | Rewrite on `mesh::build` and `submit`. |
| `examples/grow_in.rs`, `resident_stills.rs`, `prepare_timing.rs`, `generation_gpu.rs`, `gpucycle/` | `Generator` | `grow_in` as the film; the rest are device tools. |
| `scripts/benchmarks/generation-gpu-smoke.mjs`, `mature-generation.mjs` | `setTreeGpu` | Device tools. |
| Browser tests | `tests/browser/field.mjs`, `render.mjs:184`, `scripts/test-dist.mjs:125-134` (expects `growField` to reject in Node) | `test-dist` must flip. |
| `src/probe/field/worker.ts` | `growField` in a Worker, the only Worker in the repo | Follows the field entry. |

## The Wasm side [V]

- CPU grow entries exist in `telperion.wasm` (`crates/telperion-wasm`: `build()` at `lib.rs:122`, `generate` at `generate.rs:96`) and in `telperion-render.wasm` (`setTree`). Neither exposes begin, grow-to or settle.
- A Worker running `TreeEngine` cannot feed the renderer today: `Output` (`generate.rs:10-25`) carries no `Curve`, and `WebRenderer` accepts only a family's JSON. `WebRenderer::create` takes an `HtmlCanvasElement`; `web-sys` is built without `OffscreenCanvas`.
- The CPU field is read from the drawn tree (`pipeline/field.rs:1-17`), never from the grower's lattice (`telperion_space::light::Field`, `pub(crate)`).
- `STATUS-2026-10-31.md`: the site's field Wasm is 834,077 Brotli bytes against 0.1.4's 107,663. `telperion.wasm`'s Brotli size is [U].

## Tests and gate [V]

The render lib has 317 tests, 215 under `generation/`, 73 of those ignored measurements. Named holders of the CPU to the device: `cycle/grown_tests`, `grown_det_tests`, `settle_tests`, `settle_float_tests`, `hier_tests`, `hier_math_tests`, `light_tests`, `retire_tests`, `contract_tests`, `mirror_tests`, `soft64_tests`, `growth/tests`, `growth/present_tests`; `resident/agree_tests`, `field_tests`, `field_flowers_tests`, `rachis_tests`, `boots_tests`, `organ_tests`, `thin_tests`; `cpu_builds_tests`, `film_walk_tests`, `relay_join_tests`, `unfold_tests`. `.config/nextest.toml` names two of them; `.config/gate-budgets.toml` has one entry (`retire_tests::the_node_table_with_retirement_is_the_same_on_every_context`), which `gate.py` refuses once the test is gone. `compile_size` and `shader_tests` read `generation::sources::all()`. CI keys name crate directories and need no edit unless a dependency changes.

## What the device does that the CPU does not [V]

A page budget and admission ("beside", "makeRoom", `budget::BROWSER`); stepped limits with a retry; buffer-size refusals (2 GiB bindings, refusing the oak from 95); programs compiled ahead; florets counted in the field on the device. Known disagreements end with the deletion: fn-300 (one node at age 40 on the spruce), fn-320.

## The cost, with the repo's numbers

Harness build per preset: device, warm, RTX 3080 (`fn-257/RESULTS.md`) against full CPU growth in Wasm on one core (`fn-329/RESULT.md`), in ms.

| Preset | Device | CPU in Wasm |
|---|--:|--:|
| Oak | 240-262 | 3,584 |
| Spruce | 432-445 | 11,947 |
| Birch | 205-248 | 1,230 |
| Beech | 167-208 | 258 |
| Ordinary | 436-473 | 1,000 |
| Palm | 75-76 | 17 |
| Telperion | 915-1,290 | 9,836 |
| Laurelin | 865-1,189 | 8,845 |

The device refused the oak from age 95; the CPU has no such wall. A film walk frame is about 2.9 s on the CPU against 0.2 to 0.6 s. The cost of a CPU draw per year for grow-in is [U].
