# fn-292 task 2: the kept tree, the frame and the stills, before and after

Base 0a6664741 (`raw/before/`, task 1's capture). The change is measured twice. The first time is task 1's layout, where the draw reads its level's start word as storage (8ff19c81f, `raw/after/`). The second is this task's fix, which reads that word as a uniform (`raw/uniform/`). Seed 1, every preset at its default age, RTX 3080 on Vulkan. Raw output stays in `raw/` (ignored).

## Kept tree and the foliage's selection buffers

`the_drawing_s_memory` (`--ignored`; `raw/after/memory.txt`, and `raw/uniform/tests.txt`, which is the same line for line). The kept tree in MB is measured on both sides. The base's level lists are measured at 0.1 MB in `raw/before/memory.txt`. Their bytes are computed from the base's `select::sizes`: a whole leaf count per level, with the ledger's 1.25 headroom, at the leaves and levels the change's selection counted. The computed figure agrees with the base's measured MB on every preset. The after bytes are measured.

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

The spruce's level lists fall from 60.4 MB to 20.1 MB. Its 4,024,570 leaves × 4 B come to 16.1 MB, and the rest is the buffer's 1.25 headroom. The counters, indirect arguments and level deviations are unchanged and under 300 B on every preset. The selection scratch keeps its size (Boundaries) and is now the spruce's largest foliage buffer, at 20.4 MB. On this device the uniform and storage offset alignments are both 256 B, so the start slots did not move with the fix.

## Ordered per-level leaf ids

`the_drawn_levels`: `raw/before/levels.txt` is identical to `raw/after/levels.txt` and to `raw/uniform/levels.txt`, all 90 lines. That covers every preset, whole and close, each layer and level, with the count and an FNV-1a of the ids in drawn order.

## Stills

`resident_stills`: all 24 stills (8 presets × whole, mid, close) decode to pixel data identical to `raw/before/`, both in `raw/after/` and in `raw/uniform/`. Viewed: the spruce's whole and close, and Laurelin's whole and close. On 2026-10-18 every preset was viewed as one sheet, the base's whole, mid and close above the fix's (`raw/sheets/<preset>.jpg`, ImageMagick `montage`), all eight presets; `compare -metric AE` counts 0 differing pixels on each of the 24 pairs.

## Frame p50 (vegetation pass, ms)

`frames norway-spruce full whole close`. The base binary was rebuilt from 0a6664741 in a scratch worktree with its own target dir. The base and the change then ran interleaved as base, change, base, change, five pairs each:

| start word read as | view | base runs | base median | change runs | change median | change |
| --- | --- | --- | --- | --- | --- | --- |
| storage (`raw/paired-storage.txt`) | whole | 5.701-5.790 | 5.710 | 6.674-6.792 | 6.695 | **+17.3%** |
| storage | close | 6.778-7.013 | 6.907 | 7.565-7.651 | 7.629 | **+10.5%** |
| uniform (`raw/paired-uniform.txt`) | whole | 5.480-5.796 | 5.751 | 5.662-5.802 | 5.739 | -0.2% |
| uniform | close | 6.696-7.004 | 6.871 | 6.905-6.981 | 6.940 | +1.0% |

As storage, the spruce's draw was slower than every base run. As a uniform, every run falls within the base's range. Selection and surfacing time did not change in either case. The spruce draws about 2M needles of a few vertices each, so it is the one preset whose draw is bound by the vertex stage. The uniform start word ships.

Every preset with the fix, three runs each (`raw/uniform/frames.txt`), against the base rerun three times (`raw/base-rerun/frames.txt`). Runs refused as contended are left out, which leaves two runs where n is 2:

| preset | view | base runs | base median | fix runs | fix median | change |
| --- | --- | --- | --- | --- | --- | --- |
| ordinary | whole | 0.128-0.129 | 0.128 | 0.127-0.129 | 0.128 | 0.0% |
| ordinary | close | 0.366-0.368 | 0.367 | 0.364-0.366 | 0.364 | -0.6% |
| oregon-white-oak | whole | 1.578-1.589 | 1.580 | 1.572-1.597 | 1.573 | -0.4% |
| oregon-white-oak | close | 7.115-7.149 | 7.126 | 6.994-7.332 | 7.008 | -1.7% |
| norway-spruce | whole | 5.534-5.746 | 5.711 | 5.501-5.714 | 5.509 | -3.5% |
| norway-spruce | close | 6.901-6.976 | 6.952 | 6.717-6.886 | 6.737 | -3.1% |
| european-beech | whole (n 2, 2) | 0.880 | 0.880 | 0.872-0.875 | 0.874 | -0.6% |
| european-beech | close | 1.822-1.844 | 1.824 | 1.812-2.548 | 1.820 | -0.2% |
| silver-birch | whole (fix n 2) | 0.843-0.844 | 0.843 | 0.841-0.843 | 0.842 | -0.1% |
| silver-birch | close | 3.166-3.179 | 3.171 | 3.150-4.358 | 3.161 | -0.3% |
| date-palm | whole | 0.172-0.173 | 0.173 | 0.172-0.186 | 0.173 | 0.0% |
| date-palm | close | 0.404-0.405 | 0.405 | 0.404-0.405 | 0.404 | -0.2% |
| telperion | whole | 3.716-3.854 | 3.833 | 3.643-3.647 | 3.647 | -4.9% |
| telperion | close | 9.859-10.570 | 10.155 | 9.592-9.609 | 9.597 | -5.5% |
| laurelin | whole | 4.128-4.140 | 4.135 | 3.861-3.886 | 3.875 | -6.3% |
| laurelin | close | 11.441-12.443 | 11.700 | 11.292-11.340 | 11.318 | -3.3% |

No preset is slower than the base beyond noise. Telperion and Laurelin are 3 to 6% faster.

## After merging the release tip (2026-10-18, b2a262857)

The release tip 0e3c8e30d was merged in with no conflicts. No file changed on both sides. Completion review (codex, base the release branch): SHIP, R1 to R3 met, before the merge.

- Gate (`telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`): 1,406 passed, 0 failed, 110 ignored, 02:36:38 to 03:32:31, admission wait included (`raw/gate-merge.log`).
- `npm test`: 18 files, 169 of 169 tests passed (`raw/npmtest-merge.log`).

Merged again with the release tip after fn-291 landed (23fb243e3, merge de0358cb8). No file is shared with fn-291.

- Gate: 1,418 passed, 0 failed, 110 ignored, 04:18:22 to 04:52:55 (`raw/gate-merge2.log`).
- `npm test`: 19 files, 170 of 170 tests passed (`raw/npmtest-merge2.log`).
