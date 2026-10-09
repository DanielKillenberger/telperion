# fn-246: the harness at 60 fps

Measured on the RTX 3080 (Vulkan, NVIDIA 610.57.04), natively at 1920x1080 with 4x MSAA, warm driver shader cache (`__GL_SHADER_DISK_CACHE_SIZE=4000000000`), `RAYON_NUM_THREADS=4`, under the GPU lock. The probe is `crates/telperion-render/examples/frames.rs` (moved from fn-198's scratch `bench/frames.rs`): one resident build at seed 1, then the hero pose ("whole") and the same pose at a quarter of the distance ("close"), in the whole view and the bare (wood-only) view. **Frame ms** is wall time per frame over 120 pipelined frames (GPU-bound everywhere, so GPU time); **fps** is its inverse. Pass columns are timestamp p50 in ms from `measure()`: **vegetation** is the vegetation render pass (wood prepass and shaded pass, then leaves and flowers), so the bare view's vegetation is the wood alone. Each row is the best of 3 processes; **load** is the 1-minute load average at that run. A `-` is a pass the session did not report (ordinary's whole view: the sub-millisecond tail exceeded the contention ratio, so the report carries no percentile).

## R1: before (base 4bc51b6b)

| species | variant | view | runs | load | frame ms | fps | surfacing | selection | shadow | vegetation |
|---|---|---|---|---|---|---|---|---|---|---|
| ordinary | full | whole | 3 | 5.50 | 1.19 | 842 | - | - | - | - |
| ordinary | full | close | 3 | 5.50 | 1.52 | 656 | 0.90 | 0.01 | 0.02 | 0.39 |
| ordinary | full | bare-whole | 3 | 5.50 | 1.11 | 897 | - | - | - | - |
| ordinary | full | bare-close | 3 | 5.50 | 1.39 | 720 | 0.90 | - | 0.01 | 0.32 |
| oregon-white-oak | full | whole | 3 | 4.22 | 14.08 | 71 | 1.48 | 0.02 | 0.19 | 12.47 |
| oregon-white-oak | full | close | 3 | 2.87 | 35.92 | 28 | 1.77 | 0.02 | 0.19 | 33.65 |
| oregon-white-oak | full | bare-whole | 3 | 4.22 | 12.22 | 82 | 1.48 | - | 0.05 | 10.55 |
| oregon-white-oak | full | bare-close | 3 | 1.85 | 33.39 | 30 | 1.78 | - | 0.05 | 31.26 |
| norway-spruce | full | whole | 3 | 2.92 | 21.20 | 47 | 4.45 | 0.21 | 2.39 | 13.94 |
| norway-spruce | full | close | 3 | 2.92 | 19.30 | 52 | 3.38 | 0.19 | 2.39 | 13.08 |
| norway-spruce | full | bare-whole | 3 | 2.92 | 10.33 | 97 | 4.44 | - | 0.25 | 5.35 |
| norway-spruce | full | bare-close | 3 | 2.92 | 13.87 | 72 | 3.64 | - | 0.26 | 9.81 |
| european-beech | full | whole | 3 | 3.37 | 4.59 | 218 | 1.06 | 0.01 | 0.04 | 3.33 |
| european-beech | full | close | 3 | 3.37 | 9.42 | 106 | 1.20 | 0.01 | 0.04 | 8.00 |
| european-beech | full | bare-whole | 3 | 3.37 | 4.27 | 234 | 1.06 | - | 0.02 | 3.04 |
| european-beech | full | bare-close | 3 | 3.37 | 8.84 | 113 | 1.20 | - | 0.02 | 7.45 |
| silver-birch | full | whole | 3 | 3.95 | 3.31 | 302 | 1.69 | 0.01 | 0.10 | 1.33 |
| silver-birch | full | close | 3 | 3.95 | 5.33 | 188 | 1.78 | 0.01 | 0.10 | 3.28 |
| silver-birch | full | bare-whole | 3 | 3.95 | 3.10 | 322 | 1.69 | - | 0.06 | 1.14 |
| silver-birch | full | bare-close | 3 | 3.95 | 4.99 | 200 | 1.78 | - | 0.06 | 3.01 |
| date-palm | full | whole | 3 | 4.04 | 1.40 | 713 | 0.70 | 0.01 | 0.02 | 0.53 |
| date-palm | full | close | 3 | 4.04 | 2.77 | 361 | 0.58 | 0.01 | 0.02 | 2.01 |
| date-palm | full | bare-whole | 3 | 4.04 | 1.31 | 764 | 0.71 | - | 0.01 | 0.45 |
| date-palm | full | bare-close | 3 | 4.12 | 2.52 | 397 | 0.58 | - | 0.01 | 1.80 |
| telperion | full | whole | 3 | 1.55 | 120.59 | 8 | 5.21 | 0.43 | 3.02 | 111.42 |
| telperion | full | close | 3 | 1.55 | 79.70 | 12 | 5.30 | 0.38 | 3.03 | 70.60 |
| telperion | full | bare-whole | 3 | 1.55 | 10.34 | 97 | 5.21 | - | 0.25 | 4.68 |
| telperion | full | bare-close | 3 | 1.55 | 35.58 | 28 | 5.27 | - | 0.25 | 29.88 |
| laurelin | full | whole | 3 | 1.04 | 88.75 | 11 | 3.65 | 0.27 | 2.72 | 81.67 |
| laurelin | full | close | 3 | 1.04 | 61.44 | 16 | 4.08 | 0.24 | 2.72 | 53.83 |
| laurelin | full | bare-whole | 3 | 1.04 | 8.62 | 116 | 3.67 | - | 0.19 | 4.83 |
| laurelin | full | bare-close | 3 | 1.07 | 35.77 | 28 | 4.51 | - | 0.19 | 31.54 |

### The wood's shaded pass with bark relief off and on (bare views, same builds)

`relief-off` sets the drawn material's ridge scale and depth strength to zero, so the wood takes the shader's flat branch; `parallax-off` keeps the relief and sets only the depth strength (parallax) to zero. Compare with the `full` bare rows above.

| species | variant | view | runs | load | frame ms | fps | surfacing | selection | shadow | vegetation |
|---|---|---|---|---|---|---|---|---|---|---|
| oregon-white-oak | relief-off | bare-whole | 3 | 2.27 | 2.44 | 409 | 1.46 | - | 0.05 | 0.73 |
| oregon-white-oak | relief-off | bare-close | 3 | 1.77 | 3.66 | 273 | 1.72 | - | 0.05 | 1.75 |
| telperion | relief-off | bare-whole | 3 | 2.06 | 9.38 | 107 | 5.20 | - | 0.25 | 3.83 |
| telperion | relief-off | bare-close | 3 | 2.06 | 14.31 | 70 | 5.26 | - | 0.25 | 8.51 |
| laurelin | relief-off | bare-whole | 3 | 1.89 | 7.67 | 130 | 3.65 | - | 0.19 | 3.68 |
| laurelin | relief-off | bare-close | 3 | 1.62 | 12.16 | 82 | 4.08 | - | 0.19 | 7.77 |
| norway-spruce | relief-off | bare-whole | 3 | 1.52 | 5.87 | 170 | 4.48 | - | 0.26 | 1.02 |
| norway-spruce | relief-off | bare-close | 3 | 1.52 | 5.86 | 171 | 3.39 | - | 0.25 | 2.06 |
| european-beech | relief-off | bare-whole | 3 | 1.72 | 1.91 | 525 | 1.05 | - | 0.02 | 0.69 |
| european-beech | relief-off | bare-close | 3 | 1.72 | 2.89 | 346 | 1.19 | - | 0.02 | 1.54 |
| silver-birch | relief-off | bare-whole | 3 | 1.45 | 2.70 | 370 | 1.67 | - | 0.06 | 0.82 |
| silver-birch | relief-off | bare-close | 3 | 1.58 | 4.97 | 201 | 1.80 | - | 0.06 | 3.00 |
| oregon-white-oak | parallax-off | bare-whole | 3 | 1.21 | 11.07 | 90 | 1.50 | - | 0.05 | 9.39 |
| oregon-white-oak | parallax-off | bare-close | 3 | 1.17 | 29.66 | 34 | 1.78 | - | 0.05 | 27.56 |
| telperion | parallax-off | bare-whole | 3 | 2.28 | 10.31 | 97 | 5.21 | - | 0.25 | 4.67 |
| telperion | parallax-off | bare-close | 3 | 3.38 | 35.58 | 28 | 5.28 | - | 0.25 | 29.87 |
| laurelin | parallax-off | bare-whole | 3 | 3.17 | 8.65 | 116 | 3.67 | - | 0.19 | 4.62 |
| laurelin | parallax-off | bare-close | 3 | 3.31 | 36.28 | 28 | 4.11 | - | 0.19 | 31.57 |
| norway-spruce | parallax-off | bare-whole | 3 | 3.08 | 9.87 | 101 | 4.46 | - | 0.26 | 4.95 |
| norway-spruce | parallax-off | bare-close | 3 | 3.08 | 12.69 | 79 | 3.40 | - | 0.26 | 8.86 |
| european-beech | parallax-off | bare-whole | 3 | 3.44 | 4.29 | 233 | 1.06 | - | 0.02 | 3.05 |
| european-beech | parallax-off | bare-close | 3 | 3.75 | 8.90 | 112 | 1.21 | - | 0.02 | 7.53 |
| silver-birch | parallax-off | bare-whole | 3 | 3.64 | 2.99 | 334 | 1.69 | - | 0.06 | 1.10 |
| silver-birch | parallax-off | bare-close | 3 | 3.64 | 5.01 | 200 | 1.79 | - | 0.06 | 3.01 |

Bark relief is the wood's cost. Close, relief off takes the oak's wood from 31.3 to 1.8 ms, the spruce's from 9.8 to 2.1, the beech's from 7.5 to 1.5, Telperion's from 29.9 to 8.5 and Laurelin's from 31.5 to 7.8. Parallax is a small share of it: off, the oak's close wood is 27.6 ms against 31.3, and the Two Trees' does not move.

## The wood: cause and design (stated before code)

**Cause.** The shaded fragment shades a pixel as a grid of cells and reads the bark field (`bark_height`: the 27-site plate network, the 9-site column network, two banded scale walks and about ten filtered noise taps) at every corner of that grid: (cells+1)^2 reads, plus three for the sun walk (`bark_shade`), three for parallax and one plate identity. The cell count does not follow the pixel's need. A row with chipped plate edges (`plateEdgeShape` > 0: oak and spruce) always takes 4x4 cells, 25 reads; every other row takes at least 2x2, 9 reads, even where the pixel is a small fraction of a ridge width. Close, the oak's pixel is a few millimetres on a 32 mm ridge, so most of its 33 reads a fragment buy nothing: each cell is already read at a footprint far under the bound the grid exists to hold. The cost is linear in the reads: forcing one cell a pixel (an experiment, not the fix) takes the oak's close wood from 31.3 to 10.7 ms, the spruce's from 9.8 to 3.9 and Telperion's from 29.9 to 14.2.

**Design.** The cell count per axis is the fewest that keep every cell's footprint inside the widest cell footprint the present rule already shades: `cells = min(present, max(1, ceil(wide * present)))`, where `present` is today's count (2, or 3 once the pixel is a ridge width wide, or 4 for chipped edges) and `wide` is the pixel in ridge widths (or twice its plate band). So a cell is never read over more than half a ridge width (a quarter for chipped edges) where the present rule reads it over at most that, and the grid is unchanged wherever the pixel is a ridge width wide or more. Nothing else in the shader changes: the same field, the same lighting per cell, the same sun walk and parallax. It is the same picture at fewer reads, within the quadrature's own stated error; the bark tests and the stills check that.

**What it buys, measured with the rule in place (one run each, bare close):** oak 31.3 to 15.3 ms, spruce 9.8 to 7.9, beech 7.5 to 7.4, Telperion 29.9 to 29.7, Laurelin 31.5 to 31.4. The Two Trees' bark (a 2 mm ridge) puts one to six ridge widths in a close pixel, where the present grid is already the one the field needs, so a same-picture rule cannot make their wood cheaper. That is stated here as the limit of this fix; R3 reports what remains.

## The wood fix failed its same-picture check (stopped here for the host)

The rule above was built (`bark_cells` in `wood.wgsl`, a red/green test of the cell table) and put to the bark tests. `bark_resolution` compares a trunk drawn at 1600x1000 and box-reduced against the same trunk at 800x500. Under the rule the fine 1600x1000 pixels drop to one cell, and the comparison moves past its bounds:

| fixture | present rule, mean / p95 (of 255) | one-cell floor (the stated rule) | two-cell floor (chipped edges only reduced) | bound, mean / p95 |
|---|---|---|---|---|
| oak trunk | 1.91 / 6.75 | **3.09** / 9.75 | 2.08 / 7.00 | 3.0 / 12 |
| oak grazing | 2.40 / 7.00 | **3.29** / 9.25 | 2.64 / 7.00 | 3.0 / 12 |
| spruce grazing | 0.89 / 2.50 | 1.00 / 2.75 | 1.46 / 4.00 | 3.0 / 12 |

So the two-by-two floor is not idle quadrature: it is the bark's anti-aliasing at fine pixels, and taking it away changes the picture. The milder variant, which keeps the floor and cuts only the chipped-edge rows' 4x4 grid where the pixel is narrow, stays inside the bounds but still moves every fixture (the spruce's grazing mean by 64%). It is not the same picture either. It takes the oak's close wood from 31.3 to 18.4 ms (one run at load 24), which leaves the oak's close frame near 23 ms, about 43 fps.

Neither variant touches the Two Trees' wood (29.7 and 31.4 ms bare close). Their 2 mm ridge puts one to six ridge widths in a close pixel, where the dense grid is the one the field needs. With the relief off their close wood is still 8.5 and 7.8 ms.

Per the dispatch, the fix is now beyond "the shader cheaper for the same picture", so the build stops before code and goes to the host. Both variants were reverted (`177dbe4c`); the branch holds the probe and this record. Points 1 to 4 of the design (thinning, sun caching, draw on change, cached-pass timing) are not built.

## Host decision (2026-10-09): prefilter the relief instead of supersampling it

The shaded pass anti-aliases the bark by sampling it 9 to 25 times per pixel. The standard, cheaper answer, and the one that matches what a camera actually sees, is to band-limit the relief to the pixel's footprint:

- **Fade unresolvable detail.** Every octave, or ridge scale, of the bark field whose wavelength falls below about two pixels of the footprint fades to its own average, by degree as the footprint grows.
- **Keep the faded slopes' contribution.** The variance of the slopes those octaves carried goes into the shading roughness (Toksvig/LEAN-style), so the faded detail still darkens and spreads highlights as fine relief does, rather than going flat.
- **One shading cell per pixel.** Where every octave is resolved, the picture is unchanged. Where they are not, the relief is the band-limited version of the same field.

**The bark tests judge against ground truth, not against the old supersampler.** For each bark test view, the reference is the same frame rendered at 4× the resolution with the current shader and downsampled. The new shader must land within the existing bounds of that reference: mean ≤ 3.0 and the same p95 bound. Measure the old shader against it too, and report both. If the new shader is no closer than the old one, report that before changing any test.

**The other three design points do not depend on this, so build them now:**
- thinning by screen size with area kept;
- sun-side caching;
- drawing only on change.

Then measure every species again. If a species still falls short of 60 fps, report its remaining cost per pass and stop.

## After the host decision: three points built, the prefilter fails its truth check (stopped for the host)

### Built and committed

- **Sun cache** (`0ce78c7c`). The sun's wood is surfaced and its map is drawn only when the tree, the light, the view's casters (none, wood, or wood and crown) or the caster stride changed (`src/sun.rs`). On a timed frame that reuses the map, the shadow pass is opened empty so its timestamp pair is still written. The record counts that frame as exactly zero and flags it (`shadow_cached`), distinct from an unwritten pair. `tests/sun_cache.rs` checks the reuse decision across a camera move, Whole → Bare → Whole without a rebuild, a caster-stride change, a moved sun, a ground-only edit and a new tree. It also checks that the map after Whole → Bare → Whole is the first one, byte for byte, and that a frame on the reused map is byte-identical to the drawn one. `tests/timing.rs` covers the cached contract.
- **Draw on change** (`5194550c`). The page draws a frame only when the camera, tree, view, canvas size or scene changed (`src/redraw.rs`). A pending change survives the attempts skipped while a frame is in flight, and it clears only when a whole frame is submitted. A frame without its wood (programs still compiling) or a failed frame is drawn again. The redraw unit tests cover an idle canvas, a resize, a ground-only edit and a camera move during a frame in flight.
- **Thinning with area kept** (`b6735ca4`). Selection drops organs under one pixel across their bounding sphere. The share kept is the square of their size in pixels, keyed by a hash of each organ's three stored words. Kept organs are drawn enlarged by the inverse square root of that share, one rule for leaves and flowers. `tests/thinning.rs` checks a 48 px crown against the same crown at 192 px, box-reduced in linear light. 7,353 of 25,772 leaves are drawn, and the crown's linear colour change reads 0.0305 against the truth's 0.0261. Unthinned, the 48 px crown read 0.0295, so thinning moved coverage by 3.4% of itself.

### Per pass with the three points (one run each, the wood shader unchanged)

| species | variant | view | runs | load | frame ms | fps | surfacing | selection | shadow | vegetation |
|---|---|---|---|---|---|---|---|---|---|---|
| ordinary | full | whole | 1 | 1.70 | 0.78 | 1283 | 0.42 | 0.01 | 0.00 | 0.14 |
| ordinary | full | close | 1 | 1.70 | 1.06 | 939 | 0.51 | 0.01 | 0.00 | 0.39 |
| oregon-white-oak | full | whole | 1 | 11.71 | 13.58 | 74 | 0.80 | 0.02 | 0.00 | 12.54 |
| oregon-white-oak | full | close | 1 | 11.71 | 35.81 | 28 | 1.08 | 0.02 | 0.00 | 33.87 |
| norway-spruce | full | whole | 1 | 14.80 | 16.66 | 60 | 2.26 | 0.22 | 0.00 | 13.83 |
| norway-spruce | full | close | 1 | 14.80 | 14.99 | 67 | 1.21 | 0.20 | 0.00 | 13.16 |
| european-beech | full | whole | 1 | 13.61 | 4.06 | 246 | 0.55 | 0.01 | 0.00 | 3.32 |
| european-beech | full | close | 1 | 13.61 | 8.88 | 112 | 0.69 | 0.01 | 0.00 | 7.99 |
| silver-birch | full | whole | 1 | 13.33 | 2.37 | 421 | 0.87 | 0.01 | 0.00 | 1.33 |
| silver-birch | full | close | 1 | 13.33 | 4.41 | 227 | 0.97 | 0.01 | 0.00 | 3.27 |
| date-palm | full | whole | 1 | 13.33 | 1.05 | 950 | 0.38 | 0.01 | 0.00 | 0.53 |
| date-palm | full | close | 1 | 13.33 | 2.43 | 412 | 0.25 | 0.01 | 0.00 | 2.00 |
| telperion | full | whole | 1 | 6.63 | 107.47 | 9 | 2.89 | 0.44 | 0.00 | 103.13 |
| telperion | full | close | 1 | 6.63 | 74.78 | 13 | 2.95 | 0.38 | 0.00 | 70.88 |
| laurelin | full | whole | 1 | 4.33 | 61.83 | 16 | 2.06 | 0.28 | 0.00 | 58.89 |
| laurelin | full | close | 1 | 4.33 | 57.12 | 18 | 2.48 | 0.24 | 0.00 | 53.74 |

The shadow pass now reads 0.00 on every row: the static camera reuses the map, as an orbit does. Surfacing fell by the sun's half, for example Telperion from 5.2 to 2.9 ms. Thinning barely moved the Two Trees. **Their leaves are 1 to 3 px across at the whole view, not under a pixel**: dropping only the organs under 1 px (an experiment) left every Telperion leaf drawn and removed only flowers. A pixel threshold thins their flowers, and the leaves are the cost.

What a larger threshold would buy (an experiment, outside the stated rule, area kept; one run each):

| species | variant | view | runs | load | frame ms | fps | surfacing | selection | shadow | vegetation |
|---|---|---|---|---|---|---|---|---|---|---|
| telperion | T3.0 | whole | 1 | 2.98 | 26.28 | 38 | 2.91 | 0.42 | 0.00 | 22.32 |
| telperion | T3.0 | close | 1 | 2.98 | 75.62 | 13 | 2.97 | 0.38 | 0.00 | 69.21 |
| laurelin | T3.0 | whole | 1 | 2.21 | 18.16 | 55 | 2.06 | 0.27 | 0.00 | 15.14 |
| laurelin | T3.0 | close | 1 | 2.21 | 54.23 | 18 | 2.48 | 0.24 | 0.00 | 50.89 |
| telperion | T6.0 | whole | 1 | 1.76 | 13.68 | 73 | 2.91 | 0.42 | 0.00 | 9.56 |
| telperion | T6.0 | close | 1 | 1.76 | 52.86 | 19 | 2.97 | 0.38 | 0.00 | 48.66 |
| laurelin | T6.0 | whole | 1 | 1.37 | 10.47 | 96 | 2.07 | 0.26 | 0.00 | 7.55 |
| laurelin | T6.0 | close | 1 | 1.37 | 44.19 | 23 | 2.50 | 0.24 | 0.00 | 40.89 |

At a 6 px threshold Telperion's whole view reaches 73 fps and Laurelin's 96. The close views stay at 19 and 23 fps, because their wood is still about 30 ms. A 6 px threshold draws each leaf as a blob a few pixels across in place of 1 to 3 px leaves, which is a change to the picture and needs your call.

### The bark prefilter against ground truth

Tool: `examples/bark_truth.rs`. Every bark test view (the resolution, grazing, distance and smooth-bark fixtures, plus the harness's close pose for four species at 480x270) is drawn at its size and at 4x. The wood is surfaced for the smaller size in both, so only the shading differs. The comparison is over the wood the clay room paints, eroded 4 px, in mean / p95 code values. The truth in every column is the **present** shader at 4x, box-reduced.

| fixture | wood pixels | present shader | prefilter (stated design) | fade at 4 px | fade at 2 px + lost spread in the tints |
|---|---|---|---|---|---|
| oak-trunk | 203395 | 1.77 / 6.1 | 3.38 / 11.2 | 4.29 / 15.0 | 3.39 / 11.3 |
| oak-grazing | 260012 | 1.28 / 4.9 | 2.20 / 7.5 | 2.31 / 8.0 | 2.20 / 7.5 |
| spruce-grazing | 204351 | 1.97 / 8.1 | 3.33 / 11.5 | 3.57 / 12.1 | 3.33 / 11.5 |
| oak-2x | 103188 | 2.54 / 7.7 | 5.70 / 17.0 | 8.54 / 23.9 | 5.78 / 17.2 |
| oak-4x | 68194 | 4.90 / 23.0 | 7.14 / 24.9 | 7.21 / 25.1 | 7.20 / 24.1 |
| beech-base | 290741 | 0.98 / 3.9 | 1.21 / 4.7 | 1.21 / 4.7 | 1.39 / 5.1 |
| birch-bark | 130076 | 2.35 / 7.8 | 3.58 / 13.1 | 3.82 / 13.6 | 3.58 / 13.1 |
| oak-close | 102027 | 4.50 / 20.2 | 5.34 / 21.0 | 5.49 / 21.3 | 5.34 / 20.8 |
| spruce-close | 4523 | 11.68 / 34.9 | 12.10 / 35.0 | 12.10 / 35.0 | 12.36 / 35.0 |
| birch-close | 4322 | 14.64 / 46.3 | 14.64 / 46.3 | 14.64 / 46.3 | 14.64 / 46.3 |
| telperion-close | 53820 | 7.79 / 29.1 | 7.80 / 29.0 | 7.80 / 29.0 | 7.79 / 29.0 |

The prefilter as specified is one cell per pixel, each band faded to its mean between 4 and 2 px of wavelength, and the plate walls' lost slope variance added to the roughness term (patch: `raw/prefilter.patch`, gitignored). **It is farther from the truth than the present shader on every fixture.** The oak trunk goes from 1.77 to 3.38 mean (bound 3.0), and the oak at 2x from 2.54 to 5.70. Two variants did no better: fading sooner (4 to 8 px) is worse, and carrying the faded height's spread into the crest and fissure tints changes nothing. Its distance to its own 4x draw is almost the same as to the truth (oak trunk 3.28), so the gap is not a bias in the band means. It is the lighting's nonlinearity over relief that one band-limited sample cannot carry: the sun walk's furrow shadow, the rectified crest and fissure tints, and the sheen over the slopes. The roughness term carries only the last of these. Per the decision, the build stopped here, before changing any test, and the shader was reverted.

What the prefilter would buy, measured in the same build (bare views, one run, load about 21):

| species | variant | view | runs | load | frame ms | fps | surfacing | selection | shadow | vegetation |
|---|---|---|---|---|---|---|---|---|---|---|
| oregon-white-oak | full | bare-whole | 1 | 23.91 | 2.69 | 371 | 0.80 | - | 0.00 | 1.51 |
| oregon-white-oak | full | bare-close | 1 | 23.91 | 11.54 | 87 | 1.06 | - | 0.00 | 10.23 |
| norway-spruce | full | bare-whole | 1 | 23.27 | 3.73 | 268 | 2.26 | - | 0.00 | 1.12 |
| norway-spruce | full | bare-close | 1 | 23.27 | 4.22 | 237 | 1.21 | - | 0.00 | 2.67 |
| european-beech | full | bare-whole | 1 | 23.27 | 1.40 | 712 | 0.55 | - | 0.00 | 0.70 |
| european-beech | full | bare-close | 1 | 23.27 | 2.43 | 411 | 0.69 | - | 0.00 | 1.59 |
| silver-birch | full | bare-whole | 1 | 21.97 | 1.86 | 537 | 0.86 | - | 0.00 | 0.84 |
| silver-birch | full | bare-close | 1 | 21.97 | 4.03 | 248 | 0.96 | - | 0.00 | 2.91 |
| telperion | full | bare-whole | 1 | 20.85 | 6.93 | 144 | 2.89 | - | 0.00 | 3.82 |
| telperion | full | bare-close | 1 | 20.85 | 11.92 | 84 | 2.93 | - | 0.00 | 8.70 |
| laurelin | full | bare-whole | 1 | 19.82 | 6.02 | 166 | 2.05 | - | 0.00 | 3.69 |
| laurelin | full | bare-close | 1 | 19.82 | 10.67 | 94 | 2.47 | - | 0.00 | 7.94 |

The oak's close wood drops from 31.3 to 10.2 ms and Telperion's from 29.9 to 8.7, which is about the relief-off cost. So the speed is there; the picture is not.

### Where each species stands (with the three points, present wood shader)

| species | whole fps | close fps | what the short view still costs |
|---|---|---|---|
| ordinary, beech, birch, date palm | 246 to 1283 | 112 to 939 | meets 60 |
| oak | 74 | **28** | wood 31 ms of the 34 ms vegetation pass; surfacing 1.1 |
| spruce | **60** (16.7 ms) | 67 | vegetation 13.8 (wood 5.3, needles 8.5); surfacing 2.3; selection 0.2 |
| Telperion | **9** | **13** | whole: leaves about 98 ms, wood 4.7, surfacing 2.9. Close: wood about 30, leaves and flowers about 40, surfacing 3.0 |
| Laurelin | **16** | **18** | whole: leaves about 54 ms, wood 4.6, surfacing 2.1. Close: wood about 31, leaves and flowers about 22, surfacing 2.5 |

Two decisions are open for you:
1. **The wood.** Either a prefilter that carries the lighting's nonlinearity rather than the relief's mean (the furrow shadow and tints as functions of the faded variance, which needs fitting against the truth tool), or accepting the prefilter's distance as the bark's new picture.
2. **The leaves.** Whether the thinning threshold may rise past a pixel (3 to 6 px) for the Two Trees.

The gate, npm test and impl-review were not run, because the build stopped.

## Host decision, second (2026-10-09): both answers come from the truth tool

Your truth tool (`bark_truth.rs`) is the right instrument. Both open questions are settled by fitting against it, not by taste.

1. **Wood: a prefilter that carries the lighting.**
   - **Where the gap is.** The measurement shows the remaining distance comes from the lighting's nonlinearity over the relief: furrow shadowing and the crest and fissure tints, not the band averages.
   - **What to add.** Give the band-limited relief two more terms, each a function of the faded octaves' height and slope variance:
     - an expected furrow occlusion that darkens with the faded variance;
     - an expected tint mix between crest and fissure.
   - **How to fit them.** Fit their coefficients once against the truth tool on the oak and spruce fixtures, then check them on held-out fixtures: beech, birch, Telperion and Laurelin trunks, plus the 2x distance views. These are shader constants with their derivation recorded, not species rows.
   - **Acceptance.** On every fixture the new shader must be within the existing bounds of the truth (mean ≤ 3.0, p95 bound). It must also be no farther from the truth than the present shader by more than 0.3 mean.
   - **If it misses,** stop and report with the table. The picture then goes to the owner.
2. **Leaves: the threshold is fitted, not fixed at one pixel.**
   - **The reference.** The same tool renders each crown view thinned and unthinned at 4x, box-reduced. The unthinned 4x render is the truth.
   - **The threshold.** Choose the largest threshold whose thinned 1x frame stays within a stated bound of that truth on covered pixels and mean colour, at the whole and mid views of every flowering and dense species. Use the bark tests' 3.0 mean as the starting bound, and state the one used.
   - **Scope.** This is one rule for every species. Its value comes from that fit, not from a per-species choice.
   - **Viewing.** Every still is viewed beside its truth.

The three points already built stay as they are. Then measure every species at 60 fps. If one still falls short, report its cost per pass and stop.

## After the second decision: the wood terms cannot meet the acceptance; leaves fitted at 2 px (stopped for the host)

### Wood: why expected-value terms cannot close the gap

Before fitting the two terms, I measured what any such term could reach. `bark_truth` now also reports the signed bias, the mean of (1x minus truth). An expected occlusion or tint term driven by the faded variance can move only that part of the error: it is the same function of the footprint at every pixel of a neighbourhood. The bound below is an oracle stronger than any such term. It takes the error image, subtracts its own 9x9 local mean (every smooth bias, wherever it lies) and reports what is left (`raw/oracle.py`, numpy over the stills).

| fixture | present shader: mean / bias | prefilter: mean / bias | prefilter with every 9 px bias removed (oracle) | acceptance: ≤ 3.0 and ≤ present + 0.3 |
|---|---|---|---|---|
| oak-trunk | 1.78 / +0.15 | 3.38 / +0.41 | **3.21** | 2.08 |
| oak-grazing | 1.29 / +0.06 | 2.22 / -0.02 | **2.10** | 1.59 |
| spruce-grazing | 1.97 / +0.18 | 3.32 / -0.63 | **3.06** | 2.27 |
| oak-2x | 2.54 / -0.11 | 5.69 / +0.44 | **5.60** | 2.84 |
| beech-base | 0.98 / -0.61 | 1.21 / -0.41 | 1.03 | 1.28 |
| birch-bark | 2.36 / +0.58 | 3.59 / +0.57 | **3.54** | 2.66 |

The prefilter's error is not bias: its signed mean is under 0.7 of a code value on every fixture. It is per-pixel sampling noise. One band-limited sample cannot see the sharp plate walls and furrow edges that the truth's 16 samples average, even where the band is resolved (λ ≥ 4 px), and that is the regime the oak trunk and the 2x view are in. Its distance to its own 4x draw is the same as to the truth's (oak trunk 3.28). Even the oracle misses the acceptance on five of six fixtures. So the occlusion and tint terms were not fitted: they would have been fitted to a gap they cannot reach. **The wood misses. Per the decision, the picture goes to the owner.**

### Leaves: the threshold fitted against a 4x crown truth

Tool: `examples/crown_truth.rs`. Each species is drawn at the harness whole view and at a mid view (half the distance), at 1920x1080 and, unthinned, at 4x box-reduced. The truth is that 4x draw. The comparison is over the pixels the crown changes in the truth (whole against bare, more than 8 code values), in mean code values.

**The bound used.** The unthinned 1x crown is itself 2.4 to 15 code values from its own 4x truth: MSAA against supersampling over 1 to 3 px leaves. So an absolute 3.0 cannot be met by any threshold, including none. The bound used is the bark tests' 3.0 mean as an **increase**: a thinned view may be at most 3.0 mean farther from the truth than the unthinned view is. The table gives that increase per threshold; bold breaks the bound.

| species | view | unthinned (T=0) | 1 px | 2 px | 3 px | 4 px | 6 px |
|---|---|---|---|---|---|---|---|
| date-palm | mid | 5.00 | +0.00 | +0.00 | +0.00 | +0.00 | +0.00 |
| date-palm | whole | 9.67 | +0.00 | +0.00 | +0.00 | +0.00 | +0.00 |
| european-beech | mid | 2.96 | +0.00 | +0.00 | +0.00 | +0.00 | +0.00 |
| european-beech | whole | 5.39 | +0.00 | +0.00 | +0.00 | +0.00 | +0.00 |
| laurelin | mid | 9.86 | +0.00 | +0.12 | +1.18 | +2.57 | **+5.29** |
| laurelin | whole | 10.91 | +0.08 | +1.39 | **+4.50** | **+8.18** | **+12.92** |
| norway-spruce | mid | 12.71 | +0.00 | +0.00 | +0.07 | +0.37 | +1.23 |
| norway-spruce | whole | 12.17 | +0.00 | -0.01 | +0.11 | +0.39 | +1.03 |
| ordinary | mid | 4.99 | +0.00 | +0.00 | +0.00 | +0.00 | +0.00 |
| ordinary | whole | 7.03 | +0.00 | +0.00 | +0.00 | +0.00 | +0.00 |
| oregon-white-oak | mid | 2.44 | +0.00 | +0.00 | +0.00 | +0.00 | +0.00 |
| oregon-white-oak | whole | 3.47 | +0.00 | +0.00 | +0.00 | +0.00 | +0.00 |
| silver-birch | mid | 10.34 | +0.00 | +0.00 | +0.00 | +0.00 | +0.05 |
| silver-birch | whole | 10.47 | +0.00 | +0.00 | +0.01 | +0.60 | **+5.70** |
| telperion | mid | 11.82 | +0.00 | +0.02 | +0.61 | +2.41 | **+9.72** |
| telperion | whole | 15.18 | +0.03 | +2.96 | **+9.45** | **+15.30** | **+22.36** |

**2 px is the fit**: the largest tried threshold within the bound on every view. Telperion's whole view binds at +2.97 (+9.5 at 3 px). It is now `THIN_PIXELS` (`5fd63ef8`), one rule for every species. Stills viewed, truth beside the unthinned, 2 px and 3 px 1x draws, crown crops of the whole view for Telperion and Laurelin (`raw/telperion-whole.png`, `raw/laurelin-whole.png`). At 2 px both crowns read as their truth in density and colour. At 3 px Laurelin's crown is a shade yellower and blotchier, which is what the table's +4.5 shows.

### Where the species stand (sun cache, draw on change, 2 px thinning, present wood; one run, load 2.7 to 5.4)

| species | variant | view | runs | load | frame ms | fps | surfacing | selection | shadow | vegetation |
|---|---|---|---|---|---|---|---|---|---|---|
| oregon-white-oak | full | whole | 1 | 2.73 | 13.58 | 74 | 0.80 | 0.02 | 0.00 | 12.29 |
| oregon-white-oak | full | close | 1 | 2.73 | 34.60 | 29 | 1.08 | 0.02 | 0.00 | 33.75 |
| norway-spruce | full | whole | 1 | 4.03 | 12.99 | 77 | 2.27 | 0.21 | 0.00 | 10.01 |
| norway-spruce | full | close | 1 | 4.03 | 14.60 | 68 | 2.49 | 0.19 | 0.00 | 14.08 |
| telperion | full | whole | 1 | 3.94 | 47.83 | 21 | 2.89 | 0.43 | 0.00 | 42.11 |
| telperion | full | close | 1 | 3.94 | 74.47 | 13 | 2.95 | 0.38 | 0.00 | 70.25 |
| laurelin | full | whole | 1 | 5.39 | 29.79 | 34 | 2.06 | 0.28 | 0.00 | 26.65 |
| laurelin | full | close | 1 | 5.39 | 56.84 | 18 | 2.48 | 0.24 | 0.00 | 53.45 |

Ordinary, beech, birch and date palm were at 112 fps or more before this threshold, and the threshold only removes work from them.

| species | short of 60 | cost per pass in the short view |
|---|---|---|
| oak | close 29 fps | vegetation 33.8 ms (wood about 31, leaves about 3); surfacing 1.1; selection 0.02; shadow cached |
| Telperion | whole 21, close 13 | whole: vegetation 42.1 (leaves and flowers about 37, wood 4.7); surfacing 2.9; selection 0.4. Close: vegetation 70.3 (wood about 30, leaves and flowers about 40); surfacing 3.0; selection 0.4 |
| Laurelin | whole 34, close 18 | whole: vegetation 26.7 (leaves and flowers about 22, wood 4.6); surfacing 2.1; selection 0.3. Close: vegetation 53.5 (wood about 31, leaves and flowers about 22); surfacing 2.5; selection 0.2 |
| spruce | none: 77 whole, 68 close | |

So the oak's and the Two Trees' close views wait on the wood, which is the owner's picture decision. The Two Trees' whole views wait on leaves: 2 px, the most the crown truth allows, leaves them at 21 and 34 fps. The gate, npm test and impl-review were not run.

## Owner decision (2026-10-09)

The owner chose from the host's options:
- On the wood, "b and c sound sgood". (b): the prefiltered bark ships in this release. Its measured distance to the 4x truth is its accepted picture. (c): bark computed into a texture when each tree is built, at runtime, mipmapped and filtered by the hardware, follows after the release as its own spec.
- On the Two Trees' leaves, "2a is gud". Laurelin and Telperion get larger leaves, so they carry fewer of them. That is a values change in their own specs, judged on stills.

For this spec, that means:
- **Restore the prefilter** from `raw/prefilter.patch`.
- **Re-pin the bark tests.** Set their bounds to the prefilter's measured distance to the truth, plus a stated margin. Record each fixture's truth distance for the present and the prefiltered shader beside the bounds, and cite this decision.
- **Re-measure every species** at 60 fps, whole and close. The Two Trees are re-measured after their leaf change lands.
