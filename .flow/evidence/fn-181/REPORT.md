# fn-181: the growth path is removed

Two commits on `fn-181-the-growth-path-is-removed-to-be`, base master `5e7796fb`:
`b3e1bb05` removes the growth-over-time layer, `a43efbba` folds its branches out of the
direct build and retires the growth traits. About 12,000 lines removed.

## R2: the mature tree did not move

`generation_digest` (every pipeline artifact for a full request and for the field alone, the
mesh, the GPU executor at both deliveries) for all 8 presets at seeds 1 and 7, against master:

- After `b3e1bb05`: 16 of 16 cases equal, every key (the growth key aside, which master
  printed and the branch no longer can).
- After `a43efbba`: every artifact equal in all 16 cases. Two digests move, by design:
  `pipeline.tree`, the tree's `Debug` text, which no longer carries the shoot's growth state;
  and the GPU `metrics` in the 12 GPU-backed cases, through `base_cpu_bytes =
  nodes.capacity() * size_of::<Node>()`. The 4 CPU-fallback cases (date-palm, european-beech)
  never set it and stay equal. `size_of::<Node>()` is 96 bytes, was 216 (`ShootState` 1, was
  120); about 8.8 MB less on the birch's 73,337 nodes.

Raw lines: `raw/digest-master.jsonl`, `raw/digest-phase1.jsonl`, `raw/digest-phase2.jsonl`.

Species metrics (`species_measure`) for the 10 runnable cases ran on master
(`raw/metrics-master*.jsonl`); the other six need profiles the runner refuses (FRICTION.md).
Every input those metrics read is covered by the digests above.

## Re-pinned tests, each with its proof

- `catalogue_identity` (4 digests): master's texts with the seven `/growth` rows stripped
  equal the branch's family, walk and override texts; all 1,750 single-row refusals of the
  rows that remain read as before, `age`'s included. Only the refusal pairs, paired by row
  index, re-pair.
- Species pins (49 seeds, oak, spruce, beech, birch): a digest of the same tree, wood,
  reference and leaves with the shoot's growth state left out of the tree bytes is equal on
  master and the branch, seed for seed.
- `every_row_has_its_own_rank_on_the_wire` now 0..250: ranks above 7 moved down by 7, order
  kept, so decode order is unchanged.
- The generation-limit inventory drops the sites of deleted code and of the band search, now
  test-only.

## R6: shipped artifacts

| Artifact | master (CI package, d4b1c86a) | branch (local `npm run build`) |
|---|---|---|
| telperion.wasm | 1,419,351 | 775,512 |
| telperion-render.wasm | 1,994,454 | 1,462,336 |
| telperion-field.wasm | 368,213 | 321,417 |
| telperion.js | 119,870 | 113,036 |
| field.js | 2,720 | 2,720 |
| voxelize.js | 4,526 | 4,526 |

The branch side is a local build; the PR's CI package gives the paired figure.

## R5: specs that depended on the growth path

Closed: fn-67 (growth parity report), fn-30 (calibrated growth). Re-scoped: fn-28 (smooth
growth on scroll) waits for the rewrite. Mentions edited: fn-105, fn-16, fn-175, fn-125. Left
as written, each with an active branch: fn-173 (its R3 clause on the hidden growth path is
moot), fn-123 (R1 names what the growth path refuses), fn-15 (growth path stays unanimated).

## Gates

- `cargo test --profile ci --workspace --no-fail-fast` ran once: 12 failures in 3 targets, every
  one a pin or fixture naming the removed rows, fixed as above and rerun focused (24 tests and
  the two integration targets green).
- `npm test` (catalogue check, vitest 129 tests), `npm run typecheck`: green.
- `npm run rust:test:wasm`: fails at `complete identity catalogue` (`PRESETS.length === 6`),
  stale since the date palm shipped (#115, 2026-09-25) and not run by CI; the probes this
  change edited run before it and pass.
