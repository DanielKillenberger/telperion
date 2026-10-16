---
satisfies: [R2, R3]
---
# fn-292-the-foliages-level-lists-hold-only-what.2 Measure the kept tree, the frame and the stills before and after for every preset

## Description
The proof R2 asks for, against the base already captured in `.flow/evidence/fn-292-the-foliages-level-lists-hold-only-what/raw/before/` (0a6664741: `memory.txt` from `the_drawing_s_memory`, `frames.txt` from `examples/frames` whole and close, and every preset's whole/mid/close stills from `examples/resident_stills`). Then the gate and npm test.

**Size:** S
**Files:** `.flow/evidence/fn-292-the-foliages-level-lists-hold-only-what/MEASURE.md`, `crates/telperion-render/src/generation/resident/drawn_memory_tests.rs`
**Touches:** [.flow/evidence/fn-292-the-foliages-level-lists-hold-only-what/**, crates/telperion-render/src/generation/resident/drawn_memory_tests.rs]

### Approach
- `the_drawing_s_memory` prints only the eight largest kept labels at 0.1 MB: extend it to print every foliage selection buffer (lists, starts, counters, arguments, scratch) in bytes; the base's selection sizes follow from `select::sizes` at the base's leaf counts (the stills' leaf counts, `raw/before/stills.txt`), stated as computed, not measured.
- Rerun the same three commands on the change into `raw/after/` (heavy runs through `telperion-heavy 10 --gpu 6 --threads 4`, one at a time).
- Compare every still pixel for pixel (decode the PNGs; identical bytes of the pixel data is the bar); view at most four images per preset pair, the spruce and Laurelin first.
- Frame time: whole and close p50 per preset, before and after; a regression past noise is investigated, not reported away.
- The ordered per-level leaf ids (task 1's `raw/before/levels.txt` against `raw/after/levels.txt`) are the byte-identity proof; pixel equality is reported separately.
- MEASURE.md: kept tree and each foliage label per preset before → after, frame p50 per view, the still comparison, the scratch's remaining size.
- Gate: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` once; `npm test`.

### Investigation targets
**Required**:
- `crates/telperion-render/src/generation/resident/drawn_memory_tests.rs` — the memory print
- `crates/telperion-render/examples/frames.rs`, `crates/telperion-render/examples/resident_stills.rs`
## Acceptance
- [ ] MEASURE.md: every preset's kept-tree memory before and after with the foliage labels; the spruce's level lists down from 60.4 MB to about its leaf count × 4 B.
- [ ] Frame p50 whole and close per preset not worse beyond run-to-run noise.
- [ ] Ordered per-level leaf ids identical before and after for every preset at the hero and close views (exact equality, recorded apart from pixels).
- [ ] Every preset's whole, mid and close stills pixel-identical to the base (or each difference named and explained).
- [ ] Gate and npm test green, results in the done summary.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
