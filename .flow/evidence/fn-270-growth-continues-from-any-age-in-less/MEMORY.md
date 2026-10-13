# fn-270 task 3: memory, one lever at a time (worker, 2026-10-13)

R4. Each lever is applied alone, a labelled census taken after it, and the output checked against the lever before: every preset's node table at its own age and seed (the one-call build's digest, and the same tree grown on from H/2 and H-1), and the focused device tests that hold the young presets' splits and retirement verdicts to the CPU's. Raw output is in `raw/t3-<lever>-*.log` (ignored).

## How the census is taken

`presets_grow_on_on_the_device` (one preset a process, `PRESET=<id>`) prints a `CENSUS` line, the ledger's bytes by label, at each peak: the build grown and laid in one call (taken before the digest's readback, which no build makes), the build held paused at H-1, grown on from H-1 to H beside the tree drawn at H-1, and the tree at H drawn beside it. `build_time` gives the product build's peak (the resident build, its dress included); `BUILD_AGE=80` gives the oak at 80. All in decimal MB. "Continuation" below is the larger of the last two columns: the previous tree stays drawn through both.

## Baseline (task 2's tip, 728b90d79)

| Preset | Product build | Grown and laid | Paused at H-1 | Grown on from H-1 beside the drawn | Drawn at H beside the drawn | Hash |
|---|--:|--:|--:|--:|--:|---|
| date-palm | 32 | 32 | 5 | 32 | 5 | f22d2412f12fe872 |
| european-beech | 304 | 304 | 143 | 418 | 273 | 6f35c7809c6f62d6 |
| ordinary | 274 | 274 | 139 | 346 | 222 | 936105bc23a6ec6e |
| silver-birch | 800 | 800 | 478 | 946 | 647 | a1dbcff52c415bbb |
| oregon-white-oak | 1272 | 1272 | 837 | 1439 | 1029 | fd7d5eb0a8392ae6 |
| norway-spruce | 2445 | 2445 | 1681 | 3071 | 2383 | becc195a1ee62c89 |
| laurelin | 3853 | 3853 | 2762 | 5403 | 4415 | 8193cab2694c8679 |
| telperion | 4209 | 4209 | 3027 | 5876 | 4808 | 1e4f17ab77dd41f0 |

The oak at 80 is refused before its first cycle: its pencils alone want 2,207 MB, past the device's 2 GiB a buffer.

Telperion's standalone peak falls during growth, by label: the state 2,158 (pencils 644, axes 567, phytomers laid 448, units 339, grown 160), retirement's records 412 and columns 186, the hierarchy 650 (sums 207, counts 131, lists 130, shape 65, buckets 55, living and normaliser 28), the cycle's scratch 451 (job counts 195, tree 164, inputs 70, flags 22), the compactor 155, the light 148. Grown on beside the drawn tree, the drawn tree (501) and the last draw's settle scratch (work 523, nodes 362, place 238), destroyed but not yet freed by the device, stood beside the cycle's scratch.

The Wasm core (the CPU grower, `raw/t3-base-wasm.log`): Telperion 3,114 MB, Laurelin 2,984, spruce 1,270, oak 344, birch 280, beech 181, ordinary 135, palm 2.

## Lever 1: state, settle inputs and scratch apart

- Retirement's six columns a cycle writes before it reads (closed, the closing axis's own living and end, and each child's weight, living before and gap) move out of `ret` into the cycle's own buffer past the block stamps, let go between calls as the stamps are. The seventh, "seen", is gone: the living and end are assigned as an axis closes, the only cycle that reads them. `ret` keeps ten columns.
- A paused build waits for the device before it makes its scratch again, so the last draw's settle scratch is freed before the cycle's is made: their lifetimes do not overlap.

| Preset | Product build | Grown and laid | Paused at H-1 | Grown on from H-1 beside the drawn | Drawn at H beside the drawn | Hash |
|---|--:|--:|--:|--:|--:|---|
| date-palm | 32 | 32 | 5 | 32 | 5 | f22d2412f12fe872 |
| european-beech | 304 | 304 | 140 | 345 | 270 | 6f35c7809c6f62d6 |
| ordinary | 274 | 274 | 136 | 297 | 218 | 936105bc23a6ec6e |
| silver-birch | 798 | 798 | 468 | 847 | 637 | a1dbcff52c415bbb |
| oregon-white-oak | 1270 | 1270 | 819 | 1322 | 1011 | fd7d5eb0a8392ae6 |
| norway-spruce | 2438 | 2438 | 1628 | 2622 | 2330 | becc195a1ee62c89 |
| laurelin | 3843 | 3843 | 2693 | 4314 | 4346 | 8193cab2694c8679 |
| telperion | 4198 | 4198 | 2951 | 4699 | 4732 | 1e4f17ab77dd41f0 |

Kept: the continuation peak falls 19.5% on Telperion (5,876 to 4,732) and 19.6% on Laurelin; the standalone peak 0.3%, as the six columns are still made for each cycle. Every hash is the baseline's, and the focused tests hold (`raw/t3-l1-focused.log`: retirement's verdicts and the young presets' laid trees against the CPU's, the splits, the halted cycle, twelve tests). The compactor's 155 MB could share the cycle's scratch (it runs between cycles), a ceiling of 3.7% of Telperion's standalone peak; not built.
