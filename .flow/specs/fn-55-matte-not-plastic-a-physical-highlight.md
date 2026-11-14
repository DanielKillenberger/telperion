## Goal

Every material read as plastic up close: bark and leaves shaded as smooth gradients with a soft white sheen, where photographs are matte and grainy at the pixel. Two causes: unnormalised highlight lobes scaled by the full sun (a moderately smooth row reflected a large fraction of the sun where bark and leaves reflect a few per cent), and no grain below the relief's scale. This spec makes the highlight physical and adds pixel-scale grain as renderer terms every material shares. The values part (beech and birch bark roughness, beech leaf gloss) was folded into fn-40 and fn-54 on 2026-09-15.

## Decisions

- user (2026-09-15, on the round-16 bark close-ups): "i mean it's all still to plasticesque need to make it rough less reflective. All the materials have this problem."
- user (2026-09-15): "but that's probably a later spec"
- Owner, 2026-09-18: the 4x distance-series bound in `bark_distance.rs` is recalibrated from 3.0 to 3.25. The old sheen put 15% of the sun on the lit trunk, sitting the frame on the tone curve's shoulder; base read 2.904, the physical highlight 3.134, no highlight at all 3.167. The relief's drift at distance is a real defect seen before fn-55 and is fn-71's, which restores 3.0; the 2x and p95 bounds stand. The bark grain rows stay held off in the tables until fn-71 sets their value.
- Owner, 2026-09-18, R4: judged on the six matched stills at master and on this branch side by side: "i see barely a difference". The difference is under 1.2% RMSE in every view; the branch is accepted as the shading foundation and the visible step (grain on bark with its bound) is fn-71's.
- Boundaries: no new bark or leaf pattern, no lighting beyond the highlight, relief drift and grain value are fn-71's.

## Acceptance Criteria

- **R1:** one normalised, Fresnel-weighted highlight per material, native and browser, pinned stills re-recorded once — met (`highlight()` in `canopy.wgsl`: Schlick Fresnel on `barkReflectance`/`leafReflectance` default 0.04, hemisphere-normalised Blinn-Phong, energy taken from the diffuse; GPU hemisphere integral returns 1% to 8% of the sun; the only pinned still, the clay room, unchanged).
- **R2:** grain rows on bark and blade, footprint-faded, distance/resolution/redraw tests at unchanged bounds — met with the owner's 4x recalibration (`barkGrainScale/Strength`, `bladeGrainScale/Strength`; bark tables hold grain off because at 2 mm the birch close-up read 4.00 to 4.46 against 3.0).
- **R3:** frame cost recorded — met (oak 5.22 to 5.21 ms, birch 3.38 to 3.38, beech 10.59 to 10.53 total p50; inside spread).
- **R4:** close-ups and leaf-on pairs re-rendered, implementer read, owner judges — met ("i see barely a difference"; accepted as foundation).

## Result

Implemented in-host (the routed gpt-6-astra bridge refused on quota). Implementer read: B-BASE matte but smooth like clay (grain held off); S-BARK far less glossy than round 16, peel rims still bright; both leaf-on wholes matte with no white sparkle. The six stills the verdict rests on: `.flow/evidence/fn55/stills/european-beech/european-beech-1-B-BARE.jpg`, `-B-BASE.jpg`, `-B-WHOLE.jpg`, `.flow/evidence/fn55/stills/silver-birch/silver-birch-1-S-BARE.jpg`, `-S-BARK.jpg`, `-S-WHOLE.jpg`.

## Open

Relief drift at distance and the bark grain's value: fn-71.

## Friction

- Codex bridge refused on quota (~2 min, whole task on session model) -> probe quota before composing the brief.
- Resolution bounds sat within 3% of their limits at base, invisible on green runs (~12 min) -> record measured margins to a receipt on green.
- Species runner quick look needs fn34 profiles by hand (~3 min) -> resolve a preset id to its profile set.

## Evidence

- .flow/evidence/fn55/stills/european-beech/*.jpg, .flow/evidence/fn55/stills/silver-birch/*.jpg (the six verdict stills)
