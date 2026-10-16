# fn-257 task 9: growth's tables, narrowed losslessly where they are made (worker, 2026-10-16)

## Method

- **Peaks:** `growth_profile_tests::growth_tables`, warm, RTX 3080, the spruce at 40 and Telperion at 82, seed 1 (`raw/t9-tables.log`). Growth's peak is 1,624.3 MB on the spruce and 2,740.2 MB on Telperion, so 2 % is 32.5 MB. Each measurement polls the device before its peak restarts, so the last tree's buffers are let go first (the review's finding: without it the palm after the birch read 62.6 MB, 34.7 alone).
- **Identity:** `drawn_hashes` hashes every shipped preset's drawn tree at seeds 1, 7 and 42, base and narrowed binaries in one run (`raw/t9-hashes.txt`).
- **Time:** `drawn_warm` runs warm builds; the base and narrowed binaries were alternated, four times four builds each (`raw/t9-warm-order.txt`).

## The inventory

Rows: the spruce's tables hold 1,090,688 axes (and pencils), 1,765,374 units and 2,581,741 phytomers (Telperion 1,572,224, 4,559,713 and 6,668,273). A column of the axes is 4.4 MB on the spruce; a phytomers' column is 10.3 MB.

| Table | Rows | Words a row | Spruce MB | Telperion MB | Columns by kind | Narrowest lossless | Saving (spruce) |
|---|---|--:|--:|--:|---|---|--:|
| pencils | axes | 63 | 274.9 | 396.2 | 59 f32 (walk, pulls, 8 lifts × 4, scale, going, rank, wood, fresh, take, line); `P_LIFTS_N` (≤ 8); `P_TRUNK` (flag); `P_UNITS`, `P_LAID` (row ids, 26 bits) | the count and the flag in one word | 4.4 MB (0.3 %) |
| axes | axes | 52 | 226.9 | 327.0 | ids and row links (parent, node, phyts, units, tail, head, successors: 26+ bits); 64-bit keys (lineage, root, whorl, blend); ticks and f64 pairs (birth, spent, horizon, b0, b1); flags (`A_FLAGS` 6 bits, `A_ORIGIN` already packed, `A_FAULT`); small counts (`A_PA`, `A_END` ≤ age + 1); layout f32 (whorl, blend, b0, b1, phase, rank, base, heading, side) | `A_PA`, `A_END`, `A_FLAGS`, `A_FAULT` and the three bud flags into two words | about 5 columns, 22 MB (1.3 %) |
| counts | the units' jobs (65,536 at most) | 575 a job, then 70 a zone (3 on the spruce, 4 on Telperion) | 210.9 | 237.4 | job records: 130 words, then 11 frames of an axis's 35 columns, then a 10-entry stack; a zone: 6 words, then its 64 nodes' order, each an index under 64 in a word | **the order a byte a node: 64 words to 16** | **37.7 MB (2.3 %), taken** |
| phytomers laid | phytomers | 14 | 144.6 | 373.4 | 14 f32 (tip, heading, side, scale, grown, size, rank, area) | none lossless | 0 |
| units | units | 13 | 91.8 | 237.1 | ids (axis, prev, next, first); f32 survival and persistence; ticks (share, size); two f64 that hold f32 (`lay_state.wgsl`: shade size and light); count | the two f64 as f32 | 14.1 MB (0.9 %) |
| phytomers grown | phytomers | 5 | 51.6 | 133.4 | 64-bit key, axis id, cycle (≤ age), f32 presence | none past a word (the cycle has no small neighbour) | 0 |
| hierarchy sums | axes | 19 | 82.9 | 119.5 | `hier_cycle.wgsl`'s `S_`: flags lit, shed, growing, gone (each written 0 or 1); rank (a place in subtree order, 26+ bits); light, cost, living, keep, memory, kept, size, own, presence, sub, buds, incoming, bud, rest (f32) | the four flags in one word | 13.1 MB (0.8 %) |
| cycle tree | axes | 15 | 65.4 | 94.3 | `hier_cols.wgsl`'s `T_`: bearer and node (ids), PA (small), lineage (64-bit key), wood and light (f64 bits), base, running, kept, memory, size (f32), frozen upkeep (f32) | the PA beside the node if its range allows: unchecked | at most 4.4 MB |
| light depths | the lattice's nodes × directions | | 59.8 | 154.2 | a depth a node a direction (`light_device.rs`), f32 | none lossless | 0 |
| retire columns | axes | 13 | 56.7 | 81.8 | `retire_cols.wgsl`'s `R_`: open, wake, wsub (cycles), fate (0-3), pend (flag); lived, alive, qb1, root lived and alive, frozen radius and sums (f32) | the cycles, fate and pend in two words | 13.1 MB (0.8 %) |
| retire verdicts | axes | 11 | 48.0 | 69.2 | the cycle's own `C_`: closed (flag), gap (cycles), lived, end, before, weight and girth (f32) | closed beside gap | 4.4 MB |
| hierarchy counts | axes | 12 | 52.4 | 75.5 | `atomic<u32>` counters | WGSL's narrowest atomic is 32 bits; packing two needs proven bounds | 0 |
| inputs, shape, lists, buckets, light points, layout lists and the rest | | | under 32 each | | | under 2 % each | |

## The step taken: a zone's order, a byte a node

`grow.wgsl`'s `order_zone` writes each zone's sorted node indices (`zones.wgsl`'s `sorted`, under `MOST` = 64), four a word, where it makes them; `ordered` reads a byte. `device.rs`'s `ZW` is 6 + 16 words a zone, which the counts buffer is sized from.

| | Spruce 40 peak | Telperion 82 peak | Spruce warm `tree::drawn` | Telperion warm |
|---|--:|--:|--:|--:|
| Base | 1,624.5 MB | 2,740.3 MB | 391.0 ms (sd 2.7, n 16) | 856.4 ms (sd 5.6) |
| A zone's order a byte a node | **1,586.8 MB** | **2,690.0 MB** | 389.4 ms (sd 2.4) | 857.1 ms (sd 4.4) |

Every preset's peak, base → narrowed, the same at seeds 1, 7 and 42 except where noted: ordinary 193.9 → 180.1, oak 909.8 → 872.0, beech 220.4 → 201.0 (325.0 → 296.0 at 42), birch 614.6 → 576.8, palm 34.7 → 33.9, Laurelin 2,519.9 → 2,469.5 MB.

Every shipped preset's tree is byte-identical at seeds 1, 7 and 42: 24 of 24 hashes. The warm time, which is the harness's build, is unchanged within the spread.

**The next step stops:** the largest remaining lossless narrowing is the axes' small counts and flags, about 22 MB (1.3 %), under 2 % of the spruce's peak. Each other table's step is 13 MB or less.

## Not narrowing: room held unused (for the host)

Two tables hold fixed room that most rows never use. Taking it back is a layout change with an allocator, not a narrowing:

- **A pencil's eight lifts:** 32 f32 columns, 140 MB on the spruce. `P_LIFTS_N` says how many a pencil holds; a variable list would keep only those.
- **The units' jobs:** 65,536 jobs × 11 frames × 35 words, about 126 MB on both trees. Most jobs hold one or two frames. A smaller `RANGE` halves the counts but doubles the units' ranges dispatched.

## Quantising growth's own floats (reported for fn-285, not taken)

| Floats | Spruce MB at f32 | Saving at 16 bits | Read by | Risk |
|---|--:|--:|---|---|
| pencils' walk, pulls and lifts (59 columns) | 257 | about 128 | `lay_walk`, `lay_frame`, `lay_stand`: the layout's walk accumulates them step by step | high: rounding accumulates along an axis, so tips move and every laid position downstream changes |
| phytomers laid (14 columns) | 145 | about 72 | layout, `settle`, the dress | positions: a half at 30 m is a 16 mm step, visible, so fixed point within the tree's box (20 bits) is needed. Heading and side are unit vectors: octahedral 2 × 16 bits keeps them to about 1e-4 rad, saving 4 of 6 words |
| axes' layout f32 (13 columns) and ticks/f64 pairs | 57 + | about 28 | survival, presence and abortion draws compare them against thresholds | high: a threshold compare that flips changes topology (a bud lives or dies), so the tree is no longer the same tree |
| units' survival, persistence (f32) | 14 | about 7 | `decide.wgsl` draws | high, as above |
| the hierarchy's sums, the cycle tree's and retirement's f32 (about 33 columns) | about 145 | about 72 | the balance and allotment (`hier_cycle.wgsl`, `hier_sweep.wgsl`) and retirement's verdicts | high: these sums decide which apexes are shed and which buds grow |
| light depths | 60 (154 on Telperion) | about 30 | the light sweep's shading | moderate: light feeds survival through the same thresholds |

The safe quantisation is the geometry growth never compares against a draw: headings and sides octahedral, positions as fixed point in the box. That is about 80 MB on the spruce (5 %) and changes the trees' bytes, so fn-285 would need the owner's look at stills. Everything a draw is compared against has to stay as it is.
