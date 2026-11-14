# Tree space F3: wood surfaced on the GPU at the screen's error

## Goal

The owner wants realtime generation with full fidelity near the camera and fast zoom: "If i stare at a twig from 5cm away it should appear in high detail ... this should be intelligent not have hardcoded numbers ... we should be able to zoom in fast and show the right detail. It should be that performant" (owner, 2026-10-05). Wood was uploaded as one full mesh with 12 sides at every node, so the 80-year engine spruce reached about 200M wood triangles and ran the GPU out of memory (fn-194). Leaves already take their level per frame by projected error (`select.rs`); wood should follow the same rule, never baked offline (owner, 2026-10-05). Drawing only: the trees do not change. F1 resolves structure on demand; scene-level bars are F4.

## Decisions

- Design (host, 2026-10-05): the generator hands each axis as a curve with radii; the GPU builds the tube surface each frame; sides and ring spacing from projected radius and curvature against the half-pixel error; clusters culled by frustum and projected size into indirect draws; bark coordinates continuous across detail.
- 1-6: curve data (run table, clusters of up to 32 points with bounds) for every tree; sides `ceil(π / acos(1 − 0.5/ρ))` rounded to 3·2^k, rings on a nested curvature ladder, fixed budgets that coarsen evenly and report their scale; analytic normals (every tree changes slightly, judged on stills); fine shoots tube → ribbon, sub-pixel wood never dropped; the CPU reference is the same tessellation at a given view; the palm's leaf-base cells checked in step 1.
- Step 2 found bark shading 72 to 97 % of the wood pass at the 5 cm twig. 7: shading follows the same screen error (sub-pixel features fade by projected size, never a fixed distance), a depth prepass, a measured sample count — before the tessellator.
- 8: the packed point is 32 bytes so Laurelin's metres-wide trunk is exact at 5 cm. 9: the palm's cells beside their run (~104 B per shaped run) accepted.
- 10: the depth prepass runs everywhere (spruce hero +0.6 ms accepted). 11: hard bark steps become filtered steps; retry one shading cell. 12: `Gpu::supports_samples` fixed with a test.
- 13: the one-cell shading rate is dropped (it passed almost nowhere at standard views and cost 2 to 10 %; spruce trunk +29 %); `one-cell-trial.patch` stays as evidence; analytic lighting of a furrow's V becomes its own spec. 14: the error splits half to a ring's polygon, half to the sag between rings. 15: `supports_samples` reads the device's granted format features.
- 16: coverage ribbons dropped and deleted (they read pale; true-width ribbons matched today and drew faster, 2.7 vs 4.4 ms). 17: hero triangle counts above estimate accepted; R4 is frame time and memory. 18: one compute thread per ring. 19: budgets sized from the viewport. 20 (the owner's one-pipeline decision): the curve is the renderer's only wood path for every tree; the mesh wood upload is retired; the CPU mesh stays as the reference build (`Expansion::mesh`). 21: buffers split under WebGPU's 128 MB binding, Wasm budget checked, one browser still.
- 22: the grazing test pins both resolutions' wood to 1600 × 1000 (`Renderer::pin_curve_viewport`). 23: budgets are the lesser of the screen's and the tree's demand (`Curve::demand`). 24: `casterTexels` deleted. 25: surfacing's six cluster walks a view left as an open optimisation.

## Requirements

- **R1:** Curve data handed over, documented in `docs/pipeline.md`; today's trees produce it (one wood path) — met.
- **R2:** Bark shading at the half-pixel error with depth prepass and measured sample count; GPU surfacing at half-pixel error with cluster culling; silhouette within half a pixel of a dense reference at several distances, sides and rings growing as the camera approaches — met.
- **R3:** Stills of beech, spruce, oak, palm at standard views against today's wood, viewed: no visible regression; a 5 cm twig close-up in high detail — met.
- **R4:** Frame time and wood GPU memory per view for the 80-year spruce and oak at standard views and a close-up; bark time per shaded pixel at hero and 5 cm twig; against today — met.
- **R5:** Workspace gate, `npm test`, Codex; every artifact within its CI size budget — met.

## Result

- R4 (80-year trees, seed 1, 960 × 720, 4 samples), frame ms today → curve: beech hero 19.6 → 6.7, twig 26.1 → 12.8; spruce hero 51.9 → 17.8, twig 43.0 → 22.3; oak hero 42.2 → 20.2, twig 54.5 → 38.6; every bare frame 1.3 to 3× faster. Surfacing costs 2 to 10.5 ms (spruce most). Wood GPU memory today → within-demand budgets: spruce 7.49 → 1.59 GB, beech 3.15 → 1.33 GB, oak 2.19 → 1.15-1.20 GB, palm 2.6 MB → 0.13-0.18 GB. Depth prepass alone: today's oak twig 19.70 → 14.05 ms.
- R3: RMSE against today bare 1.0 to 6.0 %, whole 0.3 to 2.9 % (palm twig 11 %, its cells' analytic normals, accepted); "the same trees" at hero; the shot along a twig shows smooth round tubes, no facets (`along-bare.jpg`). Browser draws the ordinary tree and spruce through the curve path (`browser-ordinary.jpg`); `telperion-render.wasm` 1,498,275 B against its 2,200,000 ceiling.
- Device matches the CPU reference (positions within 3.8e-6 m). Grazing test after decision 22: oak mean 1.93 (bound 3), spruce 0.89. Deleted: coverage ribbons, the mesh wood upload and resident wood expansion, `casterTexels`, fn-26's flat bark calibration patch (a flat patch is not a curve).
- Gate green after fixes; `npm test` 129 green; Codex SHIP (rounds 2 and 3).

## Open

- Surfacing walks each cluster six times a view (decision 25): 2 to 10.5 ms to cut.
- Analytic lighting of a furrow's V (decision 13), its own spec with the owner.
- Exact boundary deferred: a stretch on an integer piece-count boundary may be cut once more in float32 on the device (77,792 vs 77,789 vertices); the GPU test allows counts within a thousandth.

## Friction

- No runner timed a chosen tree at chosen views -> a `--time` flag on the stills runner.
- dcg hook refuses loop-built redirects -> local hook, nothing in the repository.
- Session tmpfs quota full from earlier rounds' scratch -> local setup, for the owner.
- `generation_limit_guard` only surfaced at the 8-min gate; the beech memory ceiling flaked under load -> run the guard in the crate's own tests; a nextest group for the ceiling test.
- `supports_samples` reported 8 the device was not granted -> fixed (decision 15); its test never shown red -> extract the decision first, test red, then fix.
- The vegetation timer missed the compute passes (~20 min bisecting) -> closed by decision 18 (a fourth timer pair).

## Evidence

- `.flow/evidence/fn-208-tree-space-f3-wood-surfaced-on-the-gpu/along-bare.jpg` (R3, the 5 cm shot along a twig)
- `.flow/evidence/fn-208-tree-space-f3-wood-surfaced-on-the-gpu/browser-ordinary.jpg` (decision 21, browser still)
- `.flow/evidence/fn-208-tree-space-f3-wood-surfaced-on-the-gpu/one-cell-trial.patch` (decision 13, the rejected one-cell rate)
