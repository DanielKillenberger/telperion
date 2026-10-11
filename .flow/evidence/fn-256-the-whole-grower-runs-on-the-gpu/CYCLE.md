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

## Performance pass (2026-10-11, after the host's review)

What changed:
- **Each cycle records only what its counts need** (`gate.rs`). The host sets the cycle's limits from the census it read a cycle late, each count two cycles on at its rate plus a slack:
  - the units', draw's, deposit's and lattice's ranges;
  - the hierarchy step's rounds, placing passes and block levels;
  - the layout's generations, the sweep's layer ranges, and each scan's levels.

  It records only the dispatches the limits allow and writes the limits into the census.
  - A count past a limit raises its flag. Guards are raised in a point before the dispatches they guard.
  - A raised flag halts every dispatch a later point sizes, so no kernel reads what an overrun left. The first version without the halt hung the GPU (Xid 109) on axes a short layout-generation limit left unframed.
  - The build then begins again with that limit's slack doubled. Telperion and Laurelin begin again twice from the least sizes (cycles 51 and 52); a build at its found sizes never does.
- **Shared compute passes:** unprofiled, consecutive dispatches share one compute pass.
- **The units' own product guards** (`kept_apart`, seven sites in `grow.wgsl` and `advance.wgsl`) are now the switch's `exact_kept`: plain builds them plain.
  - Young plain agreement is unchanged in every reported field: first integer to part, unmatched axes, and each float's differing rows and worst relative difference (`raw/perf-gate-3.txt`).
  - Exact still equals the CPU in every word on all eight young presets, and determinism holds.

Plain, seed 42 unless named. Wall before was with every dispatch stamped; after, the median of three unstamped runs (`raw/time-6.txt`).

| Build | Dispatches before → after | Stamped ms before → after | Wall before (stamped) → after (unstamped) |
|---|--:|--:|--:|
| european-beech 40 | 20,666 → 9,085 | 252.6 → 179.9 | 0.40 → 0.185 s |
| oregon-white-oak 40 | 23,827 → 11,141 | 330.6 → 247.9 | 0.49 → 0.249 s |
| norway-spruce 40 | 24,122 → 12,487 | 441.3 → 363.5 | 0.62 → 0.393 s |
| date-palm 40 | 16,294 → 5,538 | 129.9 → 67.0 | 0.24 → 0.071 s |
| silver-birch 40 | 23,907 → 10,661 | 328.7 → 242.3 | 0.49 → 0.252 s |
| ordinary 40 | 26,592 → 13,570 | 398.1 → 314.8 | 0.58 → 0.313 s |
| laurelin 80 | 57,822 → 26,916 | 1,178.1 → 971.0 | 1.62 → 1.049 s |
| telperion 82 | 59,936 → 28,016 | 1,255.7 → 1,042.3 | 1.75 → 1.134 s |
| oregon-white-oak 80 seed 1 | 53,939 → 26,376 | 1,170.0 → 985.9 | 1.56 → 1.045 s |
| **Exact** oregon-white-oak 40 | 23,827 → 11,141 | 466.1 → 385.4 | 0.62 → 0.387 s |
| **Exact** telperion 82 | 59,936 → 28,016 | 1,682.1 → 1,484.7 | 2.19 → 1.576 s |
| **Exact** oregon-white-oak 80 seed 1 | 53,939 → 26,376 | 1,780.4 → 1,607.7 | 2.17 → 1.665 s |

**Telperion 82, plain, after:**

| Phase | Before | After |
|---|--:|--:|
| Units and wake | 445.4 ms, 5,740 dispatches | 424.6 ms, 2,972 |
| Balance and allotment | 569.7 ms, 46,737 dispatches | 394.5 ms, 19,642 |
| Layout | 161.8 ms | 135.9 ms |
| Light | 63.2 ms | 57.0 ms |
| Sizing | 14.9 ms | 29.6 ms |

**Where the unstamped 1.134 s goes:**
- 0.881 s waiting for the census: the GPU's own time, about 1.0 s with the dispatches' gaps.
- 0.118 s recording, overlapped with the GPU except where a cycle is short.
- About 0.13 s setting up: buffers, the plan's bind groups, the seed.

Stamping itself added about 0.25 s (1.38 s stamped wall).

**What remains, Telperion 82:**
- **The balance and allotment** are mostly real work. The four level sweeps take 265 ms in 5,248 dispatches. They do not share a dispatch:
  - each level waits for the deeper ones (bottom-up) or the shallower (top-down);
  - the walk and the sub are separated by the shed, the living's compaction scan and the presences, which read the whole tree;
  - only the root-level hand-overs (totals into walk, sub into split) could share, two of about 70 dispatches a cycle, not done.

  The placing passes are 9 a cycle, and the scans 7,246 dispatches of under 10 µs each.
- **The units:** 352 ms of `cycle_main`, the count and emit run twice by design (DESIGN.md section 2.1), and 69 ms of `fine/scan.wgsl`'s one-thread spines and serial blocks.

**Plainly: growth alone, no settle, is still over 1.0 s wall on Telperion.** It is 1.13 s unstamped, about 1.0 s of it GPU time. Units and wake is 425 ms and the balance and allotment 394 ms of the 1,042 ms stamped. The oak 80 and Laurelin 80 are at 1.05 s.

**Longest dispatch:**
- Plain: 1.59 ms, the ordinary's units `cycle_main` (one range's slowest apex; under 2 ms). Every other preset is under about 1.13 ms.
- Exact: 2.01 ms, the oak 80's normaliser `partials_main`: one workgroup sums the contract's block of 1,024 buds serially.

**The cycle's scans moved onto `hier_scan.wgsl` (2026-10-11).** The units', wake's and layout generations' scans now run on the levelled scan, extended to scan the first `fields` words of records `stride` words apart. Each scan is gated by its own levels limit and flag. `fine/scan.wgsl` remains for the fine pipeline and the task-1 single-cycle units harness (`device.rs`). On Telperion 82, plain: the unstamped wall went from 1.130 s to 1.052 s (median of 3), units and wake from 425.7 ms to 363.7 ms, and layout from 136.4 ms to 119.0 ms. The final counts are unchanged (`raw/scan-before.txt`, `raw/scan-after.txt`). The exact young presets and determinism tests stay green.

## Direct dispatches only (2026-10-11, DESIGN.md section 7a)

The growth loop has no indirect dispatch (`b5aff4bbe`). Each dispatch whose size only the device knows is given the most its expression reaches over the host's bounds of the census words (`given.rs`). Its invocations exit past the true count. A sizing program flags a dispatch given too few (`Flag::Given`), and the build begins again with the words that dispatch reads widened.

**A defect in `b5aff4bbe`, not yet fixed in the worktree.** `given.rs` takes each block level's first block (`c::LVL..c::GRID`) as a count that never falls. On Telperion it does fall, so a level's width is short every few cycles and its words widen without end. The result was 62 begins again, sizing in 53 s, and then a dispatch of 65535 × 65537 workgroups, which wgpu refused. The fix is one line: drop `(c::LVL..c::GRID).contains(&w)` from `grows`. The "after" figures below are `b5aff4bbe` with that line dropped, measured from a separate copy. The worktree was held for the settle worker's retirement edits.

Plain, before (`a48f8ea9e`, indirect) and after (direct), one stamped run and the median of three unstamped runs at the sizes a first run found (`raw/direct-before-4.txt`, `raw/direct-after-3.txt`). The owner's desktop was idle for both.

| Build | Dispatches a build (a cycle), before → after | Stamped ms, before → after | Unstamped wall, before → after | Sized from the least, before → after |
|---|--:|--:|--:|--:|
| Telperion 82 | 28,293 (345) → 26,983 (329) | 1,028.8 → 1,004.5 | 1.115 s → 1.135 s | 1.71 s (2 begun again) → 1.77 s (3) |
| Oak 80, seed 1 | 26,701 (334) → 25,454 (318) | 971.9 → 946.6 | 1.018 s → 1.036 s | 1.01 s (0) → 1.05 s (1) |
| European beech 40 | 9,078 (227) → 8,559 (214) | 175.7 → 157.4 | 0.179 s → 0.172 s | 0.18 s (0) → 0.19 s (1) |

**Over-dispatch** (workgroups launched against those the true counts needed, tallied from each cycle's census as it is read):

| Build | Given | Needed | Ratio |
|---|--:|--:|--:|
| Telperion 82 | 34.8 M | 7.84 M | 4.4× |
| Oak 80 | 30.0 M | 7.48 M | 4.0× |
| Beech 40 | 0.74 M | 0.19 M | 3.9× |

**Its cost is under the noise of the phase totals.** The empty workgroups sit inside each dispatch's stamp, so no stamp separates them. The stamped totals fell by 24 ms on Telperion, 25 ms on the oak and 18 ms on the beech, and Telperion's wall rose by 20 ms.

**Per phase on Telperion:**

| Phase | Before | After |
|---|--:|--:|
| Sizing | 36.5 ms | 68.2 ms |
| Units and wake | 382.0 ms | 373.6 ms |
| Layout | 125.5 ms | 148.5 ms |
| Light | 61.3 ms | 78.4 ms |
| Balance and allotment | 422.6 ms | 335.5 ms |

- **Layout and light:** they carry most of the over-dispatch, because the light's lattice products and the layout's new-axis counts are bounded loosely.
- **Balance and allotment:** they gained from dispatches given none, which are skipped.
- **Sizing:** it doubled because a point now runs in one workgroup, its flags before a barrier and the rest after.

**In Chrome terms.** At about 7 µs a direct dispatch (DESIGN.md 7a), Telperion's 26,983 dispatches are about 0.19 s of dispatch overhead, against 12 s had they stayed indirect. The beech's 8,559 are about 0.06 s. Both are inside the 0.2 s and 0.1 s targets.

## The device's age limit (2026-10-11, review finding P1)

A PA's lifespan is held in 64-bit ticks. The validator allows a million cycles, which is 2^40 ticks at 2^20 a cycle. A unit's share is decided against the lifespan's two words (`advance.wgsl`'s `step`, `wake.wgsl`). `grown_life_tests` holds a stem of 2,048, 4,096 and 1,000,000 cycles to the CPU in every word, exact, at ages 1 and 3.

Accumulated time is still held in 32-bit ticks, and signed where an age's share is cut:
- what an apex spent in its age;
- an axis's aged and grown time;
- the time a bud slept;
- the cycle × tick products in the layout and the settle.

So 2,048 cycles is the device's age limit. `build_grow::MOST_CYCLES` refuses a request aged 2,048 cycles or more with a named error before any dispatch, and a test reads that error. Widening the accumulated time is open, a question for the host (task 3).

## Cold compile after exact mode was deleted (2026-10-12, DESIGN.md section 9 item 3)

This is native on the RTX 3080 with `__GL_SHADER_DISK_CACHE=0`, each module made with its pipelines through device completion (`raw/compile-before.txt`):

| Module | Cold, s |
|---|--:|
| Units and wake | 43.74 |
| Layout | 4.50 |
| Hierarchy (build, sums, scan) | 0.36 |
| Light | 0.91 |
| Glue and sizing | 0.11 |
| Retirement | 0.15 |
| Settle shed and order | 0.37 |
| Settle place `walk_main` | 15.12 |
| Settle place `frame_main` | 2.04 |
| Settle place, the other five entries | 0.18 |
| **All** | **about 67.5** |

**Bisection** (fn-235's method, `compile_tests.rs`: one module's WGSL edited by a file of replacements, one entry made cold, one variant a process):

| Units and wake, `cycle_main` | Cold, s |
|---|--:|
| As built | 42.7 |
| `flag`, `gather`, `seed_main`, each alone | 0.01 to 0.03 |
| No apex run (`run` called nowhere) | 0.24 |
| `advance` never reached | 2.57 |
| `stop`, `relay`, `relay_after` or `move_on` never reached, each | 42.1, 43.9, 38.3, 28.7 |
| `grow_unit` not called | 20.3 |
| A failed unit's path not reached | 28.1 |
| The abortion draw (`persists`) not reached | 32.6 |
| All three of those together | 9.4 |
| The 64-bit key mix (`mix`) made one cheap expression | 30.3 |
| The private frames shrunk from 578 words to 68 | 49 to 50 (no gain) |

| Layout entry | Cold, s |
|---|--:|
| `frame_main` | 2.84 |
| `draw_main` | 1.64 |
| The other four | 0.01 to 0.04 |

**What the driver chokes on** is the volume of code inlined into one entry, as fn-235 found for `grow`. Everything outside the apex run makes in 0.24 s. The cost grows faster than the body: the advance alone accounts for about 40 s, and no single part of it is the cause. The unit's zones (`grow_unit`) take about half. The inlined 64-bit key mixes, about 12 s of it, sit behind every `child` and `draw`. A failed unit's path and the abortion draw take about 10 to 15 s each. The private frame array is not the cause.

**So 2 s needs the apex run split, not slimmed.** Even with its three largest parts stubbed out, `cycle_main` takes 9.4 s.
- **The split:** a resumable apex machine across small kernels. Each operation (survival and decision, the zones and their nodes, the sleepers, a stop, a relay, a move, the wake) becomes its own entry. The machine's state between them (frames, the operation and its stack, the append counts) moves to buffers, and it runs in rounds until no apex has an operation left. The count and emit passes run the same way.
- **The other modules:** the settle's `walk_main` (15 s) and `frame_main` (2 s), and the layout's `frame_main` and `draw_main` (4.5 s together), need the same treatment.
- **Open:** that is a design call beyond DESIGN.md, so it is for the host before it is built. It also adds dispatches a cycle, against the dispatch budget.
