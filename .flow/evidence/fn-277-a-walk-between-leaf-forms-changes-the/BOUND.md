# fn-277 R3: the oracle's bound

Measured 2026-10-14 on this branch with `examples/walk_leaves.rs` (`--profile ci`): leaf counts of consecutive frames at steps of 0.001 in three windows (t 0 to 0.01, 0.495 to 0.505, 0.99 to 1), at 40, seeds 1, 7 and 42, on three walks whose leaf forms do not change (no rosette, no leaflets, no cluster at either end). Raw rows in `raw/bound/` (ignored).

| walk | seed 1 | seed 7 | seed 42 |
|---|---|---|---|
| oak to spruce | 1.038 | 1.062 | 1.036 |
| oak to ordinary | 1.020 | 1.016 | 1.047 |
| spruce to ordinary | 1.030 | 1.030 | 1.032 |

The largest ratio between consecutive frames is 1.062 (oak to spruce, seed 7, t 0.499 to 0.500). On the base the oak end of the oak to palm walk moved 1.10 between t 0.002 and 0.003, before any leaf form changed. The bound is 1.2: above both, and below the smallest jump the film saw from a leaf form (1.25, spruce to birch frame 323). The oracle (`tests/walk_leaves.rs`) holds every pair of consecutive frames to it; two frames that bear no leaf are dormant.

## The instance count along oak to palm (host ruling 3)

Under rulings 1 to 3 together, oak to palm at 40, t in steps of 0.025 (`raw/peak-seed*.tsv`): the peak is the oak itself at t = 0 at every seed (342,878 at seed 1, 523,164 at seed 7, 350,269 at seed 42); every frame after it bears fewer. The count falls with the engine's walk of the crown (65,000 to 21,000 over t 0.13 to 0.15, continuous at steps of 0.0005), to the palm's 6,554 at t = 1. No frame exceeds the instance budget: `maxInstances` is unbounded (`usize::MAX`) on both ends and so on every frame.
