# Bark relief, leaf veins and transmission

## Goal

Split out of fn-14 at the interview write-back on 2026-09-11, after the owner said the tree looked bland. fn-14 delivers the lit tree (rows, coordinates, sun and shadow map); this spec adds the close-view surface detail drawn along fn-14's coordinates: bark ridges and plates, leaf veins and margin tone, and light through the blade, so the oak and spruce hold up beside their photographs at trunk, branch and leaf scale.

## Decisions

- > user (interview, 2026-09-10): "atm it's looking pretty bland and i want it to be more impressive soon."
- > user (interview, 2026-09-10): "is there a way we can do full translucency properly? in the budget we have in this spec? or should we aim simpler and refine transclucency more later?"
- > user (interview, 2026-09-11): "looks like a massive spec with many reqs. Should we split it? or can we make it leaner?"
- Two-sided transmission (strength, thickness, tint, gated by the shadow map) over a single strength number or physically based subsurface scattering.
- Normal perturbation over vertex displacement, so the silhouette and the fn-24 pins are untouched. No image textures; every value is a material-row field.
- The owner's own references (white oak, black oak, Norway spruce) reshaped the bark during the run.

## Acceptance Criteria

- **R1:** Bark ridges and plates perturb the shading normal only; silhouette byte-identical to fn-24 — met.
- **R2:** Leaf veins and margin tone from the blade coordinates, absent at section roundness 1 — met.
- **R3:** Two-sided transmission scaled by strength and thickness, tinted, shadow-gated; needle near zero — met.
- **R4:** Owner judges oak and spruce at trunk, branch and leaf scale beside fn-9's references with fork seams exposed — met (accepted 2026-09-13).
- **R5:** Oak native frame at or under fn-14's 3.8 ms; browser orbit at 60 fps — met.

## Result

- Bark: vertical grain, shouldered furrows, lifted plates, fine flake; filtered by pixel footprint on both axes (averaged, not switched), removed relief handed to roughness. Eight new validated, blendable material fields. Level selection compacted in placement order so a redraw is exact.
- R5 final (session 7, RTX 3080, 1600x1000, seed 7): native total p50/p95 3.6879/4.0284 ms (fn-14 4.949, session 5 3.9276 over the bound before the shared-height optimisation); browser orbit wall p95 16.70 ms, GPU p50 3.7379 ms, 601 frames.
- Owner verdict (R4, 2026-09-13), on the run-six stills and the live harness at `f1bdd8c`: on the session-four oak, "that's actually much better though we have issues with aliasing"; on the spruce, "the oak was good but i think the spruce isn't quite right. Gaps seem too big?"; both addressed in runs five and six with grazing-angle and distance filtering; final verdict: "ok i'll accept".

## Open

Colour, cavity and occlusion went to fn-29; the sapling look and growth lag seen on the fn-11 harness belong to fn-30 and fn-28.

## Evidence

- `.flow/evidence/fn26/stills/*.jpg` (the twelve verdict stills: oak and spruce trunk, branch, leaf/needle front- and backlit, grazing, oak distance 2x/4x)
- `.flow/evidence/fn26/stills.json` (session, capture and hash per still), `.flow/evidence/fn26/references.json` (reference sources and checksums)
