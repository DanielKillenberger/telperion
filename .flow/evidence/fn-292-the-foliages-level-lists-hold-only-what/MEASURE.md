# fn-292 task 2: the kept tree, the frame and the stills, before and after

Base 0a6664741 (`raw/before/`, task 1's capture), change 8ff19c81f (`raw/after/`), seed 1, every preset at its default age, RTX 3080 on Vulkan. Raw output stays in `raw/` (ignored).

## Kept tree and the foliage's selection buffers

`the_drawing_s_memory` (`--ignored`, `raw/after/memory.txt`). The kept tree in MB is measured on both sides. The base's level lists are measured at 0.1 MB in `raw/before/memory.txt`; their bytes are computed from the base's `select::sizes` (a whole leaf count per level, with the ledger's 1.25 headroom) at the leaves and levels the change's selection counted, and the computed figure agrees with the base's measured MB on every preset. The after bytes are measured.

| preset | kept MB before → after | level lists B before (computed) → after | scratch B (unchanged) | starts B (new) | leaves × levels |
| --- | --- | --- | --- | --- | --- |
| ordinary | 3.8 → 3.7 | 118,080 → 39,360 | 39,868 | 960 | 7,849 × 3 |
| oregon-white-oak | 13.6 → 12.4 | 1,353,600 → 225,600 | 231,628 | 1,920 | 45,086 × 6 |
| norway-spruce | 181.0 → 140.8 | 60,368,640 → 20,122,880 | 20,437,272 | 960 | 4,024,570 × 3 |
| european-beech | 5.7 → 5.4 | 502,080 → 167,360 | 169,756 | 960 | 33,427 × 3 |
| silver-birch | 15.0 → 12.8 | 2,808,000 → 561,600 | 574,628 | 1,600 | 112,291 × 5 |
| date-palm | 0.4 → 0.3 | 131,840 → 32,960 | 33,420 | 1,280 | 6,554 × 4 |
| telperion | 108.6 → 96.0 | 16,503,040 → 3,923,200 | 4,002,556 | 3,200 | leaves 547,923 × 3, organ 236,624 × 7 |
| laurelin | 134.3 → 102.2 | 38,434,560 → 6,406,720 | 6,581,656 | 3,520 | leaves 512,761 × 3, organ 768,530 × 8 |

The spruce's level lists fall from 60.4 MB to 20.1 MB: 4,024,570 leaves × 4 B is 16.1 MB, and the rest is the buffer's 1.25 headroom. The counters, indirect arguments and level deviations are unchanged and under 300 B on every preset. The selection scratch keeps its size (Boundaries); it is now the spruce's largest foliage buffer at 20.4 MB.

## Ordered per-level leaf ids

`the_drawn_levels`: `raw/before/levels.txt` and `raw/after/levels.txt` are identical, all 90 lines (every preset, whole and close, each layer and level, the count and an FNV-1a of the ids in drawn order).

## Stills

`resident_stills` into `raw/after/`: all 24 stills (8 presets × whole, mid, close) decode to pixel data identical to `raw/before/`. Viewed: the spruce's whole and Laurelin's close.

## Frame p50 (vegetation pass, ms)

`frames <preset> full whole close`, three runs per preset after (`raw/after/frames.txt`). The base was captured once, so the base binary was rebuilt in a scratch worktree and rerun three times (`raw/base-rerun/frames.txt`; one beech whole run was refused as contended and is left out). Medians:

| preset | view | base (14:36) | base rerun | after | change |
| --- | --- | --- | --- | --- | --- |
| ordinary | whole | 0.128 | 0.128 | 0.128 | 0.0% |
| ordinary | close | 0.364 | 0.367 | 0.366 | -0.3% |
| oregon-white-oak | whole | 1.573 | 1.580 | 1.572 | -0.5% |
| oregon-white-oak | close | 7.049 | 7.126 | 6.979 | -2.1% |
| norway-spruce | whole | 5.509 | 5.711 | 6.411 | **+12.3%** |
| norway-spruce | close | 6.705 | 6.952 | 7.323 | **+5.3%** |
| european-beech | whole | 0.905 | 0.880 | 0.871 | -0.9% |
| european-beech | close | 1.848 | 1.824 | 1.812 | -0.6% |
| silver-birch | whole | 0.841 | 0.843 | 0.839 | -0.5% |
| silver-birch | close | 3.160 | 3.171 | 3.151 | -0.6% |
| date-palm | whole | 0.173 | 0.173 | 0.172 | -0.6% |
| date-palm | close | 0.405 | 0.405 | 0.404 | -0.2% |
| telperion | whole | 3.820 | 3.833 | 3.645 | -4.9% |
| telperion | close | 9.841 | 10.155 | 9.591 | -5.6% |
| laurelin | whole | 4.122 | 4.135 | 3.864 | -6.6% |
| laurelin | close | 12.171 | 11.700 | 11.424 | -2.4% |

The spruce is slower past noise. Whole, the slowest base run (5.746) is under the fastest after run (6.366); close, 6.976 against 7.319. Selection and surfacing are unchanged (whole: 0.193 and 1.60 ms on both sides), so the cost is in the foliage draw. Every other preset is level or faster. This fails the "not worse" criterion and stays open, with the cause unmeasured.
