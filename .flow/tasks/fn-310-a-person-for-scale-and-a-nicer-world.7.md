---
satisfies: [R2, R5]
---
# fn-310-a-person-for-scale-and-a-nicer-world.7 The person placed per shot and wholly in frame; the after frames

## Description
The spec's "Host rulings on task .7 (2026-10-23)": D6 as ruled (whole in the hero frame where a camera-side fit exists, otherwise D2 with the exemption recorded by name), and D5 as ruled (the film passes one placement per frame, lerped between the crossfaded years' placements). Then the after frames 105, 255, 720 and 825 through `film --frames`, beside task .6's before frames. The film is judged by eye.

**Size:** M
**Files:** `crates/telperion-render/src/scene/person.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/src/lib.rs`, `crates/telperion-render/examples/film.rs`, `crates/telperion-render/examples/film/*.rs`, `crates/telperion-render/tests/person_frame.rs`, other render tests, `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/`
**Touches:** [crates/telperion-render/src/**, crates/telperion-render/examples/**, crates/telperion-render/tests/**, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach
- D6 in `person::stand`: along the D2 azimuth, distance = max(ellipse reach + 0.5 m, the nearest distance at which the whole figure projects inside the hero frame with a small stated margin), found by projection, where such a distance exists on the camera's side. Where none exists, keep D2's distance. The function reports which case applied, so tests can name exemptions.
- D5: the renderer draws the placement it is given. Add a way to state the figure's placement (foot and facing) explicitly. The plain submit keeps placing per tree, so the harness and `createRenderer` are unchanged. The sun-map cache redraws when the placement changes.
- The film: for a crossfaded frame of years a and b at fade share s, compute place(a) and place(b) from each year's bounds through the same `stand`, lerp foot and facing by s, and set that one placement for both renders. In walk segments, lerp between the frame's neighbouring builds where the film crossfades there; otherwise use the frame's own tree.
- Tests:
  - `tests/person_frame.rs`: wholly in frame, head to feet, for every shipped preset at catalogue age, Telperion at 80, and every preset at age 1, with each exemption asserted by name, so a new exemption fails the test.
  - Film: in a crossfaded frame both renders get the same placement.
  - Film: placement is continuous across a segment's frames (no step larger than a stated bound between consecutive frames).
  - The canopy, shadow and sun-cache tests still hold.
- After frames 105, 255, 720 and 825 via `film --frames`, and one before/after sheet at `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/stills/film-before-after.jpg` (atomic write), with a compressed committed copy at the evidence folder's top level. View it. 825 should show one person.
- Run `--lib tint_rule_tests`, `--lib compile_size` and `--test conformance` before review.

### Investigation targets
**Required:**
- `crates/telperion-render/src/scene/person.rs` - `stand`, `figure_points`
- `crates/telperion-render/tests/person_frame.rs`
- `crates/telperion-render/examples/film.rs`, `examples/film/{path,map,camera}.rs` - the crossfade and walk frames
- `crates/telperion-render/src/scene.rs` - `place_figure`
- `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/person-fit/` - task .7's first-round measurements
## Acceptance
- [ ] Distance is max(ellipse reach + 0.5 m, nearest whole-in-frame distance) where a camera-side fit exists, otherwise D2's; found by projection with a stated frame margin
- [ ] Wholly in frame, head to feet: every shipped preset at catalogue age, Telperion at 80, every preset at age 1; each exemption asserted by name
- [ ] The renderer draws an explicitly stated placement; plain submit still places per tree (harness and createRenderer unchanged)
- [ ] The film passes one lerped placement for both renders of a crossfaded frame; tests show the same placement in both renders and continuity across a segment's frames
- [ ] Canopy, shadow and sun-cache tests green; tint rule, compile_size and conformance green
- [ ] Before/after sheet of film frames 105, 255, 720 and 825 via `film --frames` at the raw path with a committed compressed copy; 825 shows one person
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
