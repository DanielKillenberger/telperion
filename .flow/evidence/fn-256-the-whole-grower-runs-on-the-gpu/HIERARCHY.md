# fn-256 task 2: the hierarchy, balance and allotment on the device (worker, 2026-10-11)

DESIGN.md sections 1.6, 2 (steps 1, 2 and 5), 3 and 4. This file records what ran and what it measured. It does not extrapolate to a build or judge the budget.

## How it ran

- **Device:** NVIDIA GeForce RTX 3080, Vulkan, native wgpu, through `crate::Gpu`, as UNITS.md.
- **Capture:** `telperion_space::grow_capture_hierarchy_at` (`grow/cycle/capture_hierarchy.rs`) grows the one cycle to the captured cycle, recording the axes laid at each cycle's start. It takes each laid axis's bearer, node, PA, lineage and pencil, the living apexes and each PA's row, then runs `open`, `retire`, `balance_one` and `allot_one` and takes what they leave: each axis's children in `Working::children`'s order, the open axes and their sums, each pencil's memory, kept share and size, the apexes shed and the living after.
- **Replay:** `generation/cycle/hier_*.rs` and `hier_*.wgsl`, test-only, registered with the naga check as "cycle hierarchy build", "cycle hierarchy sums" and "cycle hierarchy scan".
- **The build** (`hier_build.wgsl`, `hier_blocks.wgsl`, `hier_members.wgsl`) takes in one cycle's laid axes at a time:
  - Depth and generation, then the children: counted onto their bearers by atomic adds, scanned, placed in a bucket, sorted by (node, index) and merged with the old run at its new start, into the second list.
  - Blocks by DESIGN.md section 3's revised rule (sibling spills, commit d8388e42e), one generation of the cycle at a time, in placing passes:
    - A waiting axis targets its bearer's block while it has room, else that block's open spill while it has room.
    - A target's waiters join it in index order up to `cap`; the rest wait for the next pass.
    - Where both are full, the block's waiters fill fresh spills of it in index order, `cap` each, a level below it; the last becomes its open spill.
    - Openers are numbered by a scan in index order. Ranks in index order are counted over each target's waiters.
    - Two or three passes a generation on the busy cycles. The host reads back the waiting count after each pass.
  - Each block's members by (depth, index): the new members are ranked within their block, then old and new members find their places by a binary search of the other run, all in flat dispatches. Each level's blocks are merged by index (a workgroup scan per level).
  - No index comes from an atomic slot. An atomic places a record in a bucket only for the rank or sort that follows.
  - The host reads back the generation count after the depth pass and the blocks per level at the step's end, the small reads DESIGN.md section 5 allows, to size the dispatches.
- **The scan** (`hier_scan.wgsl`): an exclusive prefix sum in levels of 256 words, a workgroup each, up then down. It replaces `fine/scan.wgsl`'s one-invocation spine in the hierarchy, and `fine/scan.wgsl` is unchanged.
- **The sums** (`hier_cycle.wgsl`, `hier_sweep.wgsl`, `hier_split.wgsl`) run over the whole tree:
  - Bottom-up: one workgroup per block, looping the block's depths (its first member's to its last's) deepest first, a barrier between them; levels deepest first. Top-down: the reverse.
  - A block's least and greatest depths are read from its sorted run's ends. DESIGN.md keeps them as order-free min/max; the values are the same.
  - The passes: mark, totals, walk (draws keyed as `passes.rs` keys them), shed (flag, scan, compact), presence, sub (with growing buds counted), split (with each growing bud's preorder place written to the order), the normaliser's blocks of 1,024 per PA, their sum in order, then sizes.
- **The f64 products:** presence and lit presence (`base_scale × running × kept`, then times the light) are formed in `hier_real.wgsl` as 106-bit integer products rounded to 53 bits. `the_allotments_products_are_the_cpus` finds 0 of 1,048,576 products, and 0 of their f32s, differing from the CPU's.
- **Sites marked:** every product an add takes is written `hm` and every quotient `hdiv`, plain operations, as `q.wgsl` marks the layout's.
- **Logs:**
  - Under the first block rule: `raw/hier-young-1.txt`, `raw/hier-busy-1.txt`, `raw/hier-young-cold.txt`; the experiment `raw/hier-young-exact-2.txt` and `-3.txt`.
  - Under sibling spills: `raw/hier-spill-young-5.txt` and `raw/hier-spill-busy-1.txt`.
  - The host simulations: `raw/hier-spill-probe-1.txt` (the chained wording) and `-2.txt` (sibling spills).

## Blocks: the rules measured

The rule was simulated on the host first: sequentially, each cycle's new axes by generation and then index order, from the captures' parents and laid counts.

| Rule | Tree | Cap 64 | Cap 256 | Cap 1,024 | Cap 4,096 |
|---|---|--:|--:|--:|--:|
| **Sibling spills** (blocks / levels / axes a block) | Telperion 82 | 30,932 / 34 / 47.3 | 7,886 / 25 / 185.6 | 2,005 / 20 / 730.1 | 510 / 17 / 2,870.4 |
| | Oak 80, seed 1 | 30,352 / 23 / 48.5 | 7,892 / 19 / 186.7 | 1,993 / 17 / 739.2 | 513 / 15 / 2,871.7 |
| Chained spills (first wording), blocks = levels | Telperion 82 | 22,874 | 5,719 | 1,430 | 358 |
| | Oak 80, seed 1 | 23,019 | 5,755 | 1,439 | 360 |
| A block per overflowing child (first rule), device | Telperion 82 | | | 166,539 / 6 | |
| | Oak 80, seed 1 | | | 161,263 / 7 | |

- **The device under sibling spills, at cap 1,024:** Telperion 2,005 blocks on 20 levels, the oak 1,993 on 17. Both equal the simulation.
- **The young trees at cap 1,024:** oak 30 at 30 has 89 blocks on 6 levels (88 on 7 built at once); the spruce has 39 on 5.

## Agreement

`young_hierarchies_and_their_integers_agree_with_the_cpu` passes on seven captures: the oak 16 at 12 and 30 at 30, Telperion 14 at 10, ordinary 16 at 12, norway-spruce 24 at 16, european-beech 10 at 8 and silver-birch 16 at 12.

**Exact on every capture, the busy ones included:**
- **Children:** every axis's children, in the CPU's order.
- **The blocks' rules:** at most `cap`; every axis in its bearer's block or a spill of it; each spill a level below the block it spills from; each open spill one of its block's own; depths, members and level lists ordered; every block listed once.
- **Built at once against cycle by cycle:** the children are identical.
  - The blocks differ (oak 30 at 30: 88 blocks at once, 89 cycle by cycle), because the placing follows generations.
  - Every sums word is still identical.
- **Caps 64, 256, 1,024 and 4,096:** every word of the sums and the living after is identical.
- **Integers:** living at the open axes, the apexes shed, the living after with their times.
- **f32s:** light, upkeep and lit presence at the open axes, and every kept share.
- **The oak sheds by its balance:** 2 apexes at 30 and 1,931 at 80.

**Not exact, in their last places:**

| Capture | Incoming | Bud vigour | Memory | Size |
|---|--:|--:|--:|--:|
| oregon-white-oak 16 at 12 | 1,995 / 2,346 | 1,958 / 2,346 | 226 / 3,496 | 2,032 / 3,496 |
| oregon-white-oak 30 at 30 | 14,292 / 16,509 | 13,305 / 16,509 | 1,827 / 66,981 | 13,105 / 66,981 |
| Telperion 14 at 10 | 246 / 289 | 227 / 289 | 0 | 0 |
| ordinary 16 at 12 | 816 / 958 | 629 / 958 | 95 / 1,398 | 532 / 1,398 |
| norway-spruce 24 at 16 | 12,305 / 13,884 | 12,260 / 13,884 | 0 | 0 |
| european-beech 10 at 8 | 127 / 192 | 133 / 192 | 0 | 165 / 228 |
| silver-birch 16 at 12 | 1,934 / 2,388 | 1,892 / 2,388 | 0 | 2,116 / 2,962 |
| Telperion 82 (busy) | 251,084 / 264,858 | 221,516 / 264,858 | 73 / 1,463,911 | 32,716 / 1,463,911 |
| oregon-white-oak 80, seed 1 (busy) | 129,107 / 147,064 | 117,147 / 147,064 | 14,528 / 1,473,173 | 121,554 / 1,473,173 |

Incoming and bud are compared at the open axes. Memory and size are compared at every laid axis.

**The causes, each confirmed by an uncommitted experiment:**
- **Incoming and bud:** the split's fused products and its divisions.
  - Device: `hier_split.wgsl:46` (`l·main + (1-l)·lateral`), `:51`, `:54`, `:60`, `:64`.
  - CPU: `working.rs:224`, `:227`, `:229`, `:235`, `:237`.
- **Memory:** `now`'s division, `hier_sweep.wgsl:100` against `contract/shed.rs:42`.
- **Size:**
  - The normaliser's fused product, `hier_cycle.wgsl:250` against `working.rs:278`.
  - The size's product and division, `hier_cycle.wgsl:292` against `allocation.rs:189`.
  - `drawn`'s `log2c` (division and fusion, `fine/contract.wgsl:64` and `:66-67`, against `contract.rs:81` and `:84`).
  - `exp2c`'s polynomial (fusion, `fine/contract.wgsl:76`, against `contract.rs:96-101`).
  - Each bud vigour that differs.
- **The experiment:** `hm` kept apart and `hdiv` correctly rounded, with `exact-units.patch` and the contract hunks of `exact-layout.patch` applied. Under it, **all seven young captures agree in every word.** Without the contract hunks, sizes still differ (`raw/hier-young-exact-2.txt`).
- **As committed:** `young_sums_agree_with_the_cpu_in_every_word` keeps the exact assertion and is `#[ignore]`d until the host rules on the f32 contract (STEP2.md, call 1).

## Timing: the busy cycle, median of 9 (GPU ms in stamped passes)

The run restores the state before the last step by unstamped copies. It then records the last build step and the sums in one submission. This is under sibling spills, at cap 1,024. "Longest" is the median over runs of an entry's longest single dispatch.

| | Telperion 82, seed 42 | | oregon-white-oak 80, seed 1 | |
|---|--:|--:|--:|--:|
| | **Total** | **Longest** | **Total** | **Longest** |
| Laid axes / new in the step | 1,463,911 / 67,430 | | 1,473,173 / 58,504 | |
| Living apexes | 233,233 | | 132,098 | |
| Generations / placing passes | 2 / 3 | | 3 / 4 | |
| Blocks / block levels | 2,005 / 20 | | 1,993 / 17 | |
| Dispatches (the step's / the sums') | 163 (71 / 92) | | 149 (69 / 80) | |
| **Build:** children (place, merge) | 0.13 | 0.12 | 0.13 | 0.12 |
| **Build:** depth and placing passes | 0.16 | 0.04 | 0.19 | 0.05 |
| **Build:** members (count, place, rank, old, new, starts) | 0.16 | 0.06 | 0.18 | 0.08 |
| **Build:** levels merge | 0.01 | 0.01 | 0.01 | 0.01 |
| **Scans,** all (up, down) | 0.20 | 0.02 | 0.18 | 0.02 |
| **Sums:** totals | 3.95 | 0.27 | 2.79 | 0.26 |
| **Sums:** walk | 2.04 | 0.15 | 1.17 | 0.09 |
| **Sums:** sub | 3.39 | 0.23 | 2.38 | 0.23 |
| **Sums:** split | 6.74 | **0.49** | 4.31 | **0.45** |
| **Sums:** normaliser (partials, combine) | 0.34 | 0.30 | 0.47 | 0.44 |
| **Sums:** per apex (mark, shed, compact, presence, sizes) | 0.07 | 0.02 | 0.05 | 0.02 |
| **First stamp to last** | **18.29** | | **12.88** | |
| Longest dispatch | `split_main`, 0.50 | | `split_main`, 0.45 | |
| Scratch | 258.4 MB | | 257.1 MB | |

Against the first rule's run (same trees, cap 1,024):
- **Longest dispatch:** 0.50 / 0.45 ms against 1.80 / 2.78 ms (`member_merge_main`, now gone).
- **Scans:** 0.20 ms against 3.58 ms.
- **Members:** 0.16 ms against 1.67 ms.
- **The four sweeps** take longer: 16.1 / 10.7 ms against 6.1 / 5.8 ms. The cycle is 18.3 ms against 13.0 on Telperion, and 12.9 ms against 13.8 on the oak.
  - About 2,000 blocks on about 20 levels give each sweep dispatch about 100 workgroups of 64 lanes. Each loops its block's depths (a range up to some tens) with a barrier per depth.
  - The first rule's 166k small blocks gave many more workgroups and shorter loops.
  - Whether to change the sweeps (wider workgroups, or a level split by depth range) is not judged here.
- **Compile:** the three modules in 0.28 s on the first run of the new sources, 0.01 s from the driver's cache. A cold run with the cache off was not repeated for this version (0.31 s before).

## Open, for the host

1. **The f32 contract,** as STEP2.md call 1. Every integer, order and flag agrees here. Incoming, bud, memory and size agree only with the sites held to the CPU's order.
2. **The sweeps under sibling spills** are now the cycle's cost: 16.1 ms of 18.3 on Telperion. Every dispatch is under 0.5 ms.
