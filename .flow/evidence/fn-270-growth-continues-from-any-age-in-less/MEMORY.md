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

## Lever 3: a closed subtree's girth frozen as it closes; the records and the inside axes gone

The host's rulings (d4519b88f, 1616c7204): option (B), each retired subtree's radius and its two leaf-mean sums frozen, the settle adding them where the root stands in the reduction order, on the CPU reference and on the device alike; frozen at the subtree's close ("dead wood doesn't ripen once nothing in it lives"), carried up in the living sweep the device already runs.

- **The rule** (`retire.rs::frozen_at`, the CPU's reference; `retire_axis.wgsl::own_girth`, `retire_sweep.wgsl::frozen_girth`). As an axis closes, at the age grown: each phytomer's pipe at the axis's own base (its scale by its unit's size), ripened to that age, lit by light^χ but not by the tree's mean; their sum, their lit sum and the sum of their lit pipes' sections, in order. Then each child's frozen girth by its base against the axis's (a lateral's node scale, vigour and dominance share; a carried axis's vigour), in the children's order: sections, sums. The radius is the sections' radius. A retired root's frozen girth stands for its subtree: in the leaf mean as one term (its sums by its base) where it stands in axis order, and as its base radius by its base and the tree's mean.
- **What went.** The retired rows' records (`retire records`, `grow_records`, the records' scan, `RECORDS`), the settle's records binding, and the guard that kept the axes inside retired subtrees: they compact on every preset. `ret` holds 13 columns (the frozen radius and sums added), the cycle's own buffer 10 (the own sums and the base ratio added).
- **The CPU and the device.** The young presets' retirement verdicts and settled trees hold to the CPU's as before (`raw/t3-l3-focused.log`, 12 tests: the verdicts word for word, the laid trees to the ruled standard).
- **Compaction renumbers an axis's grown index.** With the inside axes now compacting on the spruce and the beech, the node table differed between compaction schedules only in its `NA_GROWN` column, each axis's slot in the arena: spruce 40 at seed 1 grown from its hint, in its final tables, compacted every cycle and never, one tree once that column is left out (four schedules, one digest). The tests' digest now leaves it out (DESIGN.md section 13: an axis is known by its lineage). Every preset's 0->H equals 0->a->H at a = H/2 and H-1, and the gate's moved and unmoved builds are one tree on all 24 (`raw/t3-l3-arena-gate.log`).
- **Ratios recalibrated** (`raw/t3-l3-ratios.log`): the axes 1.43 of their hint at most (was 1.65), the units 1.02 (was 1.46), the phytomers 2.25 (the beech at 42; was 1.79).

### The girth shift against today's tree (the CPU reference, `node_buffer` at 95bc4b7f8 against now, `raw/t3-l3-cpu-shift.log`)

| Preset | Nodes | Median relative shift | Most | Trunk base (m) |
|---|--:|--:|--:|---|
| date-palm | 993 | 0 | 0 | 0.297000, unchanged |
| european-beech | 249,111 | 4.6e-6 | 1.2e-5 | 0.253494 -> 0.253495 |
| ordinary | 152,173 | 3.5e-7 | 1.4e-6 | 0.145112, unchanged |
| silver-birch | 302,660 | 2.0e-6 | 7.2e-6 | 0.138982, unchanged |
| oregon-white-oak | 284,355 | 6.3e-7 | 3.4e-6 | 0.294153, unchanged |
| norway-spruce | 1,005,582 -> 1,005,581 | (one node fewer) | | |
| laurelin | 3,139,945 -> 3,139,949 | (four nodes more) | | |
| telperion | 3,363,723 | 8.6e-6 | 5.4e-5 (a 0.0296 m limb) | 1.766938 -> 1.766953 |

The spruce and Laurelin differ by one and four fine nodes, a twig on either side of the skeleton's fine cut. The stills (`raw/{oak,beech,telperion}-today-left-frozen-right.png`, whole above, middle below, today left): no visible change; 1 to 7 pixels of 691,200 differ on the oak and the beech, 83 and 109 on Telperion.

### Census after lever 3 (decimal MB)

| Preset (target) | Product build | Grown and laid | Paused at H-1 | Grown on from H-1 beside the drawn | Drawn at H beside the drawn | Hash | Warm ms (base) |
|---|--:|--:|--:|--:|--:|---|--:|
| date-palm (32) | 32 | 32 | 5 | 32 | 5 | f22d2412f12fe872 | 74 (73) |
| european-beech (250) | 282 | 282 | 107 | 306 | 234 | 5bf0ca47168fe174 | 201 (181) |
| ordinary (250) | 169 | 169 | 74 | 193 | 151 | d0e75decaf420b9 | 457 (479) |
| silver-birch (450) | 520 | 520 | 250 | 569 | 421 | fa423edb42519a16 | 220 (223) |
| oregon-white-oak (650) | 782 | 782 | 438 | 834 | 633 | ef8a535a5d6a4ebd | 233 (244) |
| norway-spruce (1,200) | 1522 | 1522 | 937 | 1686 | 1611 | f8aad0283da4b221 | 449 (472) |
| laurelin (2,000) | 2281 | 2281 | 1461 | 2735 | 2836 | 3de23359a4524498 | 1156 (1171) |
| telperion (2,000) | 2484 | 2484 | 1602 | 2968 | 3072 | ee97618751d508ba | 1248 (1244) |

The standalone peak falls 41% on Telperion (4,209 to 2,484) and 41% on Laurelin (3,853 to 2,281); the continuation peak is 3,072 MB on Telperion and 2,836 on Laurelin, under 4 GiB. Against the targets: the palm and the ordinary meet theirs; the beech misses by 32 MB, the birch by 70, the oak by 132, the spruce by 322, the Two Trees by 284 and 484 (a target the host released, R4). What holds Telperion now: the state's tables at its start sizes (pencils 483 at their hint, axes 426), its phytomers (laid 426, grown 152, sized by the beech's 2.25), the cycle's job counts and the hierarchy's sums; the settle's scratch (about 1,050 MB) is the later spec's.

### The oak at 80, recalibrated

Its own ratios at 80 after lever 3 (`raw/t3-l3-oak80-ratios.log`, grown from the least tables): axes 0.12 to 0.37 of its hint, units 0.09 to 0.20, phytomers 0.25 to 0.59, where the margin starts it at 1.43, 1.02 and 2.25: its first tables alone want 4.2 GB, and it is refused by name before its first cycle. Measured (`raw/t3-x3-*`): at a start margin of 0.5 it builds at 3,715 MB (inside the 4 GiB default), at 0.3 at 2,299 MB; but at 0.5 Telperion takes 1,368 ms (+10%), the spruce 507 ms (past 0.5 s) and the ordinary 496 ms, at 0.3 the ordinary 548 ms. The margin stays 1.0: the oak at 80 is reported missed, its floor 2,299 MB at a margin of 0.3, at the cost of R0a's times.

### The Wasm core (the CPU grower) after lever 3 (`raw/t3-l3-wasm.log`)

Telperion 2,813 MB (was 3,114), Laurelin 2,712 (was 2,984), spruce 1,250, oak 334, birch 271, beech 177, ordinary 131, palm 2: the records the CPU no longer keeps. No CPU lever was added (host ruling).

## Gate

`cargo test --profile ci --workspace --no-fail-fast` once (`raw/t3-gate.log`): 1,356 passed, 1 failed, 89 ignored over 139 binaries. The failure, `young_trees_grown_with_retirement_are_the_reference_the_settle_is_held_to`, asserted the retiring grower's tree is the reference's to the bit, which the frozen girth now moves by design; it is held to a girth bound (1e-4, the young presets' most 1.4e-6) and passes (`raw/t3-gate-fix.log`). `npm test`: 162 of 162 (`raw/t3-npm.log`), the unsigned Wasm addresses of task 2 included.
