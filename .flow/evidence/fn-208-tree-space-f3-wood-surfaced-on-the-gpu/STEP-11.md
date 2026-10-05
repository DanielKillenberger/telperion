# fn-208 host decision 11: edge coverage and the one-cell rate (worker, 2026-10-05)

**Result:**
- The bark's steps were already filtered by their footprint, so the filtered-edge part changes nothing.
- A one-cell rate that meets the bar's quality (no hairlines, RMSE under 0.3%) costs more than it saves on every view but the oak's trunk close-up.
- It is reverted and kept as `one-cell-trial.patch`.
- The depth prepass stays as committed.

## The edges were already filtered (checked)

- **Plate walls and rims:** `bark_plate_profile` (`plates.wgsl:171-178`) draws them through `bark_edge` (`bark.wgsl:26-37`). That function widens each smoothstep edge by the variance the pixel's box adds, `sqrt(footprint² − 0.6·span²)`, and integrates it exactly (`bark_step_integral`).
- **Chip noise:** `bark_noise2_filtered`, read at the footprint (`plates.wgsl:247-254`).
- **A plate's identity:** mixed by the share of the pixel inside its edge (`plates.wgsl:262-263`).
- **Spans:** `bark_span`, the exact box integral of a mark.
- **None of these is a hard step;** each covers a pixel by its true fraction (fn-71's rule). Turning them into filtered steps would change no pixel.

## Where step (i)'s hairlines came from

- The cut-down shader read one normal across a furrow narrower than the pixel. Its V-shaped floor (0.006 to 0.012 of a plate) is steep on both sides and shallow in height.
- So a test on the height's bend relative to its amplitude must be tight:
  - At 1/32 of the amplitude across the pixel (`flat32`), hairlines remained (RMSE 1.1% on the oak).
  - At 1/64, measured along both diagonals (`diag64`), the trunk close-up matches full detail.

## The one-cell rate under the quality bar

- **The rule:** one cell where `max over both diagonals of |2·centre − corner − corner| ≤ amplitude / 64`. It reads the five heights at the cells' own footprint and reuses them in the quadrature when the test fails.

| Today's tree, view | Full detail (prepass) | One-cell rate | RMSE against full detail |
|---|--:|--:|--:|
| Oak, trunk close-up 0.75 m | 12.29 ms | 9.04 ms (−26%) | 0.16%, no hairlines (viewed, `raw/shade/sheets/today-oak-trunk-diag64.png`) |
| Spruce, trunk close-up | 8.11 ms | 10.49 ms (**+29%**) | 0.06% |
| Oak, hero | 6.94 ms | 7.05 ms (+2%) | 0.003% |
| Oak, limb | 4.60 ms | 4.85 ms (+5%) | 0.001% |
| Oak, twig 5 cm | 14.05 ms | 15.44 ms (**+10%**) | 0.004% |
| Spruce, hero | 3.64 ms | 3.50 ms (−4%) | 0.0003% |
| Spruce, limb | 3.07 ms | 3.29 ms (+7%) | 0.002% |
| Spruce, twig 5 cm | 4.58 ms | 4.98 ms (+9%) | 0.004% |

**Reading it:**
- **The test passes almost nowhere at the standard views.** The images are unchanged to 1e-5, so the rate saves nothing there, while the test and the larger shader (two shading paths, more registers) add 2 to 10%.
- **On the spruce's trunk the test mostly fails,** so it pays the overhead and gains nothing.
- **Only the oak's trunk close-up gains,** and that view is not on the bar's list.

## For the host

1. **Is the shading rate dropped?** The shader is already band-limited by construction. Its quadrature is the price of shading relief that the pixels do not resolve (sub-pixel furrows), and a cheaper rate either breaks those (step (i)) or must test for them (this step), which costs as much as it saves.
2. **The remaining per-pixel cost is quadrature and overshading.**
   - The quadrature's 16 lighting evaluations a pixel, for the chipped profile, could become analytic lighting of the furrow's V: two wall normals weighted by their coverage. That is shader design beyond an edge term, a candidate for its own spec.
   - The overshading on the engine trees is the tessellator's.
3. **Nothing ships from this step:** the trial is reverted, and the prepass and four samples stand.
