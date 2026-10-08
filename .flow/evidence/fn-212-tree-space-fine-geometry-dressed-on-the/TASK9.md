# fn-212 task 9: the stations' bases pass as a parallel scan (worker, 2026-10-07)

Host decision after task 7's profile: `bases` numbered about a million fine runs serially in one invocation, 248 to 258 ms of the stations' 284 ms on the spruce at 80.

## What was built (`2904c35d`)

`stations.wgsl` numbers the runs with an exclusive scan of each run's stations and segments, in three passes:
- **`bases`:** each block of 256 runs is scanned in a workgroup (a Hillis-Steele scan in workgroup memory), and its totals are left in the block buffer.
- **`blocks`:** one workgroup scans the blocks' totals, each lane a contiguous share, and leaves the whole tree's totals in `counts`.
- **`spread`:** each block's base is added to its runs.

**The numbering is the walk's, to the run.** `stationed`'s new `scanned` count compares every run's first station and segment with one walk over the runs in their order. It is 0 on the beech and spruce at 30, and the fast test (`fine_roots_are_stationed_on_the_device`) now holds it. Nothing downstream moves: placement reads the same words.

## Retimed: the spruce at 80, seed 1, on the real path (`raw/task9/three-x-s1.txt`)

Five builds a side, alternately, one a process, each started only below a one-minute load of 10. Builds ran at loads of 7.7 to 9.7, as task 7's seed-1 pass did.

| | Task 7 | Task 9 |
|---|--:|--:|
| Today, median, plus baseline B's conversion (965 ms) | 6,633 ms | 6,616 ms |
| Device, median, its conversion included | 857 ms | 710 ms |
| **Ratio** | **7.7×** | **9.3×** |
| Stations stage, median | 273 ms | 29 ms |

**Stage medians, device side (ms):**

| Stage | Task 7 | Task 9 |
|---|--:|--:|
| Partial conversion | 99 | 104 |
| Dress | 41 | 44 |
| Folds | 13 | 13 |
| Expansion of the part | 14 | 14 |
| CPU block: curve, stations, ring positions | 258 | 314 |
| Sweep | 64 | 70 |
| **Stations** | **273** | **29** |
| Rings and ring positions | 34 | 36 |
| Placements and mass grid | 47 | 51 |
| Curve handoff | 29 | 34 |
| Submit and drain | 5 | 5 |
| **Total** | **857** | **710** |

- **Per pass** (`PROFILE=1`, `raw/task9/profile-s1.txt`): `bases` and `spread` each took under 1.55 ms, the 24th slowest pass. `blocks` shares its label with the sweep's 31 block passes: the 32 took 7.9 ms, against 8.3 ms for the sweep's 31 in task 7. The rest of the stations stage is its other passes and its two readbacks of totals.
- **The CPU block now leads the device side.** It is the pipeline's own curve, stations and ring positions on the part, 230 to 421 ms depending on load; build 4, the slowest, also took 950 ms to settle against about 550 for the others. On a quiet build the total is about 600 ms (596 and 597 ms, builds 1 and 5).

## Gate

- **Render half,** under the GPU lock, first: 162 passed, 0 failed (`raw/task9/gate-render.txt`).
- **CPU half:** 988 passed, 0 failed (`raw/task9/gate-cpu.txt`). `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.
