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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
