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
