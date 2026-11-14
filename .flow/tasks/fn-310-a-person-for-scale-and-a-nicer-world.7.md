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
The person is placed by the host's D5/D6 rulings. `person::stand` puts it at the crown's distance (D2) when the whole figure fits the 16:9 hero frame there. Otherwise it searches outward in 1 cm steps, with the figure required within 0.98 of the half-frame, for the nearest whole fit, keeping D2 and reporting itself not whole when none exists on the camera's side. `Renderer::place_figure` draws a stated placement and redraws the sun's map when the figure moves. A plain submit still stands the figure beside its own tree, so the harness and createRenderer are unchanged. The film now settles each year before it draws the year before, so both renders of a crossfaded frame use lerp(place(a), place(b), share).

- Exemptions, asserted by name in `tests/person_frame.rs`: every shipped preset at age 1 (seven seedlings, 0.09-1.34 m tall). Every preset at catalogue age, Telperion at 80 and the owner's stills are whole at D2.
- Measured: across 480 synthetic subjects (0.25-20 m tall, 0.1-3x as wide), no subject is ever stood out beyond D2. When D2 cuts the figure, every distance further out cuts it too, because along the D2 azimuth the figure only gets nearer the camera. The rule therefore reduces to "D2, whole or exempt". Subjects up to 4 m tall and 3x as wide are exempt (test pins: below 1.75 m always exempt, above 4 m always whole).
- The canopy test (`--ignored`, 86 s) now places a leaf vertex by vertex once its reach disc comes near the figure, and checks flower stalks by their reach. Before this, the date palm at age 1 failed on the disc bound alone, although its bounds never reach the figure.
- The film's lookahead holds two settled years at once. Segment 0's device peak rose from 747 MB to 1003 MB in the 4-frame run; segment 7 stayed at 3490 MB.
- Film frames (sheet `film-before-after.jpg`, zoom `film-825-person.jpg`): 825 shows one person where it showed two. 105, 255 and 720 are byte-identical to before. At 105 and 255 the film's own camera, not the hero frame, still cuts the feet, and 720 is a seedling exemption. Per the host ruling these are judged by eye; the owner's call.
- film.rs is 457 lines (was 429); the year loop's pure parts moved to `examples/film/placing.rs`.

stage: impl-review - ran (codex, 3 draws: correctness, contracts, integration; all SHIP, no findings)
Tier: session at default effort (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 3e63d7e3240b35e85cded2aae0e235827461859b, 4f896951a36dcfdeac2a86f78e81db6da74448e3
- Tests: cargo test --profile ci -p telperion-render --lib person|scene|tint_rule_tests|compile_size, cargo test --profile ci -p telperion-render --test person_frame --include-ignored --test film_placing --test sun_cache --test shadow --test film_path --test conformance, cargo fmt -p telperion-render --check; cargo clippy --profile ci -p telperion-render --tests --examples
- PRs: