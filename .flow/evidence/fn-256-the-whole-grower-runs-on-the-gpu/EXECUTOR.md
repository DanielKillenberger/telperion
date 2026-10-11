# fn-256 task 2: the executor's acceptance (worker, 2026-10-11)

This page summarises the whole device build: growth with retirement, then the settle and the lay. Details are in SETTLE.md (settle) and RETIRE.md (retirement). Commits 05f780bc5 and the ones before it on this branch. Raw output is in `raw/executor-3.txt`, `raw/retire-det-2.txt`, `raw/exec-suite.txt`, `raw/compile-cold-2.txt` and `raw/compile-cold-3.txt`.

## Agreement

**Exact, every word equals the CPU's**, on the young trees of all eight presets (CYCLE.md's ages, seed 42):
- **Settle:** shed and order, scale and thicken, and laid (`young_trees_shed_and_order_…`, `young_trees_scale_and_thicken_…`, `young_trees_are_laid_…`).
- **Retirement decisions** (`every_verdict_is_the_cpus`): every root, cycle, verdict and living value, including the woken-bud waits (spruce 18, ordinary 16).
- **The laid tree grown with retired subtrees removed at each move** (`young_trees_settle_as_on_the_cpu_with_retired_subtrees_let_go`). The trees are grown from 512-row phytomer tables so they move several times. The oak and the ordinary read their retired rows back from the records.
- **The reference:** the CPU's `grow_cycle` with retirement returns `Working::reference` on all eight presets (risk 5).

Every one of these was rerun after the last change (`raw/exec-suite.txt`: 36 passed).

**Plain:** every axis of every young tree matches the CPU's by lineage (0 unmatched). Floats differ in their last places, inherited from the plain growth (`young_trees_settled_plain_are_reported`).

## Determinism

Retirement and compaction on, plain (`the_node_table_with_retirement_is_the_same_on_every_context`). For all eight young presets, the laid node table, with its error words, is identical in every word:
- over ten fresh device contexts;
- over hierarchy block caps 64, 128 and 1,024.

That is 96 comparisons, none differing. The tables run from 5,506 words (palm) to 1,183,746 (spruce).

## Per-preset build and settle (plain, seed 42 at default age; oak 80 seed 1; Telperion and oak 80 also exact)

Retirement runs every cycle in every build below.

Each build is measured two ways (`the_executor_is_measured`):
1. **Table A: at its sizes.** The sizes are taken from an earlier build without compaction, so the build never restarts or moves, and every dispatch maps to its phase.
2. **Table B: from the least sizes with compaction on.** This is the only way retired rows are removed: they are dropped at moves. Its stamps include the work from restarts and moves.

### A. At its sizes: stamped GPU time

| Build | Build dispatches | A cycle | Build stamped | Build's longest | Settle dispatches | Settle stamped | Settle's longest |
|---|--:|--:|--:|---|--:|--:|---|
| european-beech 40 | 9,491 | 237 | 186 ms | 0.757 ms cycle_main | 766 | 71 ms | 0.458 ms walk_main |
| oregon-white-oak 40 | 11,589 | 290 | 268 ms | 1.016 ms cycle_main | 787 | 174 ms | 0.892 ms leaf_main |
| norway-spruce 40 | 13,221 | 331 | 377 ms | 0.833 ms cycle_main | 630 | 77 ms | 0.755 ms walk_main |
| date-palm 40 | 5,117 | 128 | 70 ms | 0.509 ms weights_main | 159 | 14 ms | 0.505 ms compact_main |
| silver-birch 40 | 11,187 | 280 | 255 ms | 0.858 ms living_main (retirement) | 869 | 184 ms | 0.908 ms walk_main |
| ordinary 40 | 14,186 | 355 | 291 ms | **1.889 ms cycle_main** | 942 | 217 ms | 0.890 ms leaf_main |
| laurelin 80 | 28,877 | 361 | 1,168 ms | 1.278 ms draw_main | 2,378 | 282 ms | 0.775 ms assign_main |
| telperion 82 | 30,093 | 367 | 1,256 ms | 1.264 ms draw_main | 2,498 | 302 ms | 0.662 ms walk_main |
| oak 80 seed 1 | 28,184 | 352 | 1,161 ms | 1.218 ms draw_main | 2,583 | 832 ms | 0.940 ms walk_main |
| **Exact** telperion 82 | 30,093 | 367 | 1,733 ms | 1.981 ms draw_main | 7,115 | 1,495 ms | 0.757 ms thick_main |
| **Exact** oak 80 seed 1 | 28,184 | 352 | 1,829 ms | **2.259 ms partials_main** | 7,067 | 3,123 ms | 1.486 ms lever_main |

### Per phase, stamped ms over the whole build

| Phase | beech 40 | oak 40 | spruce 40 | palm 40 | birch 40 | ordinary 40 | laurelin 80 | telperion 82 | oak 80 s1 | telperion 82 exact | oak 80 s1 exact |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| start | 0.1 | 0.1 | 0.2 | 0.1 | 0.1 | 0.1 | 0.4 | 0.4 | 0.3 | 0.3 | 0.3 |
| sizing | 29.6 | 32.6 | 33.9 | 25.2 | 34.1 | 36.1 | 68.8 | 69.9 | 70.1 | 71.5 | 68.9 |
| units and wake | 39.9 | 67.5 | 152.2 | 22.6 | 71.1 | 112.0 | 344.7 | 376.5 | 368.6 | 719.4 | 806.8 |
| layout | 16.1 | 26.1 | 49.6 | 7.3 | 25.4 | 31.4 | 136.8 | 150.9 | 132.5 | 209.0 | 189.3 |
| light | 8.6 | 24.1 | 18.8 | 5.3 | 12.3 | 18.8 | 71.1 | 79.5 | 96.0 | 78.8 | 94.7 |
| balance and allotment | 66.5 | 77.6 | 95.6 | 7.5 | 72.7 | 74.5 | 321.6 | 341.1 | 288.2 | 412.6 | 457.4 |
| retirement | 25.7 | 39.5 | 26.6 | 1.4 | 39.5 | 18.4 | 224.2 | 237.4 | 205.1 | 241.3 | 211.7 |
| settle shape | 0.2 | 0.2 | 0.7 | 0.1 | 0.2 | 0.1 | 1.6 | 1.7 | 1.9 | 1.7 | 1.6 |
| settle mark and assign | 0.4 | 0.5 | 1.2 | 0.2 | 0.8 | 0.6 | 7.5 | 7.9 | 4.0 | 8.2 | 3.9 |
| settle sweeps | 0.9 | 3.0 | 2.9 | 0.0 | 0.8 | 2.9 | 6.3 | 7.5 | 18.9 | 7.2 | 42.1 |
| settle leaf mean | | 52.5 | | | | 16.6 | | | 429.3 | | 480.1 |
| settle order | 0.9 | 0.6 | 0.8 | 0.0 | 0.8 | 0.8 | 2.8 | 3.0 | 1.6 | 3.0 | 1.8 |
| settle compact | 1.0 | 1.1 | 2.9 | 0.5 | 1.9 | 1.1 | 12.8 | 12.9 | 6.3 | 13.4 | 6.8 |
| settle scale | 0.7 | 0.4 | 0.2 | 0.1 | 0.5 | 0.6 | 2.6 | 2.7 | 1.2 | 2.6 | 1.0 |
| settle thicken | 1.7 | 1.9 | 1.1 | 0.3 | 1.9 | 2.5 | 5.8 | 6.5 | 4.7 | 23.3 | 13.4 |
| settle place | 65.1 | 38.8 | 22.2 | 12.5 | 55.8 | 51.9 | 242.8 | 260.3 | 118.0 | 1,435.4 | 813.1 |
| settle levers | | 1.2 | 1.4 | | 1.9 | 1.5 | | | 3.4 | | 13.5 |
| settle sag place | | 73.3 | 43.9 | | 120.0 | 138.9 | | | 242.6 | | 1,745.8 |

### B. From the least sizes with compaction on: memory and unstamped wall

| Build | Restarts / moves | Dispatches | Longest | Wall, unstamped (median of 3) | Arena rows of tables | Records | Resident / scratch at end | Peak growing / settling |
|---|--:|--:|---|---|--:|--:|--:|--:|
| european-beech 40 | 3 / 22 | 24,269 | 0.804 ms cycle_main | 0.53 s (0.44 s growing) | 436,833 of 732,221 | 0 | 127 / 33 MB | 215 / 239 MB |
| oregon-white-oak 40 | 6 / 30 | 26,038 | 0.940 ms cycle_main | 0.71 s (0.52 s) | 553,074 of 835,130 | 852,631 | 268 / 66 MB | 411 / 454 MB |
| norway-spruce 40 | 5 / 35 | 23,065 | 0.820 ms cycle_main | 0.66 s (0.57 s) | 1,232,600 of 1,810,636 | 0 | 940 / 244 MB | 1,300 / 1,634 MB |
| date-palm 40 | 0 / 1 | 5,115 | 0.475 ms weights_main | 0.10 s (0.08 s) | 480 of 16,384 | 0 | 5 / 4 MB | 18 / 9 MB |
| silver-birch 40 | 3 / 33 | 33,317 | 0.952 ms cycle_main | 0.95 s (0.74 s) | 709,888 of 1,035,266 | 0 | 291 / 73 MB | 430 / 499 MB |
| ordinary 40 | 1 / 30 | 15,435 | 1.705 ms cycle_main | 0.58 s (0.35 s) | 194,027 of 258,986 | 195,716 | 97 / 36 MB | 163 / 184 MB |
| laurelin 80 | 5 / 62 | 71,203 | 1.267 ms draw_main | 2.58 s (2.18 s) | 5,650,058 of 8,590,256 | 0 | 2,175 / 525 MB | 3,017 / 3,670 MB |
| telperion 82 | 5 / 64 | 72,415 | 1.260 ms draw_main | 2.69 s (2.28 s) | 7,325,244 of 8,590,256 | 0 | 2,242 / 525 MB | 3,017 / 3,830 MB |
| oak 80 seed 1 | 2 / 60 | 32,760 | 1.168 ms draw_main | 2.26 s (1.33 s) | 3,350,484 of 4,463,385 | 6,706,171 | 1,936 / 459 MB | 2,647 / 3,182 MB |
| **Exact** telperion 82 | 5 / 64 | 72,415 | 1.985 ms draw_main | 4.96 s (3.08 s) | 7,325,244 of 8,590,256 | 0 | 2,242 / 525 MB | 3,017 / 3,831 MB |
| **Exact** oak 80 seed 1 | 2 / 60 | 32,760 | 2.271 ms partials_main | 5.46 s (2.05 s) | 3,350,493 of 4,463,382 | 6,706,163 | 1,936 / 459 MB | 2,647 / 3,182 MB |

The peaks are the device ledger's most bytes held by owned buffers, read after the build and again after the settle. A Telperion 82 built at its sizes without compaction held 18.2M arena rows and 3,410 MB resident (RETIRE.md).

## Longest dispatch

Every **settle** dispatch is under 1 ms in plain on every build. The one exception is the birch's sag walk, measured once at 1.04 ms (`raw/settle-time-10.txt`); in the acceptance run it was 0.91 ms. In exact, every settle dispatch is under 1.6 ms (the longest, 1.59 ms, was `thick_main` on the oak at 80, `raw/settle-time-10.txt`).

How the settle got there:
- **The walk** fits each dispatch to the depth:
  - it takes as many phytomers as keep the depth's axis-phytomer steps within 2^17 plain, or 2^15 exact;
  - at most 4 plain and 1 exact;
  - a depth wider than the budget is walked a run of axes at a time.
- **The ordering pass** ranks each child in its own invocation (the spruce was 15.8 ms).
- **The compaction, the upward sweep and the depth count** are cut into ranges.
- **The cost in dispatch count:**
  - Telperion 82's settle went from 946 to 2,498 dispatches plain, and from 946 to 7,115 exact;
  - its stamped GPU time stayed at about 300 ms;
  - its settle wall went from 0.39 to 0.46 s plain and from 1.56 to 1.92 s exact (`raw/settle-time-3.txt` against `raw/settle-time-10.txt`).

**Still past 1 ms, all growth kernels** (not changed in this round):
- `draw_main` (layout): 1.2 to 1.3 ms plain on the 80-cycle builds, 1.98 ms exact on Telperion.
- `cycle_main` (units and wake): **1.89 ms plain on the ordinary at 40**, and 1.0 ms on the oak at 40.
- `partials_main` (balance and allotment): **2.26 ms exact on the oak at 80, the only dispatch past 2 ms.**

Retirement's own longest dispatch is `living_main`, at 0.86 ms on the birch. It is 0.72 to 0.91 ms on Telperion 82.

## Compile, cold (`__GL_SHADER_DISK_CACHE=0`)

| Module | Plain | Exact |
|---|--:|--:|
| units and wake | 39.6 s | 96.3 s |
| layout | 4.2 s | 5.1 s |
| hierarchy | 0.4 s | 0.4 s |
| light | 0.9 s | 0.9 s |
| glue and sizing | 0.1 s | 0.1 s |
| retirement (with compaction) | 0.15 s | 0.17 s |
| settle shed and order | 0.36 s | 0.41 s |
| settle place, `walk_main` | 13.3 s | 57.4 s |
| settle place, `frame_main` | 1.8 s | 2.4 s |
| settle place, other entries | 0.2 s | 0.3 s |

In the exact `walk_main`, the gradual-underflow products cost 51 s: built with plain products and sums, it compiles in 6.6 s.

## Plain against the CPU's cycle, by lineage, at default ages

Plain build, retirement on, laid tree at each preset's default age, seed 42. It is compared with the CPU's laid tree from `grow_cycle` (fn-255), axes matched by lineage. This is a report, not an assertion (`default_ages_laid_plain_by_lineage`, one preset per process, `raw/floats-<preset>.txt`). The host had 13 GB available and the device 8.5 GiB free before the two oldest.

**Matching: nothing unmatched on any preset.** Every axis, unit and phytomer of every preset matches by lineage:

| Build | Axes | Units | Phytomers |
|---|--:|--:|--:|
| european-beech 40 | 37,751 | 79,530 | 332,405 |
| oregon-white-oak 40 | 52,248 | 83,140 | 354,122 |
| norway-spruce 40 | 309,600 | 580,011 | 1,212,283 |
| date-palm 40 | 1 | 40 | 480 |
| silver-birch 40 | 67,739 | 176,047 | 518,131 |
| ordinary 40 | 25,521 | 51,618 | 163,214 |
| laurelin 80 | 350,255 | 839,538 | 3,534,309 |
| telperion 82 | 373,607 | 896,091 | 3,773,761 |

**Integer fields: all equal on matched axes except `sleep`.** That holds for kind, parent, node, whorl, lineage, PA, birth, apex end, unit and phytomer counts, and each phytomer's cycle and key. `sleep` is a tick count (2^-20 cycle) quantised from a float. It differs on 1 axis on the beech, 261 on the oak at 40, 177 on the spruce, 24 on the birch, 12 on the ordinary, and 2 each on Laurelin and Telperion; the palm has none. 

**After the held sites (2026-10-12, DESIGN.md section 8).** The wake's ticks, a failed unit's lived ticks and its part below a tick, the shade, and the survival and abortion thresholds are now held to the CPU's order in both arithmetics (`cycle/held.rs`). Rerun one preset a process:

| Preset | `sleep` differs, before → after |
|---|--:|
| Beech | 1 → 0 |
| Oak at 40 | 261 → 249 |
| Birch | 24 → 0 |
| Ordinary | 12 → 2 |
| Palm | 0 → 0 |
| Spruce | 177 → 0 |
| Laurelin | 2 → 0 |
| Telperion | 2 → 0 |

Every preset matches by lineage, with nothing unmatched. The spruce, Laurelin and Telperion were rerun once the GPU had 8 GiB free.

**The cost, Telperion 82 plain** (`telperion_is_timed`; before is `519957c91`, after `e3d7246bf`, each from its own build directory):

| | Before | After |
|---|--:|--:|
| Units and wake | 360.6 ms | 362.0 ms |
| Unstamped wall | 1.313 s | 1.309 s |

Both builds take 2,372 dispatches, and the longest dispatch is unchanged. The cost is within run-to-run noise.

**Where the oak's differences come from.** All 249 are relays (kind 3) of PA 9, off by 1 or 2 ticks. They are not held sites. Swapping single modules to exact while the rest stay plain gives:

| Modules exact | `sleep` differs |
|---|--:|
| Units alone | 228 |
| Light, hierarchy and layout | 193 |
| Units and hierarchy | 240 |
| Units and light | 236 |
| Units, light, hierarchy and layout | 0 |

So the relay's lived ticks are floored from the shaded viability, whose light and sizes are plain geometry floats. That is the float input, not the operation, so it lies outside the rule's narrow reading. Holding it means building the growth's light, layout and hierarchy exact: open, for the host.

**Float fields.** Below are the absolute differences, max and p99, over matched rows (tips and positions in metres; scale and radius in their own units). The relative differences are in the raw files. For coordinates and unit-vector components near zero the relative figure runs to 1 or 2 and says nothing, so read the absolute column there.

| Field | beech 40 | oak 40 | spruce 40 | palm 40 | birch 40 | ordinary 40 | laurelin 80 | telperion 82 |
|---|---|---|---|---|---|---|---|---|
| tip x | 6.3e-5 / 2.4e-5 | 1.6e-4 / 3.3e-5 | 9.3e-5 / 4.9e-5 | 2.5e-6 / 2.5e-6 | 4.7e-4 / 7.7e-5 | 7.5e-5 / 4.9e-5 | 7.8e-4 / 1.9e-4 | 1.4e-3 / 2.1e-4 |
| tip y | 4.6e-5 / 2.2e-5 | 2.2e-4 / 3.2e-5 | 1.1e-4 / 5.0e-5 | 4.1e-6 / 4.0e-6 | 6.9e-4 / 1.1e-4 | 7.4e-5 / 5.1e-5 | 1.9e-3 / 2.9e-4 | 1.7e-3 / 3.2e-4 |
| tip z | 2.2e-5 / 8.1e-6 | 1.6e-4 / 2.6e-5 | 7.2e-6 / 1.4e-6 | 1.2e-7 / 1.2e-7 | 9.3e-5 / 2.4e-5 | 1.4e-5 / 5.2e-6 | 3.9e-4 / 7.2e-5 | 4.5e-4 / 7.8e-5 |
| heading (worst component) | 6.2e-5 / 2.1e-5 | 6.7e-3 / 1.3e-4 | 5.2e-5 / 4.3e-5 | 9.6e-7 / 8.2e-7 | 9.0e-4 / 5.7e-5 | 1.8e-4 / 3.0e-5 | 8.7e-3 / 8.9e-5 | 1.3e-3 / 7.6e-5 |
| scale | 3.8e-5 / 2.4e-6 | 1.6e-3 / 5.2e-5 | 2.5e-5 / 7.8e-7 | 0 | 2.4e-5 / 1.5e-6 | 1.7e-5 / 3.5e-6 | 9.5e-5 / 4.8e-6 | 5.0e-5 / 4.8e-6 |
| radius (m) | 2.4e-7 / 1.3e-8 | 2.2e-6 / 1.8e-7 | 9.6e-8 / 1.6e-9 | 0 | 7.0e-8 / 4.7e-9 | 1.7e-6 / 1.7e-8 | 8.5e-7 / 3.0e-8 | 7.9e-7 / 3.0e-8 |

**Relative max / p99** of the fields that are not near zero:

| Field | beech | oak 40 | spruce | birch | ordinary | laurelin | telperion |
|---|---|---|---|---|---|---|---|
| scale | 2.3e-3 / 1.6e-5 | 9.9e-2 / 1.8e-4 | 4.0e-3 / 2.4e-6 | 9.3e-3 / 1.4e-5 | 1.9e-2 / 1.5e-5 | 3.8e-1 / 2.6e-5 | 3.8e-2 / 2.5e-5 |
| radius | 2.3e-3 / 1.6e-5 | 9.9e-2 / 2.0e-4 | 4.0e-3 / 2.6e-6 | 9.3e-3 / 1.3e-5 | 9.5e-4 / 1.5e-5 | 2.5e-1 / 2.6e-5 | 3.7e-2 / 2.5e-5 |
| tip z | 3.2e-6 / 1.1e-6 | 7.9e-5 / 7.7e-6 | 1.1e-2 / 2.1e-6 | 3.7e-4 / 2.7e-6 | 2.3e-2 / 3.4e-6 | 3.1e-5 / 6.7e-6 | 3.5e-5 / 6.9e-6 |
| vigour | 4.4e-4 / 2.4e-6 | 4.3e-2 / 2.2e-6 | 3.8e-3 / 0 | 9.3e-3 / 0 | 6.1e-4 / 4.2e-6 | 8.5e-3 / 2.4e-6 | 2.1e-2 / 2.4e-6 |
| unit presence | 4.8e-4 / 0 | 9.9e-2 / 5.6e-5 | 4.0e-3 / 0 | 5.2e-4 / 0 | 3.6e-4 / 0 | 2.5e-1 / 0 | 1.9e-2 / 0 |

What the numbers show:
- **The tails are a few axes, not the bulk.** At p99, scale and radius are within 3e-5 relative everywhere but the oak at 40 (2e-4). Tips are within 0.3 mm everywhere.
- **The worst tip** is 1.9 mm, on Laurelin. The worst heading component is 8.7e-3, also on Laurelin.
- **The large relative maxima** on scale, radius and presence are where a presence or fade is near zero. One last-place difference there is a large fraction: presence on Laurelin reaches 0.25 relative, but its absolute difference is 1.3e-5.
- **The oak at 40 is the furthest at p99.** Its light and size also differ (size 2.7e-5 at p99, light 1.1e-5). Only the oak and the ordinary show any light difference at all (the ordinary's is 5.4e-7 at p99); every other preset's light is exact.
- **Where the differences come from** (an inference, not measured): plain mode flushes subnormals to zero and rounds in its own order. Those last-place differences in growth's presences carry through scale, girth and the lay.

## Open

1. **Growth kernels still past 1 ms**, and one past 2 ms in exact (above). These are the growth worker's kernels; I did not touch them.
2. **Compaction's cost.**
   - **Wall time:** compaction happens only at moves. A build from the least sizes restarts up to 6 times and moves up to 64 times, so its wall is 2.69 s against 1.55 s for a Telperion 82 built at known sizes without compaction (RETIRE.md).
   - **Records:** the oak at 80 keeps 6.7M record rows, more than its 3.35M arena rows, because its PAs keep leaf girth.
3. **The oak's leaf mean** is 429 ms plain at 80 (SETTLE-PLAN risk 1, the host's call).
4. **The retirement phase** is 237 ms of Telperion 82's 1,256 ms build.

## Review round 1 (codex, 2026-10-12)

**Fixed**, each with a test that failed first:
- **Lifespans past 32 bits of ticks:** widened (49c58268a). The executor refuses requests of 2,048 cycles or more by name; accumulated time stays in 32-bit ticks, so widening it is open for the host.
- **The probe's and the spike example's composers:** they now include `fine/arith.wgsl` (de6772a75).
- **Exact `partials_main`:** it sums in runs of 256 buds across dispatches, in the same order (0fbdb34e9). The exact builds' longest dispatch is now `draw_main` at 1.991 ms (oak 80) and 1.994 ms (Telperion 82), under 2 ms by about 9 µs. Plain mode's longest is 1.89 ms.
- **No move after the last cycle:** a move there had left the settle a stale phytomer count (70e6f557c).

**Declined:** plain mode's integer `sleep` ticks, floored from device-rounded floats, differing from the CPU's (261 axes on the oak at 40). This is the f32 contract the host has not ruled on (STEP2.md call 1). Exact mode equals the CPU in every word, and plain mode is COUNCIL.md's tolerance standard.

## After the host's rulings of 2026-10-12 (plain arithmetic, native transcendentals, blocked leaf mean, units as small kernels)

Commit 2e3094688 and the commits before it. Raw output: `raw/native-floats-40s.txt`, `raw/native-floats-80s.txt`, `raw/executor-native-1.txt`, `raw/native-math-seq-1.txt`, `raw/native-math-modules-1.txt`.

### Agreement with the CPU's laid tree, by lineage (ruling 9.1)

Plain, default age, seed 42. "Identity integers" are kind, parent, node, whorl, lineage, PA, birth, apex end, the unit and phytomer counts, and each phytomer's cycle and key.

| Preset | Axes | Phytomers | Unmatched (device / CPU) | Identity integers that differ | `sleep` differs (float-derived) | Tips, max / p99 | Radius, relative p99 |
|---|--:|--:|--:|---|--:|--:|--:|
| european-beech 40 | 37,751 | 332,405 | 0 / 0 | none | 0 | 0.08 / 0.03 mm | 1.6e-5 |
| oregon-white-oak 40 | 52,248 | 354,122 | 0 / 0 | none | 294 | 0.40 / 0.05 mm | 3.2e-4 |
| norway-spruce 40 | 309,600 | 1,212,283 | 0 / 0 | none | 0 | 0.11 / 0.05 mm | 2.6e-6 |
| date-palm 40 | 1 | 480 | 0 / 0 | none | 0 | 0.005 / 0.005 mm | 0 |
| silver-birch 40 | 67,739 | 518,131 | 0 / 0 | none | 0 | 0.81 / 0.11 mm | 1.3e-5 |
| ordinary 40 | 25,521 | 163,214 | 0 / 0 | none | 1 | 0.08 / 0.05 mm | 1.5e-5 |
| laurelin 80 | 350,255 | 3,534,309 | 0 / 0 | none | 0 | 1.77 / 0.31 mm | 2.6e-5 |
| telperion 82 | 373,607 | 3,773,761 | 0 / 0 | none | 0 | 1.70 / 0.32 mm | 2.5e-5 |

### Cold compile (ruling 9.3): native, driver cache off, 1.67 s

| Measure | Cold |
|---|--:|
| Modules one after another, each module's entries made side by side | **1.67 s** |
| Every module at once | **0.98 s** |
| Units (27 entries) / layout / settle / light / hierarchy | 0.39 / 0.48 / 0.35 / 0.18 / 0.11 s |
| Every entry strictly one after another (neither path builds this way) | 6.0 s |

### Per phase (ruling 9.6), at the build's own sizes, stamped GPU ms

| Phase | Telperion 82 | european-beech 40 |
|---|--:|--:|
| Units and wake | 270.7 (34,742 dispatches) | 46.7 (12,438) |
| Balance and allotment | 318.7 (19,471) | 59.6 (5,841) |
| Retirement | 224.9 (3,027) | 24.7 (893) |
| Layout | 92.4 (2,782) | 6.6 (887) |
| Light | 72.2 (1,131) | 8.0 (470) |
| Sizing | 67.0 (1,463) | 28.4 (707) |
| **Build** | **1,046.1 (62,700; 765 a cycle)** | **174.1 (21,278; 532 a cycle)** |
| **Settle** | **62.4 (2,498)**, place 22.1 | **10.9 (766)** |
| Longest dispatch | 0.795 ms (layout `frame_main`) | 0.390 ms |

- **Grown from the least sizes,** with retirement letting go at moves:
  - Telperion 82: 2.16 s unstamped wall (5 restarts, 61 moves), peak 3.2 GB growing and 3.9 GB settling.
  - The beech at 40: 0.47 s.
- **The oak at 80, seed 1:** 1.33 s, its longest dispatch 0.857 ms.
- **Every dispatch** of every build is under 1 ms.
- **Dispatches against the Chrome target** (DESIGN.md 7a, about 340 a cycle): they are over it, at 765 a cycle on Telperion, about 0.44 s at 7 µs. This goes to task 4's budget with the units' round gating (ruling 9.6).
