# fn-230: points between species, sized

Raw stills and timings are in `raw/walks/` (gitignored): `strips-40.png`, `strips-80.png`, `times.tsv`. Frames come from `target/ci/examples/space_walk <out> <from> <to> <t> 1 <age>`, seed 1, bare, neutral light, on the beech's dressing.

## Host decisions (2026-10-08)

1. **Only shrink.** The factor is min(1, solved). fn-230 exists to stop blends from growing too large. A palm blend is predicted below its ends' mean (0.01x to 0.22x), and no legal factor reaches the mean. It keeps the engine's walk as it is, and at its middle it is smaller than either end (beech to palm at 0.5, age 40). This is a known property, not a defect.
2. **N points, one rule.** `blend::centroid` mixes every row by the weighted centroid, each point a share `w / W` of the way from the points before it. One factor is then solved against the points' weighted geometric mean, with no sizing in between, so every order gives the same tree. `blend::families` is the N = 2 case. The ordinary tree is this rule at equal weights and age 80: factor 0.7419269039293191, against fn-215's 0.7419269039293579 (the 5e-14 gap is the solver's tolerance). Only its lateral rows moved, in the last digits. A rerun leaves the file byte-identical.
3. **Walk age.** `space_walk` sets both ends to the walk's age before blending.
4. **Cost.** The solve is reported below.

## The solve (decision 4)

Regula falsi (Illinois) on log factor against log prediction, bracketed from 1 down. It takes 7 to 10 predictions per blend at t = 0.5. Measured once on the RTX 3080 machine:

| Walk at t = 0.5 | Blend at 40 | Blend at 80 | One prediction at 80 | Ages lived |
|---|---|---|---|---|
| beech, oak | 0.16 s | 0.80 s | 0.08 s | 11 |
| beech, spruce | 0.39 s | 2.0 s | 0.20 s | 13 |
| oak, spruce | 0.29 s | 2.7 s | 0.37 s | 14 |
| beech, ordinary | 0.63 s | 7.2 s | 0.61 s | 15 |
| oak, ordinary | 0.50 s | 3.1 s | 0.25 s | 15 |
| spruce, ordinary | 0.75 s | 5.4 s | 0.64 s | 15 |

A corner predicts in 0.03 to 0.05 s at 80 and lives in 9 or 10 of the 15 ages; the ordinary tree predicts in 0.77 s and lives in all 15. A blend lives in the union of its ends' ages, because an age only one end lives in carries that end's neighbour at lifespan 0 and mixes up from there. Each added age adds substructures to the predictor's recursion. This is inferred from the ages lived alongside the times, not profiled. No cheap bracket was found: the ends' factors are 1, so they bracket nothing that the first steps do not.

## Walks (R2)

Each frame's wall time is for the whole `space_walk` process: blend, solve, growth, mesh and render.

**At 40**, all 30 frames build, in 1.0 to 3.9 s each. The slowest frame is the spruce corner.

**At 80**, 29 of 30 frames build. The slowest built frame is spruce to ordinary at 0.25, in 15.0 s.

| Walk | 0 | 0.25 | 0.5 | 0.75 | 1 |
|---|---|---|---|---|---|
| beech, oak | 17.8 m | 19.0 m | 20.4 m | 18.4 m | 20.2 m |
| beech, spruce | 17.8 m | 20.2 m | 21.2 m | 22.0 m | 21.7 m |
| beech, ordinary | 17.8 m | 17.6 m | 19.9 m | 22.1 m | 23.1 m |
| oak, spruce | 20.2 m | 23.5 m | 24.3 m | **refused (budget)** | 21.7 m |
| oak, ordinary | 20.2 m | 20.9 m | 22.9 m | 22.9 m | 23.1 m |
| spruce, ordinary | 21.7 m | 22.2 m | 22.8 m | 22.9 m | 23.1 m |

## Open for the host

**Oak to spruce at 0.75 and 80 outgrows the budget.** Its light-neutral prediction is the target, 4.60M kept, but it is predicted to grow 21.7M phytomers, shed ones included, against the 20M budget. The corners themselves are predicted to grow 18.6M (oak) and 18.9M (spruce). Matching kept phytomers does not bound grown ones, because a blend can shed a larger share.

**The walks pop.** In `strips-40.png` and `strips-80.png`, the frames at 0.25 and 0.75 between the beech and the oak, and at 0.25 toward the ordinary tree, look much sparser than both their ends. The phytomer count is matched, but the phytomers are small. Seed 1 at 80, light-neutral:

| beech to oak | Predicted kept | Grown kept | Phytomers weighted by scale |
|---|---|---|---|
| 0 (beech) | 2.19M | 1.61M | 1.20M |
| 0.25 sized | 2.61M | 1.98M | 0.17M |
| 0.25 engine walk | 23.9M | 17.3M | 1.40M |
| 0.5 sized | 3.11M | 2.67M | 0.87M |
| 0.75 sized | 3.71M | 1.44M | 0.31M |
| 1 (oak) | 4.43M | 1.94M | 1.10M |

**The engine's walk jumps at its ends.** A millionth of the way from the beech to the spruce, at 40, the engine's walk (`telperion_space::blend`) is predicted at 2.89M kept against the beech's 0.15M, and it grows 2.14M against 0.11M. The same holds from the spruce (1.80M against 1.15M), from the beech toward the oak, and from the oak toward the spruce. The factor takes the jump out of the size (`space_walks.rs`, `the_size_is_neutral_at_either_end_and_continuous_off_it`), but not out of the shape. An inferred cause, not checked: an age one end does not live in becomes lived, at a small lifespan, the moment the walk leaves that end.
