# A walk never grows a tree larger than its ends

## Goal & Context

Found by the teaser session (2026-10-09) on F4 d1b1efa9: the oak → birch walk at age 80, seed 7, fails at t ≈ 0.0750 with "resource limit: engine phytomer budget" (the hard 20M budget, `pipeline/space.rs:25`). Repro: `headless --preset oregon-white-oak --family {"age":80} --to silver-birch --to-family {"age":80} --seed 7 --walk 6 --hold 0`; frames 1-23 render and frame 24 at t = 0.07504803866484144 fails. Both ends are far below the budget (oak 2.19M nodes, birch 2.93M at seed 7, age 80), so a point near the oak end grows more than 6× either end. fn-230's size factor is shrink-only against the ends' weighted geometric-mean counts, and fn-231 made the ends continuous, so this contradicts both. Unknown, to be measured: whether the solve's predicted counts at that point are within budget (the closed form is light-neutral, and the oak grows by light past its prediction, fn-214), whether the shrink factor reaches its floor, or whether something else grows. Host (2026-10-09): release map item (the harness's walks), with the age-walk spec fn-250.

## Requirements

- **R1 (repro first):** the failure reproduced as a standalone test at the exact t; at that point, the predicted kept and grown counts against the realized ones, the solved factor and whether it hit a bound, and which ages or rows grow; the cause named and checked against the code before any fix.
- **R2 (fixed):** every frame of a walk between two shipped species at 40 and 80 stays within its ends' sizes by the factor's own rule, and never fails on the budget; the fix by the cause R1 names, stated before code, by degree, both executors.
- **R3 (proved):** a sweep of oak, birch, beech, spruce and ordinary pairs at 40 and 80, seeds 1, 7 and 42, at fine t near both ends and in between, every frame's realized kept and grown counts within the rule; fn-230/fn-231 tests and the walk digest held or re-pinned with evidence; gate and npm test green.

## Boundaries

The walk's sizing. Release map item.
