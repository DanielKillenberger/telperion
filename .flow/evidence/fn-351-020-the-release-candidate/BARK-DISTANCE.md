# The bark-distance gate on the candidate (14 Nov 2026)

`telperion-render::bark_distance resolved_scales_survive_until_the_two_pixel_boundary` failed on the candidate: the oak on seed 7 read mean 6.0038 / p95 16.25 (code values of 255) at 4x against bounds 5.8 / 15.25. This note gives the frames and a motion measure for the candidate and the 11 Nov base. The host decided in advance that the 4x bound is re-pinned only if the shimmer condition below held. It held, and the bound was re-pinned.

## How it was measured

`crates/telperion-render/examples/bark_distance_frames.rs` draws what the test draws, in the test's order: the clay view, Bare at 1600x1000 and Bare at 800x500, at distances 2 and 4 along the test's ray. It reads the test's eroded clay wood mask on rows 200..300 and columns 400 ± 100/distance. Agreement is the full-size still box-reduced 2x2 onto the low grid, the absolute per-channel difference over the mask, then the mean and p95. On the candidate the helper reproduces the test exactly (4,150 / 1,563 mask pixels, 5.1708 / 13.00 and 6.0038 / 16.25). Draw order matters, because the renderer carries state from frame to frame (see FRICTION.md, 14 Nov).

- **Trunk:** the mean width over rows 200..300 of the warm clay run through the centre column.
- **Shimmer:** twelve frames with the camera distance stepped evenly over ±2.5% (3.9x to 4.1x, and 1.95x to 2.05x) along the same ray. For each consecutive pair, the raw absolute difference per channel over the strip mask both frames share, with no alignment, pooled over the 11 pairs.
  - "low" is the 800x500 frames.
  - "ref" is the 1600x1000 frames reduced to 800x500.
- **Repeatability:** the same still drawn twice is byte-identical on both checkouts.

Base: `.worktrees/gate-base` at 5ebab2de0, the helper copied in uncommitted, with its own `target/`. Candidate: `.worktrees/bark` at 3d33a8e8d.

| | trunk px | mask px | mean | p95 | shimmer low mean / p95 | shimmer ref mean / p95 |
|---|---|---|---|---|---|---|
| base 2x | 134.2 | 9,972 | 4.4203 | 12.75 | 2.2970 / 9.00 | 2.4711 / 8.50 |
| candidate 2x | 54.6 | 4,150 | 5.1708 | 13.00 | 1.8685 / 7.00 | 1.7364 / 5.75 |
| base 4x | 71.4 | 4,625 | 5.5206 | 14.50 | 2.0668 / 7.00 | 1.9065 / 6.25 |
| candidate 4x | 28.0 | 1,563 | 6.0038 | 16.25 | 1.8578 / 7.00 | 1.3663 / 4.75 |

The candidate's trunk at the strip is 2.5 times thinner than the base's. At 4x it is narrower than the strip (28 against 50 pixels), so the eroded mask now runs across the whole curved stem, edge to edge. On the base the stem filled the strip. In the strip crops, the difference concentrates along the stem's length rather than across a flat face. The 4x frames show no flicker: the low-resolution sequence differs frame to frame by 0.49 more than its reference, and by 0.21 less than the base's low-resolution sequence.

## Shader history

Since 00832b02a (9 Oct 2026), the commit that last set this test's bounds, `bark.wgsl` and `plates.wgsl` are unchanged. `common.wgsl` changed in four commits, all before the 11 Nov base:

- 369146a51: floret stalks.
- 295a31799: the `leaf_dead` comment and field meaning.
- 91eddc36b: the atmosphere and sky uniforms and `sky_radiance`.
- fc36bb605: parentheses added to `lattice_value`'s hash.

`git diff 5ebab2de0 HEAD -- crates/telperion-render/src/` is empty: the render crate's source, shaders included, is identical between the base and the candidate. What moved the reading is the tree: the oak's thinner trunk (fn-354, wood from sapwood and leaf).

## Condition and change

The condition: the candidate's 4x low-resolution shimmer lies within 1.0 mean of its own reference, and within 1.0 of the base's low-resolution shimmer. It held:

- against its own reference: 1.8578 against 1.3663, a difference of 0.49;
- against the base: 1.8578 against 2.0668, a difference of 0.21.

The 4x bounds were re-pinned by the test's own rule on today's values: mean 6.0038 + 0.5 is 6.5 (rounded to one place, as 5.79 was to 5.8), and p95 16.25 + 2 is 18.25. The comment gains one dated sentence. The test runs green by name: 2x 5.1708 / 13.00 and 4x 6.0038 / 16.25.

The 2x bound was not touched. It passes with 0.03 to spare (5.17 against 5.2).

## Images (`bark-distance/`)

- `strip-4x-base.jpg`, `strip-4x-candidate.jpg`: the 4x strip crop (columns 340..460, rows 192..308) enlarged 4 times with nearest neighbour. From left to right: the full-size still reduced, the low still, and the largest channel difference scaled so 16 is white. Pixels off the mask are dimmed.
- `frames-4x-base.jpg`, `frames-4x-candidate.jpg`: the twelve 800x500 frames from 3.9x to 4.1x, the same crop at 2 times, in two rows of six, with each frame's wood mask undimmed.
