# fn-333 R1: where the oak's memory goes

Measured 3 Nov 2026 on the goal branch at 26013bcdc, before any change to the
grower. `oregon-white-oak`, seed 1, its preset's light, one thread
(`taskset -c 7`, `RAYON_NUM_THREADS=1`), through `telperion-heavy`.

Tool: `cargo run --profile ci -p telperion-core --example footprint -- stores
oregon-white-oak 1 <ages>`. It grows one `Paused` (retiring at the age grown,
no census) to each age and reads `Paused::footprint`: every store the grower
keeps between cycles, each its length and its capacity times its record's
size, heap blocks inside records counted apart. A counting global allocator in
the example gives the heap held at that moment and its most so far. Growth
only; the settle is not in these numbers. Raw output:
`raw/stores-before-80.txt`, `raw/stores-before-200.txt` (ignored).

The count accounts for the heap: counted 1.921 GiB against 1.923 GiB held at
80, 11.581 against 11.584 at 200. The resident peak (VmHWM) is below the heap
because the doubled vectors' unused capacity is never touched.

## The axes by retirement

| age | axes laid | open or kept | retired roots | inside a retired subtree |
|----:|----------:|-------------:|--------------:|-------------------------:|
| 40  |   133,619 |       49,931 |        30,049 |          53,639 (40 %) |
| 80  | 1,478,107 |      277,266 |       219,568 |         981,273 (66 %) |
| 120 | 4,725,988 |      389,625 |       367,811 |       3,968,552 (84 %) |
| 200 | 9,886,116 |      239,358 |       222,241 |       9,424,517 (95 %) |

Correction (host, 3 Nov 2026, from the codex review): this tally counted
only axes already in the retirement table, so it leaves out the last
cycle's births, which are open. "Axes laid" and "open or kept" are short by
them: at 200 the grower held 9,929,933 axes (RESULT.md), so 43,817 more are
open. The shares inside a retired subtree are a little lower than shown
(95 % becomes 94.9 %); the bytes below were counted separately and are
unaffected. The tally now counts them.

At 200, 95 % of the axis records are inside a retired subtree: their
phytomers and draws are gone, but their fixed-size records stay in every
store indexed by axis.

## The stores, GiB

| store | per axis | 80 | 120 | 200 |
|---|---|---:|---:|---:|
| `axes` (the `Axis` records) | ~316 B + slack | 0.453 | 1.812 | 3.625 |
| `pencils` (rough layout) | ~216 B | 0.297 | 1.189 | 2.379 |
| `draws` (the `Draws` records) | ~196 B | 0.281 | 1.125 | 2.250 |
| `retire.log` (every verdict) | 24 B a verdict | 0.094 | 0.188 | 0.375 |
| `successor` | 24 B + heap | 0.047 | 0.188 | 0.375 |
| `working.links`, `working.by_node` | 16 B, 24 B | 0.094 | 0.375 | 0.750 |
| `retire.fate`, `retire.frozen` | 12 B each | 0.038 | 0.148 | 0.298 |
| `horizon`, `units`, `roots`, `working.pa` | 8 B each | 0.064 | 0.248 | 0.500 |
| the other working arrays (`frozen`, `at`, `light`, `cost`, `sub`, `incoming`, `bud`, `open`, `living`, `retire.woken`, `by_node`'s lists) | 4 B or 1 B each | 0.073 | 0.274 | 0.566 |
| `axes.phytomers` (live phytomers) | 136 B a phytomer | 0.392 | 0.509 | 0.360 |
| `draws.units`, `.nodes`, `.shade` | heap, live axes only | 0.068 | 0.088 | 0.061 |
| `leaves`, `live`, everything else | | 0.025 | 0.04 | 0.04 |
| **counted** | | **1.921** | **6.197** | **11.581** |
| heap held (allocator) | | 1.923 | 6.200 | 11.584 |
| VmHWM (resident peak) | | 1.77 GB | 5.11 GB | 9.29 GB |

The live tree, its phytomers and their draws, is 0.42 GiB at 200: under 4 %.
Every other store grows with the axes ever laid, and 95 % of those are inside
retired subtrees. `retire.log` grows with the verdicts ever taken (15.4
million at 200), and only the captures and one test read it.

## What it points at

- The axes inside a retired subtree hold about 1 KB each across `axes`,
  `pencils`, `draws` and the working arrays: about 9.6 GB of the 11.6 at 200.
  Letting them go (R2) leaves the open and kept axes and the retired roots,
  461,599 at 200.
- The retired roots (222,241 at 200) stay: each stands for its subtree in
  its bearer's balance (frozen upkeep), girth (frozen girth), evaluation and
  fade (its living). They are 2 % of the records at 200.
- The verdict log is kept only where records are kept by index.
