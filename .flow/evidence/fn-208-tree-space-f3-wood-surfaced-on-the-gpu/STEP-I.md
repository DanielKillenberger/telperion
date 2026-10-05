# fn-208 step (i): shading at the screen's error, the depth prepass and the sample count (worker, 2026-10-05)

## What was built (kept)

- **A depth prepass for the wood.**
  - Each lit wood pipeline gets a prepass pipeline from the same shader module (`pass::prepass_pipeline`). It runs the wood's own vertex stage, with an empty `depth_only` fragment, writing depth and no colour.
  - The lit pipeline then tests `LessEqual` without writing (`Depth::Prepassed`), so the bark runs once per pixel for the nearest wood, not once for every branch behind it.
  - The clip position is `@invariant`, so both pipelines compute the same depth.
  - `Wood::draw` issues the two draws (`draw_calls` 2).
- **`Renderer::with_samples`**, which builds a renderer at a given multisample count. `Renderer::new` keeps 4, and `pass::samples` takes the count.
- **Tests:** the renderer's tests are all green under the GPU lock (`raw/t1.txt`).

## What was tried and reverted: a quadrature level of detail

- **The rule tried:** one shading cell, not 2 to 4 per axis, wherever the relief's narrowest feature (a ridge, half a plate, a chip of an eighth of a plate) spans at least 4 pixels.
- **Cost:** on today's oak at the 5 cm twig it cut 14.0 → 12.1 ms. On a trunk close-up 0.75 m from the bark it cut 12.3 → 4.7 ms.
- **Why reverted:** that close-up shows hairline dark lines along the plate walls and chip edges (`raw/shade/sheets/today-oak-trunk.png`, RMSE 1.8%). Those features are steps, not widths: no projected size resolves an edge, so the quadrature there is doing real work.
- **The host's bar:** close-up bark keeps its full detail, so the rule is out.
- **What a working level of detail needs:** an analytic edge term (the edge's own coverage of the pixel), not fewer cells. That is a design question for the host.

## The cost: wood pass at 960 × 720, median ms (`raw/shade/*.log`)

| Tree | View | Before | Depth prepass (kept) | Prepass + one cell (reverted) | Prepass at 1 sample |
|---|---|--:|--:|--:|--:|
| Today's oak | hero | 8.35 | **6.94** | 6.20 | 2.99 |
| Today's oak | limb | 7.63 | **4.60** | 4.39 | 2.60 |
| Today's oak | twig 5 cm | 19.70 | **14.05** | 12.11 | 5.98 |
| Today's spruce | hero | 3.05 | **3.64** | 3.39 | 1.63 |
| Today's spruce | limb | 3.04 | **3.07** | 2.86 | 1.79 |
| Today's spruce | twig 5 cm | 5.03 | **4.58** | 4.43 | 2.93 |
| Engine oak | hero | 66.99 | **41.31** | 41.15 | 21.06 |
| Engine oak | limb | 40.40 | **25.74** | 25.56 | 17.02 |
| Engine oak | twig 5 cm | 131.58 | **53.10** | 52.10 | 30.45 |

Today's oak and spruce at the trunk close-up, with the prepass: 12.29 ms and 8.11 ms.

## The bar: bark shading per covered pixel (4 samples)

- **How measured:** bark shading is the bark view minus the flat (clay) view. Covered pixels are the pixels where the clay frame changes when the wood is put in.

| Tree | View | Before | Prepass | Change |
|---|---|--:|--:|--:|
| Today's oak | hero | 49.1 ns | 30.0 ns | −39% |
| Today's oak | twig 5 cm | 34.7 ns | 20.9 ns | −40% |
| Today's spruce | hero | 42.1 ns | 33.9 ns | −19% |
| Today's spruce | twig 5 cm | 8.5 ns | 6.1 ns | −28% |
| Engine oak | hero | 531.8 ns | 256.0 ns | −52% |
| Engine oak | twig 5 cm | 239.0 ns | 84.7 ns | −65% |

**Reading it:**
- **The prepass pays wherever branches overlap:** the oaks, and the crowns seen from inside.
- **It costs where they do not.** The prepass draws the geometry twice, so the flat pass rises from 1.4 to 2.7 ms on today's oak and from 6.2 to 12.0 ms on the engine oak. Today's spruce at the hero view, a sparse crown with cheap bark, ends 0.6 ms slower (+19%).
- **The engine oak still shades 256 ns a covered pixel, about 8 times today's oak.** That remainder is micro-triangle overshading: 58M triangles, each shading whole 2 × 2 quads. It is the tessellator's to remove (step (iii) onward), not the shader's.
- **The engine spruce was not measured.** Its 6 GB mesh, twice over with the hidden copy, did not run on a card where other processes held 1.3 to 2.5 GB.

## The sample count

- **One sample against four, with the prepass:** one sample halves the wood pass or better (today's oak at the hero view 6.94 → 2.99 ms).
- **But the thin wood aliases into speckle:**
  - visible on every 1x still (`raw/shade/sheets/*-hero.png`, third panel);
  - plain at the hero view, where the fine twigs become dotted lines;
  - and in the twig close-up's background.
- **That is a visible regression, so four samples stay.**
- **Eight samples cannot be drawn portably.** wgpu refused 8 for `Rgba8UnormSrgb` without the adapter-specific feature `TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`; WebGPU guarantees only 1 and 4. `Gpu::supports_samples` reported 8 as offered anyway, so that check reads the adapter, not the device. It is a small defect, recorded here, not fixed.
- **Decision for the host:** 4 is the measured choice. Shading per sample (rather than per pixel) is not in use, and was not needed.

## Stills (viewed: today's oak at the twig and hero views, and the trunk close-up)

- **Location:** `raw/shade/sheets/<tree>-<view>.png`, each before | depth prepass | one sample, for today's oak and spruce and the engine oak, at the hero view, the limb close-up and the 5 cm twig.
- **RMSE, before against the prepass:** 0.08% to 1.1%. The largest is today's oak at the twig view, from edge samples where coplanar socket surfaces now resolve `LessEqual` rather than `Less`.
- **No visible change** at any of the standard views (worker's reading; for the host to judge).
- **Trunk close-up:** `today-oak-trunk.png` and `today-spruce-trunk.png`, quadrature as today against one cell: the reverted change's artifacts.

## For the host

1. **The depth prepass is in:** −20 to −60% on the oaks and on views from inside a crown, and +19% on the sparse spruce at the hero view. An alternative is a prepass only where the overdraw measures high. That would be a per-view switch, so it is a design call.
2. **The shading level of detail did not hold.** Reducing the bark's quadrature visibly breaks plate and chip edges up close. A working level of detail needs an analytic edge coverage term in `plates.wgsl`, `bark_plate_profile`. That is shader design: I stopped there.
3. **Four samples stay;** one sample aliases.
4. **The remaining engine-tree shading cost is overshading,** which the tessellator removes. That supports doing (ii) to (iv).
