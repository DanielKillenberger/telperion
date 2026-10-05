# Tree space F3: wood surfaced on the GPU at the screen's error

## Goal & Context

The owner wants realtime generation with full fidelity near the camera and a fast zoom that always shows the right detail: "If i stare at a twig from 5cm away it should appear in high detail ... this should be intelligent not have hardcoded numbers ... we should be able to zoom in fast and show the right detail. It should be that performant" (owner, 2026-10-05). Today the wood is uploaded as one full mesh (`crates/telperion-render/src/wood.rs`): every ring has 12 sides at every node whatever its size, so the 80-year engine spruce reaches about 200M wood triangles and ran the GPU out of memory (fn-194). Leaves already take their level per frame by projected error (`select.rs`: the coarsest level under half a pixel, frustum-culled, no hysteresis). Wood should follow the same rule. Never baked offline (owner, 2026-10-05).

## Design (host, 2026-10-05)

- **Structure in, surface out.** The generator hands the renderer each axis as a curve with radii (positions, frames, radius per node), not triangles. The GPU builds the tube surface every frame from those curves.
- **One error budget.** Each segment's sides and ring spacing come from its projected radius and curvature against the same half-pixel error `select.rs` uses for leaves: no hardcoded side or ring counts. A twig 5 cm from the eye gets as many sides and rings as its silhouette needs; the same twig far away gets three sides or is dropped when sub-pixel.
- **Clusters.** Segments are grouped into clusters with bounds; a compute pass culls clusters by frustum and projected size and emits the visible tessellation into indirect draws. Triangles per frame then follow the pixels on screen, not the tree's size.
- **Continuity across detail.** Level changes stay below half a pixel by construction (as for leaves); bark texture coordinates are continuous along the curve and around it, so detail changes do not slide the bark.
- **Fine shoots with their leaves.** Leaf-bearing shoots below a projected size may be drawn as part of the foliage instance, so they share the leaves' selection; the threshold is the same error, not a radius.

## Requirements

- **R1:** The curve data the generator hands over, defined and documented in `docs/pipeline.md`; today's trees produce it too (the renderer has one wood path).
- **R2:** GPU surfacing at the half-pixel error, with cluster culling; a test that the silhouette of a reference segment stays within half a pixel of a dense reference mesh at several distances, and that sides and rings grow as the camera approaches.
- **R3:** Stills of every passed species (beech, spruce, oak, palm) at the standard views, viewed by the host against today's wood: no visible regression; a 5 cm close-up of a twig in high detail.
- **R4:** Measured on the owner's GPU: wood triangles and frame time for the 80-year spruce and oak at the standard views and at a close-up; GPU memory for wood. Report against today.
- **R5:** Workspace gate, `npm test`, Codex review; every shipped artifact within its CI size budget.

## Boundaries

Drawing only: the trees the generator grows do not change. Structure resolved on demand is F1 (its own spec); the scene-level bars are F4.
