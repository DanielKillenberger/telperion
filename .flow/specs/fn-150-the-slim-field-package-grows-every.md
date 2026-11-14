# The slim field package grows every preset through the one build pipeline

## Goal

killenberger.com draws voxel trees from the slim field package and could not offer the date palm that 0.1.3 shipped: the slim crate kept its own build chain (`crates/telperion-field/src/grow.rs`), refused a family without a leaf plan and skipped the rosette steps. The palm was the first shipped preset on the core pipeline's placed-leaf fallback.

Owner bug report (2026-09-25): "In telperion 0.1.3 the date palm grows through the main entry (`telperion.js`) but not through the slim field package (`telperion/field`). `growField("date-palm", seed)` throws `Field core: invalid input: family without a leaf plan` at every seed tried. Every other preset grows through the slim package." Acceptance asked: seeds 1, 7, 1407, 4242 succeed; occupancy matches `engine.build(presetById("date-palm"), { field: true }).field.query` byte for byte; an npm test grows every `PRESETS` entry through `telperion/field`; patch release 0.1.4 with the slim Wasm size noted.

## Decisions

- The slim crate asks the core pipeline for the field artifact only; no chain of its own. Placement and the `placed_field` fallback sit behind `geometry`, which the slim crate does not enable (`pipeline/drawn.rs` vs `pipeline/planned.rs`).
- Host, first: routing through `pipeline::build` with `geometry` grew `telperion-field.wasm` 48% (526,965 bytes); the owner rejected it. Instead the leaf plan describes a rosette.
- Host design rounds (2026-09-25), each measured on the quarter-metre grid at four seeds: frond capsules (a round bush, 81% agreement, 2.8x cells); flat ribbons per leaflet row (95.1-95.4%, 1.24-1.28x); tapered ribbons after the owner's verdict on the ribbon still ("right one looks much better" — the placed field read as pointed fronds with gaps, the plan as blunt merged fronds) still missed (95.3-95.5%, 1.15-1.18x). A ceiling measure showed the placed field's world-aligned leaf boxes are not the truth: the exact leaflets agree with it on only ~95%.
- Final (host, fourth): one oriented box per leaflet (`field/cuboid.rs`), from the shared leaflet code (`foliage/leaflet.rs`) and replayed draws, margin 1 mm + 0.5% of reach, judged against the real leaflet shapes (`tests/oriented/mod.rs`): targets >= 99% agreeing, <= 1.05x cells, every vertex covered.

## Acceptance Criteria

- **R1:** The palm grows through the slim crate at seeds 1, 7, 1407, 4242, red on master first — met.
- **R2:** Every preset's slim field flags equal the main entry's byte for byte — met (Rust and between the two Wasm artifacts).
- **R3:** An npm test grows every `PRESETS` entry through `telperion/field` — met (also `test:dist` on built `dist/field.js`).
- **R4:** Slim Wasm size reported; gate and `npm test` green; version 0.1.4 — met.
- **R5:** The main entry's `field: true` result exposes `bounds` — met.

## Result

Boxes: 99.72-99.73% of cells agree with the leaflets at 1.042-1.043x their cells, every vertex covered; the palm's plan plus field take 1.4-1.6 ms native vs 3.7-3.8 ms for placement, cull and placed field. Every other preset's field and the placed palm crown are byte-identical to master. `telperion-field.wasm` 344,798 raw / 131,051 gzip / 107,758 brotli (0.1.3 + 4.4 KB gzip, +3.0 KB brotli; names stripped in `build-wasm.mjs`). Gate: 950 passed, two stale expectations fixed and rerun; `npm test` 128. Commits `281926e9`, `509237f7`, `461a96bf`, `c58482a2`, `928d4bbb`.

## Open

- The owner's look at the box-field three-panel still was pending when the task closed.

## Friction

- No size budget for the slim Wasm stop -> state budgets in bytes or percent in the spec.
- The host's frond capsule failed the visual check (~40 min) -> check a primitive's shape against the organ in design.
- Ribbons took three fits (~45 min) -> name the cell test and draw replay in design; measure Wasm size per candidate.
- The agreement target was set against an inflated reference (~30 min) -> measure a target's ceiling first (`examples/palm_field_ceiling.rs`).
- Four primitives before the truth measure (~2 h) -> build a truth field before choosing a primitive.
