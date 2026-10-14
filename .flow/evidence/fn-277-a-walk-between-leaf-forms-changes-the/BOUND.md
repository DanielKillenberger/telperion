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

## The law's bound (host ruling 5a)

`tests/walk_law.rs` sweeps every film walk (beech to spruce, spruce to birch, birch to oak and oak to palm at 40, palm to Laurelin at 1, Laurelin to Telperion at 80) at steps of 0.001, seeds 1, 7 and 42, and computes each form's expected count per bearer from the walked rows, unrounded: the wood's leaves per station (cauline share by leaflets), the crown's leaves per apex (living and dead fronds by leaflets), spines per frond, bases per stem, and each age's cluster leaves per short shoot (by leaflets and cauline share). The rows are walked as `blend::families` walks them before its size solve, which moves only the lateral probabilities no form reads. A step is the change over 0.001 as a share of the form's largest value along the walk (`raw/law.log`).

| form | steepest step | where |
|---|---|---|
| wood leaves per station | 0.0128 | oak to palm at t 1, palm to Laurelin at t 0.001: the cauline share falls to none as the leaflets rise to 113 on the log scale |
| crown leaves per apex | 0.0057 | the same ends: fronds and dead fronds by the leaflets' log walk |
| cluster leaves per short shoot | 0.0040 | palm to Laurelin, t 0.124 |
| any linear row | 0.0010 | |

The bound is 0.02. A ratio between steps would refuse any row that rises linearly from none (fronds 0.042 to 0.084 is twice), so the step is held as a share of the form's range. The rosette's old takeover moved the wood's leaves per station from one to none in one step, a share of 1.

## The draw's noise (host ruling 5b)

`tests/walk_leaves.rs` holds each frame's drawn counts to their expectation: leaflets per station, per cluster leaf and per frond (binomial on the leaflet count's fraction), the runs and tips kept at the cauline share (binomial on the share), and the leaves per short shoot (binomial on the cluster count's fraction); 4 sigma, dormant under 30 organs. Fronds per apex are drawn on one or two apices on every film tree, always dormant.
