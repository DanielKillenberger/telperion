# fn-217 task 3: floret form, horns and blossom at distance

DESIGN.md sections 6, 7, 8 and 9, release half (b). The raw stills are in `raw/` (ignored). Every still named here was viewed, except the single PNGs that the sheets were made from.

## What was built

- **8.3, the bark's lengths under scale.** `Family::drawn_material` multiplies `shootRadius`, `matureRadius` and `matureSpan` by the group's scale, once. The renderer's `set_family` draws through it. At scale 1, and with the group off, the material is the applied one to the bit (`supernatural::tests::the_bark_s_lengths_are_drawn_at_the_group_s_scale`).
- **8.2, the rim in leaf space.** The margin band `[1 - marginWidth, 1]` is box-filtered over the pixel's footprint on the blade and renormalised to the blade.
  - The edge coordinate does not run with the blade's area. So, as the footprint grows past the leaf, the band is weighed to its area share.
  - That share is measured once per submitted element on its finest triangles (`scene/band.rs`) and carried in `leaf_colour_detail.w`.
- **F3.** The organ block takes no canopy normal.
- **9.1, `/bloom/petalCurl`.** The row feeds the petal's existing curl, with its sign turned so that positive flares outward: the blade's curl bends toward its face, which looks in at the axis (the red test measured the tip crossing the axis). At 0 the element is unchanged to the byte.
- **F1, the eye.** Four rows: `/material/bloomEye{Red,Green,Blue}`, and `bloomEyeShare`, the share of the petal's length from the base. The petals mix toward the eye colour over that share, filtered as the rim is. It applies only in the organ block, as the `bloom_eye` uniform.
- **F2, stamens.** Three rows: `/bloom/stamens` (fractional: the last stamen grows in by the count's fraction), `stamenLength` and `stamenWidth`.
  - Each stamen is one quad from the floret's centre, leaning out half the opening and turned between the petals.
  - Its coordinate sits at the petal's base, so the eye colours it.
  - **LOD:** a level keeps the stamens while its deviation is under their length. Their reach is the far corner, `hypot(length, width / 2)`, after review. A level as coarse as that already stands for their absence, so the ladder and its deviations are the petal's.
  - **Cost:** 2 triangles per stamen on the levels that keep them.
- Every new `/bloom` row has its `/supernatural/bloom` override.
- **Organ mask.** `Renderer::set_organ_mask` is an instrument for M0. In the clay view the flowers draw white and everything else black, unlit. With the mask off, the clay room is unchanged (`organ_coverage::the_mask_shows_the_flowers_and_nothing_else`).

## Proof (measured)

| Claim | Run | Result |
|---|---|---|
| Rim at distance | `rim_at_distance` (red before each fix) | Width 0.1, before the fix: near 0.059 and far 0.520. Width 0.5, coordinate share only: near 0.709 and far 0.495. Area-weighted (review fix): width 0.1 near 0.115, far 0.117; width 0.5 near 0.709, far 0.712. |
| Curl flares out | `bloom::tests::a_positive_curl_flares_the_petal_tips_away_from_the_axis` | Red at first: the tip went to -4.7 mm, past the axis. Green after the sign. |
| Stamens | `bloom::tests::stamens_stand_in_a_ring_by_degree_and_leave_the_coarse_levels` | The element at 0 is unchanged. 5 stamens add 20 vertices; the sixth starts under 0.1 mm. Coarse levels drop the stamens by their length. |
| Eye and stamens dormant, then acting | `bloom_form::the_eye_and_the_stamens_act_only_where_set` | 0 pixels change with an eye colour but no share, or a stamen size but no count. Pixels change when they are set. |
| Catalogue | `catalogue_identity` | With the 12 new rows stripped, the family, walk and override digests are task 2's to the bit. The refusal digest re-pairs by index. |

**M0** (`organ_coverage::measure_the_florets_coverage_at_the_hero_pose`) on a cherry-dense beech at 960 x 720. It is in two parts, and each part holds everything else fixed (corrected after review: levels run coarsest first, so the first run's `Forced(0)` was the coarsest, and the leaves changed level with the flowers).

| Tree | Florets | Sampling: reference, 4x at the finest | Frame, the same geometry | Levels (leaves as specks): finest | Selected |
|---|---|---|---|---|---|
| beech 20 | 23,840 | 0.01901 | 0.01911 (1.005) | 0.02297 | 0.02120 (0.923) |
| beech 40 | 307,972 | 0.01563 | 0.01569 (1.004) | 0.03962 | 0.03798 (0.959) |

- **Sampling.** With matching visibility, sampling loses nothing: 1.004 to 1.005 of the reference. Sub-pixel florets keep their coverage, so there is nothing for M2's alpha-to-coverage to restore.
- **Levels.** The organ's coarse levels keep 92 to 96 % of the finest geometry's coverage.
- **Decision: M1 (values) by the measurement.**
- **Values do move it.** Flowering on every outer age with 12 florets of 20 mm petals gives a coverage of 0.0294 (age 20) and 0.0286 (age 40), with 3 to 3.9 times the florets.

## Stills (viewed)

- **Curl** (`form7/sheet-curl.png`). At petalCurl 0.6 the tips flare away from the axis: a throat that opens into a mouth. At 0 the floret is a straight cone.
- **Raceme, 9.2 values** (`form7/sheet-raceme.png`, beside Laburnum). At middle distance the racemes hang as vertical strands. Close up, at a share of 1, they crowd into a dense curtain in front of the camera. The worker's reading: strands, not a cloud, so the work went on (9.2).
- **Blossom** (`form8/sheet-blossom.png`, beside the cherry corymb). Before: flat white pentagons. After: a five-petal flower with a yellow-green eye and a ring of stamens, close and at corymb distance.
- **Margin on natural species** (`margin/sheet-beech.png`, `margin/sheet-oak.png`; headless at 5bc4cde1 against HEAD).
  - No visible change. 0.03 to 0.15 % of pixels differ, with an RMSE of 0.2 to 0.6 %.
  - The silver birch has no engine species and is refused ("no engine species"), so no birch is drawn on this branch.
  - Accepted under the generator-evolution policy. No pixel baseline pinned these trees.
- **Gold rim at whole-tree distance** (`distance/sheet-gold-rim.png`). On task 2's gold beech the crown was gold; it is now green with a faint warmth. Laurelin's test tree (`distance/sheet-whole.png`: group on, scale 3, gold rim, horn racemes) reads green, not gold.
- **Blossom at whole-tree distance** (`distance/sheet-whole.png`, `distance/sheet-cherry.png`).
  - The cherry-dense beech reads as a green crown with a grey speckle, nowhere near the cherry photograph's white mass.
  - The denser values change only 0.27 % of the pixels.

## Open for the host

1. **Blossom at distance is not reached by values on the beech.** M0 shows that the renderer keeps coverage. Three times the florets and twice the petal area give only 1.7 times the visible coverage: the outer leaves hide most florets. The petals read grey at that distance. This is Astra's P1-3: crown coverage is a question of where the florets stand, not only how many there are.
2. **The close-up raceme at a share of 1** is a dense curtain. Whether it matches Laburnum closely enough is the host's verdict.
3. **The birch's margin** cannot be checked until the birch has an engine species.
