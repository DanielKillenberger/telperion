# fn-230: points between species, sized

Raw stills and timings are in `raw/walks/` (gitignored): `strips-40.png`, `strips-80.png` and `times.tsv`, with the kept-only round in `kept-only/` and `times-kept-only.tsv`. Frames come from `target/ci/examples/space_walk <out> <from> <to> <t> 1 <age>`, seed 1, bare, neutral light, on the beech's dressing.

## Host decisions (2026-10-08)

1. **Only shrink.** The factor is min(1, solved). fn-230 exists to stop blends from growing too large. A palm blend is predicted below its ends' mean (0.01x to 0.22x), and no legal factor reaches the mean. It keeps the engine's walk as it is, and at its middle it is smaller than either end (beech to palm at 0.5, age 40). This is a known property, not a defect.
2. **N points, one rule.** `blend::centroid` mixes every row by the weighted centroid, each point a share `w / W` of the way from the points before it. One factor is then solved against the points' weighted geometric mean, with no sizing in between, so every order gives the same tree. `blend::families` is the N = 2 case. The ordinary tree is this rule at equal weights and age 80: factor 0.7419269039293191, against fn-215's 0.7419269039293579 (the 5e-14 gap is the solver's tolerance). Only its lateral rows moved, in the last digits. A rerun leaves the file byte-identical.
3. **Walk age.** `space_walk` sets both ends to the walk's age before blending.
4. **Cost.** The solve is reported below, and is left open for the harness.
5. **Kept and grown (option A).** Kept phytomers do not bound grown ones, since a blend can shed a larger share than either end. A factor is solved against the predicted kept phytomers and another against the predicted grown ones (`expected_counts`, the shed included, which is what the engine's budget counts); the smaller is taken. The grown factor is solved from the kept one down, so where kept binds it costs one grown prediction. The ordinary tree is bound by kept and did not change.
6. **The visible pop is fn-231's.** R2 now asks only that every frame builds within the budget. The pop is owned by fn-231, "the engine's walk is continuous at its ends", and re-judged when it lands.

## Walks (R2)

Each frame's wall time is for the whole `space_walk` process: blend, solve, growth, mesh and render. Times are measured once on the RTX 3080 machine, other work sharing it.

**At 40**, all 30 frames build, in 1.0 to 5.2 s each. Every height is the kept-only round's: kept binds every blend at 40.

**At 80**, all 30 frames build. Oak to spruce at 0.75, refused by the budget when sized on kept alone, now builds at 22.6 m, bound by its grown count. Every other height is the kept-only round's. **The slowest frame at 80 is spruce to ordinary at 0.25, 87.4 s.** Most of that is the solve; the same frame took 15.0 s sized on kept alone.

| Walk at 80 | 0 | 0.25 | 0.5 | 0.75 | 1 |
|---|---|---|---|---|---|
| beech, oak | 17.8 m | 19.0 m | 20.4 m | 18.4 m | 20.2 m |
| beech, spruce | 17.8 m | 20.2 m | 21.2 m | 22.0 m | 21.7 m |
| beech, ordinary | 17.8 m | 17.6 m | 19.9 m | 22.1 m | 23.1 m |
| oak, spruce | 20.2 m | 23.5 m | 24.3 m | 22.6 m (63.3 s) | 21.7 m |
| oak, ordinary | 20.2 m | 20.9 m | 22.9 m | 22.9 m | 23.1 m |
| spruce, ordinary | 21.7 m | 22.2 m (87.4 s) | 22.8 m (71.9 s) | 22.9 m (67.1 s) | 23.1 m |

Frame times at 80 otherwise run 3.7 to 28 s.

## Open for the harness: the cost of a solve

A blend costs the time of every prediction it makes. Regula falsi (Illinois) on the log factor against the log prediction takes 7 to 10 predictions per count. One `blend::families` call, measured once:

| Walk | At 40 | At 80, t = 0.5 | At 80, t = 0.75 |
|---|---|---|---|
| beech, oak | 0.42 s | 6.5 s | 5.6 s |
| beech, spruce | 0.81 s | 3.0 s | 2.8 s |
| oak, spruce | 2.45 s | 9.3 s | 51.4 s |
| beech, ordinary | 1.27 s | 18.4 s | 19.0 s |
| oak, ordinary | 1.11 s | 14.6 s | 17.0 s |
| spruce, ordinary | 1.42 s | 60.0 s | 57.7 s |

(At 40 the column is t = 0.5.)

| One prediction at 80 | Kept | Grown |
|---|---|---|
| beech | 0.03 s | 0.54 s |
| oak | 0.03 s | 0.81 s |
| spruce | 0.05 s | 0.14 s |
| ordinary | 0.76 s | 5.75 s |
| a blend (t = 0.5 or 0.75) | 0.07 to 0.87 s | 0.33 to 7.0 s |

**Why a blend predicts slower than a corner** (inferred, not profiled): a corner lives in 9 or 10 of the 15 ages; a blend lives in the union of its ends' (11 to 15), because an age only one end lives in carries that end's neighbour at lifespan 0 and mixes up from there. Each added age adds substructures to the predictor's recursion. The grown count, `expected_counts`, keeps a per-cycle table for every substructure, so it costs 5 to 20 times the kept count. No cheap bracket was found: the ends' factors are 1, so they bracket nothing the first steps do not. Solving grown from the kept factor down was taken, and it cut the ordinary tree's derivation from 59 s to 14 s.

A walk in the harness would pay up to a minute a frame at 80. Making `expected_counts` cheaper is engine work outside this spec's blend rule.

## The visible pop (fn-231)

In `strips-40.png` and `strips-80.png`, the frames at 0.25 and 0.75 between the beech and the oak, and at 0.25 toward the ordinary tree, look much sparser than both ends. The phytomer count is matched, but the phytomers are small. Seed 1 at 80, light-neutral, sized on kept alone:

| beech to oak | Predicted kept | Grown kept | Phytomers weighted by scale |
|---|---|---|---|
| 0 (beech) | 2.19M | 1.61M | 1.20M |
| 0.25 sized | 2.61M | 1.98M | 0.17M |
| 0.25 engine walk | 23.9M | 17.3M | 1.40M |
| 0.5 sized | 3.11M | 2.67M | 0.87M |
| 0.75 sized | 3.71M | 1.44M | 0.31M |
| 1 (oak) | 4.43M | 1.94M | 1.10M |

**The engine's walk jumps at its ends.** A millionth of the way from the beech to the spruce, at 40, the engine's walk (`telperion_space::blend`) is predicted at 2.89M kept against the beech's 0.15M, and it grows 2.14M against 0.11M. The same holds from the spruce (1.80M against 1.15M), from the beech toward the oak, and from the oak toward the spruce. The factor takes the jump out of the size (`space_walks.rs`, `the_size_is_neutral_at_either_end_and_continuous_off_it`), but not out of the shape. An inferred cause, not checked, which fn-231's first task checks: an age one end does not live in becomes lived, at a small lifespan, the moment the walk leaves that end.
