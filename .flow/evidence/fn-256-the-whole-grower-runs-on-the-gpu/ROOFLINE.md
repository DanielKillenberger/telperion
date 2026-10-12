# fn-256 task 7: the roofline, where the cycle's GPU time goes (worker, 2026-10-12)

Measurement only: no kernel or production path changed. Telperion 82, the oak 40 and the beech 40, each at seed 42, are grown as a build grows them (`executor::growth`'s species, the hinted arena, compaction on, no move), on the RTX 3080 with 1.7 GB held by the desktop, one GPU process at a time under the GPU lock.

- **The test:** `the_roofline_is_measured` (`cycle/roofline_tests.rs`, `cycle/roofline_life.rs`). It runs as `cargo test --profile ci -p telperion-render the_roofline_is_measured -- --ignored --nocapture` with `ROOFLINE_OUT` set.
- **What it writes:**
  - every dispatch's stamped time by cycle, phase and kernel;
  - each cycle's census;
  - what each cycle visits, from the CPU reference.
- **Its instrumentation:** two `cfg(test)` fields on `Grew`, the per-cycle dispatch ends and censuses.
- **Raw output:** `raw/t7/` and `raw/t7-run-5.txt`.
- **The tables:** `roofline.py raw/t7` assembles them.

Each figure is marked **(m)** measured or **(c)** computed.
- **Measured:** GPU time, from timestamp queries around every dispatch. Clears and copies are not stamped. Counts come from the device's census.
- **Computed:** bytes and operations are each kernel's source-level storage words and integer or float operations a record, read from the WGSL, times the census counts. They are an upper estimate, good to about 2x, and the words a record are listed in `roofline.py`'s `records`. Real DRAM traffic is lower wherever neighbouring invocations share cache lines.

## The card's practical peaks (m)

| Kernel | Best of 20 | Peak |
|---|--:|--:|
| Copy 256 MiB, vec4 loads and stores | 0.761 ms | **705 GB/s** read and written (93% of the 760 nominal) |
| fp32 vec4 FMA chains | 27.7 ms | 19.8 TFLOP/s (67% of the 29.8 nominal) |
| u32 multiply-add chains, two ops each | 33.3 ms | **16.5 Tops** int32 |

The cycle's arithmetic is integer: f64 is emulated in `soft64.wgsl` and u64 in `u64.wgsl`, and only the layout uses fp32 and hardware transcendentals. So every phase's ops are held to the int32 peak, and the layout's to the fp32 peak.

## Per phase

"× roof" is the measured time over the roofline time, `max(bytes / 705 GB/s, ops / peak)` (c): how far the phase runs from what the card could do with the same work.

**Telperion 82, seed 42:** 1,313.5 ms stamped (m) in 34,810 dispatches (m). Run 2 of the same build stamped 1,380.7 ms, so run-to-run variation is about 5%.

| Phase | GPU ms (m) | Share (m) | Dispatches (m) | Visits over the build (m) | GB (c) | GB/s (c) | Of 705 (c) | Gops/s (c) | Of peak (c) | × roof (c) |
|---|--:|--:|--:|---|--:|--:|--:|--:|--:|--:|
| units and wake | 318.3 | 24.2% | 11,874 | 4.98M jobs, 18.2M phytomers, 1.53M new axes | 37.9 | 119 | 16.9% | 267 | 1.6% | 6x |
| light | 315.6 | 24.0% | 2,430 | 16.9M lattice nodes × 9 directions (unlit), 18.2M leaves, 5.2M tips | 7.8 | 25 | 3.5% | 343 | 2.1% | 29x |
| balance and allotment | 289.2 | 22.0% | 13,373 | 16.4M built axes × 4 sweeps, 4.98M buds | 6.4 | 22 | 3.1% | 21 | 0.13% | 32x |
| retirement | 218.9 | 16.7% | 2,353 | 17.9M held axes | 3.0 | 14 | 1.9% | 8 | 0.05% | 52x |
| layout | 120.4 | 9.2% | 2,621 | 18.2M phytomers, 4.97M grown axes, 1.53M new | 7.2 | 60 | 8.5% | 233 | 1.2% (fp32) | 12x |
| sizing | 29.7 | 2.3% | 1,463 | about 18 VM points a cycle | 0.1 | 3.5 | 0.5% | 9 | 0.05% | 200x |
| compaction | 21.1 | 1.6% | 612 | the tables' rows, after cycles 65, 74, 80 | 11.8 | 557 | 79% | 167 | 1.0% | 1.3x |

**The oak 40, seed 42 (lit):** 222.0 ms stamped (m) in 14,492 dispatches (m).

| Phase | GPU ms (m) | Share (m) | Dispatches (m) | Visits over the build (m) | GB (c) | GB/s (c) | Of 705 (c) | Gops/s (c) | Of peak (c) | × roof (c) |
|---|--:|--:|--:|---|--:|--:|--:|--:|--:|--:|
| balance and allotment | 72.2 | 32.5% | 6,531 | 1.63M built axes × 4, 369k buds | 0.64 | 8.9 | 1.3% | 7 | 0.04% | 80x |
| units and wake | 57.7 | 26.0% | 4,440 | 369k jobs, 1.21M phytomers, 164k new axes, 438 sleeper visits | 2.8 | 49 | 6.9% | 110 | 0.67% | 14x |
| retirement | 37.4 | 16.8% | 783 | 1.79M held axes | 0.30 | 8.0 | 1.1% | 5 | 0.03% | 88x |
| light | 23.4 | 10.6% | 519 | 156k nodes × 9 directions, 1.21M leaves, 396k tips | 0.77 | 33 | 4.7% | 671 | 4.1% | 21x |
| layout | 17.8 | 8.0% | 1,470 | 1.21M phytomers, 367k grown axes, 164k new | 0.57 | 32 | 4.5% | 118 | 0.60% (fp32) | 22x |
| sizing | 13.3 | 6.0% | 707 | VM points | 0.04 | 3.3 | 0.5% | 8 | 0.05% | 216x |

The oak made no compaction.

**The beech 40, seed 42 (unlit):** 159.3 ms stamped (m) in 13,160 dispatches (m).

| Phase | GPU ms (m) | Share (m) | Dispatches (m) | Visits over the build (m) | GB (c) | GB/s (c) | Of 705 (c) | Gops/s (c) | Of peak (c) | × roof (c) |
|---|--:|--:|--:|---|--:|--:|--:|--:|--:|--:|
| balance and allotment | 62.7 | 39.3% | 5,318 | 369k built axes × 4, 177k buds | 0.16 | 2.6 | 0.4% | 3 | 0.02% | 272x |
| units and wake | 37.5 | 23.5% | 4,440 | 177k jobs, 658k phytomers, 67k new axes | 1.35 | 36 | 5.1% | 81 | 0.49% | 20x |
| retirement | 26.3 | 16.5% | 649 | 436k held axes | 0.07 | 2.8 | 0.4% | 2 | 0.01% | 252x |
| sizing | 12.5 | 7.8% | 707 | VM points | 0.04 | 3.2 | 0.4% | 8 | 0.05% | 223x |
| layout | 10.3 | 6.4% | 1,281 | 658k phytomers, 175k grown axes, 67k new | 0.27 | 27 | 3.8% | 103 | 0.52% (fp32) | 27x |
| light | 9.1 | 5.7% | 519 | 69k nodes × 9 directions (unlit), 658k leaves, 204k tips | 0.27 | 30 | 4.3% | 436 | 2.6% | 23x |
| compaction | 0.9 | 0.6% | 204 | after cycle 37 | 0.24 | 251 | 36% | 75 | 0.46% | 3x |

The start phase took 0.1 to 0.4 ms in each build.

### The costliest kernels (m)

| Preset | Kernel (phase) | ms | Dispatches | µs a dispatch |
|---|---|--:|--:|--:|
| Telperion | `path_main` (light) | 215.5 | 607 | 355 |
| Telperion | `living_main` (retirement) | 170.7 | 814 | 210 |
| Telperion | `written_main` (units) | 135.8 | 1,135 | 120 |
| Telperion | `split_main` (balance) | 75.9 | 814 | 93 |
| Telperion | `gather_main` (layout) | 55.9 | 285 | 196 |
| oak | `living_main` (retirement) | 27.1 | 218 | 124 |
| oak | `written_main` (units) | 16.8 | 400 | 42 |
| oak | `vm_main` (sizing) | 13.3 | 707 | 19 |
| beech | `vm_main` (sizing) | 12.5 | 707 | 18 |
| beech | `split_fold_main` (balance) | 11.3 | 76 | 149 |
| beech | `living_fold_main` (retirement) | 10.9 | 76 | 143 |

## Where the time goes

- **No phase is near the memory or the ALU roof.** Only the compaction, a column copy, runs at the copy kernel's rate (1.3x to 3x its roofline time). Every other phase runs 6x to 270x above its roofline time (c).
  - The highest bandwidth is the units' on Telperion: at most 119 GB/s, 17% of the measured peak, and an upper estimate.
  - The highest ALU rate is the oak's light: 4% of the int32 peak.
- **The time is latency, not throughput.** It goes in three ways (m):
  1. **Small dispatches.**
     - Telperion: 25,221 of 34,810 dispatches (72%) take under 20 µs, 137 ms (10%).
     - Oak: 12,162 of 14,492 (84%), 59 ms (27%).
     - Beech: 11,577 of 13,160 (88%), 52 ms (32%).
     - The units' and the balance's median dispatch is 3 to 4 µs.
  2. **Single-workgroup folds.** The narrow levels folded into one workgroup (`*_fold_main`) are one 256-lane workgroup each:
     - beech: 39.6 ms, 25% of its GPU time;
     - oak: 22.0 ms, 10%;
     - Telperion: 85.6 ms, 6.5%.
  3. **Empty invocations.** Telperion and the beech are unlit (extinction 0), yet the lattice's kernels run over 9 directions × the lattice's nodes, and each invocation exits at once.
     - Telperion's lattice kernels: 254.6 ms, 19.4% of its GPU time; `path_main` alone 215.5 ms over 821,500 nodes at cycle 82.
     - Beech's: 5.2 ms, 3.3%.
- **Which phases dominate.**
  - **Telperion:** the units (24%), the light (24%, of which 81% is the unlit lattice), the balance and allotment (22%) and the retirement (17%).
  - **The oak and the beech:** the balance and allotment (33%, 39%), the units (26%, 24%) and the retirement (17%, 17%).
  - **On every preset,** the per-axis sweeps (balance, allotment, retirement) together take 39% to 56% (m) and move under 3.1% of peak bandwidth (c).

## Records visited that did not change since the last cycle

Shares are of each phase's visits summed over the build.
- **Axis rows:** the CPU reference's per-axis history (c), held to the device's census (m). The modelled held axes match the census every cycle to within 0.04%: Telperion 17,909,874 against 17,909,850.
- **"Subtree unchanged":** no apex in the axis's subtree grew that cycle.
- **"Apex stopped":** the axis's own apex had stopped, so the axis is dead but attached.

| Phase | What a cycle visits | Telperion 82 | Oak 40 | Beech 40 |
|---|---|--:|--:|--:|
| balance and allotment (4 sweeps), retirement | every held axis | **71.1%** subtree unchanged; 66.1% apex stopped; last cycle 66.0% | **82.0%**; 74.5%; last 86.9% | **59.0%**; 47.7%; last 58.0% |
| units | living apexes (4.98M / 369k / 177k jobs) | 0%: each is a living apex | 0% | 0% |
| wake | sleeping buds | none held | 438 visits, 19 woke: **95.7%** | none held |
| layout | the cycle's phytomers, grown and new axes | 0% | 0% | 0% |
| light, leaves and tips | the cycle's leaves and living tips | 0% | 0% | 0% |
| light, lattice | 9 directions × every node, rebuilt each cycle | **100% idle** (unlit): 254.6 ms (m) | per-node change not measured | **100% idle** (unlit): 5.2 ms (m) |
| compaction | every kept row, moved | 100% unchanged by design: 21.1 ms | none | 0.9 ms |

- **The units do each decision twice** (a counted pass and an emitted one) by design (DESIGN.md 2.1).
- **Not measured:** how many of the five recorded units rounds a job uses. The census holds only a round-overflow flag (`T_ROUNDS`), so round visits that find a job already done are not counted.

## Short-lived laterals

A lateral's life (c) is the cycles its apex grew in, first to last inclusive, across its continuations and relays. A lateral whose apex still lives at the tree's age is never short-lived. Phytomers are counted on the lateral's own axes, not on the laterals it bears. The count is from the CPU reference. It matches the device's census of phytomers grown exactly: 18,239,310, 1,213,968 and 657,516.

**Time attribution (c):** each phase's measured time in a cycle is split by the short-lived share of the records that phase visits in that cycle:
- the units by the apexes that grew;
- the layout, and the light's leaf and tip kernels, by the phytomers laid;
- the balance, allotment, retirement, sizing and compaction by the held axes.

The light's lattice kernels are not attributable to a lateral. The lower figure gives them none; the upper figure splits them by the leaves' share. Direct timing per lateral was not possible: one dispatch covers every lateral.

| Preset | Phytomers grown (m) | On laterals living under 3 / 5 / 10 years (c) | Laterals under 3 / 5 / 10 (c) | GPU time under 3 (c) | Under 5 (c) | Under 10 (c) |
|---|--:|---|---|--:|--:|--:|
| Telperion 82 | 18,239,310 | 7.0% / **76.2%** / 86.9% | 16.5% / 78.1% / 86.3% of 1,315,851 | 9.2 to 10.7% | **61.3 to 76.7%** | 70.2 to 87.7% |
| Oak 40 | 1,213,968 | 19.2% / **73.0%** / 86.7% | 34.5% / 80.3% / 86.5% of 118,440 | 21.0 to 22.3% | **66.1 to 71.5%** | 76.0 to 82.5% |
| Beech 40 | 657,516 | 6.6% / **61.5%** / 71.2% | 14.3% / 58.1% / 64.6% of 52,856 | 11.8 to 12.0% | **67.3 to 69.3%** | 77.6 to 79.9% |

The step is between 3 and 5 years: most laterals grow 3 or 4 years and stop. On Telperion, 76% of all phytomers grown are on laterals whose apex grew fewer than 5 years. By records visited they cost 61% to 77% of the GPU time.

## Method and limits

- **The builds.**
  - Each preset is grown once warm, then again with every dispatch stamped.
  - Both grew in the hinted arena with no move (asserted).
  - The census is read a cycle late, as the build reads it.
  - A compaction's dispatches are counted after the cycle they follow.
- **The CPU reference.**
  - It is the grower with every axis open (`grow_capture_grown`), plus the retirement log (`grow_capture_retired`). Their axes are asserted the same by lineage.
  - A row leaves the held set at the first compaction after the verdict of a retired subtree it is inside; the retired root's own row stays.
  - The device's living apexes (LIVE0) are 10% (Telperion, beech) to 25% (oak) more than the apexes the reference shows growing. The units' attribution assumes those carry the same short-lived share.
- **The bytes and operations (c)** take two assumed figures, not measured: 2 zones a unit and 1.5 units rounds a job. PROOF.md has "rounds used are 1 or 2 in most cycles".
  - The words a record were read from the kernels (`roofline.py`, `records`).
  - The retirement's `living_main` rescans a bearer's units for each lateral child, so its words may be low by more than 2x. That would still leave it 25x or more from its roof.
- **Clears and copies** recorded beside the dispatches are not stamped. The GPU's idle time between dispatches is not in these figures.

## Found while reading, not part of this task (inferred, not run; each fixed by tasks 8 and 9, PROOF.md)

- **`node_laterals`** (`grow.wgsl:504-520`): in the counted pass (`what == 0`), a lateral falls into the emitting branch. It reads `window_none`, and in the emitted `zones_main` it also writes 2 words at indices that ignore the zones before it. `written_main` overwrites them later. The outcome is probably harmless, but zones in one dispatch race on those words.
- **`fault_out`** (`layout.wgsl:182`) ORs into `lists[T_FAULT]` with a plain load and store. Two axes faulting in one dispatch can lose a bit.
- **The lattice on an unlit site:** Telperion's light dispatches its lattice kernels at their bound (`lights`) though every invocation exits. That is 19% of its GPU time.
