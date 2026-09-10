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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
