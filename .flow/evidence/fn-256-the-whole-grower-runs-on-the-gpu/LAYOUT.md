# fn-256 task 2: the layout on the device, measured (worker, 2026-10-11)

DESIGN.md sections 2.3 and 7 for the rough layout (`Grower::sketch`). This file records what ran and what it measured. It does not extrapolate to a build or judge the budget.

## How it ran

- **Device:** NVIDIA GeForce RTX 3080, Vulkan, native wgpu, through `crate::Gpu`, as UNITS.md.
- **Capture:** `telperion_space::grow_capture_layout_at` (`grow/cycle/capture_layout.rs`) grows the one cycle to the captured cycle and runs `step` on it, collecting `advanced` as `year` does. It then takes every input `sketch` reads, runs `sketch`, and takes every value it writes:
  - axes: 31 columns;
  - pending units: 4;
  - phytomers: 19, with the leaf area and a laid mark;
  - pencils: 59, the walk's f32 state through `Layer::words` and up to 8 pulls inline;
  - each PA's form: 19 f32s.
  - `grow_capture_layout` captures the age less one.
- **What the capture leaves out:** a phytomer grown before the cycle is captured only where the layout reads it. That is a lateral's bearer when the lateral stands at or below the bearer's old nodes (a woken bud), every relay's and continuation's carried line, and a grown axis laid short of its old nodes. The kernel reads every phytomer through one index function, which faults by name on any other. None fired.
- **Replay:** `generation/cycle/lay_*.rs` and `layout.wgsl`, `q.wgsl`, `sincos.wgsl`, `lay_walk.wgsl`, `lay_draw.wgsl`, `lay_stand.wgsl`, `lay_frame.wgsl`, test-only. The source is registered with the naga check as "cycle layout". The order of dispatches is:
  1. Draw over the grown axes.
  2. Relay node over the new axes.
  3. Generation over the new axes.
  4. Per generation: flag, scan (3 dispatches), gather, frame, draw.

  That is 3 + 7G dispatches, each in its own stamped pass. No index comes from an atomic. A generation's count reaches its kernels by a buffer copy of the scan's total.
- **The generation count** the host records dispatches for is the capture's. The device computes each new axis's generation itself, and the comparison holds the device's per-generation sizes to the CPU's.
- **libm's `sincosf`** sums in f64. It is summed here in integers of 2^-61, with the reduction rounded to 53 bits as the CPU's add rounds it. Over 1M inputs it agrees with libm's sine and cosine in every bit (`the_layouts_sine_and_cosine_are_libms`). An earlier try in pairs of f32s failed, because the driver reorders their sums (FRICTION.md).
- **Real f64 in the CPU's layout** is handled without f64. The bend's years and the relay's years are integer ticks, rounded to f32 once. `sincosf` is summed in integers.
- **Compile:** 3.2 s cold for the six entries; 0.0 s from the driver's cache. Each heavy step has one call site.
- **Timing:** median of 9 runs. Each run restores the captured state by unstamped copies.
- **Logs:**
  - `raw/layout-young-contract.txt` and `raw/layout-busy-contract.txt`, as committed.
  - `raw/layout-young-exact-all5.txt` and `raw/layout-busy-exact.txt`, under the experiment.
  - `raw/layout-young-less-*.txt`, the experiment less one part each.
  - `raw/layout-exact-experiment.patch`, not committed.

## Agreement

Every word of every column is compared: axes, the phytomer arena and pencils by row, the per-generation sizes, and the leaves as a sorted set.

**As committed:** every integer, count, generation, order and fault agrees in every capture:
- relay nodes, units and laid counts, trunk flags and pull counts;
- the phytomers' scale, grown, size and rank;
- the axes' rank.

Only f32 geometry differs:

| Capture | Axes drawn (grown + new) / generations / phytomers laid | New axes | Words that differ |
|---|---|---|--:|
| european-beech 10 at 8 | 181 + 71 / 1 / 667 | 70 lateral, 1 relay off its line | 3,010 / 43,203 |
| Telperion 14 at 10 | 270 + 110 / 1 / 1,024 | 109 lateral, 1 relay off its line | 4,450 / 69,718 |
| ordinary 16 at 12 | 722 + 427 / 4 / 2,283 | 328 lateral (3 woken), 99 continuations | 19,346 / 227,689 |
| norway-spruce 24 at 16 | 13,764 + 8,628 / 2 / 27,154 | 8,549 lateral (114 woken), 79 continuations | 282,061 / 4,420,726 |
| oregon-white-oak 16 at 12 | 2,274 + 1,182 / 2 / 8,194 | 1,179 lateral, 2 continuations, 1 relay | 40,822 / 612,164 |
| silver-birch 16 at 12 | 2,333 + 795 / 1 / 6,012 | 795 lateral | 29,290 / 476,408 |
| date-palm 16 at 12 | 1 + 0 / 0 / 12 | none | 111 / 367 |
| Telperion 82 at 81 (busy) | 225,593 + 67,430 / 2 / 823,805 | 66,847 lateral, 488 continuations, 95 relays | 3,271,818 / 151,661,364 |
| european-beech 40 at 39 (busy) | 21,581 + 7,680 / 1 / 79,789 | 7,619 lateral, 44 continuations, 17 relays | 341,610 / 7,192,023 |

**Which columns differ:** the frames and walks (base, heading, side, running, across, tip, curve), the pull from the heading, wood, and the leaves' tips and areas. Beyond those:
- one ordinary and one spruce pencil's base scale, from dominance's `1 - d (1 - held)`;
- two silver-birch pencils' turning, from `1 + spread u`.

On Telperion 82, the leaves differing are 6,933 words of 3,295,220.

**Under the experiment, all seven young captures and both busy captures agree in every word.** The experiment (`raw/layout-exact-experiment.patch`) changes four things:
- each marked product and reordered sum is held apart by its bits XORed with a uniform zero (`m`, `apart`);
- division and square root are rounded correctly (`qdiv`, `qsqrt`);
- the contract's `exp2c` and `fails` polynomials are held apart.

Every part is needed. Each run below drops one part, and with that part dropped these captures differ:

| Part dropped | Captures that differ (of 7 young) | Cause, and its sites |
|---|---|---|
| products held apart (`m`) | 7 | **The driver fuses a product into the add or subtract that takes it**, also across a variable or an inlined call, as in dominance's `held`. Sites: `q.wgsl:42-80` (dot, cross, Rodrigues), `lay_walk.wgsl` (walk, pulls, landing), `lay_frame.wgsl` (turned, frames), `lay_stand.wgsl` (stand, position, attach), `lay_draw.wgsl` (sizes, rank, wood, bend). |
| sums held apart (`apart`) | 6 | **The driver reorders and simplifies sums and product chains.** `atan2f`'s `(z - PI_LO) - PI` (`q.wgsl:199-200`) accounts for 113k of 124k arctangents that differ in 1M. `atanf`'s `hi - ((x s - lo) - x)` (`q.wgsl:172`) differs, and so does the resting step's `(tip + d len) - tip`, taken as `d len` (`lay_walk.wgsl:181`). |
| correct division | 7 | **WGSL division is not correctly rounded** (`q.wgsl:30`): `unit`'s `1/l`, the rotation's `c / norm`, `atan2f`, the stand's spans and the turns. |
| correct square root | 7 | **WGSL `sqrt` is not correctly rounded** (`q.wgsl:34`): every length and unit, the landing's ease and `rest`. |
| the contract's polynomials | 2 (spruce, birch) | **Fusion inside `fine/contract.wgsl`'s `exp2c` (:70) and `fails` (:183)**: the tropism's share and the curvature's keep. |

**Transcendentals and real f64 are not a cause.** `sincosf` is libm's in every bit, as committed. `atan2f` is libm's in every bit under the experiment (`the_layouts_arctangent_is_libms`). As committed, its 124,064 differences of 1M are those causes: 11,404 left by division alone, 114,501 by reordering, and 59 by fusion.

## Timing (median of 9, ms of GPU time in the stamped passes)

| Phase | Telperion 82 at 81, as committed | Telperion 82 at 81, experiment | european-beech 40 at 39, as committed | european-beech 40 at 39, experiment |
|---|--:|--:|--:|--:|
| draw, grown axes | 2.527 | 4.190 | 0.453 | 0.664 |
| relay node | 0.009 | 0.009 | 0.007 | 0.007 |
| generation | 0.004 | 0.004 | 0.003 | 0.003 |
| list (flag, scan, gather, per generation) | 0.333 | 0.338 | 0.086 | 0.086 |
| frame | 0.524 | 0.568 | 0.089 | 0.118 |
| draw, new axes | 0.098 | 0.250 | 0.007 | 0.006 |
| first stamp to last | 3.601 | 5.464 | 0.701 | 0.938 |

**Per run:**

| | Telperion 82 at 81 | european-beech 40 at 39 |
|---|--:|--:|
| Dispatches | 17 | 10 |
| Generations | 2 | 1 |
| Axes drawn | 293,023 | 29,261 |
| Axes framed | 67,430 | 7,680 |
| Phytomers laid | 823,805 | 79,789 |
| Resident: axes, phytomers, pencils, units and form | 596.7 MB | 27.8 MB |
| Scratch: lists, flags and scan sums | 1.72 MB | 0.18 MB |
| The capture's CPU time | 9.6 s | 0.3 s |

The captured state each run restarts from is a second copy of the resident state. It is test scaffolding and is excluded above.

## Open for the host

1. **UNITS.md's question about the f32 contract has more to it.** Fusion is not the only cause. The driver also reorders and simplifies sums: it moves constants across a subtraction, and it takes `(a + b) - a` as `b`. Its division and square root are not correctly rounded either. In the layout, holding every site to the CPU's order costs 3.6 ms against 5.5 ms on Telperion's busy cycle. Whether to hold the device to the CPU or to measure a tolerance remains the host's call. Until it is made, `young_layouts_agree_with_the_cpu` and `the_layouts_arctangent_is_libms` stay ignored, their assertions unchanged.
2. **A build needs the generation count from the device.** The test records the generation count from the capture. A build needs it from the device, by a readback or by indirect dispatches to a bound. DESIGN.md does not say which.
