# fn-212 task 8: `SHORTEST` decided in grown micrometres (worker, 2026-10-07)

Host decision on TASK6's option: the conversion's `SHORTEST` merge is decided on the recorded grown length, in whole micrometres, on both executors.

## What was built (`9bc9e02a`)

- **The rule.** A phytomer merges into the last node drawn while the wood grown since that node rounds to under 1,000 whole micrometres (`plan::micrometres` of `pending`, the internodes at their scales). Before, the rule was the chord from the last node to the phytomer's laid tip being under 1 mm. `pending` is the length each node records as `Tree::grown`, which stations already count in micrometres (task 6).
- **Both executors:**
  - `space.rs` (`SHORTEST` = 1,000 µm);
  - `dress.wgsl` (`floor(pending · 1e6 + 0.5) < 1000`, from the f32 sum of the same internodes).
  - The `behind` and `inside` merges of a relay are geometric and unchanged.
- **The probes' `SHORTEST` class** (`placed_tests::at_threshold`) is now this threshold: a node whose grown wood lies within 0.01 µm of 999.5 µm. The old chord test no longer names a decision.

## Merges per tree, before and after (`merges_per_tree`; `raw/task8/merges-*.txt`)

Merged: the phytomers the conversion takes (to the first under `FINEST` on each axis) less the nodes it draws. The behind and inside merges are in both columns alike.

| Tree | Nodes before | Nodes after | Merged before | Merged after |
|---|--:|--:|--:|--:|
| beech 40 s1 | 102,255 | 102,254 | 1,698 | 1,699 |
| beech 40 s7 | 176,627 | 176,626 | 1,384 | 1,385 |
| beech 80 s1 | 1,525,190 | 1,525,182 | 18,865 | 18,873 |
| beech 80 s7 | 2,532,840 | 2,532,779 | 29,695 | 29,756 |
| spruce 40 s1 | 1,147,614 | 1,147,167 | 889 | 1,336 |
| spruce 40 s7 | 1,107,565 | 1,107,131 | 807 | 1,241 |
| spruce 80 s1 | 4,637,481 | 4,635,551 | 4,799 | 6,729 |
| spruce 80 s7 | 4,610,321 | 4,608,272 | 4,831 | 6,880 |

- **Merges rise:** by 1 to 61 on the beech, and by 430 to 2,050 on the spruce (0.04 % of its nodes at 80). Where a tip is laid further from its node than its grown wood, the chord passed 1 mm while the grown wood did not. The spruce's short fine internodes are where this happens.
- **Instances:**
  - beech 80 s1: 2,474,226 → 2,474,225;
  - spruce 80 s1: 16,871,100 → 16,870,693.

## Stills at 80, seed 1 (`raw/task8/stills/`, all eight viewed)

- **Binaries:** before is `raw/task8/bin/headless-before`, the branch at task 7's close; after is `headless-after`.
- **Views:** `--family raw/task3/age80.json`, hero, and close (`--camera '{"fill": 10, "targetHeight": 0.45, "elevation": 5}'`).
- **Indistinguishable by eye,** all four pairs.

| Still | Pixels apart, exact | Under 6 % fuzz |
|---|--:|--:|
| beech hero | 0.19 % | 0.05 % |
| beech close | 0.17 % | 0.03 % |
| spruce hero | 0.18 % | 0.01 % |
| spruce close | 0.53 % | 0.04 % |

## The eight-tree probes after it (`raw/task8/probes.txt`, one tree a process)

- **The `SHORTEST` class is empty on all eight trees:** the placement probe's `SHORTEST` count is 0 everywhere.
- **The real path's parted run on spruce 80 s7, TASK5's `SHORTEST` flip (run 62151), is gone:** parted 0 on both seeds. Every run is within the screen on both spruce-80 seeds (673,758 and 675,507).
- **Dress:** device and reference fine nodes equal on all eight (for example 4,211,730 at spruce 80 s1), 0 strays, folds within 4.6 µm.
- **Sweep:**
  - runs equal one for one on all eight, 0 unmatched;
  - the spruce at 80 keeps its socket-sample residual against a CPU-settled reference (13 and 11 runs; TASK5 residual 2), which the real path does not have;
  - beech 80 s7 keeps its codominance-near-`FORK_FROM` run (6 %), the named f32-against-f64 class.
- **Stations:** counts equal on seven trees. Spruce 80 s1 keeps task 6's half-micrometre rounding class (111 keys, 2 counting apart).
- **Placement:** no leaf over the bound on any tree. Spruce 80 s1 has the rounding class (44 leaves); spruce 80 s7 has its one grazing seat.

## Gate

- **Render half,** under the GPU lock, first: 162 passed, 0 failed (`raw/task8/gate-render.txt`).
- **CPU half:** 988 passed, 0 failed (`raw/task8/gate-cpu.txt`). `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.
