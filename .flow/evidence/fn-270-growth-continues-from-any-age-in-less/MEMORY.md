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

## Stopped before lever 2: the ceilings of levers 2 and 3, measured

Levers 2 and 3 each need a design call the spec does not make (below). To make the calls answerable, their ceilings were measured with throwaway patches, none committed (`raw/t3-o2-*`, `raw/t3-o3-*`, `raw/t3-x3-ratios.log`, `raw/t3-oak80-ratios.log`).

- **Lever 2's ceiling** ("o2"): each preset's tables sized by its own most against its hint over seeds 1, 7, 42 (`arena_ratios`, `raw/t3-base-ratios.log`) instead of the most over every preset, the margin kept at 1.5: the best any parameter-derived estimate calibrated on the presets can do. No preset moved; every hash is the baseline's.
- **Levers 2 and 3 together** ("o3"): as o2, with the axes inside retired subtrees compacted out where a PA keeps records too, and the axes' and units' ratios measured that way (`raw/t3-x3-ratios.log`). The tree this draws is wrong (every hash but the palm's changes): the settle reads those axes (below). It measures the memory only, the archive's own cost left out.

| Preset (target) | Lever 1 | Lever 2's ceiling | Levers 2 and 3's ceiling | Continuation: lever 1 / 2 / 2 and 3 |
|---|--:|--:|--:|--:|
| date-palm (32) | 32 | 32 | 32 | 32 / 32 / 32 |
| european-beech (250) | 304 | 283 | 269 | 345 / 324 / 311 |
| ordinary (250) | 274 | 194 | 161 | 297 / 218 / 185 |
| silver-birch (450) | 798 | 540 | 476 | 847 / 589 / 525 |
| oregon-white-oak (650) | 1270 | 610 | 513 | 1322 / 662 / 566 |
| norway-spruce (1,200) | 2438 | 1537 | 1402 | 2622 / 1721 / 1586 |
| laurelin (2,000) | 3843 | 3358 | 2537 | 4346 / 4021 / 3148 |
| telperion (2,000) | 4198 | 3657 | 2745 | 4732 / 4375 / 3400 |

The oak at 80, sized by its own ratios (seed 1 at 80, widened by the seed spread measured at 40; seeds 7 and 42 at 80 could not be measured from the least tables under 6.5 GiB), builds at 4,226 MB, inside the 4 GiB default (4,295 MB); 1,486,513 instances.

At levers 2 and 3's ceiling Telperion's peak moves from growth into the settle (2,745 MB: settle work 427, phytomers laid 426, records 376, settle nodes 374, pencils 259, settle place 246, axes 228): the retired rows' records (12.5M rows at seed 42, 4.2 of its kept phytomers) and the settle's scratch sized by arena rows plus records are its next floor. No lever R4 names reaches the Two Trees' 2,000 MB; the continuation peaks come under 4 GiB (4,295 MB) only with lever 3.

## Host rulings (46865341f) and lever 2, merged into lever 4: tables grow by observed counts

The host ruled lever 2 into lever 4 (spec R4): a build's tables start from the closed form's hint at a lower margin and grow by the counts they hold. As built:

- **Growth by observed counts** (`build_size::grown`). Each table's rows two cycles on are its count times its own cycle's growth (its end against its start, which compaction between cycles does not hide). For a table the arena compacts, the count is what the last compaction kept, one cycle on when the compaction came after this cycle. A table past seven eighths of its rows grows by 1.5 (`GROWTH`) until the rows fit. The living table counts the most it held within the cycle (the living after the balance, and the axes that grew), as the ordinary at a margin of 0.5 overflowed it inside a cycle. The records grow by the same 1.5.
- **Compaction first, then the move** (`Growing::ready`). Before, a move was decided before the compaction and with stale counts, and a compacted table could move only within two cycles of a compaction. At low margins that let the beech's phytomers and the ordinary's living overflow mid-build (refused by name).
- **A move holds one group of tables twice at most** (`build_move.rs`). The scratch it will make anew is let go, and the device waited on, before any table moves. The tables move in three groups (phytomers, axes, units and lists), then the hierarchy and retirement together, the device waited on after each.
- **Start margin 1.0** (`build_arena::MARGIN`, was 1.5). Measured (warm ms and peak MB, every preset; `raw/t3-x2*`): margins 0.3 to 1.5 at growth 1.5 and 2. At 1.0 and 1.5 the Two Trees peak least (Telperion 3,360 MB, Laurelin 3,094). Lower margins trade time for memory: at 0.75 Telperion took 1,339 ms; at 0.5, 1,432 ms; at 0.3, 1,457 ms. Growth 2 overshoots (Telperion 3,676 at 1.0).

| Preset | Product build | Grown and laid | Paused at H-1 | Grown on from H-1 beside the drawn | Drawn at H beside the drawn | Hash | Warm ms (base) |
|---|--:|--:|--:|--:|--:|---|--:|
| date-palm | 32 | 32 | 5 | 32 | 5 | f22d2412f12fe872 | 73 (73) |
| european-beech | 333 | 333 | 144 | 349 | 274 | 6f35c7809c6f62d6 | 201 (181) |
| ordinary | 215 | 215 | 100 | 222 | 182 | 936105bc23a6ec6e | 461 (479) |
| silver-birch | 590 | 590 | 312 | 639 | 482 | a1dbcff52c415bbb | 218 (223) |
| oregon-white-oak | 905 | 905 | 547 | 957 | 738 | fd7d5eb0a8392ae6 | 229 (244) |
| norway-spruce | 1700 | 1700 | 1086 | 1867 | 1788 | becc195a1ee62c89 | 436 (472) |
| laurelin | 3094 | 3094 | 2088 | 3637 | 3741 | 8193cab2694c8679 | 1190 (1171) |
| telperion | 3360 | 3339 | 2283 | 3950 | 4064 | 1e4f17ab77dd41f0 | 1290 (1244) |

Kept: the standalone peak falls 20% on Telperion (4,198 to 3,360), 19.5% on Laurelin, 29% on the oak, 26% on the birch, 30% on the spruce, 22% on the ordinary; the beech rises 10% (304 to 333): at seed 42 it needs 2.25 of its hint in phytomers, past the margin, and moves late. The continuation peaks are under 4 GiB (Telperion 4,064 MB, 3,876 MiB). Every hash is the baseline's. The gate `every_preset_grows_from_its_hint_as_in_its_final_tables` (renamed from `..._in_its_hinted_arena`, which asserted no move) grows every preset at seeds 1, 7, 42 from its hint and again in its final tables from the first cycle: one node table, word for word, on all 24 (`raw/t3-l2-arena-gate*.log`; up to 3 moves and 8 compactions). Against fn-237 R0a: every tree but the Two Trees under 0.5 s; Telperion 1,290 ms (+46 ms, 3.7%, against about 1.25 s) and Laurelin 1,190 ms; the beech +20 ms. The time goes to the extra compactions and moves (Telperion at seed 42: 2 moves and 8 compactions where it made 4 compactions).

The oak at 80 is still refused, now on the budget (4,269 MiB of phytomers laid at its first move): its hint starts its axis tables at 1.65 of its expected axes, where it needs about 0.4. Lever 3 recalibrates the ratios.

## Lever 3: what reads the records (stopped for a ruling)

The retired rows' records have two readers on both executors (`girth/disused.rs`, `settle_sweep.wgsl`): the retired subtree's base radius as grown (`attachments_past`, through `disused` the only consumer of a retired root's radius), and the tree's pipe-weighted leaf mean (`leaf_partials_past`), which by fn-197's design is taken over the whole tree as grown, retired phytomers included (`grow/settle.rs`: "the one population the leaf term's mean is taken over").

Per preset (every PA): `retained` is 0.17 on the ordinary and 0 on every other preset; `leaf_girth` is 0.33 on the ordinary, 1.0 on the oak, spruce, beech, birch and the Two Trees, 0 on the palm. So on six of the seven presets that keep records, a retired subtree's radius is read by nothing (no disused pipe), and the records exist only for the leaf mean. Freezing the radius at retirement (the ruling) removes the records' radius reader on the ordinary only; the mean still reads every retired phytomer.

Measured on the CPU (a throwaway patch, `telperion_space::grow` at each preset's age and seed): the leaf mean taken without the retired phytomers moves every radius of a tree by one factor: the oak +5.6%, the birch +6.4%, the beech +3.8%, Telperion -3.6% (trunk base 1.767 to 1.703), the spruce -0.4%, the palm 0. The ordinary's run also dropped its disused pipes, so its figure (median 0, p99 13.8%, trunk -7%) overstates what freezing its radii would change.
