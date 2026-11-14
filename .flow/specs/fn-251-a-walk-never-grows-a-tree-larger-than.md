# A walk never grows a tree larger than its ends

## Goal

Found by the teaser session (2026-10-09) on F4 d1b1efa9: the oak → birch walk at age 80, seed 7, failed at t ≈ 0.0750 with "resource limit: engine phytomer budget" (the hard 20M budget), although both ends are far below it (oak 2.19M nodes, birch 2.93M), contradicting fn-230's shrink-only size factor and fn-231's continuous ends. Host (2026-10-09): release map item (the harness's walks), with the age-walk spec fn-250. Scope: the walk's sizing.

## Decisions

- Owner (2026-10-09), on interpolating multiplicative rows in their natural scale (log-odds, logs; each row's scale declared once): "cleaner design sounds good".
- Host (2026-10-09), on R1/R1b's measurement: the cause is light, not the size solve. Only the oak read light; the walk mixes its extinction and light-response rows toward the birch's zeros, so a frame is shaded less and grows toward the oak's unshaded size (37.3M predicted); the size factor (0.996) solves on light-neutral counts. Natural-scale rows rejected on evidence: 10 of 45 frames met the oracle against 30 for linear rows with the size factor, undershooting by two orders; the owner's "cleaner design" was taken on a premise that did not hold. Linear rows and the size factor stay. fn-250's age walk did not reproduce. The light half waits for fn-255; until then frames past the budget fail with the named error, never a fallback.
- Host (2026-10-14, re-measured on F4 after fn-255): the defect is gone (`the_oak_to_birch_frame_the_teaser_refused_builds`). The oracle's reference becomes the ends' spread across seeds 1, 7 and 42 at the frame's light, widened by 10 %, with two whole phytomers of slack for seedlings. Ends are grown once per walk.
- Host (2026-10-14, after the room check): a two-sided size factor is not built: the laterals cannot lift oak ↔ spruce at 40 to its ends (47 % and 32 % short at full room), "question, delete, then optimise". The size factor stays shrink-only; the oak ↔ spruce dip is a named exception and a later spec; the release film's path avoids that step (fn-272).
- Host (2026-10-14, after the seed-spread oracle): the oracle is not widened to pass the rest; the nine failing frames are named exceptions with measured numbers and fn-273 takes them. R2/R3 close on this record.

## Acceptance Criteria

- **R1:** the failure reproduced as a standalone test at the exact t; predicted against realized counts, the solved factor, and the cause named against the code — met (cause: light).
- **R1b:** natural-scale interpolation of multiplicative rows measured against linear rows and the size factor on the oracle; the size factor deleted if natural-scale rows keep every frame within its ends — met (measured; natural-scale rejected, size factor kept).
- **R2:** every frame of a walk between two shipped species at 40 and 80 meets the oracle and never fails on the budget; fix by the cause, by degree, both executors — met on the budget; 9 named exceptions to fn-273 (host, 2026-10-14).
- **R3:** sweep of every pair of shipped branching species and the date palm at 40 and 80, seeds 1, 7, 42; fn-230/fn-231 tests and walk digest held — met on the seed-spread record (86 of 95 frames).

## Result

- `examples/walk_size.rs`: 86 of 95 frames meet the seed-spread oracle; none nears the budget. The nine exceptions (oak ↔ spruce dips at 40 and 80, predicted 0.33M against 0.97M and 1.15M; birch → oak at 40 up to 21 % over at seeds 7 and 42; oak → birch at 80 t = 0.01, 6 % over; oak → ordinary at 40 t = 0.75 just under; beech → oak at 80 t = 0.5, 22 % under; palm → Laurelin at age 3) are held at their measured numbers by `tests/walk_sizes.rs` until fn-273.
- The film's seedlings at age 1 pass; its birch → oak step is judged in the film by the owner.
- Gate 1,348 passed; npm test 162. Impl-review SHIP.

## Open

- fn-273: a walk's size between its ends, from either side (the nine exceptions).
- The oak ↔ spruce dip's cause (row mixture between a monopodial whorled crown and a broad one): a later spec.

## Friction

- Measuring a frame's realized size needed a realized grown count on the build's result -> proposed, plus an oracle cached per walk.
- Re-measuring regrew both ends every frame -> ends grown once per walk (built).
- The exception test's first run waited on admission -> the repro grows through the engine alone; admission could count a job's measured peak (proposed).
- The workspace gate OOMs below 13 GB / 2 threads in telperion-core's unit tests -> a declared memory size or a nextest group for heavy tests (proposed).

## Evidence

- `.flow/evidence/fn-251-a-walk-never-grows-a-tree-larger-than/WALKS.md` (measurements and host decisions; cited by `tests/walk_sizes.rs`)
- `.flow/evidence/fn-251-a-walk-never-grows-a-tree-larger-than/natural-scale.patch` (the rejected natural-scale interpolation)
