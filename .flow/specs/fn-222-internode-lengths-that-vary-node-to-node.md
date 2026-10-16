# Internode lengths that vary node to node

## Goal

The silver birch (fn-213) reached its wrong-path stop on Astra's trait (j), "repeated comb-like arrangements of drooping shoots along long, sparsely subdivided branches", after two rounds aimed at it. The cause, in code: every internode of a physiological age within a grown unit had the same length (`geometry/lay.rs`), so shoots sat at even spacing and no value varied it. Host decision (2026-10-07): a capability, not more values, and not light for the birch. The engine's layout only.

## Decisions

- The factor is uniform on [1 - s, 1 + s] (bounded, so the reach bound stays finite); a log-normal or correlated draw is a later host choice.
- Lay only: topology, scales, the walk's run and reach and every decision are unchanged at any spread; the closed form's reach bound and the landing zone take each internode at its longest.

## Acceptance Criteria

- **R1:** A per-PA internode length variation keyed by lineage, mean one, zero by default, by degree within the walk bound of 30 — met.
- **R2:** Both executors draw it identically under FINE-CONTRACT, the contract probe extended — met.
- **R3:** The closed form's expected lengths and the cut's predictor unchanged in expectation — met.
- **R4:** Every passed species byte-identical; npm test and the gate green — met.

## Result

- `Form::internode_spread`; length = internode x scale x (1 + spread (2u - 1)), u from `Key(phytomer.key).child(INTERNODE)`. Beech and oak at 20 with spread 0.6: same counts, axes, keys and scales; laid-length ratio mean 0.997, sd 0.343-0.345 (uniform 0.346); 1.01 refused; `every_setting_changes_the_tree_by_degree` passes.
- GPU: probe selector 16 over 262,144 keys bit-equal; beech 25 at spread 0.6 tips within 1.6e-6 m of the CPU (screen 50 um).
- Predictor: every count and expected log length bit-identical at spread 0.8 on four species; reach bounds at least as large.
- `space_hash` 16/16 identical to the base `e8a9b05d`. Catalogue identity digests moved only by the new Debug field. npm test 143 passed.

## Open

Not modelled, for the host: the closed form's second moments (`Along::square`) still assume equal internodes (squared length rises by n(s^2/3)l^2); the relay point is still placed by scale-weighted share, not laid length.

## Friction

- `cargo fmt --all` rewrote 46 untouched files (no fmt check in CI) -> proposed one whole-tree format commit and `cargo fmt --check` in CI.
- The split gate fails jev `replay` on a fresh target (needs render's `headless` example) -> proposed running the render half first, or the test building or skipping by name.
