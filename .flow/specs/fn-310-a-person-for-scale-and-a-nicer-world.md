# A person for scale and a nicer world around the tree

## Goal & Context

Owner (2026-10-23), on the release film's cut 6: "video looks really good but [a friend]'s right the cylinder thing looks like a stupid unity demo. could we make the environment a bit nicer and remove that thing and add something else for scale?" The owner chose a person as the new scale reference, asked for "a concept screenshot first before rendering the whole video", and ruled that this belongs in a spec as part of the release.

The scene is the shipped renderer's: `crates/telperion-render/src/scene.rs`, `scene/room.rs` (the 1.8 m capsule figure, `FIGURE_HEIGHT`), `scene/row.rs` (`SceneRow`) and `shaders/sky.wgsl`. The film draws through the shipped renderer with no film-only drawing (fn-272 R2), so the change reaches the harness and the package's `createRenderer` too. It joins the release (fn-237), and the film (fn-272) re-renders after it.

A concept prototype is being made on branch `scene-concept` (host, 2026-10-23). It holds stills of the current look and two or three environment variants, on the oak at 40 and Telperion at 80, seed 7, at 1280×720.

## Owner verdicts

- Owner (2026-10-23), on the concept sheet (branch `scene-concept`, `concept/sheet.jpg`: the current look and variants A, B and C on the oak at 40 and Telperion at 80): "ok let's go c can we make the human a bit better?". Variant C, the clear neutral day, is chosen.
- **Host, on what makes the person better.** These are taken from the concept's own read and the host's view of the sheet:
  - The concept figure reads as a mannequin. Its legs are thin, and a dark band where the torso meets the legs looks like briefs.
  - Under warm light its grey turns skin-toned.
  - At Telperion's framing it is about 6 px tall and stands inside the crown's shadow, so it gives no scale cue there.
  - So, in R2: natural proportions (legs of believable thickness and length, shoulders, a slight taper, arms that hang at the sides), no seam or band at the hips, and a darker, cool, clothing-like matte colour that reads as a person in any of the scene's light.
  - The figure stands where every framing shows it: in front of the trunk and outside the crown's shadow, towards the camera's side. It still never stands inside the canopy.
  - The owner sees a still of the improved person before the film is rendered.
- Owner (2026-10-23), on the person-v2 sheet (`.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/person-v2.jpg`, oak at 40 and Telperion at 80 under the film's sun): "ok good. it should also cast a shadow then i think it's fine." The person is accepted, with one change: it casts a shadow onto the ground through the existing shadow pass, the caster path the tree uses, with no new pass (host, 2026-10-23; task .5).

- Owner (2026-10-23), on the film before/after sheet (`.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/film-before-after.jpg`, film frames 105, 255, 720 and 825): the person should "stand beside, not in front". Frame 825's ghost fix stands (host).

## Requirements

- **R1 (concept, owner's choice first):** the owner picks one environment variant, and passes on the person, from the concept sheet, before any task is planned for real. The pick and the owner's words are recorded here verbatim.
- **R2 (the person):** a standing person about 1.75 m tall replaces the capsule.
  - Built procedurally in code, with no copied mesh or asset.
  - Calm and low-detail, like an architectural scale figure: one matte colour from the scene palette, no face.
  - A small vertex budget, stated and tested.
  - The placement and `FIGURE_HEIGHT`'s role for the camera are unchanged.
- **R3 (the environment):** the chosen variant.
  - **Sky:** a gradient with a glow around the sun.
  - **Ground:** tonal variation from deterministic shader noise, with no textures.
  - **Horizon:** haze where the ground meets the sky, plus distance haze on far ground only. The tree is not hazed at the film's distances.
  - **Parameters:** every colour and strength is a `SceneRow` parameter, never a literal. The clay room keeps its neutral light.
- **R4 (cost):**
  - **Frame time:** measured in the harness against the base on the owner's RTX 3080, within noise or stated.
  - **Wasm size:** the render module's byte change is stated and stays inside `scripts/artifact-budgets.json`.
  - **Determinism:** the noise is deterministic, so the same frame renders the same bytes.
- **R5 (proof):**
  - The existing scene, shadow and renderer tests are green, updated only where the figure or the sky changed by design.
  - New tests cover the figure's height, its vertex budget and its placement out of the canopy.
  - The gate (`scripts/gate.py` as declared) and `npm test` are green, with a codex review per task, each dispatched inside `telperion-heavy` per AGENTS.md.
  - Final stills at the concept's poses go to the owner, then the film's cut 7 is rendered.

### Errors and boundaries per requirement (plan, 2026-10-23)

- **R2:** no error surface of its own. Its boundaries are a shrub a few metres tall, where the person must still stand clear of the trunk, and Telperion's wide crown, where it must still stand outside the canopy. The figure flag, clay and the photograph frames keep their present behaviour: the person draws wherever the capsule drew, and nowhere else.
- **R3:** each new row field is refused by name outside its range, as the existing fields are. A row JSON that omits a new field takes the default. An explicit zero turns its effect off (glow, variation, haze), with no divide by zero and no NaN: at zero the sky is the plain zenith-to-horizon gradient (dither still on, one 8-bit step at most), the floor's albedo is the flat ground colour, and the floor is not hazed. An omitted field takes the default, which is variant C's active value.
- **R4:** a frame-time difference outside noise is stated with its number, never hidden. Crossing the size ceiling is an owner decision and stops the build.

## Design (plan, 2026-10-23; D1 and D2 approved by the host, 2026-10-23)

- **D1, the default row:** variant C's sky, ground, glow, variation and haze become `SceneRow::default()`. The film builds its row as `..SceneRow::default()` with only the sun position from its path, the harness starts from the default, and so does the package's `createRenderer`. So the default is the only way the chosen look reaches all three without a film-only path (fn-272 R2).
  - The default sun position stays 135° / 30°. C's file repeats the film's own 115° / 60°, which the film keeps setting from its path.
  - The default sun is retinted from (3.078, 2.848, 2.388) to about (3.064, 2.854, 2.374). This keeps fn-228's rule, that sun plus sky at the default elevation is white at the same luminance, under C's new sky. Without it the rule's test fails by 0.35 %.
  - Host (2026-10-23): approved. This changes the harness's and the package's default scene, on the owner's pick of C. The retint is approved and the white-daylight test stays.
- **D2, the person's placement:** the foot stands outside the crown's ground footprint, on the hero camera's side of the trunk and turned off its axis to the camera's left, as the capsule was. Facing the camera.
  - The azimuth is the default shot's azimuth less 68°. The distance from the trunk is the reach, in that direction, of the crown ellipse inscribed in the tree's bounds (the same crown the hero camera frames, `camera.rs` `reach_of`), plus a 0.5 m margin. So the rule reads the tree's bounds, not only its height, and a shrub and Telperion are both handled by the same rule.
  - Host (2026-10-23), ruling on the task .2 review: the ellipse, not the bounding box. The earlier wording, "the bounds' horizontal support", was an approximation; its purpose is "outside the crown and its shadow, and in frame". The box's empty corners serve none of that, and they are what break the in-frame rule (a box footprint fits only at offset 71° with the oak's feet 2 px from the frame edge). "Never inside the canopy" is proven against the real geometry instead: no wood or foliage of any shipped tree at its catalogue age, nor of Telperion at 80, lies above the person's footprint.
  - Because the foot sits on the camera's side and within 90° of the sun's azimuth, the crown's shadow, which falls away from the sun, does not reach it. This holds for the harness default sun (135°) and the film's (115°), and is tested for both.
  - The person is placed in the world once per tree, not per frame. An orbit or a film path that swings the camera far round can hide it, as it could the capsule.
  - Host (2026-10-23): approved, the person does not follow the sun. Tests: outside the crown's shadow under the harness sun and under the film's sun; and inside the hero camera's frame for every shipped preset at its catalogue age, and for Telperion at 80, because being visible in shot is the point of moving it.
- **D3, the person's form:** a rewrite of the concept's lofted rings to the host's verdict.
  - Legs of believable thickness and length that meet the torso without a seam or a band at the hips (one lofted lower body or overlapping volumes, no step in width).
  - Shoulders, a slight taper to the waist, arms hanging at the sides, a head without a face.
  - One matte colour: a dark cool slate from the room palette, replacing the warm `FIGURE` grey, so it reads as clothing under the warm sun and the cool shade.
  - The budget is 800 vertices and 1,200 triangles at most. `FIGURE_HEIGHT` becomes the person's 1.75 m and keeps its role as the camera's floor.
- **D4, the environment:** the concept's approach, rewritten.
  - The sky's radiance along a direction is the existing gradient plus a glow lobe round the sun, in the sun's colour, scaled by the row's glow.
  - The ground is value noise from the existing integer hash, a few octaves, mixing the ground colour toward a dry colour by the row's variation.
  - Haze is applied only to floor fragments: over the row's distance, beyond the subject, plus a fade of the disc's last stretch into the horizon. The tree and the person are never hazed, whatever the distance.
  - New uniforms are whole `vec4` slots appended to `Uniforms`, mirrored in WGSL. The clay room's light and path are untouched.
  - The sky gets a dither of about one 8-bit step from the pixel hash, so a wide gradient does not band. Same frame, same bytes on one device.

## Cut 7 findings (host, 2026-10-23)

Cut 7 (rendered from the release branch at c047afedb; fn-272's FILM.md, cut 7 section, at dd2c3966a) showed three problems with the person.

- **D5, a figure per shot, not per year.**
  - **The problem:** placement follows each year's crown, so in a growth the person steps outward every year, and the film's crossfade of consecutive years shows two half-transparent people (frame 825).
  - **The rule:** the scale figure is placed for a shot. Within one film segment the person stands still.
  - **How:** the renderer's placement takes the bounds it is given. The film passes the bounds the segment reaches at its largest, the envelope its camera already pulls back to (`examples/film/camera.rs`). That is an input choice, not film-only drawing. The harness and `createRenderer` keep placing per submitted tree.
  - **The test:** the film's placement is constant within each segment.
- **D6, wholly in the hero frame.**
  - **The problem:** at the Laurelin seedling (frames 719/720) the person fills half the frame and is cut at the waist. At 105 the feet are cut, and at 255 everything below the chest.
  - **The rule:** along the D2 azimuth, the distance is the larger of the crown ellipse's reach plus the 0.5 m margin, and the nearest distance at which the whole 1.75 m figure fits inside the hero frame with a small margin. A seedling's person stands farther back, wholly visible.
  - **The tests:** the in-frame tests become "wholly in frame, head to feet". They cover every shipped preset at its catalogue age, Telperion at 80, every preset at age 1, and the film's segment envelopes. The canopy and shadow tests still hold.
- **The film's draw time, to be explained before anything changes.**
  - **The numbers:** draw time rose from 87 s (cut 6) to 141 s (cut 7) over 1,050 frames, about 51 ms more a frame. The harness measured +0.05 ms.
  - **Candidates:** the shadow map redrawn, the person in both crossfade renders, the noise at 1280x720, or the film's own path.
  - **If it is a real per-frame cost in the shipped renderer,** the number goes to the host before any fix.
- **Proof:** before and after stills of frames 105, 255, 720 and 825, built through the film path.

### Host rulings on task .7 (2026-10-23), replacing D5 and D6 as first written

Task .7 measured that D6 has no camera-side fit at age 1 (seedlings 0.09-1.34 m against the 1.75 m figure), and that D5's segment envelope left film frames 105 and 255 cut and 720 empty (`FRICTION.md`).

- **D6, as ruled: small subjects, option (a).**
  - **The rule:** "wholly in the hero frame" applies wherever a whole fit exists on the camera's side.
  - **The exemption:** where none exists, for a subject much shorter than the person, the person keeps the D2 placement. A person beside a seedling then shows as legs and shoes cut by the top of the frame, an honest scale cue the owner judges in the stills.
  - **The test:** it records each exemption by name.
  - **What does not change:** no far-side placement and no camera change. D2 stands.
- **The film is judged by eye, not tested against the hero frame.** The film's frames are the owner's call. The film's camera poses are not passed into placement.
- **D5, as ruled: interpolate, not the envelope.**
  - **The rule:** within a growth segment the film crossfades year a into year b. It passes the renderer one placement for both renders of a frame: lerp(place(a), place(b), the frame's fade share). The person walks out continuously as the crown grows, with no ghost and no extra build, since both years are built for the crossfade anyway.
  - **Walk segments:** the same lerp between the frame's neighbouring builds where the film crossfades there; otherwise the frame's own tree.
  - **The renderer's part:** it draws the placement it is given. The harness and `createRenderer` are unchanged and still place per tree.
  - **The tests:** in a crossfaded frame both renders use the same placement, and placement is continuous across a segment's frames.

### Host ruling after the owner's "stand beside, not in front" (2026-10-23), replacing D2's direction

- **D7, the person is never nearer the camera than the subject.**
  - **Depth:** the person's foot is at the trunk's depth or behind it, measured along the shot's view direction.
  - **Side and angle:** the person stands to one side, at the crown ellipse's reach plus the 0.5 m margin. The side, and an angle of at least 90° from the camera's azimuth, are chosen so the foot is outside the crown's actual shadow under the shot's sun: the sunlit side.
  - **The shot's view direction:** in the harness and `createRenderer` it is the hero camera's azimuth, as now. In the film it is the frame camera's azimuth, passed in with the placement the film already passes, keeping the year-lerp (D5 as ruled).
  - **Motion check:** because the film's orbit is slow and continuous, the person moves continuously with it. A short `film --frames` run over consecutive frames checks that this reads as natural, not as sliding. If it slides visibly, the host is told before anything else is chosen.
  - **D6 unchanged:** whole-in-frame or exempt stays as ruled. Most shots should now be whole, because a farther person is smaller.
  - **Tests:**
    - depth at or behind the trunk, for every shipped preset at its catalogue age and at age 1, and Telperion at 80;
    - out of the actual crown shadow under the harness sun and the film's sun;
    - not above any canopy, as now;
    - in frame or exempt, as now.
  - **Proof:** a before/after sheet of film frames 105, 255, 720, 825 and 975 via `film --frames`.

### Host rulings on task .8 (2026-10-23): rule for the shot, not the segment

The film is one continuous shot, so a rule held per segment only moves a jump to a segment join. Task .8 measured a D7 side change at frame 713 (the camera crosses the sun's bearing) and walk frames hopping up to 1.2 m a frame.

- **One side for the whole shot.**
  - **Film:** one side for the entire film, preferably the camera's left, which keeps the person away from the map overlay at bottom right, provided the person is out of the crown's actual shadow on every frame under the film's sun. If the left fails on any frame, the right; if both fail somewhere, the frames are reported.
  - **Harness and `createRenderer`:** they keep D7's per-tree sun-side choice, including the `set_scene` update when the sun moves. Each is a single still shot.
  - **Tests:** the film's side is constant; the shadow test runs over every film frame on the chosen side. This test replaces the ignored side-change test.
- **Distance: a smoothed reach that never falls inside the crown.**
  - **Definitions:** r(f) is the frame's crown reach plus the margin, with the year-lerped value in growth segments. M(f) is the maximum of r over frames [f-2W, f+2W] across the whole film, not per segment. The reach used is A(f), the mean of M over [f-W, f+W], with W = 15 frames (half a second).
  - **Why it works:** A(f) >= r(f) by construction, so the person is never under the current crown. A is continuous: each step is at most the largest jump in M divided by 2W+1, so the person eases out and in rather than hopping.
  - **The direction:** it still follows the frame camera's azimuth.
  - **Tests:** A >= r on every frame; the largest frame-to-frame move of the foot is under 10 cm, or the measured bound is stated; the strips 240-269 and 690-719 are re-rendered and viewed.
- **The map overlay:** the left-side preference handles it. Any frame where the person still overlaps the overlay is reported.
- **Not this spec's:** the spruce walk's crown width jumps from 8.81 to 11.19 m between frames 241 and 242. That is a step in the tree, which fn-308's by-degree rule should have caught. It is recorded in RESULT.md's Open section for the host and is not fixed here.

### Host ruling on the film's side (2026-10-23): option (a)

Task .8's estimate, from cut 7's per-frame height and width with the crown as a centred ellipse, under the film's sun at 115°/60°: the camera's left at 90° is in the crown's shadow on frames 26-487, and the right on frames 908-1049. No frame fails on both sides.

- **Placement:** the person stays on the camera's left for the whole film. On each frame the angle is the smallest angle of at least 90° to the camera's left that puts the person out of the crown's real shadow. That angle is smoothed over frames exactly as the reach is, so the person eases instead of stepping.
- **Smoothing window:** W = 30 frames for both the angle and the reach (replacing W = 15). The measured largest move per frame becomes the stated bound, written into the test and into this spec.
- **Bounds pass:** one exact pass from the whole film's real bounds. Every frame is checked against it: out of the shadow, out from under the canopy, and at or behind the trunk's depth.
- **Delegated:** the conductor makes the remaining small choices that best keep the person beside the tree, lit and steady, and records them here.
- **Stop only if:** the exact pass finds a frame no left angle can serve, or the strips look wrong to the eye (the person hidden behind the trunk, overlapping the map, or visibly sliding).

### Host ruling on the film's distance (2026-10-23), replacing option (a)'s angle search

Task .8's exact bounds pass (whole film, `raw/film-bounds.tsv`) found no angle on the camera's left, from 90° to 180°, that is out of the crown's real shadow on 309 frames: nearly all of 187-381, scattered frames in 389-589, and 609, 613, 648, 671, 678, 680, 721 and 724. Where an angle did serve in segments 0-3 it put the person nearly behind the trunk. At 90°, moving further out always clears the shadow, because the shadow is convex: at most 3.7 m more (frame 255). That figure is an estimate.

- **Film placement, as ruled:** the person stays at 90° on the camera's left, level with the trunk (at the trunk's depth). Its distance is the larger of the crown's reach plus the 0.5 m margin and the nearest distance, further out, at which the whole figure is out of the crown's real shadow under the film's sun. That distance is smoothed with W = 30 in the max-then-mean form: M(f) is the maximum over [f-2W, f+2W] across the film, and the distance is the mean of M over [f-W, f+W]. So the person never falls under the crown or into the shadow. The left-angle search is removed.
- **Tests, over every film frame from the cached bounds (with the stale-cache guard):** lit; not under the canopy; at the trunk's depth; on the left side; the largest frame-to-frame foot move within the stated bound, which is written here once measured.
- **Harness and `createRenderer`:** unchanged (D7's per-tree sun side, with the `set_scene` update).

## Boundaries

- No change to growth, the trees or the camera path.
- No new external asset and no new runtime dependency.
- The film's cut 7 is fn-272's to render once this spec's stills are approved; this spec renders stills only.
- The prototype on `scene-concept` is reference, not merged: every part is rewritten here with its tests.

## Strategy Alignment

Active tracks served by this plan:
- **Surface and rendering at scale**: the shipped renderer's scene, the one the harness, the package and the release film draw through, gets a person for scale and a finished environment at a stated cost.

## Decision Context

- A named second row for variant C (a scene preset) was rejected: the row has no named rows today, the film reads the default, and a second row would need a film-side choice that fn-272 R2 forbids.
- A sun-aware placement, which moves the person to whichever side the sun lights, was rejected for D2: a person that jumps sides when the panel's sun azimuth moves is worse than one that stays put. The stated condition, sun within 90° of the placement azimuth, holds for every default.
- Making the new fields dormant at zero by default, as the prototype did, would leave the film and the harness on the old look, and the owner chose C for both.

## Memory findings

| Track | Category | Entry | Why relevant |
|---|---|---|---|
| bug | ui | An AC that enumerates the default state is the test | D1 changes the enumerated default; the tests pin it |
| bug | data | A new catalogue row moves the wire tests and the generated tables together | New row fields: check every consumer of the row's key list (Wasm `scene()`, the panel, README) together |

## Quick commands

```bash
# focused render tests for the scene, row, person and shot
~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --lib scene
~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --test shot --test look --test neutral_light --test shadow
# a still at the concept's pose
~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2 -- cargo run --profile ci -p telperion-render --example headless -- --preset telperion --seed 7 --size 1280x720 --family <age-80.json> --out /tmp/t.png
```

## Early proof point

Task fn-310-a-person-for-scale-and-a-nicer-world.1 validates the core approach: variant C drawn as the default row through the shipped renderer, with the existing tests green. If it fails, re-evaluate promoting C to the default (D1) before continuing with .2 and later tasks.


## Requirement coverage

| Req | Task(s) | Note |
| --- | --- | --- |
| R1 | none | Met before planning: the owner's pick and words are in Owner verdicts |
| R2 | .2 | |
| R3 | .1 | |
| R4 | .1, .3 | .1 proves determinism; .3 measures frame time and Wasm bytes |
| R5 | .2, .3, work's gate | the gate, `npm test` and the per-task reviews run in the work stage |
