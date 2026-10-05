# fn-208 step 2: triangle cost against shading cost on today's wood path (worker, 2026-10-05)

**Result: shading dominates up close, on every tree. Following the host's instruction, the run stops here, before step 3.**

## How measured

- **Machine:** RTX 3080 (NVIDIA 610.57.04, Vulkan), under the GPU lock, 4x multisampling.
- **Protocol:** `telperion_render::measure` (8 conditioning, 8 warmup and 120 measured frames). Every record is valid.
- **Load:** CPU load 2.1 to 6.9, with 1.3 to 1.4 GB of the card held by others.
- **Trees:** seed 1, 80 years for the engine's, dressed as the stills runner dresses them.
- **Leaves:** removed, so both views below draw wood alone.
- **Views:** the hero pose, the limb close-up and the twig at 5 cm, as in DESIGN.md section 4.
- **Each view at four sizes of one aspect:** 64 × 48, 480 × 360, 960 × 720 and 1920 × 1440.
- **Two shaders:**
  - **Bark:** `View::Bare`, the bark shader.
  - **Flat:** `View::Clay`, whose wood fragment returns a lit flat colour before any bark work (`R/shaders/wood.wgsl:217-218`, checked).
- **What each column means:**
  - **Triangle floor:** the flat shader at 64 × 48, where almost nothing is shaded: the vertex, setup and raster cost.
  - **Bark shading:** bark minus flat at the same size.
- **Source:** the probe is `f3-frame-probe.rs` (step 2 version). The log is `raw/step2.log`.

## At 960 × 720 (wood pass, median, ms)

| Tree | View | Triangle floor | Flat | Bark | Bark shading's share | Triangle floor's share | Bark / flat at 1920 × 1440 |
|---|---|--:|--:|--:|--:|--:|--:|
| Today's oak | hero | 0.58 | 1.38 | 8.36 | 83% | 7% | 18.2 / 2.3 |
| Today's oak | limb | 0.48 | 0.80 | 7.59 | 90% | 6% | 16.5 / 1.3 |
| Today's oak | twig 5 cm | 0.50 | 1.67 | 19.95 | **92%** | 3% | 45.4 / 2.9 |
| Engine oak | hero | 4.11 | 6.20 | 67.20 | 91% | 6% | 156.0 / 6.6 |
| Engine oak | limb | 3.54 | 3.65 | 41.48 | 91% | 9% | 73.1 / 3.6 |
| Engine oak | twig 5 cm | 3.64 | 4.26 | 133.10 | **97%** | 3% | 279.1 / 4.8 |
| Today's spruce | hero | 0.42 | 1.05 | 2.94 | 64% | 14% | 6.7 / 1.8 |
| Today's spruce | limb | 0.41 | 0.74 | 3.08 | 76% | 13% | 6.0 / 1.2 |
| Today's spruce | twig 5 cm | 0.38 | 0.86 | 5.09 | **83%** | 8% | 12.8 / 1.4 |
| Engine spruce | hero | 12.61 | 21.11 | 41.17 | 49% | 31% | 85.1 / 24.2 |
| Engine spruce | limb | 12.43 | 13.20 | 23.30 | 43% | 53% | 33.5 / 13.4 |
| Engine spruce | twig 5 cm | 12.52 | 13.57 | 48.84 | **72%** | 26% | 92.0 / 14.0 |

## What the numbers say

1. **Up close, bark shading is 72 to 97% of the wood pass on every tree.** It scales with the pixels: from 960 × 720 to 1920 × 1440 the bark time grows 2.0 to 2.3x, while the flat time grows 1.0 to 1.7x.
2. **Triangles alone (the floor) are 3 to 14% of the pass on today's trees and the engine oak.**
   - Only the engine spruce is triangle-bound to a large share: 12.5 ms of its 198M triangles is spent before any pixel is shaded, which is 26 to 53% of the pass.
   - A tessellator removes that floor, about 12 ms on the spruce and 4 ms on the engine oak.
3. **Part of the bark's cost is micro-triangle overshading, which a tessellator also removes.**
   - At the same views, the engine oak's bark shading is 61 ms (hero) and 129 ms (twig), against today's oak's 7 ms and 18 ms.
   - The engine oak's crown covers somewhat more of the frame, but its triangles are 7.5 times as many and mostly sub-pixel. A GPU shades whole 2 × 2 quads per triangle, so sub-pixel triangles multiply the fragments shaded.
   - How much of the 61 ms is overshading rather than coverage is **unknown** without a coverage count. The flat shader's own growth (4.1 → 6.2 ms) is cheap per fragment, so it does not show this.
4. **The bark shader on a sane mesh is a wall of its own, and the tessellator does not move it.**
   - Today's oak at 7.7M triangles spends 7.8 ms (hero) and 18.3 ms (twig) on bark shading at 960 × 720, and 15.9 ms and 42.5 ms at 1920 × 1440.
   - The flat shader draws the same pixels in 1.4 to 2.9 ms.
   - STRATEGY.md's hero tree budget is 2 ms for the whole tree.
   - So a perfect half-pixel tessellation of today's oak still leaves about 8 to 20 ms of bark shading at these views. The bark shader's per-pixel cost, or the pixels it shades (overdraw), sets the frame, not the triangles.

## What this changes in what the tessellator buys (for the host)

- **It still buys:**
  - memory (6 to 7.5 GB of the engine spruce's wood down to about 0.37 GB);
  - the engine spruce's 12 ms triangle floor;
  - the engine trees' micro-triangle overshading, which on the engine oak is likely most of the gap between 67 ms and today's 8 ms;
  - the 5 cm silhouette.
- **It does not buy** the frame bar. Today's oak, tessellated perfectly, shades 8 to 20 ms of bark at these views.
- **The levers that would reach it** are outside fn-208's scope as written:
  - bark shading cost per fragment, a shader level of detail by the same screen error, for example plates and relief faded where they are sub-pixel;
  - overdraw (a depth prepass, so the bark runs once a pixel, or front-to-back cluster order);
  - the multisample count.
  - Which of these the 18 ms is made of is **unknown**: the bark shader's own timing test (`R/wood/calibration.rs:211-213`, `time_fullscreen_bark_and_mature_oak`, ignored by default) would split full-screen shading from overdraw.
- **Decision needed:**
  - (a) Proceed to step 3 as planned, with a bark-shading spec beside it.
  - (b) Fold a shading level of detail and a depth prepass into fn-208 before or with the tessellator.
  - (c) Re-order: shading first, because it is the larger term on today's trees.
