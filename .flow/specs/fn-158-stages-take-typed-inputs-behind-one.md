## Goal

Generation has one path and the compiler keeps it that way: stages take only the typed input they read, never the whole `Family`, and sit privately behind the pipeline; the GPU executor and the growth path reach generation through a narrow executor interface. Second of four stacked specs, after fn-152; output does not change. Prompted by Astra's review of fn-152 ("Design the executor boundary first").

## Decisions

- Astra review of fn-152: `pub(crate)` alone does not stop a second chain; GPU preparation orchestrated stages itself. "Design the executor boundary first", then migrate stage inputs and visibility together.
- owner (2026-09-25, fn-151): "structure over policing": stages private to the pipeline, so a second chain does not compile.
- owner (2026-09-26): "ok that seems reasonable. /flow-next:flow this to its completion in one stack with pr's for each spec"
- Shape: narrow typed inputs (`GrowInput`, `PlanInput`, `SurfaceInput`, ...) built once from `Family`; only `pipeline::build` and the executor interface are public; implementation tests move into private modules. Lazy requests, overlap, arithmetic order, `f32` conversion order and random consumption are preserved.
- Host decisions at NEEDS_HUMAN (visibility step): the interface admits what tests and tools need (a caller's solved tree, an element from its rows); the eight core examples that chained stages go through the pipeline.
- Out of scope: consumer migration (fn-159), presets (fn-160), the GPU leaflet defect.

## Acceptance Criteria

- **R1:** No stage takes `&Family`; a compile-fail test with a positive control shows stage calls from `telperion-wasm` and from `telperion-render` outside the interface do not compile — met.
- **R2:** The GPU executor and the growth path build through the executor interface — met.
- **R3:** Every shipped preset byte-identical in mesh, field and metrics; artifact budgets hold — met (generation digests identical to base `d8368b0b` on every line).
- **R4:** Release timings before and after on the RTX 3080, five runs, medians and spread — met; no regression beyond spread.
- **R5:** Workspace gate and `npm test` green — met.

## Result

- Byte identity: `crates/telperion-render/examples/generation_digest.rs` (added here) digests every preset at seeds 1 and 7 across pipeline, field-only request, `mesh::build`, both GPU deliveries and the growth path; head identical to base.
- The core examples are unchanged except the date palm wherever an example grew its own skeleton: the pipeline's skeleton hangs its 256 shed leaf bases (nodes 40 to 552, wood triangles 1,640 to 32,320; height, crown, dbh and leaves unchanged; numeric status pass). The palm's runner skeleton pin moved from 17313072172866236736 to 10058597765394239704; `measure` drops `boundsMs`, gains `cullMs`.
- Timings (base and head interleaved, 10 warm samples a cell): every head median inside the base's spread; largest moves +2.9% (laurelin, GPU CPU delivery) and -4.4% (spruce, resident).
- Sizes: telperion-field.wasm 359,585 to 359,177; telperion.wasm +10,170; telperion-render.wasm +13,004 (mostly `pipeline::` symbol names); all under budget.
- Gate at `d1c52a3e`: 1,046 passed, 0 failed; `npm test` 130 passed. Merged in PR #126.

## Open

None.

## Friction

- No tool compared a preset's artifacts before and after a refactor -> `generation_digest` example added.
- Timings were polluted by another checkout's test run -> interleaved base/head timing recipe; promoting it to `scripts/` proposed.
- The visibility step needed host decisions the spec did not settle -> stopped at NEEDS_HUMAN; host decided.
- The species memory ceiling assumed its own process under `cargo test` -> those four tests run in child processes.
- The rebase met a new stage caller (#121's palm pins) -> rerouted; pin moved; unavoidable.
