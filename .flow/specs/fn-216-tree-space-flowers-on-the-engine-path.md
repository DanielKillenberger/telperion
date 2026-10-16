# Tree space: flowers on the engine path

## Goal

Organs that flower, blossoms over the outer crown and hanging racemes (as laburnum's), borne on the engine's leaf-bearing nodes and expanded by the pipeline as leaves are (fn-125: an element is a pure function of source, index and seed), with a flowering phase by the year. A gap fn-10 found for natural trees, needed by Laurelin and Telperion (owner, 2026-10-07: full creative freedom for the Two Trees). Not emitted light (fn-217).

## Decisions

- Host decisions on the design (2026-10-07, `FLOWERS-DESIGN.md`):
  1. **Option B:** a `flowering` share per physiological age; engine topology unchanged; the expansion draws blossoms and racemes keyed by the engine's phytomer key; a second foliage pass through the leaf shader with its own colours; nothing drawn at zero; a season scaling flowers by degree.
  2. Proof on existing species: the oak's catkins (judged against photographs) and a blossom test on the beech (organ test). A bird cherry is its own species spec if the owner wants one.
  3. The light-driven outer-crown row waits; the first build uses the PA share only.
  4. Petal count is a whole-number row whose fraction scales the last petal, so a walk changes the flower by degree.
  5. Node identity gets its own small spec first (fn-220).
- Host, 2026-10-08: option (a), the CPU step `Expansion::organ` serves both executors; the GPU bloom pass and flower shadows go to fn-125. The stills pass as an organ test; no Astra pass ("A real beech's flowers are inconspicuous, so the beech is not wrong to show little").
- Bloom colour rows are linear albedos on the leaf scale (beech leaf 0.021); final beech petals 0.1/0.097/0.09, oak catkins 0.12/0.13/0.025.

## Acceptance Criteria

- **R1:** A design for flowering organs as species parameters by degree, decided by the host before building — met (option B).
- **R2:** Blossoms and pendant racemes on a flowering natural species, judged on stills beside photographs — met as an organ test (oak catkins, beech blossom).
- **R3:** Every passed species unchanged where flowering is zero — met (byte-identical; catalogue identity tests).

## Result

Flowers on the engine path, option B: each drawn node of a flowering age is kept as a bloom; the expansion draws each inflorescence keyed by the node's lineage (fn-220), petal fraction sizes the last petal, the season scales florets. Share, floret-count and petal-count walks tested with no pop. Oak catkins read as catkins up close but vanish at whole-tree view (sub-pixel); beech florets fuse into white patches. Gates: render 161 passed, workspace without render 994, npm test 143.

## Open

Recorded in fn-217's spec for the first blossoming species: blossom florets read as flat white scraps rather than five-petal flowers; blossom vanishes at whole-tree distance, where a cherry reads as a white mass. GPU bloom pass and flower shadows: fn-125.

## Friction

- Shared GPU lock held by whole workspace gates (40 min waiting on two stills) -> resolved in part: gate split, render alone under the lock (host, 2026-10-07).
- `cargo fmt -p` rewrote 31 unrelated files -> proposed one formatting commit and `cargo fmt --check` in CI.
- Tiny organs vanish at whole-tree view, learned only after GPU rounds -> proposed a CPU-side coverage estimate printed by the build.
- Bloom colour scale unstated (four GPU rounds) -> fixed: colour rows' docs and the parameter reference state linear reflectances (host, 2026-10-08).
- Stale `presets.generated.ts` with no check -> fixed: `scripts/build-wasm.mjs` fails on a stale copy.

## Evidence

- `.flow/evidence/fn-216-tree-space-flowers-on-the-engine-path/FLOWERS-DESIGN.md` (the design; read by `bloom.rs`)
- `.flow/evidence/fn-216-tree-space-flowers-on-the-engine-path/FRICTION.md` (cited by `scripts/build-wasm.mjs`)
