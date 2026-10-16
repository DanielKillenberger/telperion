# The engine's walk is continuous at its ends

## Goal

fn-230 found the engine's walk between species (`telperion_space::blend`) jumped at its ends: a millionth of the way from beech to spruce at 40 predicted 2.89M kept phytomers against the beech's 0.15M, and interior frames (beech to oak at 0.25 and 0.75, beech to ordinary at 0.25) looked much sparser than both ends. fn-230's size factor removed the jump from the size, not the shape. The tree space's claim (STRATEGY.md) is that every setting changes the tree by degree. The engine's walk only.

## Decisions

Full record in `ENDS.md`.
- R1 refuted the spec's guess (newly lived ages): the counts jump because the engine grew a fractional element (a bud count of 1.000002, a lifespan just off a whole number, a shedding stage) whole, sized by its share. Size-weighted mass never jumped.
- **Host decision (2026-10-08), way 1:** "a fractional element is present at its share, and when present it is whole", drawn by its own lineage key, replacing "counted whole, sized by the mean share"; the predictor counts it at its share. A lifespan below a tick lives one tick with probability equal to its fraction, so walks off the ordinary tree become valid. R4's "byte-identical" read as: identical where no row is fractional.
- Second to fifth host answers (2026-10-08): a partial unit lays its share of the nodes; lifespan crossings held at the tick's resolution (`TICK_RESOLUTION` holds `states[0..2].lifespan` at 1e-5).
- Sixth (2026-10-08): a cut unit's share comes from the nodes' presences, not their count ("nothing is ever laid whole"); R5 amended to "measured and reported", its fix deferred to its own blend-solve-cost spec.
- **Seventh (2026-10-09): the ordinary tree keeps its pre-fn-231 rows.** Re-solved, it reached the corners' geometric mean but at 80, seed 7 read as a hedge (40 km of wood against 13 km). The owner defined it as "the center point of the tree space. The mid avg tree", so the sizing measure is the owner's question. `the_ordinary_tree_is_the_centroid_of_the_branching_species` is skipped with that note; `examples/centroid.rs` stays as a tool.

## Acceptance Criteria

- **R1:** The jump located at t = 0+ and 1- for every pair at 40 and 80, checked against the guess — met; guess wrong.
- **R2:** The walk changes the tree by degree at its ends and inside, by a rule stated before code — met (host's way 1 and its refinements).
- **R3:** A convergence test at both ends for every pair at 40 and 80 replaces the raw-jump assertion; fn-230's frames re-judged for pop; fn-230's factor holds — met.
- **R4:** Gate and npm test green; every shipped species byte-identical where no row is fractional — met (16/16 hashes).
- **R5:** Solve time measured before and after and reported (amended) — met; slower frames carried to the blend-solve-cost spec.

## Result

- A millionth off each end, kept / grown / wood are within 1e-5 of the end at 40 and 80 (beech to spruce at 80 keeps 2,187,253 against 2,187,240; before 186,172,110). Walks off the ordinary tree now build.
- `the_engines_walk_converges_on_either_end_at_40_and_80` runs in 64 s; fn-192's degree tests pass. All 60 of fn-230's frames viewed: no frame sparser than both ends; the pop is gone at 40 and 80.
- `space_hash` for beech, spruce, oak and palm at 40 and 80, seeds 1 and 7: identical.
- Cost: one prediction unchanged; most solves faster (slowest 53.6 -> 8.4 s); some frames more than 10 % slower because their solve now binds on the grown count.
- The ordinary tree, unchanged in rows, now keeps 1,166,278 light-neutral phytomers at 80 against the corners' geometric mean 3,559,562 (0.328): the crowded nodes no longer count.

## Open

- For the owner: what measure sizes the ordinary tree (wood, crown volume, another age, or accepting density).
- A cheaper grown prediction for the blend solve: the blend-solve-cost spec.

## Friction

- The spec's guess was written unchecked; a single-row injection probe found the cause (25 min) -> check a guess against the engine before writing it into a spec.
- Age-80 measurement slow (~5 s per grown prediction) -> same as fn-230's: a cheaper grown count.
- Host rulings made before probing (rounds 2-5, several hours and four escalations) -> run a cheap probe of a rule against the walk tests before ruling; keep the tick probe as an example.
- The GPU lock held by another session's 45-minute sleep (30 min wait) -> proposed a lock that names its holder and purpose.
- A reboot took the `/tmp` scratchpad's base worktree; the sixth answer's premise (arithmetic mean) did not match `blend::size` -> keep base worktrees outside `/tmp`; check the code before ruling.

## Evidence

- `.flow/evidence/fn-231-the-engines-walk-is-continuous-at-its/ENDS.md` (the measurement, seven host answers, results; cited by tests)
- `.flow/evidence/fn-231-the-engines-walk-is-continuous-at-its/*.rs.txt` (the probes and timers ENDS.md names)
