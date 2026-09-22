> Historical raw-output references: see the [archive and recovery instructions](README.md).

# Owner visual verdict

On 2026-09-20 the owner viewed the side-by-side seed-1 oak and spruce CPU/GPU hero images and said, "they look the same to me".

This accepts no visible regression in those four 1280x720 views of the first native GPU foliage candidate. Files are `gpu-hero-oregon-white-oak-cpu.png`, `gpu-hero-oregon-white-oak-gpu.png`, `gpu-hero-norway-spruce-cpu.png` and `gpu-hero-norway-spruce-gpu.png`. Capture provenance is retained beside them. They predate the signed-zero extrema edge-case correction.

This verdict does not establish attachment close-up parity, other seeds, other species, browser or phone behavior. Performance, memory, geometry and repeatability checks remain separate. It does not close fn-91.

## Browser views and measured pixel differences accepted

On 2026-09-20 the owner also assessed the seed-1 browser close views as looking exactly the same. After receiving the decoded RGBA pixel comparison below, the owner said, "ok to me that's fine". This accepts the observed differences in the shown CPU/GPU pairs, including the browser close views.

Each image contains 921600 pixels. Direct comparison used ImageMagick to decode each PNG to 8-bit RGBA and counted pixels with any unequal channel:

| Pair | Different pixels | Percentage |
|---|---:|---:|
| Browser oak close | 4633 | 0.502713% |
| Browser spruce close | 6004 | 0.651476% |
| Native oak hero | 3401 | 0.369032% |
| Native spruce hero | 10070 | 1.092665% |

Acceptance is scoped to these views. It does not assert byte equality or close the remaining geometry, motion, other-seed/device, performance or memory requirements. See BROWSER-VISUAL.md for capture limitations and provenance.
