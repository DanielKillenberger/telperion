## Conversation Evidence

> user (2026-09-15, on the round-16 bark close-ups): "i mean it's all still to plasticesque need to make it rough less reflective. All the materials have this problem."
> user (2026-09-15): "but that's probably a later spec"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

Every material Telperion draws reads as plastic up close: bark and leaves shade as smooth gradients with a soft white sheen on the lit side, where a photographed trunk or leaf is matte and grainy at the pixel. The owner saw it on the beech's and birch's bark close-ups and named it for all the materials. [paraphrase]

Two causes, found in the shaders on 2026-09-15. The highlight is not a real surface's reflectance: `wood.wgsl` adds a white sun lobe of `gloss * pow(n·h, 2^(1 + 10·gloss))` with gloss one minus the roughness row, and `foliage.wgsl` a cuticle lobe of `gloss * pow(n·h, 2^(3 + 5·gloss))`, each scaled by the full sun, so a moderately smooth row reflects a large fraction of the sun where bark and leaf reflect a few per cent at normal incidence. And there is no grain below the relief's scale: the colour and the normal are smooth between the relief's features, so a close-up has nothing at the pixel for the eye to read as a rough surface. [inferred]

This spec makes the highlight physical and adds a pixel-scale grain, as renderer terms every material shares, so every tree reads matte up close without a species branch. The values part of the fix (the beech's and birch's bark roughness and the beech's leaf gloss, which were far glossier than the oak's and spruce's) was folded into fn-40 and fn-54 on 2026-09-15 and is not this spec's. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **A physical highlight.** Replace the two unnormalised lobes with one specular term per material: a Fresnel reflectance at normal incidence (a row, default near 0.04 for bark and a leaf's cuticle), a normalised lobe whose width follows the roughness row, and the same energy taken from the diffuse. At the rows' current values the frame changes, so the pinned stills move once, named. [inferred]
- **Pixel-scale grain.** A filtered noise perturbation of albedo and normal below the relief's scale on bark, and a fine vein-and-cell grain on the blade, each footprint-faded like fn-26's and fn-32's terms so distant trees converge to their mean and the distance and resolution tests hold. [inferred]
- **Rows, not species.** Reflectance and grain strength are material rows every table can set; neutral values reproduce the physical highlight with no grain. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The bark and leaf highlights are one normalised, Fresnel-weighted term each, native and browser, with the pinned stills re-recorded once and named. Errors: a highlight brighter than the physical bound on a synthetic test fails. [inferred]
- **R2:** Grain rows on bark and blade, footprint-faded, with the distance, resolution and redraw tests at their unchanged bounds. Errors: a distance or resolution test outside its bound fails. [paraphrase]
- **R3:** The frame cost is recorded beside fn-52's and fn-40's. Errors: none beyond the record; the owner sets any bound. [paraphrase]
- **R4:** The beech's and birch's close-ups (B-BASE, S-BARK) and leaf-on pairs are rendered again, the implementer answers after looking whether the bark and the leaves read matte like their photographs, and the owner judges. [user]

## Boundaries
<!-- scope: business -->

- No new bark or leaf pattern; fn-40's and fn-32's layers are kept as they are. [inferred]
- No lighting model beyond the highlight; fn-52's canopy rows are kept. [paraphrase]
- The relief's drift at distance and the bark grain's value are fn-71's. [user]

## Decisions

- **Owner, 2026-09-18.** The 4x distance-series bound in `bark_distance.rs` is recalibrated from 3.0 to 3.25. The old sheen put 15% of the sun on the lit trunk, sitting the frame on the tone curve's shoulder where the relief's cross-resolution error compressed into fewer code values; base read 2.904, fn-55's physical highlight 3.134, and no highlight at all 3.167. The relief's drift at distance is a real defect seen before fn-55 and is fn-71's, which restores 3.0; the 2x and p95 bounds stand. The bark grain rows stay held off in the tables until fn-71 sets their value.

- **Owner, 2026-09-18, R4.** Judged on the six matched stills at master and on this branch, served side by side with a slider: "i see barely a difference". [user] The measured difference is under 1.2% RMSE in every view and the close-ups' mean brightness moves 0.3/255. The value part of the fix had landed in fn-40 and fn-54, the bark grain ships held off, and the birch's peel rims are the relief's lighting, so the branch is accepted as the shading foundation and the visible step, grain on bark with its bound, is fn-71's, started straight after the merge. [paraphrase]

## Resolved via Codebase

- Bark highlight: `crates/telperion-render/src/shaders/wood.wgsl` about `:153-165` (`gloss = 1 - clamp(bark.w + ...)`, `sheen = gloss * pow(max(dot(n, half_way), 0.0), exp2(1.0 + 10.0 * gloss))`).
- Leaf highlight: `crates/telperion-render/src/shaders/foliage.wgsl` about `:104-110` (`cuticle = gloss * pow(..., exp2(3.0 + 5.0 * gloss))`).
- Roughness and gloss rows: `crates/telperion-core/src/presets/materials.rs` (`bark_roughness` oak 0.85, spruce 0.9, beech 0.4, birch 0.48 before fn-40's value pass; `cuticle_gloss` oak 0.35, spruce 0.05, beech 0.48, birch 0.28).
