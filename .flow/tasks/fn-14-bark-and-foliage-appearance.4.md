---
satisfies: [R1, R3, R7, R10, R14]
---
# fn-14-bark-and-foliage-appearance.4 The look: bark and leaf colour, per-leaf variation, crown depth, sky, tone map and the clay view

## Description
Make the shaders read the rows: bark colour and roughness under the sun and hemisphere, leaf front and back colour with each leaf's seeded offset inside the row's ranges, interior darkening by depth into the crown, the sky gradient and ground from the scene row, a filmic tone map at the end of the lit frame, and the clay view unchanged (R7, R14, the look halves of R1, R3 and R10).

**Size:** M
**Files:** `crates/telperion-render/src/shaders/wood.wgsl`, `crates/telperion-render/src/shaders/foliage.wgsl`, `crates/telperion-render/src/shaders/scene.wgsl`, `crates/telperion-render/src/shaders/tone.wgsl` (new, a fullscreen pass) or the tone map inline in the three fragment shaders, `crates/telperion-render/src/lib.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/src/view.rs`, `crates/telperion-render/src/submit.rs` (foliage bounds into uniforms)
**Touches:** [crates/telperion-render/src/shaders/**, crates/telperion-render/src/lib.rs, crates/telperion-render/src/scene.rs, crates/telperion-render/src/scene/**, crates/telperion-render/src/view.rs, crates/telperion-render/src/submit.rs, crates/telperion-render/tests/**]

### Approach
- Material uniforms: the material row and the foliage bounds (from `Instances::bounds()`, `crates/telperion-core/src/foliage.rs:71-79`) go into a per-tree uniform block at submit.
- Wood: colour and roughness under a Lambert sun term plus the hemisphere, shadowed by task 3's map; the coordinate attribute is bound but only fn-26 draws with it.
- Leaf: the identity is `list[instance]`, the placement index the shader already reads (`foliage.wgsl:28-32`); hash it to two unit values and map them into the row's hue and brightness ranges in a hue-shift on the front and back colours. Front and back lit separately by the face the eye sees. Interior darkening: the placement's translation against the ellipsoid inscribed in the foliage bounds gives a depth in 0 to 1; the hemisphere term is scaled by one minus the row's amount times that depth.
- Scene: the background becomes a sky gradient from zenith to horizon, the ground disc takes the ground colour, both from the scene row; the figure stays.
- Tone map: linear lighting throughout, one filmic curve to sRGB at the end of the lit frame; the clay view bypasses the sun, the material, the depth term and the tone map and must match today's still, pinned by a test that renders the clay view and compares to the fn-24 evidence still within a tolerance.
- Views: whole and bare lit; leaf view lit with no shadow or depth term; clay as today.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/shaders/foliage.wgsl` and `wood.wgsl` — the fragment stages to replace
- `crates/telperion-render/src/scene.rs:1-60,180-220` — uniforms and layout
- `crates/telperion-core/src/foliage.rs:60-80` — the bounds

**Optional** (reference as needed):
- `crates/telperion-render/src/view.rs` — the view enum to extend with clay
- `.flow/evidence/fn24/oregon-white-oak-hero.png` — the still the clay view must reproduce

### Key context
- Every per-leaf term must be identical at every level of the ladder; nothing may depend on the draw-local instance index.
- No species name or preset name in any shader or renderer path.
- Budget: at most four images viewed; calibrate colours by eye on one oak still and one spruce still only.

## Acceptance
- [ ] Wood and leaves read the material row: bark colour and roughness; leaf front and back colours with a per-leaf seeded offset inside the row's hue and brightness ranges, stable across levels and frames
- [ ] Leaves are darkened by depth into the crown by the row's amount; sky, sun and ground come from the scene row; a filmic tone map closes the lit frame
- [ ] The clay view reproduces the fn-24 still within tolerance, pinned by a test; leaf view lit without shadow
- [ ] No shader or renderer path names a species; `cargo test --release --workspace`, fmt and clippy pass

## Done summary
# fn-14-bark-and-foliage-appearance.4 - the look

The shaders now read the rows. Bark takes its colour and roughness from the
material row under the sun, the hemisphere and task 3's shadow; each leaf takes
the row's front or back colour turned by its own seeded offset inside the row's
hue and brightness ranges, and darkened by how deep into the crown it stands;
the sky, the sun and the ground come from the scene row; and one filmic tone map
closes every lit frame. The clay room becomes a selectable view that draws
exactly what it always drew.

One commit, `5fdbf0a`, on branch `fn-14.4` over `0ccaa7c`.

### What each acceptance item rests on

- **The material row on the surface.** `Renderer::set_material` takes the
  family's row beside the mesh, the way the view and the scene row are taken;
  the headless still, the walk's every frame and the page's `setTree` pass it.
  Bark is `colour * (ambient + sun)` plus one sheen lobe whose width and weight
  are the row's roughness, so a chalky bark spreads the sun over the whole face
  and a smooth one keeps a line along it.
- **Per-leaf offsets, stable across levels.** The identity is `list[instance]`,
  the placement index every level's list carries, hashed to two unit values and
  mapped into the row's ranges; the hue offset is a rotation about the grey axis
  (Rodrigues, no trip through HSV). Nothing reads the draw-local instance index,
  so a leaf keeps its colour when its level changes. Pinned by
  `every_leaf_takes_its_own_offset_inside_the_row_s_ranges`: opening the row's
  ranges moves the crown, closing them to zero width draws the same picture
  twice.
- **Interior darkening (R14).** The crown's ellipsoid is the box the placements
  fill, taken at submit (`submit::crown_of`); a leaf's depth is one minus its
  placement's normalised distance from the centre, and the ambient term is
  scaled by `1 - amount * depth`. Per placement, so it reads the same at every
  level. The oak and spruce stills both read as a shaded mass rather than
  speckle.
- **Sky, sun, ground and the tone map.** The sky is one triangle over the frame
  drawn before everything and writing no depth, its gradient taken along the
  pixel's own world direction from the camera's axes in the uniform block, so it
  holds still when the camera tilts. The ground disc takes the scene row's
  colour through a fourth channel on the room's vertex colour that says which
  vertices are floor; the figure keeps its neutral value. `tone()` is
  Narkowicz's ACES fit with its standard exposure applied first - without that
  stop a bark of reflectance 0.15 in full sun comes back the colour of sand.
- **The clay view.** `View::Clay` joins whole, bare and leaf; every lit shader
  branches once on `u.clay.w` and returns the old flat clay under the old
  neutral hemisphere, and the room clears to the old background and keeps its
  own ground and figure values. `the_clay_view_draws_the_still_the_room_always_drew`
  renders it at the fn-24 pose and size and compares with the committed
  `.flow/evidence/fn24/ordinary-hero.png`: **every one of 6,400,000 channels is
  identical today**, and the test's tolerance (mean channel error under 1, under
  0.5% of channels more than 24 out) is there so task 5's multisampling can move
  edge pixels without breaking the pin. Run red first with the view swapped for
  `whole`: it fails on the mean.
- **No species anywhere.** The conformance guard that forbids a family or an
  anatomy word in the selection path caught "blade" in a comment of the new
  foliage shader; reworded, and the whole suite is green.
- **Gates.** `cargo test --release --workspace` 167 passed, 0 failed, 7 ignored;
  `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets` no
  warnings; `npm run wasm:build`, `npm test` (65) and `npm run typecheck` all
  clean, because the page's `setTree` now sets the material too. Baseline: green
  via handoff at `426b31d`.

### Numbers

- Oak, seed 7, 1600x1000, RTX 3080, valid session, after the look:
  vegetation p50 **1.993 ms**, selection **0.087 ms**, shadow **1.799 ms**,
  total p50 **3.877 ms**. Task 3 measured 3.86 ms before any of this, so the
  material, the depth term, the sky and the tone map together cost about
  **0.04 ms**. R12's 3.8 ms bound is still 0.08 ms astern, for the reason task 3
  recorded (the shadow pass draws the full-resolution wood), and multisampling
  is still to come in task 5.
- Stills for the conductor, all in `/tmp/flow-handover-fn14/`: `oak-hero.png`
  (1600x1000), `oak-lit.png` and `spruce-lit.png` (1200x750), `ordinary-lit.png`
  and `ordinary-clay.png`. Four images viewed in this task, inside the budget.

### Deviations from the task, and why

- **The crown ellipsoid is taken from the placements, not from
  `Instances::bounds()`.** That call transforms every vertex of every leaf -
  7 M placements times a leaf's vertices on the spruce - and the term it feeds
  reads a placement's position, so the extra pass buys a leaf's own half
  centimetre of reach at a cost the browser would pay on every dial.
- **The tone map is inline in the shaders, not a fullscreen pass.** The task
  offered either. Inline needs no intermediate HDR target, and it leaves task
  5's resolve operating on display values, which is where a resolve belongs.
- **Files the task's Touches list does not name.** `pass.rs` (the shared
  prelude and one depth mode for the sky), `wood.rs` and `foliage.rs` (the
  pipelines they own take the prelude-built module and the depth mode),
  `timing.rs` (one line: a session records levels when the view selects, which
  the clay view now also does), `web.rs` and `examples/headless.rs` (one line
  each: the material rides with the tree). Each is additive and small.
- **`scene.rs` was split before it grew**: the room's geometry and neutral
  values moved to `src/scene/room.rs`, and `crown_of` went to `submit.rs`, so
  scene.rs (355) and lib.rs (347) both stay under the line rule.

### For task 5 and task 6

- Every pipeline now goes through `pass::pipeline(.., Depth, ..)`; the sky
  pipeline is the one `Depth::Behind` and it must keep writing no depth when
  the sample count arrives.
- The uniform block grew by twelve vec4s, append-only, and the shaders share
  one prelude at `src/shaders/common.wgsl` - a field added there reaches all
  four lit shaders at once.
- The clay pin is a real gate on the room: any change that touches the neutral
  view will be caught by it, and its tolerance is sized for multisampling and
  nothing else.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after it integrates)

stage: wave-join - ran (fast-forward 0ccaa7c..5fdbf0a, no collision; six small additive edits outside declared Touches - no sibling in flight)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5fdbf0a1bb13a7dfc28e5344e3cc6d84093d7ef2
- Tests: cargo test --release --workspace (167 passed, 0 failed, 7 ignored), cargo fmt --all --check (clean), cargo clippy --workspace --all-targets (no warnings), npm run wasm:build (ok), npm test (65 passed), npm run typecheck (clean)
- PRs: