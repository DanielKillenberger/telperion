# fn-208 step (iii): the CPU reference tessellator (worker, 2026-10-05)

## Built

`Curve::tessellate(&Viewer, error, budget) -> Tessellation`, in `crates/telperion-core/src/pipeline/surface/curve/tessellate.rs` and `tessellate/plan.rs`, documented in `docs/pipeline.md`.

- **Viewer:** the eye, the direction it looks, pixels a metre at 1 m, and the near plane.
- **Ring level (decision 2):** each cluster keeps the coarsest nested ring level whose ladder error stands within half the error at its nearest depth.
- **Pieces:** between the rings kept, a stretch turning through θ over L is cut along its Hermite curve into `m = ceil(sqrt(Lθ·P / (8·e/2)))` pieces, at most 256.
- **Sides:** `ceil(π / acos(1 − (e/2)/ρ))`, rounded up to 3·2^k, at most 384.
  - **The error is split** between the polygon round a ring and the chords along the curve, so the two together stand within it.
  - **Why the split was needed:** with the polygon allowed the whole 0.5 px (decision 2's formula as written), a 6-sided ring at ρ = 3.7 px stands exactly 0.5 px off its circle. The ring's sag then took the twig to 0.63 px at 0.5 m. The split is mine: a reading of decision 2, for the host.
- **Zipper:** rings of different side counts are stitched by angle, one triangle a side of either ring.
- **Caps** at each end of a run.
- **Normals (decision 3):** analytic, `w·radial − w_θ·around − w·w_s·tangent`, with the lobes' turn and the radius's slope from neighbouring rings.
- **Bark coordinates:** `(along, angle)`, the same as today's sweep.
- **Shaped runs (decision 9):** drawn through their cell's own rings (`Section::vertex`), sides by the same rule.
- **Budget:** a budget in triangles coarsens the error by 2, 4 or 8, and reports the `scale`. Past that it refuses with `Error::ResourceLimit`, never truncating.
- **Not in the CPU reference yet:** frustum culling of clusters. The GPU passes (iv) cull; the reference draws every cluster.

## Tests (`tessellate/tests.rs`)

| Test | Result |
|---|---|
| `a_twig_stands_within_half_a_pixel_and_gains_detail_as_the_eye_nears` | **Red first** (`2413596b`): with today's density (12 sides, a ring a node) inside the same function, the reference twig stands 2.70 px off the exact tube at 5 cm. **Green now:** 0.08 px at 50 m (4 rings), 0.37 px at 5 m (7 rings), 0.27 px at 0.5 m (22 rings), 0.40 px at 5 cm (64 rings, 2,450 vertices) |
| `a_budget_coarsens_the_error_and_names_it_or_refuses` | Green |
| `every_presets_wood_surfaces_at_a_hero_view` | Green: every index is in range and every value finite, for all eight values presets, the palm's cells included |

**The reference twig:** 60° on a 0.2 m arc, a node a centimetre, tapering from 2 to 1 mm.

**What the test measures:** the largest distance of the drawn surface from the exact tube, sampled across every triangle that is not a cap, in pixels at the sample's own depth. A silhouette stands within the surface's own distance, so this bounds R2's silhouette from above.

## Today's trees at their hero distance (CPU reference, 0.5 px)

| Preset | Triangles | Rings | Today's mesh |
|---|--:|--:|--:|
| oregon-white-oak | 1,309,206 | 214,721 | 7.7M triangles |
| norway-spruce | 732,516 | 121,319 | 5.9M |
| european-beech | 1,801,752 | 289,862 | (not measured) |
| silver-birch | 849,522 | 139,573 | |
| date-palm | 36,120 | 811 | |
| telperion | 454,290 | 68,876 | |
| laurelin | 591,456 | 72,522 | |
| ordinary | 98,310 | 13,730 | |

**Reading it:**
- **These counts are 5 to 6 times under today's mesh, but far above DESIGN.md's estimate** of 0.02 to 0.2M for the wood a view resolves.
- **The rest is sub-pixel wood.** Every run is still drawn as a tube of at least three sides with two caps: about six triangles a ring, at about one ring per node.
- **Decision 4's ribbons and coverage ribbons (step 5) replace those tubes.** That is where the hero view's count falls to the estimate.

## Decision 11's stills

- **Filtered edges change no pixel:** the bark's steps were already box-filtered (STEP-11.md).
- **The one-cell rate's sheets:**
  - `raw/shade/sheets/today-oak-trunk-diag64.png` (viewed: no hairlines, RMSE 0.16%);
  - `today-spruce-trunk-diag64.png`;
  - the standard views, `*-hero|limb|twig-reuse64.png`, unchanged to 1e-5.
- **The rate is reverted,** because it costs more than it saves (STEP-11.md).
