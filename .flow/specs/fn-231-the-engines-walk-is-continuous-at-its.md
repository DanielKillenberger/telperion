# The engine's walk is continuous at its ends

## Goal & Context

fn-230 found the engine's walk between species (`telperion_space::blend`) jumps at its ends: a millionth of the way from beech to spruce at 40 predicts 2.89M kept phytomers against the beech's 0.15M, and interior frames (beech to oak at 0.25 and 0.75, beech to ordinary at 0.25) look much sparser than both ends: at 80, beech to oak at 0.25 keeps 1.98M phytomers that weigh 0.17M against the beech's 1.20M. fn-230's size factor removes the jump from the size, not from the shape. The worker's unchecked guess: a physiological age only one end lives in becomes lived, at a tiny lifespan, the moment the walk leaves that end, growing a burst of tiny phytomers. The tree space's claim (STRATEGY.md) is that every setting changes the tree by degree.

## Requirements

- **R1 (measure first):** the jump located: which rows of `blend` change discontinuously at t = 0+ and 1-, checked against the guess, with the predicted kept and grown counts and size-weighted mass at t = 0, 1e-6, 1e-3, 0.01 for every pair of passed branching species at 40 and 80.
- **R2 (continuous):** the walk changes the tree by degree at its ends and inside: an age one end does not live in fades in (its contribution to counts, mass and shape goes to 0 as t goes to its end), by a rule stated before code and applied to every row the measurement names.
- **R3 (proved):** a walk test at the ends (t in 1e-6..1e-2) for every pair, extending the walks' degree test; fn-230's walk frames re-judged for visible pop at 40 and 80, strips viewed; fn-230's factor still holds.
- **R4:** gate and npm test green; every shipped species byte-identical (t = 0 and 1 unchanged).

## Boundaries

The engine's walk only. Depends on fn-198; fn-230's R2 pop clause depends on this.