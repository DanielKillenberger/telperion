# Reference-matched stills and the species QA pass

## Goal

The owner rejected fn-34's first beech and birch round because the comparison itself was unfair: stills used the renderer's one fixed three-quarter camera, noon sun, a scale figure and a grey-green disc, while each photograph has its own viewpoint, light, season and aspect. This spec gave every reference photograph a matched still (same camera direction and framing, sun, foliage state and aspect, figure and room out of the way), a side-by-side pair composite, and a first measured comparison (silhouette proportions and centre-crop colour), so a species is judged on fair pairs.

## Decisions

- > user (2026-09-14, on the fn-34 judging set beside the Oregon State references): "now i think we should make a shot that closely imitates the reference image to be able to compare. And after that do a QA pass. To me it's clear that it's not there yet."
- > owner (2026-09-14, the catalogue track): the owner's eye is spent on the final round of each species and the value tier spends the rest.
- A `shot` block on the fn-19 reference record (camera azimuth, elevation, fill, target, fov; sun; foliage leaf-on or hidden; aspect; confidence; derivation) over a separate camera file.
- Authored camera through the existing bounds solve, so a declared fill means the same on every seed and the fixed views stay byte-identical. Overcast through existing scene-row colours, no new light.
- Pillow for the composite and instrument (the fn-29 crop measurement had been a hand-run `magick` command nobody could rerun). Silhouette and colour first; bark structure is fn-32's. No numeric gate yet.
- Pair composites contain the photograph, so they stay in ignored directories, recorded by sha256.

## Acceptance Criteria

- **R1:** Optional closed shot block on the reference record; existing records validate; bad values refused naming the field — met.
- **R2:** Headless `--camera` and a no-figure frame compose with view, scene and size; crown at declared fill on any aspect; fixed views byte-identical — met (shipped as `--no-figure`).
- **R3:** One matched job per shot-blocked reference in the species runner; pair composite and comparison JSON per job — met (plus a horizon-sun twin).
- **R4:** Beech and birch carry shot blocks; fn-34 round two renders and tabulates matched pairs and the owner judges — met (rounds 2-25 of fn-34 were judged through the rig).
- **R5:** Template and onboarding doc judge on matched pairs; tests cover camera parsing, fill on two aspects, the no-figure frame, schema, and the compare script — met.

## Result

The rig renders each reference record through its own camera, sun and foliage state, and `scripts/compare-references.py` pairs and measures it. Every fn-34 round from 2 to 25 was judged through it. The owner accepted the silver birch in fn-34 (the final verdict moved to round 26 in the harness); the European beech's verdict moved to fn-62.

## Open

Which silhouette statistics should gate, and whether photograph masks need hand crop boxes, were left to the first species' numbers.
