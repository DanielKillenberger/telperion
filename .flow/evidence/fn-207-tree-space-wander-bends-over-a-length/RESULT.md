# fn-207: wander bends over a length, 2026-10-05

## Built

- **`Form::bend_length` (metres, neutral 0, at most 1,000):** each axis's walk (`geometry::Layer`) carries a wander curvature square to its heading, in radians per metre.
  - At each node it relaxes by exp(−internode / bend_length) and is kicked by the node's keyed draw, scaled by √(1 − keep²), so its stationary spread is the draw's.
  - The turn is the curvature times the internode, about its own direction.
  - The curvature is carried with the walk's tropism bend and sag turns.
  - A continuation or relay starts with its bearer's end curvature (`place`; the rough layout's pencils). A lateral starts with none.
- **At 0 the old per-node arithmetic runs unchanged.**

## Requirements

| R | Result |
|---|---|
| **R1** | The beech, spruce and oak 80-year hashes at seeds 1 and 7 are identical before (`97dc2233`'s engine) and after (`scratchpad/bend-hash.sh`). The palm is on its own branch and was not checked here. |
| **R2** | `tests/bend.rs`, red on the per-node draw: lag-one correlation −0.011 against exp(−0.1 / 2) = 0.951. Now green: correlation within 0.05 of that, the per-node draw uncorrelated, and the turn spread per metre both ways within 10% of wander² / 3. |
| **R3** | Walks on `bend_length` (wandering) and `wander` (bending) at every PA of the walk tree: steepest 0.16, well within 30. |
| **R4** | The oak trial: fn-195 RESULT.md, "fn-207 R4 trial"; sheet `raw/fn207/sheet.png` in fn-195's evidence. |
| **R5** | See CLOSE below. |

## Codex review 1: NEEDS_WORK, two P1s, both fixed

1. **At bend length 0 the curvature was never updated**, so a bending continuation jumped as its bearer's bend length left 0. The memoryless branch now leaves the node's own turn as the curvature (`pivot · draw`) with its rotation arithmetic unchanged.
2. **The curvature only evolved under wander > 0**, so an inherited bend froze when a successor's wander reached 0. The relaxation and turn now run whenever the bend length is above 0, with no kick without wander.

**Two walks across PA 1's continuation as PA 2** (`walk/mod.rs`):
- red on the reviewed code: jumps at bend length 0.015 and at wander 0.0025;
- green on the fix: steepest 0.12 and 0.05.

The beech and spruce 80-year hashes are still identical to `97dc2233`'s; the oak's differ only by its committed trial values.

## CLOSE

- **Workspace gate** (before review 1's fix): 1,063 passed, 0 failed, 509 s.
