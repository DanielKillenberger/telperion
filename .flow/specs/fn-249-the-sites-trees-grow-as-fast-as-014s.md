# The site's trees grow as fast as 0.1.4's

## Goal

The owner (2026-10-09): "i really don't want to ship a release with a performance regression this big". killenberger.com's grove grows each tree as a coarse field in a Wasm worker; the release branch took oak 1.1-1.5 s, birch 0.87-1.0 s, spruce 0.37-0.39 s per tree against 0.1.4's 0.28-0.32, 0.35 and 0.24 s. The stage profile (`.flow/evidence/fn-245-a-breakthrough-in-generation-time/PROFILE.md`) put the whole regression in `grow`; Astra ranked the fixes (`.flow/evidence/fn-245-a-breakthrough-in-generation-time/ASTRA-SITE.md`). This spec is fn-245's release slice: the four byte-identical changes Astra judged release-sized, measured, then the remaining gap reported for the owner's release decision. Host (2026-10-09): release blocker for fn-237. Byte-identical speed work only; no change to the oak's light model or growth.

## Decisions

- R3 measured first: placeholders cost ~51/30/10-21 ms of `place` for spruce/oak/birch (Astra's 50-160 ms estimate was high: placeholders are 80-96 % of axes but take no phytomer steps). Only the unbent duplicate was removed, and only for sagging species; an empty continuation must still be laid (its base is its bearer's far tip in `sag::gather`).
- R4: `Grid::corners` arrays only; the gathering buffers (40 allocations a tree) left as they are.
- R1's warm path does not reach the site: `growField` instantiates a fresh Wasm instance per tree, so instance reuse is a contract change raised to the host.

## Acceptance Criteria

- **R1:** closed-form `Rows` kept across grows in one Wasm session, keyed by complete species parameters, age and row mode, bounded; byte-identical; cold and warm timed — met (4 most recent keys).
- **R2:** `I256::quotient` replaced by an exact limb division; differential tests; contract A parity green — met (Knuth D on 32-bit limbs, 400k random cases plus boundaries).
- **R3:** placeholder framing measured, then the duplicate removed where consumers permit; byte-identical — met.
- **R4:** lattice deposits on fixed arrays only if a Wasm measurement confirms the gain; identical deposits — met.
- **R5:** the site's request end to end in Wasm against 0.1.4, best of 3, byte-identical, remaining gap reported — met.
- **R6:** gate and npm test green — met (npm 155 tests).

## Result

- Gains: R1 birch warm grow 752-833 → 112-192 ms (cold unchanged); R2 oak 125-250 ms; R3 spruce 22-27 ms; R4 oak shade span ~25 ms.
- Remaining gap against 0.1.4 cold, end to end: oak 2.5-4.7× (550-910 ms over), birch 2.3-2.8× (490-700 ms; 0.5-1.1× warm), spruce 1.3-1.5× (80-105 ms). Load 3.8-5.3 on 32 cores.
- 24 of 24 site field-answer hashes and 16 of 16 `space_hash` trees identical to `4b0f7b01`. Tables, hashes and reproduction in RESULT.md. Impl-review SHIP.

## Open

- Instance reuse in `growField` to give the birch its warm path (host decision; changes the contract).
- `grow/settle.rs` has the same unbent/bent pair; `place_bearers` could serve it too.

## Friction

- `cargo fmt -p telperion-space` rewrote 21 untouched files (~2 min) -> one formatting commit or CI `cargo fmt --check` (proposed).
- The site makes a fresh Wasm instance per tree -> raised to the host as a design question (see Open).

## Evidence

- `.flow/evidence/fn-249-the-sites-trees-grow-as-fast-as-014s/RESULT.md` (tables, `space_hash` lines, reproduction)
- `.flow/evidence/fn-249-the-sites-trees-grow-as-fast-as-014s/bench/` (the Wasm benchmark: `bench.mjs`, `sweep.sh`, `table.py`, `summ.py`, `voxelize.js`)
