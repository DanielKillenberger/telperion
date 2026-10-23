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
- **R3:** each new row field is refused by name outside its range, as the existing fields are. A row JSON that omits a new field takes the default. Zero is a true no-op for each strength (glow, variation, haze), with no divide by zero and no NaN at zero. At zero the frame equals the one the field's absence would draw.
- **R4:** a frame-time difference outside noise is stated with its number, never hidden. Crossing the size ceiling is an owner decision and stops the build.

## Design (plan, 2026-10-23; D1 and D2 go to the host before work)

- **D1, the default row:** variant C's sky, ground, glow, variation and haze become `SceneRow::default()`. The film builds its row as `..SceneRow::default()` with only the sun position from its path, the harness starts from the default, and so does the package's `createRenderer`. So the default is the only way the chosen look reaches all three without a film-only path (fn-272 R2).
  - The default sun position stays 135° / 30°. C's file repeats the film's own 115° / 60°, which the film keeps setting from its path.
  - The default sun is retinted from (3.078, 2.848, 2.388) to about (3.064, 2.854, 2.374). This keeps fn-228's rule, that sun plus sky at the default elevation is white at the same luminance, under C's new sky. Without it the rule's test fails by 0.35 %.
- **D2, the person's placement:** the foot stands outside the crown's ground footprint, on the hero camera's side of the trunk and turned off its axis to the camera's left, as the capsule was. Facing the camera.
  - The azimuth is the default shot's azimuth less a stated offset. The distance from the trunk is the bounds' horizontal support in that direction plus a stated margin. So the rule reads the tree's bounds, not only its height, and a shrub and Telperion are both handled by the same rule.
  - Because the foot sits on the camera's side and within 90° of the sun's azimuth, the crown's shadow, which falls away from the sun, does not reach it. This holds for the harness default sun (135°) and the film's (115°), and is tested for both.
  - The person is placed in the world once per tree, not per frame. An orbit or a film path that swings the camera far round can hide it, as it could the capsule.
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
