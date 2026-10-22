# fn-308 result: a walk's midpoint draws a tree halfway between its ends

## Owner verdicts

- **2026-10-22, on the C4 stills, relayed by the host:** "much better nice". C4 is rule B over every row with the lifespan clamped at the tree's age (oak to palm at 40, seed 1, t = 0.25, 0.5 and 0.75). The owner approved C4's direction.
- **2026-10-22, on the final C5 stills beside C4's (`raw/stills/final-vs-c4.jpg`), relayed by the host:** "nice!". The owner approved the final stills.

## Change

Between two species' ends (0 < t < 1), the species blend now changes the ends' rows before the mix (spec R2):
- **Rule B over every row.** An age one end never reaches takes every row of the other end.
- **The clamp.** Each lifespan is clamped at `max(age, 1)`, where the age is the walked whole age (`Blend::Count`), the number of cycles the build grows.
- **The exit rows.** At an age one end leaves within the tree's age and the other does not, the other end takes the leaving end's `continuation` and `relay_ended`.
- **One aged path (R5).** The blend takes the age as a required argument.
- **The laws weigh by reach.** `walk_law` and `walk_leaves` weigh a per-age row by the frame's closed-form reach (host ruling 2).

## Proof (measured)

- **R3** (`walk_midpoint::the_walk_to_the_palm_changes_by_degree`, ignored; `raw/t3-r3.log`): oak to palm at 40, seeds 1, 7 and 42.
  - (a) the largest log-wood steps are 0.680, 0.915 and 0.767, against bounds of 0.832, 0.990 and 1.004.
  - (b) the largest fine share steps are 0.183, 0.163 and 0.189, against a bound of 0.2.
  - (c) one frame per seed falls below both ends' wood, against the base's 11, 10 and 5.
- **R6** (`walk_ends.rs`): all six walks at three seeds return their ends unchanged. It went red once with a scratch edit.
- **The gate** (`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, `raw/gate.log`): 1,476 passed, 133 skipped. The test run took 1,324 s against the 1,560 s limit, and the build and listing 152 s.
- **`npm test`** (`raw/npm-test.log`): 22 files and 177 tests passed.
- **The ends** are byte-identical to the base for all six film walks at three seeds (`raw/ends/`, probe `the_ends_written`).
- **Reviews:** the plan review returned SHIP (codex, round 2). Task .1 and task .3 returned SHIP. Task .2 returned SHIP on a single reviewer told to run no build or test command; that reviewer ran none. The earlier round, a fan-out, was stopped by the host after its reviewers ran the suites themselves (FRICTION.md).

## Look here

- **The digest.** `catalogue_identity`'s walk digest moved from 18002361237904030670 to 14357345262323445191.
- **The example point.** `the_grown_count_binds…` moved to spruce to the ordinary tree at 80, t = 0.8, because beech to spruce no longer binds anywhere.
- **Two device frames to fn-300.** Spruce to birch at 40, seed 1 frame 63 and seed 42 frame 59, are now in fn-300's ignored node test (host ruling). The device grows 1 or 2 fewer nodes than the CPU: 387,693 against 387,695, and 393,600 against 393,601.
- **Below one year.** The clamp floor of 1 changes the beech-to-spruce tree at 0.5, where the clamped ends mix to lifespans the seedling lives (R1.md, "The clamp's premise and its reader"). The host ruled this intended.

## Open

- **The completion review.** Codex's `completion-review` takes no focus text, so the no-build instruction cannot reach its prompt. It is held for the host.
- **Unequal-age walks** and **centroids of more than two points** are out of R3 (spec Boundaries).
- **A structural guard against reviewers running suites**, such as a cargo shim refusing an unadmitted `cargo test`, is the host's to spec (FRICTION.md).
