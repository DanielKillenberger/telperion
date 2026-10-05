# Tree space F3: wood surfaced on the GPU at the screen's error

## Goal & Context

The owner wants realtime generation with full fidelity near the camera and a fast zoom that always shows the right detail: "If i stare at a twig from 5cm away it should appear in high detail ... this should be intelligent not have hardcoded numbers ... we should be able to zoom in fast and show the right detail. It should be that performant" (owner, 2026-10-05). Today the wood is uploaded as one full mesh (`crates/telperion-render/src/wood.rs`): every ring has 12 sides at every node whatever its size, so the 80-year engine spruce reaches about 200M wood triangles and ran the GPU out of memory (fn-194). Leaves already take their level per frame by projected error (`select.rs`: the coarsest level under half a pixel, frustum-culled, no hysteresis). Wood should follow the same rule. Never baked offline (owner, 2026-10-05).

## Design (host, 2026-10-05)

- **Structure in, surface out.** The generator hands the renderer each axis as a curve with radii (positions, frames, radius per node), not triangles. The GPU builds the tube surface every frame from those curves.
- **One error budget.** Each segment's sides and ring spacing come from its projected radius and curvature against the same half-pixel error `select.rs` uses for leaves: no hardcoded side or ring counts. A twig 5 cm from the eye gets as many sides and rings as its silhouette needs; the same twig far away gets three sides or is dropped when sub-pixel.
- **Clusters.** Segments are grouped into clusters with bounds; a compute pass culls clusters by frustum and projected size and emits the visible tessellation into indirect draws. Triangles per frame then follow the pixels on screen, not the tree's size.
- **Continuity across detail.** Level changes stay below half a pixel by construction (as for leaves); bark texture coordinates are continuous along the curve and around it, so detail changes do not slide the bark.
- **Fine shoots with their leaves.** Leaf-bearing shoots below a projected size may be drawn as part of the foliage instance, so they share the leaves' selection; the threshold is the same error, not a radius.

### Host decisions on the design note (2026-10-05; `.flow/evidence/fn-208-tree-space-f3-wood-surfaced-on-the-gpu/DESIGN.md`)

1. **Curve data:** 28 B a point, a run table and clusters of up to 32 points with bounds; every tree produces it, with no second path.
2. **Tessellation:**
   - sides by `ceil(π / acos(1 − 0.5/ρ))`, rounded up to 3·2^k;
   - rings by curvature on a nested ladder;
   - fixed output budgets that coarsen evenly and report their error scale. An overrun is named in the frame report, never silent.
3. **Normals:** analytic tube normals, with the CPU reference following. Every tree changes slightly (AGENTS.md, "Generator evolution"); the change is measured and judged on stills.
4. **Fine shoots:** tube, then ribbon, then a one-pixel coverage ribbon with alpha equal to the true width. Placed as shoot clusters in the wood path, with no `select.rs` change for now. Sub-pixel wood is never dropped. When step 5 lands, a hero comparison against today's goes to the owner, who judges whether the haze is kept.
5. **The CPU reference:** the same tessellation function, evaluated at a given view and error. It is camera-dependent by definition, and camera-independent only in the curve data it reads. Tests compare the GPU and CPU at the same view.
6. **The palm's leaf-base cells:** checked in step 1. If the curve data cannot carry their cross-section, it is reported before step 3.

Order (host): steps 1 to 3 first, stopping at step 2 if shading dominates up close. Step 2 stopped there: bark shading is 72 to 97% of the wood pass at the 5 cm twig (STEP2.md).

### Host decisions 7 to 9 (2026-10-05)

7. **Shading follows the same screen error.** Options (b) and (c) of STEP2.md, together.
   - Bark plates, relief and every procedural term fade where their features are sub-pixel. The fade is an analytic band limit, or a mip-like level from the feature's projected size, never a fixed distance.
   - A depth prepass against overdraw.
   - A measured choice of multisample count.
   - This comes first, before the tessellator: it is the larger cost, it helps today's trees too, and it does not wait on the curve's GPU path.
   - **Bar:** the bark shader's time per shaded pixel at the hero view and at the 5 cm twig, against today's. Stills show no visible change at the standard views. Bark detail's fade is judged on stills, and close-up bark keeps its full detail.
8. **The packed point is 32 bytes.** The frame gets more bits, so Laurelin's metres-wide trunk is exact at 5 cm. Correctness up close beats 14% of curve memory.
9. **The palm's cells beside their run** (about 104 B per shaped run): accepted.

Order (host, 2026-10-05):
- (i) the shading level of detail, the depth prepass and the multisample measurement;
- (ii) the 32-byte point;
- (iii) step 3, the CPU reference tessellator with the silhouette test red first;
- (iv) the GPU passes.

### Host decisions 10 to 12 (2026-10-05, after STEP-I.md)

10. **The depth prepass runs everywhere,** with no per-view switch. The spruce's +0.6 ms at the hero view is accepted: the tessellator shrinks the geometry the prepass draws twice.
11. **Shading detail through analytic edge coverage.**
   - Every hard step in the bark shader (the plate walls and chip edges in `bark_plate_profile`, and any other) becomes a filtered step. Its width is its screen-space footprint, so an edge covers each pixel by its true fraction.
   - Then retry one shading cell where the narrowest feature spans at least 4 px.
   - **Bar:** the close-up trunk sheet shows no hairlines (RMSE against full detail under 0.3%, and viewed). If the filtered edges alone visibly change the standard views, that is reported and judged on stills.
12. **Then (ii), (iii) and (iv):** the 32-byte point; the CPU reference tessellator with the silhouette test red first; the GPU passes.
   - `Gpu::supports_samples` claiming 8 is fixed with a test.
   - Stop after (iii) with a report and the filtered-edge stills.

### Host decisions 13 to 15 (2026-10-05, after STEP-11.md and STEP3.md)

13. **The shading rate is dropped.**
   - `one-cell-trial.patch` stays as evidence.
   - The bark's edges were already box-filtered.
   - The remaining per-pixel cost is the bark's quadrature over furrows narrower than a pixel. Analytic lighting of a furrow's V is its own spec, a candidate after F3 (RESULT.md, open items).
14. **The error is split between the polygon and the sag.** Half of the error in pixels goes to a ring's polygon (`sides = ceil(π / acos(1 − (e/2)/ρ))`, rounded up to 3·2^k). The other half goes to the sag between rings (cluster ring levels and Hermite pieces), so the two together stand within `e`.
15. **`supports_samples`** reads the device's granted format features: accepted.

Then (iv), the GPU passes:
- the cluster pass with the frustum and projected size, the prefix, tessellation into the fixed budgets, the indirect draw, and shadows;
- step 5's ribbons and coverage ribbons, bringing the hero view to the design's 0.02 to 0.2M triangles (measured).

Tests: the GPU against the CPU reference at the same view and error; R3 stills; R4 frame times and wood GPU memory.

### Host decisions 16 to 21 (2026-10-05, after STEP4.md)

The oak and the beech read as the same trees at every view. The palm's cells shade more coherently than today's facets, and that is accepted.

16. **Coverage ribbons are dropped.** True-width ribbons match today's crown and draw faster. Their leftovers are deleted: the coverage attribute, the alpha-to-coverage pipelines, the shader mode and their inventory entries (RESULT.md, following "question, delete, then optimise").
17. **Hero triangle counts above the 0.02 to 0.2M estimate are accepted.** The target is frame time and memory; the cost is shading, not triangles. R4 is frame time and GPU memory per view.
18. **A per-ring compute pass:** one thread a ring, so a close view does not pay the longest cluster walk. The GPU timer covers the compute passes.
19. **Budgets from the screen:** the output budgets are sized from the viewport, pixels times a bound on triangles a pixel, not fixed constants. Coarsening still reports its scale.
20. **One wood path, in F3** (the owner's one-pipeline decision).
   - The curve path is the renderer's only wood path for every tree, today's presets included.
   - The mesh wood upload is retired from the renderer.
   - The CPU mesh stays as the pipeline's reference build (`Expansion::mesh`) for tests and consumers without a GPU.
21. **The browser.**
   - Curve and output buffers are split to stay within WebGPU's default 128 MB binding.
   - The Wasm size budget is checked.
   - One still is taken in the browser where the web harness can draw the curve; where it cannot, what is missing is recorded.

R3 gains a true 5 cm twig shot, aimed along a chosen twig. Then the workspace gate and `npm test`, Codex until SHIP (exact-boundary findings deferred as fn-206 did), and `flowctl done`.

## Requirements

- **R1:** The curve data the generator hands over, defined and documented in `docs/pipeline.md`; today's trees produce it too (the renderer has one wood path).
- **R2:** Bark shading at the half-pixel error (host decision 7): every procedural term fades where its features are sub-pixel, by its projected size; a depth prepass; a measured multisample count. GPU surfacing at the half-pixel error, with cluster culling; a test that the silhouette of a reference segment stays within half a pixel of a dense reference mesh at several distances, and that sides and rings grow as the camera approaches.
- **R3:** Stills of every passed species (beech, spruce, oak, palm) at the standard views, viewed by the host against today's wood: no visible regression; a 5 cm close-up of a twig in high detail.
- **R4:** Measured on the owner's GPU: frame time and GPU memory for wood per view, for the 80-year spruce and oak at the standard views and at a close-up (host decision 17); the bark shader's time per shaded pixel at the hero view and the 5 cm twig (host decision 7). Report against today.
- **R5:** Workspace gate, `npm test`, Codex review; every shipped artifact within its CI size budget.

## Boundaries

Drawing only: the trees the generator grows do not change. Structure resolved on demand is F1 (its own spec); the scene-level bars are F4.
