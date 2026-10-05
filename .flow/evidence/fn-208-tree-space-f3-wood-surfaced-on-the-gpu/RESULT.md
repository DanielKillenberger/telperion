# fn-208 result

The renderer's wood is the curve, surfaced on the GPU each frame at half a pixel's error, for the camera and the sun. Steps and numbers: STEP1 to STEP5.md; the last round is STEP5.md.

## Deleted, and why (question, delete, then optimise)

- **Coverage ribbons** (host decision 16): pale beside today's crown; true-width ribbons matched today and were faster. Once true width was the rule nothing read coverage, so the attribute, the alpha-to-coverage pipelines and the shader mode went, not a fix for them.
- **The mesh wood upload** (decision 20): the renderer's buffers, pipelines, caster prefix and the generator's resident wood expansion. The CPU mesh stays as the reference build.
- **fn-26's flat bark calibration patch** (`wood/calibration.rs`): a flat patch is not a curve, so the one wood path cannot draw it. Two ignored evidence captures and a fixture test went with it.

## Open items for the host

- **`casterTexels` is dormant.** The sun's wood is surfaced at its own texel error, so the scene row no longer selects wood. It still shows in the panel and the README now says so. Delete the row, or give it a meaning?
- **Surfacing costs 2 to 10.5 ms** on the engine trees: each cluster is walked six times a view. Measuring the four scales in one walk, or reusing last frame's choice, would cut it.
- **Screen budgets hold 1.3 to 1.6 GB on every tree**, the palm included, against 0.5 to 0.7 GB fixed. A budget could also be capped by the tree's own finest-scale demand.
- **Analytic lighting of a furrow's V** (decision 13): its own spec after F3, with the owner.
