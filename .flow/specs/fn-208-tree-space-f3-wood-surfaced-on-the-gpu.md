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

## Requirements

- **R1:** The curve data the generator hands over, defined and documented in `docs/pipeline.md`; today's trees produce it too (the renderer has one wood path).
- **R2:** Bark shading at the half-pixel error (host decision 7): every procedural term fades where its features are sub-pixel, by its projected size; a depth prepass; a measured multisample count. GPU surfacing at the half-pixel error, with cluster culling; a test that the silhouette of a reference segment stays within half a pixel of a dense reference mesh at several distances, and that sides and rings grow as the camera approaches.
- **R3:** Stills of every passed species (beech, spruce, oak, palm) at the standard views, viewed by the host against today's wood: no visible regression; a 5 cm close-up of a twig in high detail.
- **R4:** Measured on the owner's GPU: wood triangles and frame time for the 80-year spruce and oak at the standard views and at a close-up; GPU memory for wood; the bark shader's time per shaded pixel at the hero view and the 5 cm twig (host decision 7). Report against today.
- **R5:** Workspace gate, `npm test`, Codex review; every shipped artifact within its CI size budget.

## Boundaries

Drawing only: the trees the generator grows do not change. Structure resolved on demand is F1 (its own spec); the scene-level bars are F4.
