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

## Open

1. **Growth kernels still past 1 ms**, and one past 2 ms in exact (above). These are the growth worker's kernels; I did not touch them.
2. **Compaction's cost.**
   - **Wall time:** compaction happens only at moves. A build from the least sizes restarts up to 6 times and moves up to 64 times, so its wall is 2.69 s against 1.55 s for a Telperion 82 built at known sizes without compaction (RETIRE.md).
   - **Records:** the oak at 80 keeps 6.7M record rows, more than its 3.35M arena rows, because its PAs keep leaf girth.
3. **The oak's leaf mean** is 429 ms plain at 80 (SETTLE-PLAN risk 1, the host's call).
4. **The retirement phase** is 237 ms of Telperion 82's 1,256 ms build.
