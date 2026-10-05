# fn-208 design: wood surfaced on the GPU at the screen's error (worker, 2026-10-05)

This is R1 and a design note for the host, not code. Every code claim is marked **checked** (file:line, read on this branch at `33b0cfd9`, its line's first words confirmed), **measured** (run on this branch), or **unknown**.

Paths:
- `R/` is `crates/telperion-render/src/`.
- `S/` is `crates/telperion-core/src/pipeline/surface/`.

## 0. The short of it

- **Today's wood is far over budget. Measured, the wood pass alone (`View::Bare`) at the hero view:**

  | Tree | Wood triangles | Wood pass |
  |---|--:|--:|
  | Today's oak | 7.7M | 8.4 ms |
  | Today's spruce | 5.9M | 3.0 ms |
  | Engine oak | 58M | 67 ms |
  | Engine spruce | 198M | 41 ms |

  - STRATEGY.md's frame bar is 2 ms for a whole hero tree.
  - The engine spruce's wood is about 6 GB of buffers. Its first timing run here ran out of GPU memory at submit; the second, on a quieter card, fit.
- **At a half-pixel error, a view needs about 0.01M to 1.3M wood triangles.** That is an estimate from the trees' own segments, with sides and rings set by projected radius and curvature: 10 to 1,800 times fewer than today's mesh draws.
  - **A risk:** most of the engine's wood is sub-pixel at the standard views. The spruce has 4.28M of its 4.32M segments under a quarter pixel at the hero view.
  - Dropping that wood is not a half-pixel error in appearance: its coverage is the crown's haze. It needs coverage-correct ribbons, another 0.1M to 0.7M triangles, or an aggregate (section 3).
- **The curve data already exists inside the CPU sweep.**
  - Each run's samples (position, radius with flare and fork easing, distance along) and its parallel-transported frames are computed before any vertex (`S/samples.rs`, `S/frames.rs:156`).
  - The GPU executor's compact ring (`S/compact.rs:7`, 64 B) carries nearly all of it.
  - Packed at about 28 to 32 B a node, the engine spruce's curve is about 130 MB, against about 6 GB of mesh today.
- **The selection pattern is reusable as it stands:**
  - the half-pixel test (`R/shaders/select.wgsl:52, 89-92`);
  - the frustum planes;
  - the deterministic select → prefix → scatter compaction into indirect arguments;
  - the dispatch fold past 65,535 groups.

  The wood kernels' ring emit and index writing are reusable with a per-ring side count.
- **Fragment cost is a second wall that tessellation does not remove.**
  - The 5 cm twig close-up costs more than the hero view on every tree: today's oak 19.7 ms against 8.4 ms, and the engine oak 133 ms against 67 ms.
  - The camera there is inside the crown's reach, so the cost is pixels shaded behind the twig (4x multisampling, the bark shader), not triangles.
  - How much of it fewer micro-triangles remove is **unknown** until step 2 measures it.

## Host decisions 1 to 6 (2026-10-05)

The design is approved, in the order of section 6.

1. **Curve data:** 28 B a point, a run table and clusters of up to 32 points with bounds; every tree produces it, with no second path. Accepted.
2. **Tessellation:** sides by `ceil(π / acos(1 − 0.5/ρ))`, rounded up to 3·2^k; rings by curvature on a nested ladder; fixed output budgets that coarsen evenly and report their error scale. Accepted. An overrun is named in the frame report, never silent.
3. **Normals:** analytic tube normals, with the CPU reference following. Accepted under AGENTS.md, "Generator evolution": every tree changes slightly. The change is measured and judged on stills.
4. **Fine shoots:** tube, then ribbon, then a one-pixel coverage ribbon with alpha equal to the true width. Placed as shoot clusters in the wood path, with no `select.rs` change for now. Accepted. Sub-pixel wood is never dropped. When step 5 lands, a hero comparison against today's goes to the owner, who judges whether the haze is kept.
5. **The CPU reference:** the same tessellation function, evaluated at a given view and error. It is camera-dependent by definition, and camera-independent only in the curve data it reads. Tests compare the GPU and CPU at the same view.
6. **The palm's leaf-base cells:** checked in step 1. If the curve data cannot carry their cross-section, it is reported before step 3.

Build steps 1 to 3 now. At step 2, stop and report if shading dominates up close.

## Host decisions 7 to 9 (2026-10-05, after STEP2.md)

7. **Shading follows the same screen error.** Options (b) and (c) of STEP2.md, together.
   - Bark plates, relief and every procedural term fade where their features are sub-pixel, by an analytic band limit or a mip-like level from the feature's projected size, never a fixed distance.
   - A depth prepass against overdraw.
   - A measured choice of multisample count.
   - This comes first, before the tessellator: it is the larger cost, it helps today's trees too, and it does not wait on the curve's GPU path.
   - **Bar:** the bark shader's time per shaded pixel at the hero view and at the 5 cm twig, against today's. Stills show no visible change at the standard views. Bark detail's fade is judged on stills, and close-up bark keeps its full detail.
   - In the spec, R2 gains the shading level of detail and R4 the per-pixel cost.
8. **The packed point is 32 bytes.** The frame gets more bits, so Laurelin's metres-wide trunk is exact at 5 cm.
9. **The palm's cells beside their run** (about 104 B per shaped run): accepted.

Order (host):
- (i) the shading level of detail, the depth prepass and the multisample measurement;
- (ii) the 32-byte point;
- (iii) step 3, the CPU reference tessellator with the silhouette test red first;
- (iv) the GPU passes.

## 1. R1: the curve data

### What the generator hands over

Per tree, three tables and the tree's surface rows. This would be documented in `docs/pipeline.md` as a contract type, as `SurfaceMesh` is today.

**Curve points, one a ring the sweep would make today:**

| Field | Encoding | Bytes | Source today (checked) |
|---|---|--:|---|
| Centre | 3 × f32 | 12 | `Sample.p` (`S/build.rs:8-12`, `pub(super) struct Sample`) |
| Radius | f32 | 4 | `Sample.r`: girth with fork easing times the flare (`S/samples.rs:24` `let flare`, `:43` `fn girths`) |
| Along | f32, metres from the root | 4 | `Sample.d` (`S/compact.rs:96-104`) |
| Frame | normal as a quaternion, smallest-three in 32 bits as leaves pack theirs (`R/shaders/leaf.wgsl:1`) | 4 | `frames()`, parallel transport (`S/frames.rs:156` `pub(super) fn frames`) |
| Twist phase | f16, plus flags (cap, buried foot) in the other half | 4 | `TAU · twistRate · d / height` (`S/build.rs:243`) |
| **Total** | | **28** (32 aligned) | |

**Runs**, one a run of the sweep's paths (`S/paths.rs:3-14` `pub(super) struct Run`): first point, point count, the parent run and node it is sunk into, flags (trunk; leaf-bearing shoot, for section 3). 16 B.

**Clusters**, built from the runs on submit (section 2): about 48 B for 32 points, so 1.5 B a point.

**Tree-wide:** the section's `lobes`, `lobeDepth`, `twistRate` and height. The profile is `1 + lobeDepth·cos(lobes·(angle + phase))` (`S/angular.rs:16-21`, checked), evaluated on the GPU as today's CPU sweep does.

### How today's trees produce it

- `surface::build` already runs paths → samples → frames before it emits a vertex (`S/build.rs:242-249`: a vertex is `p + (N cos + B sin)·r·profile`).
- The curve data is that state, written out instead of swept. So every tree, today's generator or the engine's, produces it from `mesh::build`'s skeleton with no second path.
- The compact ring (`S/compact.rs:185-213`, `let floats = [`) is a 64 B form of it. Its vertex and cap offset words are tessellation output, not curve data. Only round, unlobed sections inside |c| ≤ 64 m qualify for it today (`S/compact.rs:28` `pub fn qualified_ring`, `:36-41`).
- **Shaped runs.** The palm's leaf bases are swept on their cell's rings, not round ones (`S/section.rs:1-3`, `:14` `pub(super) fn of`). `reshape` moves the centres and radii into the samples, which the curve carries. Whether the cell's own non-round outline also needs carrying (a per-run profile) is **unknown**; this is the first thing to check in step 1.
- **Junctions.** A lateral run starts sunk into its parent's socket, with fork swell (`S/samples.rs:97-121` `fn branch_run`). The overlap hides the joint, as today. Caps sit on the axis at a run's ends (`S/build.rs:255-257`).

### Size per tree

28 B a point plus 1.5 B for clusters, with points taken as pipeline nodes. Each run also adds a buried foot and its socket samples, so the true count is a few percent higher (estimate).

| Tree (seed 1) | Nodes | Curve data | Today's wood buffers (36 B a vertex with radii, 12 B a triangle) |
|---|--:|--:|--:|
| Today's beech | 213k | 6.3 MB | unknown (not measured) |
| Today's oak | 134k | 4.0 MB | 235 MB (3.97M vertices, 7.7M triangles, measured) |
| Today's spruce | 96k | 2.8 MB | 182 MB (3.07M, 5.9M, measured) |
| Today's palm | 552 | 16 KB | unknown |
| Engine beech | 1.94M | 57 MB | about 2.5 GB (estimated from fn-198's counts) |
| Engine oak | 1.34M | 40 MB | 1.75 GB (29.2M vertices, 58M triangles, measured) |
| Engine spruce | 4.32M | 127 MB | 6.0 GB (100M, 198M, measured) |
| Engine palm | 1,473 with leaf bases | 43 KB | 2.4 MB (35k, 69k) |

- Allocations take 1.25x headroom on top (`R/buffer.rs:8` `const HEADROOM: f64 = 1.25;`, checked), so the engine spruce asks for about 7.5 GB on a 10 GB card.
- The curve is 40 to 60 times smaller than the mesh.

## 2. GPU tessellation in wgpu

### No mesh shaders

wgpu has no task or mesh shaders, so this is the classic compute-then-draw pipeline: compute passes write vertices and indices into buffers, and one indirect indexed draw reads them.

The device asks only for `TIMESTAMP_QUERY`, and raises only `max_buffer_size` and `max_storage_buffer_binding_size` (`R/device.rs:145-149` `let limits = wgpu::Limits {`, `:163`, checked). It does not need `INDIRECT_FIRST_INSTANCE`: one draw from offset 0 serves. The same code runs in the browser, which has the same limits.

### Passes, per frame

1. **Cluster pass, one thread a cluster.**
   - A cluster is up to 32 consecutive points of one run, with:
     - a bounding sphere, including the largest radius;
     - its largest and smallest radius;
     - a ladder of ring-removal errors, one per nested ring level (below).
   - **Frustum culling:** the six planes and the sphere test of `select.wgsl:79-83`.
   - **Projected size:**
     - pixels per metre is `eye.w / depth` (`select.wgsl:89-90`), with `eye.w` the viewport height over twice the tangent of half the field of view (`R/select/frame.rs:38-39`, checked);
     - the cluster's largest projected radius sets whether it is tube, ribbon or coverage only (section 3).
   - **Output:** the ring level, and an upper bound on its vertices and indices.
2. **Prefix pass.** A deterministic exclusive scan of the counts, as `select.wgsl`'s `prefix` does (`:154-192`, a chunked Hillis-Steele scan, no global atomics), gives each cluster its output offsets and the indirect draw's index count.
3. **Tessellation pass, one 64-lane workgroup a visible cluster.**
   - Each lane takes ring vertices `k = lane, lane + 64, …`, as `positions.wgsl`'s `emit` does (`R/generation/positions.wgsl:18-33`, checked).
   - Indices are written strip by strip, as `wood.wgsl`'s `expand` does (`R/generation/wood.wgsl:10-25`, `R/generation/wood_geometry.wgsl:1` `fn triangle`), with a per-ring side count instead of `cfg.segments`.
4. **Draw.** One `draw_indexed_indirect` with Uint32 indices, as the foliage levels draw (`R/foliage.rs:291`, checked).
   - The wood vertex and fragment shaders stay as they are (`R/shaders/wood.wgsl`): they read position, normal and coord, and the radius by vertex index (`wood.wgsl:28` `out.radius = radii[index]`).

**Dispatch.** Every per-item dispatch folds past 65,535 groups into y, as select and generation already do (`R/select.rs:449-455` `fn grid`, `R/generation/io.rs:133`, checked).

### Sides and ring spacing at half a pixel

**Sides.**
- A ring of n sides around a circle ρ pixels in radius departs from it by `ρ(1 - cos(π/n))`.
- So `n = ceil(π / acos(1 - 0.5/ρ))`, at least 3: about 3 at 1 px, 10 at 10 px, 32 at 100 px.
- Each ring takes n from its own projected radius, a function of that ring alone, so two clusters that share a boundary ring agree on it.
- For crack-free stitching and smooth motion, n is rounded up to the nested ladder 3·2^k (3, 6, 12, 24, 48, 96, 192).
  - A strip between rings at different levels is a zipper of n₁ + n₂ triangles.
  - A vertex new at level k+1 starts at its level-k edge's midpoint and slides out to the circle as the error grows past the threshold (a geomorph). Its silhouette then moves continuously, never by more than half a pixel at the switch.
  - Rounding up costs at most twice the triangles of the exact count.

**Rings.**
- A centreline that turns by θ over a length L departs from its chord by about Lθ/8.
- Split into m pieces, that is Lθ/(8m²). So `m = ceil(sqrt(Lθ·P/4))`, with P the pixels per metre at its depth.
- Close up, points are subdivided along a cubic Hermite curve through the curve points and their tangents. This is what makes a twig at 5 cm smooth, where its 1 cm phytomers are about 200 px long.
- At a distance, rings are removed by a nested ladder: every other point is dropped per level, and the cluster stores the largest deviation each removal causes (centreline and radius), made monotone.
- The cluster pass keeps the coarsest level whose error is under half a pixel. Cluster ends are on every level, so neighbours meet.

**Lobes** add their profile's amplitude, `lobeDepth·r`, to the radius the side count reads.

### Buffer budgets and an over-budget frame

**Budget.**
- The output buffers are fixed per renderer and sized from the screen, not the tree. A starting point: 8M triangles (96 MB of indices) and 4M vertices.
  - Vertices are 36 B each as today (`R/wood.rs:21-23` POSITION, NORMAL, COORD, checked; radius in a storage buffer at `:95-100`), about 144 MB.
  - About 240 MB in all, against 6 to 7.5 GB for the engine spruce's mesh today.
- **Vertex pulling** could replace most of that: the vertex shader evaluates the tube from a ring list of about 16 B a ring and `vertex_index`. It is an option for step 4, not the base design.

**Over budget.**
- The prefix pass knows the frame's total before anything is written.
- The cluster pass counts demand in parallel at four error scales (0.5, 1, 2 and 4 px).
- A one-thread pass picks the finest scale that fits, and tessellation uses it.
- The frame degrades evenly, never truncates, and writes the scale it used to a status word, which the timing record reports.
- No readback stalls the frame. The status is read a frame late, as the timestamps are (`R/timing.rs`).

### Bark coordinates and normals across detail changes

**Bark.**
- Coordinates are already intrinsic to the surface, not to the mesh: `coord` is (metres along from the root, angle around the frame) (`surface.rs:240-242`, checked).
- The fragment shader reads (along, cos θ, sin θ) and the radius (`R/shaders/wood.wgsl:29` `out.surface = vec3<f32>(coord.x, cos(coord.y), sin(coord.y))`, `:220-222`).
- A vertex at a given (along, angle) gets the same coordinate at every side count and ring level, so detail changes do not slide the bark.
- One condition: the frames are part of the curve data, computed once by parallel transport on the CPU and never re-derived from the tessellated rings.

**Normals: a change.**
- Today's GPU kernel takes vertex normals as area-weighted sums over neighbouring faces (`R/generation/wood_geometry.wgsl:26` `fn vertex_normal`, checked). Those change with the tessellation and would flicker as the levels move.
- The tessellator writes the tube's analytic normal instead: the radial direction tilted by the radius's slope along the curve, `normalize(radial - (dr/ds)·tangent)`, with the lobe profile's derivative where lobes are drawn.
- That is continuous across every level.
- The CPU reference (`S/normals.rs`) must then compute the same normals, or the two executors disagree beyond their stated tolerance. Its effect on the current stills is **unknown**: step 2's CPU-reference stills show it.

**Seated leaves.**
- Leaves seated on the wood take their contacts from the swept polygons (`S/attachment.rs:1-2` "Exact contact queries on the swept polygons"; the GPU's `compact_stations` ties stations to compact ring vertices, `pipeline/executor.rs:141-149`, checked).
- With adaptive sides, contacts move to the analytic tube. This changes leaf placement within the polygon's chord error. It is allowed by AGENTS.md's evolution policy, and needs the spruce's spray close-up checked.

### Shadows

- Today the sun's pass draws a prefix of the radius-sorted runs (`R/wood.rs:287-305` `pub fn set_casters`, checked).
- Under F3, the sun runs the cluster pass with its own orthographic projection and its shadow texel as the pixel, into a smaller budget.
- That costs a second tessellation per frame. It is estimated small, since a shadow texel is coarse; **unknown** until measured.

### What is reused, and what is new

| Piece | File (checked) | Reuse |
|---|---|---|
| Half-pixel test, eye and pixels per metre | `select.wgsl:52, 89-92`; `select/frame.rs:38-39` | As is |
| Frustum planes, sphere test | `select/frame.rs:71`; `select.wgsl:79-83` | As is |
| Deterministic compaction into indirect arguments | `select.wgsl:107-208` (select, prefix, scatter); `select.rs:348-350` (per-frame reset) | Pattern reused for clusters |
| Dispatch fold past 65,535 | `select.rs:449-455`; `generation/io.rs:133` | As is |
| Ring emit from centre, frame and radius | `generation/positions.wgsl:18-33` | Per-ring side count, Hermite subdivision |
| Strip indices, triangle order | `generation/wood.wgsl:10-25`; `wood_geometry.wgsl:1` | Variable sides, zipper strips |
| Buffer fit and the Oversize error | `submit.rs:46-48, 134` | Checks the curve and the fixed budgets |
| Wood vertex layout and bark shaders | `wood.rs:21-23, 95-100`; `shaders/wood.wgsl`, `bark.wgsl`, `plates.wgsl`, `smooth.wgsl` | As is |
| Area-weighted normals | `wood_geometry.wgsl:26` | Replaced by analytic normals |
| Uniform segment count | `S/compact.rs:95`; `generation/wood.wgsl:1` (Config) | Replaced |
| No culling | `R/pass.rs:194` `cull_mode: None` | Unchanged (tubes are closed) |

## 3. Fine shoots with the foliage

**The rule, from the same error and not a radius:**
- **Drop.** Removing a segment changes the picture by its projected width, 2ρ pixels of coverage along its length.
- **Ribbon.** A camera-facing ribbon of the same width has the tube's silhouette exactly. Only shading differs, so a ribbon replaces the tube where the shading's error is under the threshold. ρ < 1 px is the stand-in; the real bound is a shading error to measure.
- **Coverage.** Below a quarter pixel, wood is not dropped. It is drawn as a ribbon one pixel wide whose alpha is its true width, through alpha-to-coverage at the 4x multisampling the renderer already uses (`multisample` 4 in every timing record). Thousands of sub-pixel twigs then sum to the crown's haze, rather than shimmering or vanishing.

**Two ways to place them:**
- **(A) Shoot clusters in the wood path.**
  - Leaf-bearing runs (`plan::bearing_runs`, `pipeline/foliage/plan.rs:270`, checked) are flagged in the run table.
  - Their clusters choose tube, ribbon or coverage in the cluster pass.
  - No change to `select.rs`. **Recommended first.**
- **(B) Shoots as foliage instances, as the spec's design suggests.** A shoot shares the leaves' selection and pass. `select.rs` then needs:
  - **A second instance encoding.** Placement decoding is hard-wired to 3 words a leaf: `select.wgsl:70-72` `let base = instance * LEAF_WORDS;`, `foliage.wgsl:52-55`, `leaf.wgsl:12`.
  - **Its own bound and deviation.** The error test is `deviation[level] · scale` over one element's sphere (`select.wgsl:75-92`), and deviations are per element and level (`select.rs:240`). A shoot needs its own sphere and a deviation that scales with its radius and length.
  - **A ladder:** tube, ribbon, coverage, unseen. That is a second `Select` over a shoot stream, or one generalised to several streams.
  - **Bark inputs.** The bark shader's inputs, radius and (along, angle), come today from a storage buffer by vertex index (`wood.wgsl:28`). A shoot instance supplies them itself.
  - **Shadows.** The foliage's shadow draws a fixed placement stride at the coarsest level (`foliage.rs:302-319`). Shoots would follow.
  - (B) is the larger change. Take it only if (A)'s cluster pass over a million shoot clusters measures too slow. Estimate: one thread a cluster, about 0.1 ms for a million, so it should not.

## 4. Cost estimates

### Measured: today's wood path on this GPU

- RTX 3080, NVIDIA 610.57.04, Vulkan, under the GPU lock, 960 × 720 at 4x multisampling.
- `telperion_render::measure`'s protocol: 8 conditioning frames, 8 warmup, 120 measured. Every record's verdict is `valid` (p95 within 2x of the median).
- Seed 1, 80 years for the engine trees, dressed as the stills runner dresses them.
- Cameras:
  - the hero pose;
  - the limb close-up of `still.rs` (9 m off, a third of the width to the side, at half height);
  - a 5 cm twig close-up: the outermost childless fine node at mid-height, from 5 cm further out, near plane 5 mm.
- The source is `f3-frame-probe.rs`; logs are in `raw/`.

| Tree | Wood triangles | Hero, bare / whole | Limb, bare / whole | Twig at 5 cm, bare / whole | Shadow pass |
|---|--:|--:|--:|--:|--:|
| Today's oak | 7.71M | 8.4 / 9.8 ms | 7.6 / 8.3 ms | 19.7 / 21.2 ms | 0.33 ms |
| Engine oak | 58.0M | 66.9 / 69.5 ms | 41.0 / 41.7 ms | 132.9 / 135.7 ms | 1.0 ms |
| Today's spruce | 5.94M | 3.0 / 21.4 ms | 3.1 / 7.2 ms | 5.1 / 7.3 ms | 4.5 ms |
| Engine spruce | 198.2M | 40.7 / 71.0 ms | 23.1 / 26.2 ms | 48.0 / 53.2 ms | 7.3 ms |

- "Bare" is the vegetation pass with wood only, since foliage draws nothing in `View::Bare` (`R/foliage.rs:258`); "whole" adds leaves.
- **Load.** The engine spruce went out of memory on its first run, at a load of 35 with another 2.5 GB of the card in use, and fit on the retry at 1.3 GB in use. The loads were 2.5 to 35 on 32 threads. The GPU numbers are judged valid by their own spread, but the CPU load during them is recorded in `raw/runs.log`.
- **Reading the times:**
  - Wood time does not follow triangles alone. The engine oak's 58M triangles cost more than the spruce's 198M (67 ms against 41 ms at the hero view).
  - The twig close-ups cost more than the hero view everywhere. The camera there sits inside the crown's reach, looking into it, so most of the cost is shading pixels behind the twig at 4x multisampling. Micro-triangles shade whole 2 × 2 quads, so fewer of them should help. How much is **unknown**.
  - The oak's bark has plates; the spruce's does not. Whether the plate shader is the difference is **unknown**.

### Estimated: half-pixel tessellation of the same trees

The estimator (in `f3-frame-probe.rs`, `F3_ESTIMATE=1`) walks every segment, parent node to node:
- It culls by the frustum, with the segment's radius and half its length as margin.
- It reads ρ from the segment's larger radius at its midpoint's depth.
- It sets sides and rings by the rules of section 2.
- It counts ribbons (2 triangles a piece) for ρ < 1 px.

It reports three totals:
- **Unmerged:** at least one ring a segment.
- **Merged:** rings removed along straight wood, up to 32 px apart on tubes and 8 px on ribbons, which the nested ring ladder would give.
- **Coverage ribbons:** for the sub-pixel wood.

It does not estimate fragment cost.

| Tree, view | Today's mesh | Unmerged | Merged | Coverage ribbons | Tube / ribbon / sub-pixel / culled segments | Most sides |
|---|--:|--:|--:|--:|---|--:|
| Today's oak, hero | 7.71M | 0.04M | 0.02M | 0.21M | 299 / 14,880 / 118,455 / 0 | 10 |
| Today's oak, limb | 7.71M | 0.11M | 0.13M | 0.04M | 3,035 / 14,299 / 9,182 / 107,118 | 12 |
| Today's oak, twig 5 cm | 7.71M | 0.18M | 0.19M | 0.15M | 3,825 / 35,503 / 43,346 / 50,960 | 16 |
| Engine oak, hero | 58.0M | 0.68M | 0.05M | 0.09M | 40,391 / 140,201 / 1,161,822 / 0 | 16 |
| Engine oak, limb | 58.0M | 0.29M | 0.05M | 0.01M | 17,868 / 39,703 / 27,790 / 1,257,053 | 21 |
| Engine oak, twig 5 cm | 58.0M | 1.32M | 0.16M | 0.07M | 83,917 / 156,830 / 523,555 / 578,112 | 25 |
| Today's spruce, hero | 5.94M | 0.01M | 0.004M | 0.22M | 198 / 1,417 / 94,558 / 0 | 9 |
| Today's spruce, limb | 5.94M | 0.02M | 0.02M | 0.11M | 252 / 3,657 / 18,641 / 73,623 | 10 |
| Today's spruce, twig 5 cm | 5.94M | 0.09M | 0.19M | 0.03M | 1,463 / 8,524 / 2,159 / 84,027 | 18 |
| Engine spruce, hero | 198.2M | 0.11M | 0.01M | 0.72M | 3,447 / 39,880 / 4,277,379 / 1,913 | 10 |
| Engine spruce, limb | 198.2M | 0.16M | 0.03M | 0.17M | 3,504 / 61,946 / 350,993 / 3,906,176 | 7 |
| Engine spruce, twig 5 cm | 198.2M | 0.84M | 0.18M | 0.25M | 9,902 / 349,091 / 429,992 / 3,533,634 | 20 |

Per-frame cost (estimate):
- **Raster:** at about 2 G triangles/s (the oak's measured rate, the slower one), 0.1M to 1.5M triangles is 0.05 to 0.75 ms.
- **Compute:**
  - The cluster pass runs one thread a cluster: about 135k clusters for the engine spruce at 32 points, about 0.05 ms.
  - Tessellation writes at most the budget, 240 MB, at a few hundred GB/s: about 0.5 ms at the budget, and well under at these demands.
- **Fragment shading of covered pixels: unknown.** Today's close-ups show it can dominate (the oak's twig close-up). The bark shader at full screen has its own timing test (`R/wood/calibration.rs:211-213`, `time_fullscreen_bark_and_mature_oak`, ignored by default, checked). Step 2 takes the wood pass at these views with the tessellator against today's, so the triangle term and the fragment term are measured apart.
- **GPU memory for wood:**

  | Tree | Today | Under F3: curve + fixed budget |
  |---|--:|--:|
  | Engine spruce | 6.0 to 7.5 GB | 127 + 240 MB |
  | Engine oak | 1.75 to 2.2 GB | 40 + 240 MB |

## 5. Risks and unknowns

1. **Fragment cost at close range** may dominate whatever the triangle count. It is measured in step 2, before more is built. If it dominates, the lever is the bark shader's cost and overdraw (a depth prepass, or occlusion culling of clusters by a hierarchical depth buffer), and the error rule does not reach it.
2. **Coverage of sub-pixel wood.** Coverage ribbons are a new representation. Whether their alpha-to-coverage haze reads like today's micro-triangles at the hero view is for the host's stills. Today's renders show sub-pixel wood as multisampled micro-triangles, so a change in crown density is possible.
3. **Normals change** from area-weighted to analytic, on the GPU and in the CPU reference. Every wood still changes slightly.
4. **Seated leaves' contacts** move from the polygon to the analytic tube (needles on the spruce).
5. **Shaped sections** (the palm's leaf-base cells, `S/section.rs`): whether a per-run profile is needed is unknown.
6. **The CPU reference.** The pipeline's CPU mesh (`Expansion::mesh`) stays the reference "that defines correct" (STRATEGY.md). It needs a view-independent form:
   - **Proposed:** the tessellation at a stated error for a stated camera, run in Rust by the same rules, for the tests (R2's silhouette test) and for consumers without a GPU.
   - **Unknown:** whether every existing consumer of `SurfaceMesh`, the field, voxels and the wasm entries, keeps a fixed-density mesh. The answer decides whether the old sweep stays as the CPU reference at a fixed error.
7. **f32 positions close up.** At 20 m from the origin an f32 unit in the last place is about 2 µm. A 1 mm twig at 5 cm is 48 µm a pixel, so this is fine (estimate). Camera-relative positions are not needed for a single tree, but may be for a forest (F4).
8. **"High detail" at 5 cm** asks more than the silhouette: bark texture at a 1 mm twig's scale, buds, leaf scars. F3's error rule gives the silhouette and the bark field's own filtering (`bark_field_filtered`, `R/shaders/bark.wgsl:174`). Whether the bark field has detail at that scale is **unknown**; the host's close-up still answers it.
9. **WebGPU limits.** The browser's `max_storage_buffer_binding_size` is often 128 MB to 2 GB, depending on the adapter. The engine spruce's 127 MB of curve data needs splitting across bindings or a per-cluster page table. **Unknown** on the target browsers.
10. **Timing under load.** The GPU runs were valid by their own spread, but the machine's CPU load swung from 2.5 to 35 (FRICTION.md).

## 6. Order of steps

1. **The curve data (R1), CPU only.**
   - Emit it from the sweep's samples and frames for every preset.
   - Define it in `docs/pipeline.md`'s contract section.
   - Test that the existing sweep's vertices are reproduced from it to the bit at today's 12 sides. That proves the curve carries everything, the shaped sections included, or finds what it lacks (risk 5).
2. **A measurement before building the tessellator.**
   - Take today's wood pass at the four views, splitting triangle cost from fragment cost: the same trees at 2x and 0.5x resolution, and with the bark shader replaced by a flat one.
   - This decides how much of the frame F3 can win (risk 1). Small before large (AGENTS.md): the 8-tree forest is not needed.
3. **The CPU reference tessellator at a stated error.**
   - Tube, ribbon and coverage; nested sides and rings; analytic normals.
   - R2's silhouette test against a dense reference mesh at several distances, red first on today's fixed 12 sides at 5 cm.
4. **The GPU cluster, prefix and tessellate passes.**
   - The indirect draw, the fixed budgets and the even degradation over budget.
   - Agreement with the CPU reference within a stated tolerance.
   - Shadows by the same passes.
5. **Fine shoots (A),** coverage ribbons, and the seated-leaf contacts on the analytic tube.
6. **R3 stills and R4 measurements.**
   - Every species at the standard views beside today's.
   - The 5 cm twig close-up.
   - Wood triangles, frame time and wood GPU memory for the engine spruce and oak, against the table in section 4.
7. **R5:** workspace gate, `npm test`, Codex review, artifact budgets.

## Files

- This file; `FRICTION.md`.
- `f3-frame-probe.rs`: the scratch timing and estimating example. It runs as `crates/telperion-render/examples/f3_frame.rs`, `f3_frame <engine|today> <spruce|oak> <seed>`; `F3_ESTIMATE=1` estimates without the GPU.
- `raw/` (ignored): `today-oak-1.log`, `runs.log`, `spruce2.log`, `estimate.log`, `estimate2.log`, `run.sh`.
