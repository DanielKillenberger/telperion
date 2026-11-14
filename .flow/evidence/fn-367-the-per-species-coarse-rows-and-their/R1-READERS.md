# R1: every reader of the coarse rows and of `family.fidelity`, 14 Nov 2026

Read at `78a8e6a94` (fn-367 branch, before any deletion), by grep over `crates/`, `src/`, `harness/`, `tests/`, `scripts/` and `catalogue/` for `Coarsening`, `coarsened`, `COARSEST`, `coarse` and `fidelity` (any case), then by reading each hit. Nothing was run for this reading.

## `Coarsening`, `coarsened`, `COARSEST`

| Reader | What it does | Where | Mark |
|---|---|---|---|
| The type, its `NONE`, `validate`, `Species::coarsened`, `COARSEST` and their unit tests | define the rows and apply them at `t = fidelity / COARSEST` | `crates/telperion-space/src/species/coarse.rs` | coarse rows only |
| The space crate's export | `pub use species::coarse::{Coarsening, COARSEST}`; `pub mod coarse` | `telperion-space/src/lib.rs:72`, `species.rs:10` | coarse rows only |
| `Space.coarse` and its default | the field the rows sit in | `telperion-core/src/presets/space.rs:81-93` | coarse rows only |
| Preset parser and writer | reads `["coarse", name]`, validates the rows, writes those off `NONE` | `presets/space.rs:128, 146, 157-158, 271-284` | coarse rows only |
| `Family` validation | validates the rows and the species coarsened to `COARSEST` | `family.rs:139-143` | coarse rows only |
| Blend | walks two species' rows | `blend.rs:171-194` | coarse rows only |
| `GrowInput::of` | coarsens the species where `fidelity > 0` and the rows are not `NONE` | `pipeline/input.rs:64-69` | coarse rows only |
| Tests | `pipeline/fidelity_tests.rs` (whole file) | as named | coarse rows only |

## `family.fidelity` (the `/fidelity` parameter)

| Reader | What it does | Where | Mark |
|---|---|---|---|
| The row's declaration | the field and its catalogue entry: wire 369, scalar check at `Site::Age`, `Blend::Linear`, `Dial::Excluded`; from it the macro emits the wire, the walk, the reference and the browser metadata | `family.rs:57-72`, `FIDELITY` at 74-76, default at 96 | coarse rows only (the parameter itself) |
| `GrowInput::of` | the one read that changes a tree: coarsens the species by the rows | `pipeline/input.rs:65-67` | coarse rows only |
| `GrowInput.fidelity`, `fidelity_admitted` | refuses a value off `FIDELITY` by name, from the conversion and the growth entry; changes no tree | `pipeline/input.rs:51-52, 74, 80-89`; `pipeline/space.rs:108`; `pipeline/space/growth.rs:36` | coarse rows only (the parameter's own admission) |
| Catalogue group | puts `/fidelity` in the panel's advanced group | `catalogue/group.rs:52` | coarse rows only (the parameter's panel row) |
| Geometry benchmark | asserts the published schema's `fidelity` is its default 0 | `suite/geometry_benchmark.rs:382-387` | coarse rows only (a check of the parameter) |
| Sweep | holds `/fidelity` out of the walked rows | `suite/sweep.rs:85` | coarse rows only (a check of the parameter) |
| Tests | `cycle_cost_tests.rs` (`family(.., fidelity)` and the pin at both ends; two ignored measurements at 0 and 1); `fidelity_tests.rs`; a comment in `tests/catalogue_identity.rs:106` | as named | coarse rows only |
| Generated mirrors and docs | `src/browser/parameters.generated.ts:12, 840`; every preset's `"fidelity": 0` in `presets.generated.ts`; `docs/parameters.md` | as named | coarse rows only (generated from the row) |
| Harness panel | no file names it; the row reaches the panel through `parameters.generated.ts` (`harness/rows.ts`, `harness/panel.ts`) | as named | coarse rows only |

## The word elsewhere, another meaning

- `pipeline/branching/audit.rs:1`, "retained fidelity counterexamples": botanical fidelity, a comment; reads no parameter.
- `crates/telperion-render`, `crates/telperion-wasm`: no file contains "fidelity" (`grep -rli`, 0 files). The render crate's "coarsen" is the shadow map's casters and the wood's view budget.
- The foliage levels (`pipeline/foliage/levels.rs`, `bloom/stalk.rs`, `element.rs`, `surface/curve`): "coarse/coarsest" names a level of detail; none reads `family.fidelity` or the rows.
- `telperion-space/src/tolerance.rs:48` and `lineage/keys.rs:96`, "the coarse tree": the field's coarse route (fn-214, fn-323), not the dial.
- `README.md`, `STRATEGY.md`, `AGENTS.md`: prose, "fidelity" as botanical faithfulness.

## Verdict

No reader of the parameter reads it for anything but the coarse rows: its one tree-changing read is `GrowInput::of`'s coarsening, and every other read is the parameter's own declaration, admission, panel row, checks or generated mirrors. The design's stop does not apply; the rows and the parameter are deleted together.

Walk and blend tests that use the dial (the spec's third stop): `a_walk_between_two_fractional_requests_keeps_their_fidelity` holds that a walk keeps `fidelity` itself, and `a_walk_between_two_dials_moves_its_first_age_by_degree` holds that a walk moves `coarse.first`. Each reaches only the dial or its rows, no other state, so both go with the thing they hold.
