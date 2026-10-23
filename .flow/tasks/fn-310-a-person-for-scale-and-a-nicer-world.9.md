---
satisfies: [R5]
---
# fn-310-a-person-for-scale-and-a-nicer-world.9 Fix the rim_at_distance regression from the cut-7 placement work

## Description
The gate at b49512928 failed one test that passed in the green gate at f58a01cc8: `telperion-render::rim_at_distance a_leaf_under_a_pixel_takes_the_band_s_area_share_not_the_band` (width 0.1: near 0.114, far 0.137; `|far/near - 1| < 0.2` fails). It fails deterministically on three reruns. Since f58a01cc8 only tasks .6-.8 changed the renderer (`generation/submit.rs`, `lib.rs`, `scene.rs`, `scene/beside.rs`, `scene/person.rs`), so one of them moved the far view of this test. Log: `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/gate/gate-3.log`.

**Size:** S
**Files:** whichever renderer file is at fault; `crates/telperion-render/tests/rim_at_distance.rs` only if the test's premise was changed by design, which must be shown, never assumed
**Touches:** [crates/telperion-render/src/**, crates/telperion-render/tests/**, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach
- Bisect over the task commits (for example 3e63d7e32 for .7, 837853234 for D7, f82a3ea75 for the film placement) without moving the branch: build a commit from `git archive` into the scratch dir with its own target dir, or test by reasoning about the diff and confirming with one measured run. Name the commit that turns it red.
- Find the cause by measurement. Is the person, or its shadow in the sun's map, now inside the test's far view of the beech where it was not before? Did a placement or sun-map change alter what the band-share measurement counts?
- Fix the cause in the renderer, so the person never changes a leaf measurement it should not. Weakening the test is not a fix. If the test's own premise is what changed by design, stop and return NEEDS_HUMAN with the numbers.
- Run the test red first, then green after the fix, plus the person, scene, shadow, sun_cache, film_placing and person_frame tests and the static checks (tint rule, compile_size, conformance).

## Acceptance
- [ ] The commit that turned the test red is named, with the measured cause (D7, 837853234: the sun map refit by the person's placement, and the far share's rounding noise)
- [ ] rim_at_distance averages the far share over five distances (7.6-8.4 times the hero distance), as the host ruled; its claim and 20 % bound are unchanged; the comment states the measured noise; green
- [ ] Sun-map metres per texel with and without the person in the fit, measured for every shipped preset at catalogue age, Telperion at 80, under 135/30 and 115/60, and the film's worst frame; the table is in the evidence
- [ ] No case grows by more than 10 % and a test holds that bound, or the task returned NEEDS_HUMAN with the numbers
- [ ] Person, placement, film and static checks stay green
## Done summary
rim_at_distance went red at 837853234 (D7). Moving the person refits the sun's map, and the test's single far picture is too coarse to tell 0.137 from the 0.1365 bound: about 110 band steps and 840 face steps, so ±0.01 of rounding noise. Following the host's rulings, the test now averages the far share over 7.6 to 8.4 times the hero distance (far 0.128 against near 0.114). The person's cost to the sun's map is held by two bounds: at most 10 % on both axes beside every catalogue still, and at most 2.3 cm or 10 % on every film frame.

- Bisect, measured from exports: 3e63d7e32 green (far 0.129), 837853234 red (far 0.137). Measured cause: the person's box in the map's fit moves the far share 0.130 to 0.136, and its shadow on the leaves moves it to 0.137. Within ±1 % of the camera distance the share spans 0.120 to 0.137.
- Catalogue bound (`person_frame` `the_person_coarsens_the_suns_map_by_at_most_a_tenth`): fast, always run. Every shipped preset at its catalogue age and Telperion at 80, under 135/30 and 115/60, measures 1.000 on both axes.
- Film bound (inside `film_placing`'s `holds`): always run over the modelled crowns. The real crowns are covered by the ignored `the_person_keeps_to_the_cameras_left_beside_the_films_real_crowns`, which reads the gitignored raw/film-bounds.tsv; it ran green once, worst 2.18 cm at frame 780.
- Tables: SHADOW-FIT.md, including the correction to the worst film texel. Renderer surface: `sun_light`, `SunLight` and `SUN_RESOLUTION` re-exported for the tests.
- Checks run green: rim_at_distance, person_frame and film_placing (ignored tests included), shadow, sun_cache, conformance, lib scene/tint_rule_tests/compile_size, fmt --check, clippy --tests. The reviewer's sandbox had no GPU, so it did not run rim_at_distance; it ran green here.

stage: impl-review - ran (codex, one correctness reviewer, SHIP)
Tier: session at default effort (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 18834f2ad7b6dba738bb5fefe950e3ae6022619f, 8ef7432cc4d7c1992f8f6d6e1947dbad9f6d58c2, 98a6da1f7f9b8d57a787acc9c90593519fed3bc6, 1b33177e785827f8b1efd6fa8c6a90ff1b0d8661, 5301bd4d33c4491efacf99c64c8da494a44373e0, d65e88f4cd2fb6ee39d2f88c01da562ea8eac77b, da2b787c7199a32ed25bb58b9610fc29b2c86a64, 359e5da969def712aae0c5736d084ace281f7868, 195a91593f85b5aeb93b357898c44080bc812c46
- Tests: cargo test --profile ci -p telperion-render --test rim_at_distance --test person_frame --test film_placing -- --include-ignored, cargo test --profile ci -p telperion-render --test shadow --test sun_cache --test conformance, cargo test --profile ci -p telperion-render --lib -- scene tint_rule_tests compile_size, cargo fmt -p telperion-render --check; cargo clippy --profile ci -p telperion-render --tests
- PRs: