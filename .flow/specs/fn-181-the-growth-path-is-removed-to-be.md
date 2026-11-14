# The growth path is removed, to be rewritten later

## Goal

The growth path grew a specimen year by year and showed it at an age behind the harness's `?growth=1`; since 2026-09-18 it was hidden, never a default or gate, kept buildable and pinned. Its design did not hold: on 2026-09-17 the birch at seed 1 was 73,337 nodes on the direct build and 590,410 on the growth path, and fn-173's prepared crown shell, a 19 to 25% win on the mature build, made age-12 growth 70× slower because every yearly slice rebuilt shells. The owner decided to remove it now and rewrite growth later with a different approach.

## Decisions

- user (2026-09-27, on fn-173 making the hidden growth path 70× slower): "Why is it 70x slower now? I mean we can scrap it and we'll have to rewrite it at a later date. The implementation was never good enough anyway. We'll need some different approach."
- user (2026-09-27, selected): "Capture the removal"
- This overturns AGENTS.md's 2026-09-18 rule that the growth path is "kept buildable and pinned" (owner, 2026-09-27).
- owner (2026-10-02), on the lifecycle traits: "i'd keep the documentation about it but remove the code other than the age. This way we can rebuild a growth path later". `/growth` traits are retired wire rows; `Family.age` stays; README's growth section moves to `docs/growth-path.md` under the lessons; the catalogue keeps its `growth` column.
- `Specimen` and the direct build stay; shared code the direct build no longer reads is deleted, not kept for the rewrite. No new growth implementation here.
- Strategy conflict flagged for the owner's next strategy pass: STRATEGY.md's "growth ... shaping its lifecycle" has no implementation until the rewrite; the direction stands.

## Acceptance Criteria

- **R1:** No growth-over-time code remains (timeline, keyframes, chronicle, history, retention, storage, specimen-at-an-age reads, `SpecimenStore`/`SpecimenView`, executor `present`, web growth view, Wasm growth exports, `?growth=1`) — met; about 12,000 lines removed, including braided branches in the direct build (twig-layer clock, growing envelope, sleeping buds, scaffold pause, shoot growth state).
- **R2:** Every preset's direct build byte-identical at seeds 1 and 7; gate passes — met: `generation_digest` equal on every artifact in all 16 cases; only `pipeline.tree`'s Debug text and GPU byte metrics moved (a `Node` is 96 bytes, was 216). Catalogue identity and 49 species pins re-pinned with proofs (digests equal with the growth state left out).
- **R3:** The harness draws every preset with no growth control — met.
- **R4:** AGENTS.md records the removal and the rule that a future growth feature is its own spec — met.
- **R5:** Dependent open specs closed or re-scoped — met: fn-67 and fn-30 closed; fn-28 re-scoped to wait for the rewrite; fn-105, fn-16, fn-175, fn-125 mentions edited; fn-173, fn-123, fn-15 left as written on active branches.
- **R6:** Lessons note under `docs/`; artifact sizes before and after — met.

## Result

Shipped artifacts (master CI package d4b1c86a vs branch local build): telperion.wasm 1,419,351 to 775,512; telperion-render.wasm 1,994,454 to 1,462,336; telperion-field.wasm 368,213 to 321,417; telperion.js 119,870 to 113,036; field.js and voxelize.js unchanged. Workspace gate: 12 pin and fixture failures naming removed rows, fixed and rerun focused; `npm test`, `npm run typecheck` green. `npm run rust:test:wasm` fails at a stale `PRESETS.length === 6` check (not run by CI, stale since #115). Impl-review SHIP, no findings.

## Open

The growth rewrite is a later spec.

## Friction

- `species_measure` refuses unready catalogue profiles, so six metric baselines could not run -> proposal: a metrics-only mode.
- Each byte-identity check costs a fat-LTO release build (~10 minutes) -> proposal: run `generation_digest` under `--profile ci`.
- The dcg hook refuses heredocs with backticks or `<` -> local setup, reported.
