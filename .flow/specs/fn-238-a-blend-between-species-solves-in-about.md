# A blend between species solves in about a second

## Goal

The owner's aim for the harness: "looking at trees in the harness and adjusting params should yield near instant results." A walk frame between species (`telperion_core::blend`, fn-230's size factor) solves before it grows, and the solve was slow at mature ages: fn-230 measured up to 87 s at 80 near the ordinary tree, and fn-231 found the slow frames are those whose size moved, because the solve must match the costly grown count. Host (2026-10-09): a release item, its own spec, as fn-231's R5 deferral named. Scope: the blend solve only; depends on fn-231.

## Decisions

- R1 measured (counters, no profiler): the solve itself costs nothing; predictions are the cost, and grown predictions take 64 to 97 % of a frame at 80 wherever the ordinary tree is an end (one grown prediction of the ordinary tree at 80: 7.4 s; kept 0.88 s).
- Host decision (2026-10-09): build route 2 (the grown count as a running total on the first-unit recursion, like `expected_length`) plus exact caching of a walk's two ends; route 1 (faster tables, same bits) not built; the walk digest may be re-pinned when factors move only within the solve's tolerance, with every factor's change listed, the strips viewed before and after, and shipped species byte-identical; frames at 80 with the ordinary tree as an end are measured and reported, not held to 1 s; route 4 (a faster kept predictor for species with fractional rows) becomes its own post-release spec.
- Host decision (2026-10-09, second), option (c): the six frames still over 1 s spend their time on the solve's own steps; a better first guess (route 3) was kept out ("exact, not a new first guess"). The 1 s bar moves to fn-240 with route 4 and the solve's step count; R3 amended to measure and report every frame.

## Acceptance Criteria

- **R1:** where the solve's time goes (predictions per solve, cost of each, kept against grown) on the slow frames at 40 and 80 — met.
- **R2:** the solve made cheap by the cause R1 names, stated before code, solved factor equal within the solve's tolerance — met (route 2 plus cached ends).
- **R3:** every walk frame of fn-230's strips at 40 and 80 measured before and after on one core (best of 3), walk digest re-pinned only within tolerance with moved frames' stills unchanged, shipped species byte-identical; the 1 s bar moved to fn-240 (host, 2026-10-09) — met.
- **R4:** gate and npm test green — met.

## Result

- `telperion_space::expected_grown` sums grown phytomers as a running total; it matches the count table's sum within 1e-12 in the log (7e-15 at 80). On the ordinary tree at 80 it takes 1.3 s against 13.7 s. `Count::of_end` caches the 8 most recent ends' predictions, keyed exactly.
- 161 of 165 walk-digest factors bit-identical; the other 4 moved at most 3.9e-16 in the log. The nine moved strip frames render byte-identical PNGs. Family digest unchanged. Walk digest re-pinned.
- Frame times, best of 3 on one core: at 40, 0.04 to 1.38 s (was 0.29 to 4.03); at 80 without the ordinary tree, 0.14 to 3.48 s (was 2.72 to 9.16); at 80 with it, 0.40 to 10.1 s (was 13.1 to 74.6).
- Six frames remain over 1 s: spruce → ordinary at 40 (0.5, 0.75), beech → oak at 80 (0.25), beech → spruce at 80 (all three).
- Gate green; npm test 143/143.

## Open

- The 1 s bar and route 4 (cheaper kept predictor, fewer solve steps) live in fn-240.
- Review P2: `expected_grown` recurses one native stack frame per cycle; an extreme age (~1e6 cycles) could exhaust the stack. A heap-backed stack would remove it.

## Friction

- No sampling profiler (`perf_event_paranoid`, no `perf`): counters hand-added, ~10 min -> local setup, reported, not specced.
- Shell guard refuses redirects to a variable path: scripts written instead, ~5 min -> local tooling, reported, not specced.

## Evidence

- `.flow/evidence/fn-238-a-blend-between-species-solves-in-about/SOLVE.md` (the measurement, host decisions and frame tables; cited by `catalogue_identity.rs`)
- `.flow/evidence/fn-238-a-blend-between-species-solves-in-about/` probes: `solve_probe.rs.txt`, `grown_probe.rs.txt`, `walk_factors.rs.txt`, `walk_times.rs.txt`, `strips.sh.txt`, `r1-instrumentation.diff.txt` (replay sources SOLVE.md names)
