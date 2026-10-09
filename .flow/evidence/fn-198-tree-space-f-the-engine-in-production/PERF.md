# Telperion harness performance, fn-198 at 1f9afd53

Measured on 2026-10-09 on an RTX 3080 (Vulkan, NVIDIA 610.57.04) with a 32-core host. Every run used RAYON_NUM_THREADS=8, a warm driver shader cache with `__GL_SHADER_DISK_CACHE_SIZE=4000000000`, and the GPU lock. The 1-minute load average ranged from 0.7 to 7.5 at run starts and was mostly 1.5 to 3.5; the only other GPU users were desktop apps. Each figure is the best of 3 processes. The probes are scratch code in `bench/` (`src/bin/frames.rs` covers frames, `src/main.rs` covers builds and memory). Both drive the harness's GPU route natively: `Generator::prepare(Resident)`, then `submit_prepared`, then `Renderer::draw`, which is the same path `setTreeGpu` and `frame()` take in `src/web.rs`. These are native numbers, not browser ones. Chrome adds wasm overhead, and in the browser the CPU stages run on a single thread.

## Per-frame cost (owner's focus)

1920x1080 at 4x MSAA, 60 warm frames at each view. "Whole" is the harness's hero pose. "Close" is the same pose at 25% of the distance (wheel in). GPU pass times are timestamp p50 from `measure()`:
- **surfacing** is the wood curve compute for the camera view and the sun view together.
- **selection** is foliage LOD selection.
- **shadow** is the sun depth pass.
- **wood draw** is the bare view's vegetation pass at the same camera. It holds only the wood (a depth prepass plus a shaded pass).
- **leaf draw** is the whole view's vegetation pass minus the wood. It includes the flower (organ) draw.
- **room + rest** is the pipelined frame minus the timed passes. It covers the untimed room pass, the untimed organ selection, the MSAA resolve and the present. A slightly negative value is overlap between the measured passes.

**GPU frame ms** is wall time per frame with 120 frames pipelined. The device is the bottleneck in every case, so this time is GPU time. **CPU encode** is the host time for one `draw()` call (encode plus submit).

Triangle counts are as drawn. Wood is the device's tessellated count. Leaf triangles are instances per LOD level multiplied by that level's triangles. The renderer's own `FrameStats.triangles` reports finest-level triangles times all instances, which is an upper bound and not what is drawn: it reads 2.1 billion for telperion. Telperion and laurelin also draw 10.4M and 8.6M flower instances, which the "instances drawn" column leaves out.

| species | view | runs | CPU encode ms | GPU frame ms (pipelined) | fps | surfacing | selection | shadow | wood draw | leaf draw | room + rest | wood tris (camera) | leaf tris drawn | instances drawn | leaves never drawn | sun wood tris | sun leaf casters |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ordinary | whole | 3 | 0.13 | 1.18 | 850 | 0.82 | 0.008 | 0.02 | 0.11 | 0.02 | 0.20 | 133,168 | 40,860 | 7,851 | 0 | 100,276 | 1,963 |
| ordinary | close | 3 | 0.13 | 1.45 | 687 | 0.90 | 0.008 | 0.02 | 0.32 | 0.08 | 0.14 | 292,440 | 113,520 | 3,900 | 3,951 | 100,276 | 1,963 |
| oregon-white-oak | whole | 3 | 0.17 | 14.07 | 71 | 1.47 | 0.020 | 0.19 | 10.53 | 1.73 | 0.14 | 674,572 | 7,915,308 | 343,180 | 0 | 457,984 | 85,795 |
| oregon-white-oak | close | 3 | 0.20 | 35.69 | 28 | 1.76 | 0.019 | 0.19 | 30.90 | 2.85 | -0.04 | 950,900 | 5,810,328 | 107,662 | 235,518 | 457,984 | 85,795 |
| norway-spruce | whole | 3 | 0.19 | 20.98 | 48 | 4.43 | 0.213 | 2.34 | 5.31 | 8.49 | 0.20 | 2,418,768 | 36,789,480 | 4,598,685 | 0 | 2,245,492 | 1,149,672 |
| norway-spruce | close | 3 | 0.20 | 19.14 | 52 | 3.37 | 0.193 | 2.35 | 9.54 | 3.49 | 0.21 | 1,331,380 | 14,681,016 | 1,835,127 | 2,763,558 | 2,245,492 | 1,149,672 |
| european-beech | whole | 3 | 0.16 | 4.62 | 217 | 1.05 | 0.010 | 0.03 | 3.05 | 0.27 | 0.19 | 203,664 | 793,860 | 66,155 | 0 | 155,356 | 16,539 |
| european-beech | close | 3 | 0.20 | 9.50 | 105 | 1.19 | 0.009 | 0.03 | 7.43 | 0.54 | 0.29 | 336,988 | 1,408,176 | 32,004 | 34,151 | 155,356 | 16,539 |
| silver-birch | whole | 3 | 0.15 | 3.26 | 307 | 1.67 | 0.013 | 0.10 | 1.13 | 0.19 | 0.16 | 535,572 | 636,856 | 159,214 | 73 | 497,304 | 39,822 |
| silver-birch | close | 3 | 0.19 | 5.29 | 189 | 1.78 | 0.013 | 0.10 | 2.99 | 0.27 | 0.15 | 706,460 | 769,288 | 58,430 | 100,857 | 497,304 | 39,822 |
| date-palm | whole | 3 | 0.13 | 1.40 | 716 | 0.69 | 0.008 | 0.02 | 0.45 | 0.07 | 0.16 | 37,608 | 67,648 | 6,554 | 0 | 30,240 | 1,639 |
| date-palm | close | 3 | 0.16 | 2.75 | 364 | 0.58 | 0.008 | 0.02 | 1.78 | 0.21 | 0.15 | 41,616 | 171,552 | 3,283 | 3,271 | 30,240 | 1,639 |
| telperion | whole | 3 | 0.25 | 120.69 | 8 | 5.21 | 0.433 | 3.03 | 4.69 | 106.24 | 1.08 | 3,232,720 | 36,211,048 | 9,052,762 | 0 | 2,306,904 | 2,263,191 |
| telperion | close | 3 | 0.25 | 79.74 | 13 | 5.28 | 0.381 | 3.03 | 29.93 | 40.60 | 0.52 | 3,640,476 | 14,741,356 | 1,988,403 | 7,064,359 | 2,306,904 | 2,263,191 |
| laurelin | whole | 3 | 0.35 | 89.03 | 11 | 3.65 | 0.273 | 2.72 | 4.63 | 76.54 | 1.22 | 2,612,804 | 24,504,712 | 6,126,178 | 0 | 1,738,700 | 1,531,545 |
| laurelin | close | 3 | 0.28 | 61.97 | 16 | 4.08 | 0.237 | 2.74 | 31.39 | 22.58 | 0.95 | 2,703,440 | 13,044,108 | 1,110,279 | 5,015,899 | 1,738,700 | 1,531,545 |


### Frame-rate headline

| Range | Species and views |
|---|---|
| Fine (200 fps and up) | ordinary, date-palm, birch (whole and close), beech whole |
| Marginal | beech close at 105 fps, oak whole at 71 fps |
| Under 60 fps | oak close 28, spruce 48 to 52, **laurelin 11 to 16**, **telperion 8 to 13** |

CPU encode is 0.13 to 0.35 ms everywhere, so the CPU is not the problem. Every slow case is GPU-bound.

## The harness redraws every animation frame

- **Yes, it redraws every animation frame when nothing changes.** `harness/rust-stage.ts:102-106` calls `renderer.frame()` on every `requestAnimationFrame` with no dirty check. `WebRenderer::frame` (`crates/telperion-render/src/web.rs:307-322`) skips only while the previous frame is still on the device. So on a GPU-bound tree the GPU stays 100% busy redrawing an identical picture, and every pointer event waits behind a 60 to 120 ms frame. Telperion idles at about 8 fps.
- Every frame runs the full `draw_with` (`crates/telperion-render/src/lib.rs:466-571`):
  - **Wood surfacing compute for the camera and the sun** (`lib.rs:504` into `curve.rs:184`): 0.6 to 5.3 ms per frame. The sun half depends only on the light and the tree. The camera half depends only on the camera and the tree. It runs even in the single-leaf view, which draws no wood: beech 1.6 ms, oak 2.1 ms in that view.
  - **Foliage selection compute** (`lib.rs:512`) and **flower selection compute** (`lib.rs:520`, untimed): up to 0.43 ms. These depend on the camera and the tree.
  - **Shadow map** (`lib.rs:523`): up to 3.0 ms for telperion, laurelin and spruce. It depends only on the light and the tree, so it could be drawn once per tree and sun.
  - **The room and vegetation passes**, which are needed only when the camera, the view, the scene or the tree changes.
- With a static camera and tree, everything above is redundant. A dirty flag on the camera, tree, view, scene and size would take an idle frame to zero. Caching the sun's surfacing and the shadow map would cut 1.5 to 8 ms from every orbit frame for spruce, telperion and laurelin.
- `harness/GrowerDev.tsx:240` polls `stats()` every 250 ms and sets fresh React state each time. That re-renders the whole panel four times a second. It is a CPU cost, small next to the GPU costs.

## The three biggest per-frame costs

1. **Leaf and flower draw at the whole view for the dense crowns: telperion 106 ms, laurelin 77 ms, spruce 8.5 ms.**
   - Where: `crates/telperion-render/src/lib.rs:566-568` (`foliage.draw` and `organ.draw`) into `crates/telperion-render/src/foliage.rs:271`.
   - Telperion draws 9.05M leaves plus 10.4M flowers at the whole view, about 9 instances per pixel of the 2.07M-pixel frame. Laurelin draws 6.1M plus 8.6M.
   - Every leaf is already at the coarsest level (4 triangles), and LOD has no level below that. Nothing is culled at the whole view: the never-drawn count is 0.
   - Leaves and flowers were not separated. The flowers have their own untimed selection, so their share of the 106 ms is unknown.
2. **Wood draw at close views: oak 31 ms, telperion 30, laurelin 31, spruce 9.5, beech 7.4 (oak whole 10.5).**
   - Where: `crates/telperion-render/src/curve/draw.rs:110-113` draws the wood twice, a depth prepass and then a shaded pass. The shaded fragment runs the bark relief and parallax at `crates/telperion-render/src/shaders/wood.wgsl:294` (`bark_parallax`, plus plates and smooth).
   - The cost follows screen coverage and bark type, not triangle count. Oak whole draws 0.67M triangles in 10.5 ms, while birch draws 0.54M in 1.1 ms. Close views draw only 0.95 to 3.6M triangles yet take about 30 ms.
   - **Unverified:** that the bark shader is the cause is my inference. It was not isolated by turning the bark off.
3. **Fixed passes recomputed every frame: surfacing 3.4 to 5.3 ms and shadow 2.3 to 3.0 ms on spruce, telperion and laurelin.**
   - Where: `lib.rs:504` into `curve.rs:184` (camera and sun surfacing), and `lib.rs:523` (shadow).
   - This is 6 to 8 ms per frame that a still or orbiting camera does not need for the sun half. With an idle redraw (`rust-stage.ts:102`) it is spent continuously.

## Build, rebuild and memory (measured before the owner said build time is fine; one process each)

Build is measured from parameters to the first drawn frame with the device idle. "Programs" is `prepare_programs` (the harness's `preparePrograms`) on a warm cache.
- If the programs are not prepared first, the first build in a process paid about 20 s compiling the fine-root kernels (beech).
- **The generator keeps no specimen state between builds.** `Generator` caches only pipelines and pools (`generation.rs:144-146`). Any one-row change, the seed included, is a full rebuild, and the table times it as one.

Memory is the device allocator report after 6 builds in the process. It includes the generator's pools and the drawn tree's buffers. The tree buffers are wood plus foliage.

| species | age | runs | programs ms | cold build ms | build ms (best warm) | prepare ms | first frame ms | rebuild seed change ms | foliage inst | tree buffers MB | device allocated MB (empty) | device reserved MB |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ordinary | 40 | 1 | 68 | 572 | 542 | 524 | 10.6 | 548 | 7,851 | 677 | 965 (77) | 1280 |
| oregon-white-oak | 40 | 1 | 65 | 1369 | 1206 | 1165 | 24.4 | 1296 | 343,180 | 766 | 1082 (77) | 1536 |
| norway-spruce | 40 | 1 | 67 | 437 | 368 | 319 | 37.5 | 351 | 4,598,685 | 1498 | 3810 (77) | 4774 |
| european-beech | 40 | 1 | 68 | 190 | 160 | 140 | 13.1 | 194 | 66,155 | 684 | 2370 (77) | 3192 |
| silver-birch | 40 | 1 | 65 | 742 | 709 | 691 | 11.8 | 747 | 159,287 | 882 | 2631 (77) | 3611 |
| date-palm | 40 | 1 | 65 | 11 | 6 | 4 | 1.9 | 6 | 6,554 | 107 | 402 (77) | 768 |
| telperion | - | 0 | failed |
| laurelin | 80 | 1 | 68 | 723 | 616 | 500 | 104.0 | 613 | 6,126,178 | 1700 | 4495 (77) | 6054 |



- **Telperion fails this probe natively with `DeviceRefused { experimental generation: Out of Memory }`** on the 10 GB card. The probe builds repeatedly and holds the drawn tree while the next one builds. A single build, as in the frame probe, succeeds.
- Laurelin holds 4.5 GB allocated and 6.1 GB reserved. Spruce holds 3.8 GB and 4.8 GB.
- Wood surfacing targets dominate the tree buffers: beech's curve is 713 MB.
