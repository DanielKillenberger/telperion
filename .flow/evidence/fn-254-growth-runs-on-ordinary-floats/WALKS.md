# Continuity walks under f32: decision 9 at f32 resolution (host, 2026-10-10)

A continuity walk checks the model, not the last bit of the arithmetic. Decision 9 (2026-10-05) refines a walk's steepest steps six times by eighths. Under f32 it runs as follows (`tests/walk/measure.rs`, `Refined`):

- Refinement stops once a sub-step spans no more than 16 f32 places (ulps) of the setting's value (`ULPS`).
- Where a change fails its shrink alone, the test measures a noise floor at the point where refinement ended. It nudges the setting by ±1 to 16 f32 places and takes the largest change in the tree.
- The shrink requirement (20-fold in the walks and the relay census, 100-fold for landing) applies only to a change above four times that floor. The test prints the floor and its changes.

Decision 9's premise is that the model is smooth, and that still holds. Only the resolution of the check changes. No f64 islands were added (option b was declined).

## The two walks this decides, as measured

| Walk | Changes by level (share of the tree) | Floor | Verdict |
|---|---|---|---|
| sag landing, seed 1 | 0.050, 0.046, 0.029, 0.0051, 0.00090 (stops at level 4, within 16 places) | 9.0e-4 | slope |
| light: sky, seed 3 at 0.005 | 0.044, 0.070, 0.031, 0.020, 0.013, 0.0042, 0.0067 | 6.7e-3 | slope at f32 resolution |

On sag, the finest sub-step is flat for 12 of 16 samples. It then steps once, by 5.5e-4 of the tree's height (branch `b2b16247`): one f32 place of the sag setting.

## For fn-255: amplification its new grower must re-check

`light_changes_the_tree_by_degree` at seed 3 shows the amplification the light lattice and the balance machinery add to f32 rounding:

- **Light input to subtree totals:** a sky change of a few f32 places (about 3e-7 relative) moves the subtree light total `q` of axis `760add40` by 2.4e-4 relative, about 300 times the input.
- **Memory to shed:** that moves the subtree's balance memory by 0.27% (-0.025292 to -0.025222). The shed threshold `t` then crosses the draw `d` (2.2495e14 to 2.2434e14 against 2.2489e14). The subtree goes from shed to kept 0.05 within one step, because the shed window (0.05 log-odds) tips at that memory change.

fn-255 replaces the lattice and the balance. Its grower must measure both figures again: the gain from light to totals, and the memory change that tips the shed window.
