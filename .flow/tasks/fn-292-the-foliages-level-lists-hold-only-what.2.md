---
satisfies: [R2, R3]
---
# fn-292-the-foliages-level-lists-hold-only-what.2 Measure the kept tree, the frame and the stills before and after for every preset

## Description
The proof R2 asks for, against the base already captured in `.flow/evidence/fn-292-the-foliages-level-lists-hold-only-what/raw/before/` (0a6664741: `memory.txt` from `the_drawing_s_memory`, `frames.txt` from `examples/frames` whole and close, and every preset's whole/mid/close stills from `examples/resident_stills`). Then the gate and npm test.

**Size:** S
**Files:** `.flow/evidence/fn-292-the-foliages-level-lists-hold-only-what/MEASURE.md`
**Touches:** [.flow/evidence/fn-292-the-foliages-level-lists-hold-only-what/**]

### Approach
- Rerun the same three commands on the change into `raw/after/` (heavy runs through `telperion-heavy 10 --gpu 6 --threads 4`, one at a time).
- Compare every still pixel for pixel (decode the PNGs; identical bytes of the pixel data is the bar); view at most four images per preset pair, the spruce and Laurelin first.
- Frame time: whole and close p50 per preset, before and after; a regression past noise is investigated, not reported away.
- MEASURE.md: kept tree and each foliage label per preset before → after, frame p50 per view, the still comparison, the scratch's remaining size.
- Gate: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` once; `npm test`.

### Investigation targets
**Required**:
- `crates/telperion-render/src/generation/resident/drawn_memory_tests.rs` — the memory print
- `crates/telperion-render/examples/frames.rs`, `crates/telperion-render/examples/resident_stills.rs`

## Acceptance
- [ ] MEASURE.md: every preset's kept-tree memory before and after with the foliage labels; the spruce's level lists down from 60.4 MB to about its leaf count × 4 B.
- [ ] Frame p50 whole and close per preset not worse beyond run-to-run noise.
- [ ] Every preset's whole, mid and close stills pixel-identical to the base (or each difference named and explained).
- [ ] Gate and npm test green, results in the done summary.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
