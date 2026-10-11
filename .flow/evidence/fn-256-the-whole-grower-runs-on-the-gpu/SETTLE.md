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

Rerun after the bounded walk (WALK = 32 phytomers a dispatch) and the ranged fill, assign and compact, with 1.7 of 10.2 GiB of device memory in use (`raw/settle-time-3.txt`; the earlier figures are in `raw/settle-time-1.txt`). GPU time is the sum of the settle's stamped dispatches. Wall is the host's time for the settle, its readbacks included.

| Build | Kept axes | Kept phytomers | Depths | Dispatches | Stamped | Wall | Longest |
|---|--:|--:|--:|--:|--:|--:|---|
| european-beech 40 | 37,751 | 332,405 | 26 | 375 | 69.3 ms | 0.08 s | 1.9 ms walk |
| oregon-white-oak 40 | 52,248 | 354,122 | 13 | 352 | 171.8 ms | 0.18 s | 3.7 ms sag walk |
| norway-spruce 40 | 309,600 | 1,212,283 | 8 | 206 | 115.1 ms | 0.13 s | 15.8 ms order place |
| date-palm 40 | 1 | 480 | 1 | 54 | 12.3 ms | 0.01 s | 0.8 ms walk |
| silver-birch 40 | 67,739 | 518,131 | 13 | 293 | 189.5 ms | 0.20 s | 6.4 ms sag walk |
| ordinary 40 | 25,521 | 163,214 | 17 | 367 | 216.2 ms | 0.23 s | 4.5 ms sag walk |
| laurelin 80 | 350,255 | 3,534,309 | 62 | 913 | 292.5 ms | 0.36 s | 4.4 ms walk |
| telperion 82 | 373,607 | 3,773,761 | 64 | 946 | 312.0 ms | 0.39 s | 4.4 ms walk |
| oregon-white-oak 80 seed 1 | 277,300 | 1,878,063 | 27 | 1,219 | 836.3 ms | 0.91 s | 7.5 ms sag walk |
| **Exact** telperion 82 | 373,607 | 3,773,761 | 64 | 946 | 1,489.3 ms | 1.56 s | 13.5 ms walk |
| **Exact** oak 80 seed 1 | 277,329 | 1,878,375 | 27 | 1,219 | 2,950.4 ms | 3.02 s | 32.9 ms sag walk |

By pass, plain (ms in dispatches):

| Pass | Telperion 82 | Oak 80 seed 1 |
|---|--:|--:|
| shape | 1.9 in 32 | 1.7 in 32 |
| mark and assign | 8.2 in 28 | 4.2 in 28 |
| sweeps | 4.4 in 192 | 14.0 in 81 |
| leaf mean | | 423.8 in 614 |
| order | 14.7 in 268 | 13.3 in 115 |
| compact | 9.2 in 14 | 4.6 in 14 |
| scale | 2.6 in 64 | 1.1 in 27 |
| thicken | 6.7 in 65 | 4.2 in 28 |
| place | 264.3 in 283 | 121.9 in 127 |
| levers | | 3.4 in 27 |
| sag place | | 244.3 in 126 |

The bounded walk cut the longest dispatch (Telperion 10.6 to 4.4 ms, the oak 22.7 to 7.5 ms plain; 67 to 13.5 ms and 217 to 33 ms exact) and left the totals about where they were.

**Still past 2 ms (open; the bound is the brief's, not met):**
- **The walk, on every busy build.** 32 phytomers of a long axis are still up to 4.4 ms on Telperion, 7.5 ms on the oak's sag, 13.5 and 33 ms exact. A smaller WALK trades these for more dispatches; I have not tuned it.
- **The spruce's order `place_main`: 15.8, 7.2 and 6.5 ms.** One invocation places each parent's kept children, so a parent with thousands of children (the spruce's trunk) runs alone. It needs ranging by child, as fill and assign were; not done.
- **`compact_main` on the 18M-phytomer builds:** up to 4.4 ms. `up_main` on the oak at 80 (seven or eight past 1 ms).
- **The oak's leaf mean:** about 0.7 ms each now, 614 dispatches, still 424 ms in all (risk 1, the host's call).

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
