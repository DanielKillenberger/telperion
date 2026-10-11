# fn-256 task 2: the hierarchy, balance and allotment on the device (worker, 2026-10-11)

DESIGN.md sections 1.6, 2 (steps 1, 2 and 5), 3 and 4. This file records what ran and what it measured. It does not extrapolate to a build or judge the budget.

## How it ran

- **Device:** NVIDIA GeForce RTX 3080, Vulkan, native wgpu, through `crate::Gpu`, as UNITS.md.
- **Capture:** `telperion_space::grow_capture_hierarchy_at` (`grow/cycle/capture_hierarchy.rs`) grows the one cycle to the captured cycle, recording the axes laid at each cycle's start. It takes each laid axis's bearer, node, PA, lineage and pencil, the living apexes and each PA's row, then runs `open`, `retire`, `balance_one` and `allot_one` and takes what they leave: each axis's children in `Working::children`'s order, the open axes and their sums, each pencil's memory, kept share and size, the apexes shed and the living after.
- **Replay:** `generation/cycle/hier_*.rs` and `hier_*.wgsl`, test-only, registered with the naga check as "cycle hierarchy build" and "cycle hierarchy sums".
- **The build** (`hier_build.wgsl`, `hier_blocks.wgsl`) takes in one cycle's laid axes at a time:
  - Depth and generation, then the children: counted onto their bearers by atomic adds, scanned, placed in a bucket, sorted by (node, index) and merged with the old run at its new start, into the second list.
  - Blocks, one round per generation in the cycle: joiners counted per block, an axis joins its bearer's block while it holds fewer than `cap` (the round's lowest indices first; a block that fills ranks its joiners by index), otherwise it opens a block one level below its bearer's. Openers are numbered by a scan in index order.
  - Each block's members by (local depth, index), and each level's blocks by index (a workgroup scan per level), merged the same way.
  - No index comes from an atomic slot. An atomic places a record in a bucket only for the sort that follows.
  - The host reads back the generation count after the depth pass and the blocks per level at the step's end, the small reads DESIGN.md section 5 allows, to size the dispatches.
- **The sums** (`hier_cycle.wgsl`, `hier_sweep.wgsl`, `hier_split.wgsl`) run over the whole tree:
  - Bottom-up: one workgroup per block, local depths deepest first, a barrier between them; levels deepest first. Top-down: the reverse.
  - The passes: mark, totals, walk (draws keyed as `passes.rs` keys them), shed (flag, scan, compact), presence, sub (with growing buds counted), split (with each growing bud's preorder place written to the order), the normaliser's blocks of 1,024 per PA, their sum in order, then sizes.
- **The f64 products:** presence and lit presence (`base_scale × running × kept`, then times the light) are formed in `hier_real.wgsl` as 106-bit integer products rounded to 53 bits. `the_allotments_products_are_the_cpus` finds 0 of 1,048,576 products, and 0 of their f32s, differing from the CPU's.
- **Sites marked:** every product an add takes is written `hm` and every quotient `hdiv`, plain operations, as `q.wgsl` marks the layout's.
- **Logs:** `raw/hier-young-1.txt`, `raw/hier-busy-1.txt`, `raw/hier-young-cold.txt`; the experiment `raw/hier-young-exact-2.txt` and `-3.txt`.

## Agreement

`young_hierarchies_and_their_integers_agree_with_the_cpu` passes on seven captures: the oak 16 at 12 and 30 at 30, Telperion 14 at 10, ordinary 16 at 12, norway-spruce 24 at 16, european-beech 10 at 8 and silver-birch 16 at 12.

**Exact on every capture, the busy ones included:**
- **Children:** every axis's children, in the CPU's order.
- **The blocks' rules:** connected, at most `cap`, local depths and levels consistent, members and level lists ordered, every block listed once.
- **Built at once against cycle by cycle:** the children are identical.
  - The blocks differ: the oak 30 has 15,496 blocks at once against 7,495 cycle by cycle, because the rounds follow generations.
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

The run restores the state before the last step by unstamped copies. It then records the last build step and the sums in one submission.

| | Telperion 82, seed 42 | oregon-white-oak 80, seed 1 |
|---|--:|--:|
| Laid axes / new in the step | 1,463,911 / 67,430 | 1,473,173 / 58,504 |
| Living apexes | 233,233 | 132,098 |
| Generations in the step | 2 | 3 |
| Blocks / block levels (cap 1,024) | 166,539 / 6 | 161,263 / 7 |
| Dispatches (the step's / the sums') | 73 (39 / 34) | 88 (50 / 38) |
| **Build:** children (place, merge) | 0.12 | 0.12 |
| **Build:** blocks (depth, rounds) | 0.08 | 0.09 |
| **Build:** members (count, place, merge) | 1.67 | 2.51 |
| **Build:** levels merge | 0.42 | 0.33 |
| **Scans,** build's and the living's (blocks, spine, spread) | 0.90 + 2.63 + 0.05 | 1.07 + 2.68 + 0.06 |
| **Sums:** totals / walk | 1.62 / 0.94 | 1.56 / 0.84 |
| **Sums:** sub / split | 1.31 / 2.29 | 1.29 / 2.16 |
| **Sums:** normaliser (partials, combine) | 0.32 | 0.44 |
| **Sums:** per apex (mark, shed, compact, presence, sizes) | 0.07 | 0.04 |
| **First stamp to last** | **12.95** | **13.83** |
| Longest dispatch | `member_merge_main`, 1.80 | `member_merge_main`, 2.78 |
| Scratch (the build's counts, buckets, second list, sums, living, normaliser) | 229.2 MB | 227.7 MB |

- **The scans' spine** (one invocation over the blocks of 256) is 2.6 ms of the busy cycle, as UNITS.md found for the units.
- **The members' merge** is one invocation per block. Its longest thread sorts and merges a full block's run.
- **The blocks:** DESIGN.md section 3's rule (join the bearer's block, else open one) gives about 28 times the spike's blocks at cap 1,024 (166,539 against 5,961), averaging about 9 axes a block. The sums' four sweeps take 6.1 ms here against the spike's B schedule's 4.2 ms.
- **Compile:** both modules and the scan in 0.31 s cold (`__GL_SHADER_DISK_CACHE=0`), 0.01 s from the driver's cache.

## Open, for the host

1. **The f32 contract,** as STEP2.md call 1. Every integer, order and flag agrees here. Incoming, bud, memory and size agree only with the sites held to the CPU's order.
2. **The block rule's shape.** Incremental joining as DESIGN.md section 3 has it yields many small blocks. Whether that matters beside the scans' spine and the members' merge is not judged here.
