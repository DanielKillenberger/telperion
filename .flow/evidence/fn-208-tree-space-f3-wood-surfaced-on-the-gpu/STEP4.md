# fn-208 (iv) and step 5: the GPU passes, ribbons, R3 stills and R4 (worker, 2026-10-05)

## Built

**The CPU reference** (`curve/tessellate.rs`, `tessellate/plan.rs`) is now planned and drawn cluster by cluster, in the order and layout the GPU writes:
- **Frustum culling:** a cluster outside the view's planes is not drawn.
- **Orthographic views,** for the sun.
- **Neighbours:** clusters share identical end rings.
- **Tube or ribbon per ring:**
  - A ring of ρ ≥ `RIBBON` (1 px) belongs to a tube. A stretch between two tube rings is drawn as a tube's strip; any other as a ribbon's quad.
  - A tube is capped where it ends, at a ribbon or at a cluster's end.
  - Ribbons carry three vertices a ring: the two silhouette edges and the middle. Each has the tube's normal and bark angle there, so a ribbon shades as the half of the tube that faces the eye.
- **Budget:** a `Budget` caps vertices, tube indices and ribbon indices separately.

**The GPU passes** (`crates/telperion-render/src/curve.rs`, `curve/draw.rs`, `shaders/curve.wgsl`, `curve_walk.wgsl`):
- **Upload, once:** the 32-byte points, the packed clusters (16 words) and the cells (28 floats).
- **Per view, eight compute passes:**
  1. `measure`: each budget scale's totals.
  2. `choose`: the finest scale that fits; past ×8 the frame is flagged as overrun.
  3. `count`: each cluster at the chosen scale.
  4. to 6. `scan_local`, `scan_blocks`, `scan_add`: the exclusive scan into offsets.
  7. `emit`: the same walk as the CPU reference. On the GPU a ring is held back one step until the next ring is known, because a stretch's form depends on both its ends.
  8. `draws`: the two indirect draw arguments.
- **Over budget:** a cluster that does not fit even at ×8 is left out and named in the report (bit 2).
- **Drawing:** a depth prepass for the tubes and the ribbons, then each shaded over the nearest surface, through new `curve_vertex` and fragment entries in `wood.wgsl`.
- **The sun's map:** the same passes, orthographic at its texel, drawing tubes and ribbons.
- **API:**
  - `Renderer::submit_curve(&Curve)` switches the wood to the curve; the mesh still brings the leaves and the bounds.
  - `curve_report`, `curve_mesh` and `curve_bytes` read back what was drawn.
  - `curve_viewer(camera, viewport)` builds the `Viewer` both the CPU reference and the GPU read.
- **Camera budget:** 6M vertices, 12M tube indices and 18M ribbon indices. The first budget, with 6M ribbon indices, overran on the spruce at the hero view even at ×8.
- **Sun budget:** 3M vertices, 4M tube indices and 9M ribbon indices.

## Tests

| Test | Result |
|---|---|
| `crates/telperion-render/tests/curve.rs`: the GPU against the CPU reference at the same view and error, for `ordinary` (round wood, tubes and ribbons) and `date-palm` (shaped cells), at the hero view and close in | Every count and every index is identical. The largest position gap is 3.8e-6 m, normals agree to a cosine of 1.000000, and coverage agrees to 1e-7 |
| `tessellate/tests.rs` (core): the twig within half a pixel from 50 m to 5 cm, budget, every preset at its hero distance | Green, with ribbons in the measure (edges against the silhouette) |
| `generation_limit_guard` | Green: the new limit sites are declared |
| `cargo test -p telperion-render` | Green, all 42 test binaries (before the ribbon-width change; the curve test is green after it) |

## A decision that changed host decision 4 (needs the host)

- **Coverage ribbons read pale.** Pixel-wide ribbons, with the wood's share as alpha-to-coverage, made the beech's crown pale and sparse beside today's at the hero view.
- **Ribbons at their true width match today.** Drawn at the wood's own width and covered by the 4-sample rasteriser, they look as today's crown does, and they are faster: 2.7 against 4.4 ms.
- The A/B is `raw/r3/crop-beech3.png`: today | coverage ribbons | true-width ribbons | all tubes.
- **What I did:** I made the true width the rule in both the CPU reference and the GPU.
- **What is left of coverage ribbons:**
  - the per-vertex `coverage` (now always 1);
  - the alpha-to-coverage ribbon pipelines;
  - the shader's mode 1.
- **Decision needed:** delete those, or bring coverage ribbons back with a fix for the paleness, whose cause is unknown. The pale haze may come from the shadow and receiver lookups at the off-wood ribbon edges; that is not tested.
- **Also changed:** the sun's map now casts ribbons at their true width. With tubes alone, thin wood cast nothing and the crown lightened.

## R4: the engine trees at 80 years, seed 1, 960 × 720, 4 samples, under the GPU lock

- **Vegetation:** the GPU timer around the vegetation pass. For the curve path it does not include the compute passes.
- **Frame:** the host's wall time from draw to done, which does include them, median of 40.
- **Wood bytes:**
  - for the mesh path, its five buffers with headroom;
  - for the curve path, the curve and both views' fixed budgets.
- **Logs:** `raw/r3/final.log` (mesh), `raw/r3/final2.log` (curve). The load was 1 to 5.

| Species | View | Today's mesh: vegetation / frame | Curve: vegetation / frame | Today's triangles | Curve triangles, tube + ribbon (scale) |
|---|---|--:|--:|--:|--:|
| Beech | hero | 18.9 / 19.6 ms | **2.7 / 8.6 ms** | 83.7M | 0.32M + 1.51M |
| Beech | limb | 17.1 / 18.1 | **4.5 / 9.0** | 83.7M | 0.27M + 0.14M |
| Beech | twig | 25.1 / 26.1 | **9.4 / 15.8** | 83.7M | 0.82M + 0.53M |
| Spruce | hero | 51.4 / 51.9 | **6.0 / 12.2** | 198.2M | 0.04M + 3.75M (×2) |
| Spruce | limb | 30.6 / 31.2 | **3.6 / 8.0** | 198.2M | 0.05M + 1.04M |
| Spruce | twig | 42.4 / 43.0 | **13.6 / 19.6** | 198.2M | 0.20M + 1.90M |
| Oak | hero | 41.4 / 42.2 | **17.3 / 23.4** | 58.0M | 0.58M + 0.97M |
| Oak | limb | 25.9 / 26.5 | **13.2 / 18.9** | 58.0M | 0.50M + 0.17M |
| Oak | twig | 53.8 / 54.5 | **34.5 / 43.5** | 58.0M | 1.88M + 0.85M |
| Palm | hero | 0.14 / 0.26 | 0.15 / 2.56 | 69k | 23k + 0 |
| Palm | limb | 0.02 / 0.13 | 0.01 / 1.52 | 69k | 0 |
| Palm | twig | 1.96 / 2.12 | 2.57 / 11.39 | 69k | 20k + 0 |

**In leaf (whole), vegetation:**
- spruce hero 81.3 → 36.0 ms;
- oak hero 43.6 → 19.5 ms;
- beech hero 23.0 → 6.7 ms.

**Wood on the device:**

| Species | Today | Curve |
|---|--:|--:|
| Spruce | 7.49 GB | 0.73 GB |
| Beech | 3.15 GB | 0.61 GB |
| Oak | 2.19 GB | 0.59 GB |
| Palm | 2.6 MB | 0.53 GB |

The curve's memory is mostly the fixed budgets, the same for every tree, which is why the palm holds 0.53 GB.

**Reading it:**
1. **The wood pass is 3 to 9 times faster on the beech and spruce, and 1.5 to 2.4 times on the oak.** Wood memory falls by 4 to 10 times.
2. **The compute passes cost 1 to 9 ms a frame.**
   - Each cluster is one thread that walks its rings serially, so a close view with large clusters pays the longest walk's latency: the palm's twig view costs 9 ms of compute for 20k triangles.
   - The fix is a ring-level pass, one thread a ring (ring records written by the cluster pass, then an indirect dispatch). It is not built.
3. **The hero view's triangles are not the design's 0.02 to 0.2M.** They are 1.5 to 3.8M on the engine trees.
   - Ribbons dominate. Each ring of sub-pixel wood is a stretch of four triangles, and the ring levels can only merge within a 32-point cluster, held by the wander's sag at a quarter pixel.
   - Ribbon thresholds of 2, 4 and 8 px cut the tube count (oak hero: 0.58M down to 0.04M tube triangles) without a faster frame. The cost is fragments, not triangles.
   - On today's presets, the CPU reference gives 0.15 to 0.5M at the hero view.
4. **The palm got slower,** 0.26 → 2.6 ms a frame: fixed compute cost on a small tree.

## R3 stills (`raw/r3/sheets/<species>-<bare|whole>.png`: today | curve, at hero, limb and twig)

- **I viewed:** the oak bare sheet, the spruce whole sheet and the beech hero crops.
- **Oak and spruce:** the same tree, read the same, at all three views.
- **RMSE against today** (`raw/r3/rmse-final.txt`):
  - bare: 2.3 to 6.0%;
  - whole: 0.3 to 2.9%;
  - the palm's twig view: 11% (below).
- **The beech's crown at the hero view matches today's** with true-width ribbons (`crop-beech3.png`).
- **The palm's "twig" view** stands 5 cm off the stem's surface inside its leaf-base cells. The cells shade differently from today's: their analytic normals point out from each cell ring's centre, where today's averaged face normals.
- **The "twig" views are not on a twig.** The probe picks the outermost childless node at mid-height, but the camera there stands inside the crown, looking at the branches behind. A real 5 cm twig shot needs a camera aimed along a chosen twig. This was not done.
- **For the host to judge:** the crown haze, the normals, the shadows (now including ribbons), and the palm's cells.

## Open

- **R1's "one wood path":** the mesh path still exists. The curve path is chosen by `submit_curve`. Retiring the mesh wood (`TreeMesh`, web, the GPU executor's resident path) is integration work, F4.
- **Coverage ribbons:** host decision above.
- **The ring-level emission pass** that removes the per-cluster serial latency.
- **Budgets sized from the screen,** not fixed constants: 0.5 GB on a palm.
- **Browser limits:** the camera's vertex buffer is 240 MB, against WebGPU's default 128 MB storage binding. Untested in a browser.
- **The shader text** grows the render Wasm. CI's size budget is not checked here.
