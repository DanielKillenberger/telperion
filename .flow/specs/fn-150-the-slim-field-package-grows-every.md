# The slim field package grows every preset through the one build pipeline

## Conversation Evidence

> owner (2026-09-25), bug report from killenberger.com: "In telperion 0.1.3 the date palm grows through the main entry (`telperion.js`) but not through the slim field package (`telperion/field`). `growField("date-palm", seed)` throws `Field core: invalid input: family without a leaf plan` at every seed tried. Every other preset grows through the slim package."
> owner's report, acceptance: `growField("date-palm", s)` succeeds for seeds 1, 7, 1407 and 4242; its occupancy matches `engine.build(presetById("date-palm"), { field: true }).field.query` byte for byte; a test in the npm package grows every entry in `PRESETS` through `telperion/field`; a patch release 0.1.4 with the slim Wasm size noted.

## Goal & Context
<!-- scope: business -->

killenberger.com draws voxel trees from field queries and loads only the slim package to keep its download small, so it cannot offer the date palm that 0.1.3 shipped. The slim package keeps its own copy of the build chain, which fn-102 set out to remove: it refuses a family the leaf plan cannot describe, where the core pipeline falls back to a field read from placed leaves. The palm is the first shipped preset on that fallback. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-25 on master (39348def).** `crates/telperion-field/src/grow.rs` runs `branching::generate`, `foliage::plan::plan` and `Field::planned`, and returns `InvalidInput("family without a leaf plan")` when the plan is `None` (`grow.rs:25`). It also skips `mesh::grow`'s rosette step (`clear_apical_twigs`) and `clothe_leaf_bases`, so for the palm even its skeleton differs from the main entry's. The core pipeline (`crates/telperion-core/src/pipeline/stage.rs`) builds the field from the plan where one exists (`planned_field`) and from placed leaves otherwise (`placed_field`). [checked]
- **Shape.** The slim crate asks the core pipeline for the field artifact only, as the main entry's Wasm binding does, and keeps no chain of its own. The field it returns carries the bounds `growField` exposes today. [inferred]
- **Unknown.** How much the placement and cull code adds to `telperion-field.wasm` (about 355 KB in 0.1.3); the implementer measures it and reports it in the PR. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A test, red on master first, grows `date-palm` through the slim crate at seeds 1, 7, 1407 and 4242. [checked]
- **R2:** For every shipped preset, the slim field's occupancy flags equal the main entry's `field: true` flags on the same cells at the same seed, byte for byte. [inferred]
- **R3:** An npm package test grows every entry of `PRESETS` through `telperion/field`, so a preset cannot ship without slim field support. [inferred]
- **R4:** The PR reports `telperion-field.wasm`'s size before and after; the workspace gate and `npm test` are green; the version is 0.1.4. [inferred]
- **R5 (lower priority):** The main entry's `field: true` result exposes the field's bounds, as `growField`'s `FieldTree.bounds` does. [inferred]

## Boundaries
<!-- scope: business -->

- Not the lean species runner (fn-149). A defect fix to a shipped package; the owner merges and the host tags the release.
