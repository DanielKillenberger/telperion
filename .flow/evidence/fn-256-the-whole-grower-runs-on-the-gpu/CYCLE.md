# fn-256 task 2: the whole grower on the device, from the seed (worker, 2026-10-11)

The device grows the tree from the seed through fn-255's cycle, every phase on one resident state, each dispatch sized on the device. This file records what ran and what it measured. It does not judge the budget.

## How it ran

- **Device:** NVIDIA GeForce RTX 3080, Vulkan, native wgpu 30, `crate::Gpu` (WebGPU's 8 storage bindings a stage; the adapter's 2,147,483,644-byte largest storage binding).
- **Code:** `crates/telperion-render/src/generation/cycle/`, test-only.
  - The state (`state.rs`): axes (the units' 35 columns, the layout's 16 beside them), pencils, units, the phytomers' columns the units write (`grown`, 5) and those the layout writes (`laid`, 14), and the units' lists. Each column is contiguous, `cap` rows apart; the 8-binding limit keeps them in these buffers, and the 2 GiB limit splits the phytomers in two.
  - The seed (`units.wgsl` `seed_main`) and each cycle in fn-255's order (`build_plan.rs`): the hierarchy's step and the balance and allotment (cycle 2 on), the units and the wake, the new axes' and phytomers' columns, then before the age the layout and the light. The modules read the shared columns; small glue kernels (`glue.wgsl`) move what passes between them.
  - Retirement is not run (DESIGN.md section 6).
- **Sizes from the device** (`plan.rs`, `vm.wgsl`, `census.rs`): a cycle is recorded once and replayed. At its points short programs turn the device's counts into each dispatch's uniform words and indirect workgroups. The host knows only the cycle, the lists' parity, and whether the cycle is the first or the last.
  - Loops with device-only counts run to bounds: the hierarchy's generations, placing passes and block levels, the layout's generations, the sweep's layer ranges. The units', draw's, deposit's and lattice's dispatches are cut into ranges of a constant count.
  - The census is read a cycle late. A count that would pass its table or bound within two cycles at its rate moves the build into larger tables on the queue (`build_move.rs`). A count already past one begins the build again from the seed with that size raised.
- **Exactness switch:** `Arith::Plain` (default) or `Arith::Exact`, chosen when the kernels are built (`cycle.rs`). Exact builds every marked product apart and every marked quotient and root correctly rounded: the units' and layout's experiments, the hierarchy's `hm`/`hdiv`, and the light's read.
- **CPU reference:** `telperion_space::grow_capture_grown(_at)`. The grower is set up as `run` sets it up, with every axis open and none retired (`Working::reference`), so every phytomer is held.
- **Logs:** `raw/grown-final-1.txt`, `raw/grown-plain-3.txt`, `raw/time-5.txt`, `raw/compile-cold-cycle.txt`, `raw/gates-final-1.txt`.

## Agreement

**Built exact, from the seed** (`young_trees_grow_on_the_device_as_on_the_cpu`): every word of the grown state equals the CPU's, for all eight presets. The words compared are 138 columns: each axis's 25 columns and 11 layout words, its units and phytomers in order, the sleeping buds left, the woken buds, the living apexes, the pencils' 59 words and the phytomers grown. The tables start at the least sizes and grow, so the moves are exercised too.

| Preset, age (seed 42) | Axes | Phytomers | Exact |
|---|--:|--:|---|
| european-beech 8 | 299 | 1,941 | every word |
| oregon-white-oak 9 | 1,750 | 9,145 | every word |
| norway-spruce 12 | 12,848 | 31,914 (182 woken) | every word |
| date-palm 8 | 1 | 96 | every word |
| silver-birch 8 | 1,193 | 4,852 | every word |
| ordinary 10 | 1,056 | 4,761 | every word |
| laurelin 8 | 299 | 1,941 | every word |
| telperion 8 | 299 | 1,941 | every word |

**Built plain** (`young_trees_built_plain_are_reported`, nothing asserted beyond completion):

| Preset | First word that parts | First integer that parts | By lineage at the age |
|---|---|---|---|
| beech, laurelin, telperion 8 | cycle 1, axis base x (f32) | none | 0 axes unmatched |
| oak 9 | cycle 1, axis base x | none | 0 unmatched; tips ≤ 5.6e-4 relative, sides ≤ 1.9e-2 |
| spruce 12 | cycle 1, tip x | cycle 4, a sleeping bud's sleep (1 of 14; 982326 against 982325 ticks) | 0 unmatched |
| date-palm 8 | cycle 1, tip x | none | 0 unmatched |
| silver-birch 8 | cycle 1, base x | none | 0 unmatched |
| ordinary 10 | cycle 1, axis rank | none | 0 unmatched |

The floats part from the first cycle in their last places, as UNITS.md, LAYOUT.md and HIERARCHY.md found per phase. At the default ages, the oak 80 seed 1 grows 10,056,655 phytomers plain and 10,056,656 exact. Telperion 82 grows 18,239,310 both ways, the CPU's count in STEP2.md.

## Determinism (R2)

`the_grown_state_is_the_same_on_every_context_and_partition`, built plain. The oak 9 and the spruce 12 grown state is the same in all 138 columns:
- over 10 fresh device contexts;
- over hierarchy block caps 64, 128 and 1,024;
- over two sets of small ranges: units 64 and 1,024 apexes a dispatch, draw 128 and 4,096, deposit 192 and 8,192, lattice nodes 64 and 2,048, paths 640 and 16,384.

## Whole builds (`whole_builds_are_timed`, seed 42 unless named, median not taken: one stamped run at the sizes a first run found)

GPU ms of the stamped dispatches by phase over the whole build. "Sizing" is the census programs. "First to last" includes the gaps between dispatches. Wall is the host's time from the seed to the last census read.

| Plain | Units and wake | Layout | Light | Balance and allotment | Sizing | Stamped | First to last | Wall | Dispatches |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| european-beech 40 | 54.9 | 29.6 | 6.3 | 154.5 | 7.0 | 252.6 | 391 | 0.40 s | 20,666 |
| oregon-white-oak 40 | 82.2 | 42.6 | 18.6 | 179.9 | 7.1 | 330.6 | 488 | 0.49 s | 23,827 |
| norway-spruce 40 | 181.2 | 54.2 | 11.6 | 186.9 | 7.0 | 441.3 | 603 | 0.62 s | 24,122 |
| date-palm 40 | 23.4 | 13.4 | 4.9 | 81.2 | 6.7 | 129.9 | 236 | 0.24 s | 16,294 |
| silver-birch 40 | 95.5 | 42.8 | 8.2 | 174.7 | 7.2 | 328.7 | 482 | 0.49 s | 23,907 |
| ordinary 40 | 134.4 | 47.8 | 15.8 | 192.8 | 7.0 | 398.1 | 571 | 0.58 s | 26,592 |
| laurelin 80 | 409.6 | 151.9 | 57.9 | 543.4 | 14.5 | 1,178.1 | 1,577 | 1.62 s | 57,822 |
| telperion 82 | 445.4 | 161.8 | 63.2 | 569.7 | 14.9 | 1,255.7 | 1,690 | 1.75 s | 59,936 |
| oregon-white-oak 80 seed 1 | 426.9 | 147.5 | 77.6 | 502.8 | 14.4 | 1,170.0 | 1,517 | 1.56 s | 53,939 |
| **Exact** oregon-white-oak 40 | 151.9 | 58.8 | 18.4 | 229.7 | 7.1 | 466.1 | 618 | 0.62 s | 23,827 |
| **Exact** telperion 82 | 760.1 | 212.9 | 62.3 | 631.1 | 14.9 | 1,682.1 | 2,116 | 2.19 s | 59,936 |
| **Exact** oregon-white-oak 80 seed 1 | 831.9 | 199.8 | 77.4 | 656.1 | 14.5 | 1,780.4 | 2,128 | 2.17 s | 53,939 |

"Start" (the living taken in, under 1 ms) is left out. The balance and allotment are dispatch-bound: 46,737 of Telperion's 59,936 dispatches, about 12 µs each, most of them the hierarchy step's placing passes recorded to their bounds.

**Sizing a cold build** (from the least sizes, with its moves and restarts):
- Telperion 82: 1.35 s plain, 59 moves.
- Laurelin 80: 1.29 s, 57 moves.
- The oak 80: 1.32 s, 60 moves.
- The silver birch 40 began again twice: its axes and then its units and phytomers outgrew the projection at cycles 36 and 38.

**Longest dispatch** (plain):
- ordinary 40: units `cycle_main`, 1.62 ms;
- Telperion 82: 1.01 ms, the same kernel;
- every other preset: 0.27 to 0.98 ms.

The units' longest is one range's slowest apex. Halving the range to 8,192 apexes left the ordinary at 1.43 ms and cost 47% more units time on Telperion (`raw/time-4.txt`), so the range stayed 16,384.

Built exact, the longest are:
- the normaliser's `partials_main`, 1.93 to 2.24 ms: one workgroup sums its block of 1,024 buds serially, the contract's block, which a range cannot cut;
- units `cycle_main`, 1.62 ms.

**Memory** (at the sizes the build grew to):

| Build | Resident | Scratch |
|---|--:|--:|
| Telperion 82 | 3,160 MB | 525 MB |
| Laurelin 80 | 2,702 MB | 525 MB |
| The oak 80 | 2,108 MB | 458 MB |
| The spruce 40 | 962 MB | 244 MB |

- **Resident:** the state, the hierarchy's shape and lists, the tables.
- **Scratch:** the sums, the tree view, the light, the layout's lists, the counts and scans. The plan's 2 MB of uniforms and 128 KB of arguments come on top.
- **Telperion's tables:** 1,833,088 axes, 5,949,046 units, 23,950,613 phytomers. The largest buffer, the layout's phytomer columns, is 1.34 GB.

## Compile (`the_kernels_compile`, the driver's disk cache off)

| Module | Plain | Exact |
|---|--:|--:|
| units and wake | 37.67 s | 69.68 s |
| layout | 3.68 s | 4.29 s |
| hierarchy | 0.33 s | 0.38 s |
| light | 0.91 s | 0.88 s |
| glue and sizing | 0.08 s | 0.08 s |
| **all** | **42.67 s** | **75.31 s** |

Warm, from the driver's cache: 0.02 s or less a module.
