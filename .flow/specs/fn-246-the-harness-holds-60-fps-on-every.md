# The harness holds 60 fps on every shipped tree

## Goal & Context

The owner (2026-10-09), after the release branch's harness: build time is acceptable for now, "It's the framerate after." Measured natively on the RTX 3080 at 1920×1080, 4x MSAA, warm cache (`.flow/evidence/fn-198-tree-space-f-the-engine-in-production/PERF.md`): Telperion 8 fps at the whole-tree view and 13 close, Laurelin 11 and 16, the oak 28 close, the spruce 48-52; beech, birch, palm and ordinary 105-850. Every slow case is GPU-bound (CPU encode 0.13-0.35 ms). The three costs: (1) the leaf and flower draw of dense crowns: Telperion draws 9.05M leaves and 10.4M flowers, about 9 instances a pixel, every leaf already at the coarsest level (4 triangles), nothing culled; (2) the wood's shaded pass at close views, about 30 ms on the oak and the Two Trees, following screen coverage (bark relief and `bark_parallax` suspected, untested); (3) sun-side work (the curve's sun surfacing and the shadow map, 6-8 ms) recomputed every frame although it depends only on the light and the tree. The harness also redraws every animation frame when nothing changes (`harness/rust-stage.ts:102-106`). Host (2026-10-09): release blocker.

## Design (host, 2026-10-09)

- **Leaves and flowers thinned by their size on screen, area kept.** Where an organ's projected size falls below a stated fraction of a pixel, a deterministic share of the organs is dropped, keyed by a hash of each organ's own stored position words (present in both the CPU `Instances` and the device's resident placements, so no lineage needs carrying and generation is unchanged; the same organs stay drawn as the camera moves) and the kept ones are scaled up so the expected covered area is unchanged (stochastic simplification with area preservation). The share and the scale vary continuously with projected size, so nothing pops; at sizes above the threshold nothing changes. One rule for leaves and flowers, inside the existing selection pass, no new pass.
- **Sun-side work only when it changes.** The curve's sun surfacing is recomputed only when the tree or the light changes; the shadow map only when the tree, the light, the view's caster membership (Whole draws foliage casters, Bare only wood) or the caster stride changes; never per camera frame. Verified across Whole → Bare → Whole without rebuilding the tree, and on a caster-stride change.
- **No redraw when nothing changed.** The harness draws a frame only when something in the picture changed: the camera, the tree, the view, the viewport (canvas resize) or any scene setting (sun, sky, ground, shadow). A pending change survives frame attempts that `WebRenderer::frame()` skips while GPU work is in flight, and is cleared only when a frame is submitted. Verified on a resize, a ground-only scene edit, and a camera update received during an in-flight frame.
- **Timing of cached passes.** A pass skipped because its result is cached is recorded as cached (zero cost, flagged), distinct from an unavailable measurement; `Session::resolve`, `Report::with_passes` and the renderer's timing tests are updated to that contract, and performance is verified with the cache behaving as it does in use.
- **The wood's shaded pass measured, then made cheaper by its cause** (bark relief off against on first); the cause and the fix stated before code.

## Requirements

- **R1 (measure):** per pass, per species, whole and close, before; the wood's shaded-pass cost with bark relief off and on.
- **R2 (build):** the four design points; every change keeps one pipeline and changes the picture by degree.
- **R3 (proved):** every shipped species at 60 fps or more at the harness's whole-tree and close views, natively at 1920×1080 4x MSAA (best of 3, load stated), with each pass's cost before and after; stills before and after at whole, mid and close views, viewed, showing no visible loss (crown density and colour at distance unchanged within a stated tolerance of covered pixels); idle frames cost nothing; the CPU and device paths unchanged in what they build.
- **R4:** gate and npm test green.

## Boundaries

Drawing only; generation unchanged. Release blocker for fn-237.
