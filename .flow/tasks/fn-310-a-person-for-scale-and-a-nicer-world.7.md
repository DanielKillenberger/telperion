---
satisfies: [R2, R5]
---
# fn-310-a-person-for-scale-and-a-nicer-world.7 The person placed per shot and wholly in frame; the after frames

## Description
The spec's "Cut 7 findings" D5 and D6: the person stands still within a film segment (placed from the segment's largest bounds, the envelope its camera pulls back to), and stands wholly in the hero frame, head to feet, at the larger of the crown ellipse plus margin and the nearest distance where the whole 1.75 m figure fits in the hero frame with a small margin. Then the after frames 105, 255, 720 and 825 through the film path, set beside task .6's before frames.

**Size:** M
**Files:** `crates/telperion-render/src/scene/person.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/src/lib.rs`, `crates/telperion-render/examples/film.rs`, `crates/telperion-render/examples/film/*.rs`, `crates/telperion-render/tests/person_frame.rs`, other render tests, `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/`
**Touches:** [crates/telperion-render/src/**, crates/telperion-render/examples/**, crates/telperion-render/tests/**, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach
- D6 in `person::stand` (`scene/person.rs`): along the D2 azimuth, distance = max(ellipse reach + 0.5 m, the nearest distance at which the whole figure projects inside the hero frame with a small margin). The hero frame is the default shot's (`camera.rs` hero pose for the bounds, 16:9); find the distance by projection, not a fitted constant. State the frame margin.
- D5: the renderer's placement takes the bounds it is given. Give the renderer a way to place the figure for stated bounds (for example a method that places it for an envelope and holds it until the next such call or the next plain submit), so the film passes each segment's largest bounds (`examples/film/camera.rs` already pulls back to that envelope). The harness and `createRenderer` keep placing per submitted tree: their behaviour does not change. The sun's map cache must still redraw when the person moves.
- Tests:
  - `tests/person_frame.rs`: "wholly in frame, head to feet" for every shipped preset at its catalogue age, Telperion at 80, every preset at age 1, and the film's segment envelopes (from `examples/film` release.json, measured or tabled and checked against the generator as the bounds table is).
  - The film's placement is constant within each segment.
  - The canopy, shadow and sun-cache tests still hold.
- After frames 105, 255, 720 and 825 through the film path (task .6's frame selection), and one before/after sheet at `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/stills/film-before-after.jpg` (atomic write), with a compressed committed copy at the evidence folder's top level. View it: 825 shows one person, 105, 255 and 720 show the whole figure.
- Run `--lib tint_rule_tests` and `--test conformance` before review.

### Investigation targets
**Required:**
- `crates/telperion-render/src/scene/person.rs` - `stand`, `figure_points`
- `crates/telperion-render/tests/person_frame.rs` - the in-frame and canopy tests
- `crates/telperion-render/examples/film/camera.rs` - the segment envelope
- `crates/telperion-render/src/camera.rs` - the hero pose and its projection
- `crates/telperion-render/src/scene.rs` - `place_figure`

## Acceptance
- [ ] The person's distance is the larger of the ellipse reach plus 0.5 m and the nearest whole-in-frame distance, found by projection, with a stated frame margin
- [ ] Wholly in frame, head to feet: every shipped preset at catalogue age, Telperion at 80, every preset at age 1, and the film's segment envelopes (tested)
- [ ] The renderer places the figure for stated bounds; the film passes each segment's largest bounds; a test shows the placement constant within each segment
- [ ] The harness and createRenderer still place per submitted tree (unchanged behaviour, tested or shown by the existing tests)
- [ ] Canopy, shadow and sun-cache tests still green; tint rule and conformance green
- [ ] Before/after sheet of frames 105, 255, 720 and 825 through the film path exists at the raw path with a committed compressed copy; 825 shows one person; 105, 255 and 720 show the whole figure

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
