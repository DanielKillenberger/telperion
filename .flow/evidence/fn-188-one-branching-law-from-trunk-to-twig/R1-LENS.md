# fn-188 R1: the judging lens, 2026-10-02

## Setup

- **Branch:** `fn-188-one-branching-law-from-trunk-to-twig` at `1f79831d`, with its own `target/`.
- **Subject:** the shipped European beech, seed 1, bare view, on an NVIDIA GeForce RTX 3080.
- **Exploratory code**, never merged:
  - `FN188_NO_TWIGS` skips the twig layer's growth (`specimen.rs`, the `local.advance` call). It was committed as `84b38070` and reverted in `b04847d2`.
  - A scratch example, `crates/telperion-render/examples/lens.rs`, kept out of the tree. It renders through the same `Renderer`, `render` and `write_png` the headless stills use.

## Sample count

**Headless captures run at 4x MSAA on this machine.** `Renderer::samples()` reads 4 here. The headless still prints it on every capture, for example "960x720 on NVIDIA GeForce RTX 3080 at 4 samples a pixel". The lens tool printed `samples 4` too. The count comes from `pass::samples` (`pass.rs:22`), which gives `MULTISAMPLE` = 4 when the colour and depth formats both take it; the frame resolves into a single-sample texture (`headless.rs:59`).

## Fixed pose

- **The rule:** the shipped beech's seed-1 hero pose, solved once from its own bounds at 960x720 and then pinned for every render.
- **The pose:**
  - position (56.622, 33.141, −26.335)
  - target (0.102, 18.267, 0.021)
  - vertical field of view 38°, near 0.1, far 864.1
- **The file:** `raw/pose.json`, as [x, y, z, target x, target y, target z, fov, near, far]. A candidate is rendered by passing this file.

## The renders

Same geometry, lighting and pose throughout.

- **native:** 960x720.
- **ss4:** 3840x2880, each 4x4 block averaged in linear light (sRGB decoded, averaged, re-encoded) to 960x720. Each output pixel therefore carries 16 pixels of 4 MSAA samples.
- **crop:** 960x720 from the same position, aimed at an upper limb: node 281 at (0.97, 22.41, 1.70), radius 0.108 m, the thickest scaffold node between 0.7 and 0.8 of the height. Its field of view is 9.84°, a quarter of the tangent, so it has the supersampled render's pixel density.
- **notwigs:** the same tree without the twig layer, rendered native and ss4 from the same pose.

## Coverage

- **Region:** the crown's on-screen box, the projection of every scaffold node at or above the crown base: x 201–750, y 48–604, 305,800 pixels.
- **Measure:** linear relative luminance (Rec. 709).
- **Threshold:** a pixel counts as changed when its difference exceeds 5% of the brighter of the two.

| Comparison | Mean abs ΔL | Mean ΔL | Pixels > 5% |
|---|--:|--:|--:|
| native vs ss4 | 0.0155 | −0.0052 | 39.8% |
| notwigs native vs notwigs ss4 (control, no twigs) | 0.0042 | −0.0003 | 17.5% |
| twig tone at ss4 (ss4 − notwigs ss4) | 0.0270 | −0.0181 | 39.7% |
| twig tone at native (native − notwigs native) | 0.0327 | −0.0229 | 39.1% |

**Mean crown luminance:**

| Render | With twigs | Without twigs |
|---|--:|--:|
| native | 0.2298 | 0.2528 |
| ss4 | 0.2350 | 0.2531 |

**What the numbers say:**
- **The twigs darken the crown,** by 0.018 in mean luminance at ss4. That is 7% of the twigless crown's 0.253.
- **Native carries more twig tone than ss4, not less:** −0.0229 against −0.0181, about 27% more.
- **The differences come almost entirely from the twigs.** Without twigs, native and ss4 agree in mean (−0.0003). With twigs, native is darker by 0.0052.
- **The 39.8% of pixels that differ** are mostly per-pixel placement: the mean signed difference is small against the mean absolute one. The no-twig control still differs on 17.5% of pixels, from the edges of the thicker wood.

## Statement

**At the native 960x720 still with 4x MSAA, the fine twigs are not lost.** The native render carries all of the twig layer's tone that the 4x-supersampled render shows, about 27% more on average, rather than less. Its pixels place that tone differently, on 40% of crown pixels. The upper-limb crop at the supersampled density resolves the twigs as discrete strands along the limbs.

## Cost

These are scratch measurements on the RTX 3080, warm, including readback to the host.

| Step | Cost |
|---|--:|
| Native 960x720 render | 1–3 ms |
| 3840x2880 render | 29–32 ms |
| Crop render | 14 ms |
| Linear-light downsample (scratch CPU loop, unoptimised `powf`) | 484–498 ms |
| Generation and mesh, with twigs (cold, `mesh::build`) | 3,318 ms |
| Generation and mesh, without twigs (cold, `mesh::build`) | 280 ms |

## Images

Under `raw/stills/`:
- `beech-lens-native-s1-bare.png`
- `beech-lens-ss4-s1-bare.png`
- `beech-lens-crop-s1-bare.png`
- `beech-lens-notwigs-s1-bare.png`, the ss4 render without twigs
- `beech-lens-notwigs-native-s1-bare.png`, kept as measurement input
- `raw/stills/crown-bbox.json`, the crown box

Two were opened: ss4 and the crop.
