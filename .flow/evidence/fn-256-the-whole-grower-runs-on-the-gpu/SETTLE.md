# fn-256 task 2: the settle on the device (worker, 2026-10-11)

The device settles the tree it grew (SETTLE-PLAN.md steps 1 to 3) and writes the node table in lineage order. This file records what ran and what it measured. It does not judge the budget.

## How it ran

- **Device and code:** the RTX 3080 under Vulkan, as in CYCLE.md. The code is `crates/telperion-render/src/generation/cycle/settle_*`, test-only.
- **Two modules:**
  - *Shed and order* (`settle_cols`, `settle_tree`, `settle_assign`, `settle_sweep`, `settle_order`, `settle_compact`) covers G0 to G7 and S0.
  - *Place* (`settle_state`, `settle_thick`, `settle_libm`, `settle_bend`, `settle_sag`, `settle_place`) covers S1 to S6.
  - The place module reads the node table under the layout's names, so the layout's stand, walk and frame lay the settled tree unchanged. The frame is split out of `lay_frame.wgsl` into `framed`, and the sketch's pencil moved to `lay_pencil.wgsl`. The walk takes a radius and a sag through two hooks that the layout leaves at none.
- **Dispatches:**
  - Every dispatch is direct, its count known to the host.
  - There are four readbacks:
    - the grown depths, after the shape;
    - the tallies and kept depths (G6);
    - the lay's first error;
    - the sized check.
  - Phase G runs a flat dispatch a depth over the grown axes. Each depth's list is filled by atomic slots, which order nothing that any result reads.
  - The hierarchy's blocks are not used, because the last cycle's new axes are not yet in its lists. This departs from the plan's G sweeps and is a scheduling choice: no value depends on it.
- **Bindings:** each module binds at most 6 storage buffers. The place module binds the form, the shed module's `work` and `atom`, the node table, and its own columns.
- **The lay's walk, bounded (host decision, 2026-10-11):**
  - Each kept depth is framed in one dispatch (`frame_main`), then walked in ceil(longest kept axis / 32) dispatches (`walk_main`), 32 phytomers an axis each.
  - Each axis's walk (its `Layer`) is carried in its own columns between dispatches.
  - The keep sweep counts each depth's longest kept axis into the G6 readback, so the host knows every count.
  - The sag's lay walks the same way.
  - The per-axis passes that walk an axis's units and phytomers in one invocation (fill, assign, compact) are cut into dispatches of 131,072 axes, and the leaf mean into 16,384 phytomers.
  - Thicken and the levers are not cut: none of their dispatches passed 1 ms.
- **Growth scratch:** before the settle takes its buffers, the build's growth-only buffers are destroyed (`settle_scratch.rs`). These are the hierarchy, the sums, the tree view, the light and the layout's lists, which covers SETTLE-PLAN risk 4.
- **CPU side:**
  - `Grower::settle` is split into `settle_shed` and `settle_lay`, and `settle` calls them in turn.
  - `grow_capture_settled_at` captures the tree at `Shed`, `Thick`, `Placed` (the first lay and its levers) or `Laid`, in lineage order. It also takes the leaf mean, the disused pipes, each grown axis's final index and the error.
  - `lineage_permutation` is factored out of `lineage_order_marked`.
- **Logs:** everything is under `raw/settle-*.txt`.

## Agreement

**Built exact, every word equals the CPU's**, for the young trees of all eight presets (CYCLE.md's ages, seed 42), at each step:

| Step | Test | Compared |
|---|---|---|
| Shed and order | `young_trees_shed_and_order_…` | the axis columns, units, phytomers (cycle, key, rank, size, light, grown, scale), the grown index, leaf mean and disused |
| Scale and thicken | `young_trees_scale_and_thicken_…` | adds each phytomer's radius and each relay's join |
| Laid | `young_trees_are_laid_…` | adds each axis's frame and each phytomer's tip, heading and side, after the sag's lay |

A date palm with no nodes collapses on both sides (`a_tree_without_phytomers_collapses_…`).

What exactness took, beyond the marked arithmetic:
- **libm ports:** `asinf` (with its f64 root taken correctly in integers) and `hypotf`. Both are held to libm on 2^20 inputs (`sags_arcsine_and_hypotenuse_are_libms`).
- **Gradual underflow.** The device flushes every f32 subnormal to zero, as operand and as result; `the_device_keeps_subnormals` prints 0 against the CPU's 1.2e-40.
  - Sag's levers at fine tips (radius about 3e-6 m) are about 1e-20, and their squares are subnormal.
  - Exact mode therefore takes the sag's dot products, scalings and roots in integers with gradual underflow (`settle_sub.wgsl`).
  - `exact_div` now rounds a subnormal quotient instead of writing garbage.
  - Plain builds still flush.
  - The sites are `settle_bend.wgsl` `unit_w` and `frame_carry` against `sag/turn.rs` `unit_w` and `geometry/sagged.rs` `Frame::carry`.

**Built plain**, laid, every axis of every young tree matches by lineage (0 unmatched, `raw/settle-plain-laid-1.txt`), and the floats part in their last places, inherited from the plain growth. `young_trees_settled_plain_are_reported` reports this and asserts nothing.

The bounded walk changed none of this: all three exact steps, the collapse and the determinism were rerun after it (`raw/settle-walk-1.txt`).

## Determinism (R2, R2a)

The laid node table, built plain, is the same in every word for the oak at 9 and the spruce at 12 (`the_node_table_is_the_same_on_every_context_and_block_cap`):
- over ten fresh device contexts;
- over hierarchy block caps 64, 128 and 1,024.

The CPU's `grow_cycle` retires subtrees as it grows: 28 to 588 for the young trees. It returns the reference grower's tree for all eight young presets (`young_trees_grown_with_retirement_are_the_reference_…`, risk 5), so the tree the device is held to is the one production grows.

## Timing (`settles_are_timed`, one stamped settle after a cold build, seed 42 unless named)

**Pending.** These figures were taken before the bounded walk and the cut passes (`raw/settle-time-1.txt`). The rerun waits on device memory: when I checked, the owner's game held 8,289 of 10,240 MiB, and a busy build needs at least 6 GiB free. The longest dispatches below are the ones the bounded walk addresses; the rerun reports each pass's longest after it.

GPU time is the sum of the settle's stamped dispatches. Wall is the host's time for the settle, its readbacks included.

| Build | Kept axes | Kept phytomers | Depths | Dispatches | Stamped | Wall | Longest |
|---|--:|--:|--:|--:|--:|--:|---|
| european-beech 40 | 37,751 | 332,405 | 26 | 306 | 68.6 ms | 0.08 s | 5.2 ms place |
| oregon-white-oak 40 | 52,248 | 354,122 | 13 | 243 | 167.8 ms | 0.18 s | 14.3 ms sag place |
| norway-spruce 40 | 309,600 | 1,212,283 | 8 | 150 | 132.5 ms | 0.15 s | 19.5 ms sag place |
| date-palm 40 | 1 | 480 | 1 | 39 | 12.3 ms | 0.01 s | 11.2 ms place |
| silver-birch 40 | 67,739 | 518,131 | 13 | 207 | 182.0 ms | 0.19 s | 18.2 ms sag place |
| ordinary 40 | 25,521 | 163,214 | 17 | 259 | 208.4 ms | 0.22 s | 20.0 ms sag place |
| laurelin 80 | 350,255 | 3,534,309 | 62 | 682 | 278.9 ms | 0.33 s | 9.0 ms place |
| telperion 82 | 373,607 | 3,773,761 | 64 | 704 | 302.7 ms | 0.37 s | 10.6 ms place |
| oregon-white-oak 80 seed 1 | 277,300 | 1,878,063 | 27 | 690 | 802.8 ms | 0.85 s | 22.7 ms sag place |
| **Exact** telperion 82 | 373,607 | 3,773,761 | 64 | 704 | 1,622.8 ms | 1.69 s | 67.0 ms place |
| **Exact** oak 80 seed 1 | 277,329 | 1,878,375 | 27 | 690 | 3,373.9 ms | 3.42 s | 216.7 ms sag place |

**Telperion 82, plain, by pass** (ms, dispatches):

| Pass | ms | Dispatches |
|---|--:|--:|
| shape | 1.7 | 32 |
| mark and assign | 8.0 | 6 |
| grown sweeps | 4.4 | 192 |
| order | 15.0 | 268 |
| compact | 9.5 | 12 |
| scale | 2.4 | 64 |
| thicken | 6.3 | 65 |
| place | 255.3 | 65 |

**The oak 80 seed 1, plain, by pass** (ms, dispatches):

| Pass | ms | Dispatches |
|---|--:|--:|
| leaf mean | 401.3 | 307 |
| sweeps | 10.6 | 81 |
| order | 12.8 | 115 |
| place | 124.9 | 28 |
| levers | 3.4 | 27 |
| sag place | 234.7 | 27 |

The leaf mean is reported apart, as asked:
- it is the CPU's sequential f32 sum of 10M grown phytomers, one lane at a time;
- 32,768 phytomers a dispatch, each about 1.3 ms;
- this is SETTLE-PLAN risk 1, and a blocked order is the host's call.

**Dispatches past 1 ms** (`raw/settle-time-1.txt` lists each):
- **Place and sag place, one per depth, on every preset.** Each is one invocation per axis, walking its phytomers in turn, so the depth with the longest axes bounds it (risk 3). Plain, they run up to 10.6 ms on Telperion and 22.7 ms on the oak at 80. Exact, up to 67 ms and 217 ms.
- **The oak's leaf mean:** about 1.3 ms each.
- **One each, about 1 to 1.5 ms on the 18M-phytomer builds:** `fill_main`, `assign_main` and `compact_main`.

**Every other pass stays under 1 ms.**

## Memory

| Build | Growth resident | Growth scratch | Freed before the settle | The settle's own |
|---|--:|--:|--:|--:|
| Telperion 82 | 3,160 MB | 525 MB | 745 MB | 1,237 MB |
| Laurelin 80 | 2,702 MB | 525 MB | 745 MB | 1,143 MB |
| The oak 80 seed 1 | 2,108 MB | 458 MB | 680 MB | 786 MB |
| The spruce 40 | 962 MB | 244 MB | 364 MB | 476 MB |

- The settle's own is `work` (sized by the grown axes, phytomers and units), `atom`, the node table and the place's columns.
- The freed figure includes the hierarchy's resident lists and shape, which the settle no longer needs.

## Compile (`the_settle_compiles`, the driver's disk cache off)

| Module | Plain | Exact |
|---|--:|--:|
| shed and order | 0.35 s | 0.38 s |
| place `scale_main`, `attach_main`, `thick_main`, `lever_main`, `sized_main` | 0.01 to 0.06 s each | 0.01 to 0.13 s each |
| place `place_main` | 28 s | 95 s |

These were timed before the walk was split into `frame_main` and `walk_main`, which now share `place_main`'s code; the cold compile is to be remeasured with the timing rerun.

`place_main` dominates: it inlines the frame, the stand, the walk and the sag.

In the exact build, the gradual-underflow integer operations cost most:
- The same entry with plain products and sums compiled in 12 s.
- At first the exact entry took 228 s. Two changes brought it to 95 s:
  - calling sag's `settled` and the lever's `unit_w` from one site each;
  - carrying the lever and its chord in one loop.
- What remains inlines the `sub_mul` and `sub_add` products, sums and roots in `frame_carry` and in `sag_of`'s scaling.
