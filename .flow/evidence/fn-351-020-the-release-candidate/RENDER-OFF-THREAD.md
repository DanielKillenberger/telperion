# The renderer's growth off the page's thread: reading, and the question it stops at

14 Nov 2026, branch `fn-351-render-grows-off-thread` from `fn-351-candidate` at `7685ee85b`. Design step 3 applies: the render module has no entry that takes a built tree, so no code was written. What follows is the reading, the measured buffers and the smallest entry that would be needed, for the host to decide.

## 1. Reading

### The field path's worker

- The package does not ship a worker. `telperion/field`'s `growField(family, n, options)` (`src/field/index.ts`) is a plain async function that runs wherever it is called: a page, a worker or Node. It holds one `TreeEngine` per realm (`engine()`), built once from a source the first call names. `compileField` compiles `telperion.wasm` once, and the caller passes the `WebAssembly.Module` to each worker it starts.
- The only worker in the repository belongs to the probe page: `src/probe/field/worker.ts`. Its whole protocol is `postMessage(Case[])` in and `{kind:"timed", row} | {kind:"failed", error: string} | {kind:"done"}` out. The module is not posted: the worker loads `telperion.wasm` from its own URL through `wasmSource`. An error comes back as `String(error)`, which keeps the message and drops the type. The protocol has no cancellation, no supersession and no transfer; the only result that crosses is a timing row. `tests/browser/field.mjs` runs the field in a module worker as a browser smoke test.
- The fn-369 named refusal exists only in `telperion.wasm`. Its allocator records a refused allocation (`refusal_ptr`, `stage_name_ptr`; `crates/telperion-wasm/src/live.rs`). `TreeEngine.trapped` (`src/browser/core.ts`) turns the trap into an `Error` that names the stage, the bytes asked and the bytes live, then replaces the instance.
- Conclusion: there is no worker protocol to reuse, only a pattern. The core entry runs unchanged in any realm, and a worker of the caller's own posts plain messages.

### The render path

- `src/browser/render.ts`, `createRenderer(canvas)`: the returned `Renderer` has `setTree(family): Submitted` and `beginGrowth(family): Growth`, whose `growTo` returns a number and whose `draw` returns `Submitted`. **All of these are synchronous.** Only `createRenderer`, `timing` and `orbit` return promises. Callers: `harness/rust-stage.ts:149`, `harness/grow-in.ts:64-65,106,112`, `tests/browser/render.mjs:184`, `scripts/benchmarks/mature-generation.mjs:45`.
- `crates/telperion-render/src/web.rs`, `WebRenderer::set_tree(&str)`, does `params::parse`, then `mesh::build`, then `Renderer::submit(&mesh)`, all in one Wasm call inside the render module's own memory. `web/growing.rs` does the same through `growth::begin`/`grow_to`/`mesh`, then `submit`. The module header says so: "nothing about a tree crosses the JavaScript boundary except its parameters going in and its counts coming out."
- **Does the render module take a built tree? No.** None of its Wasm exports uploads without growing. Natively, `Renderer::submit(&TreeMesh)` (`crates/telperion-render/src/lib.rs:233`) does take a built tree, but `TreeMesh` has no byte form (no serde, no encode or decode), and `telperion.wasm`'s `TreeEngine.build` returns no `Curve`, which is what the renderer draws the wood from (fn-208). The renderer does not read `mesh.wood` except for the two counts in `Submitted`.
- The render module has no fn-369 allocator. A tree too large for the render module traps without a name today.

## 2. What would cross (measured)

These figures are native `mesh::build` at seed 1 under `--profile ci`, from a scratch example that was not committed. The upload form is the core's own packing: `Curve::packed` at 8 u32 a point (32 B), `packed_clusters` at 16 u32 a cluster (64 B), `packed_sections` at 28 f32 a section, and leaves and florets at their 12-byte storage. The element meshes, the bounds, the leaf reference, the curve's `demand()`, `lobes`/`lobe_depth`/`twist_rate`/`height` and the two wood counts add a few kilobytes.

| tree | build | points | clusters | leaves | florets | upload form | f64 `Curve` + instances | CPU wood surface (built, not uploaded) |
|---|---|---|---|---|---|---|---|---|
| oak at 40 (oregon-white-oak) | 1.16 s | 344,644 | 31,539 | 31,925 | 0 | **13.4 MB** (11.03 + 2.02 + 0.38) | 36.0 MB | 88.6 MB (2,791,154 vertices) |
| Telperion at 82 | 17.5 s | 3,713,726 | 230,269 | 697,902 | 308,051 | **145.7 MB** (118.8 + 14.7 + 8.4 + 3.7) | 376.5 MB | 1,181 MB (37,060,756 vertices) |
| Laurelin at 80 | 14.9 s | 3,397,010 | 212,387 | 648,205 | 995,470 | **142.0 MB** (108.7 + 13.6 + 7.8 + 11.9) | 353.4 MB | 1,097 MB (34,419,405 vertices) |

No sections were drawn at seed 1 on these three. All of the upload form is transferable: owned `ArrayBuffer`s, posted with zero copies. Each realm still makes one copy of its own. The worker copies out of its linear memory (wasm-bindgen's `Vec<u32>` return copies to a JS buffer). The page copies into the render module's memory before `write_buffer`, because wgpu's web backend writes from Wasm memory. Writing straight from the JS `ArrayBuffer` through `GPUQueue.writeBuffer` would need a JS-side upload outside the Rust renderer.

A side finding: `mesh::build` builds the CPU wood surface (88.6 MB for the oak, 1.18 GB for Telperion) on every `setTree`. The renderer reads only its two counts from it. That is likely part of the 23 s and of the Wasm peak.

## 3. The smallest Rust entry, and its cost (for the host)

1. **A packed tree, one type, in the core.** It would hold `points: Vec<PackedPoint>`, `clusters`, `sections`, `demand`, the curve's shape row, both `Foliage` (element plus 12-byte leaves) and the bounds and counts, made from a `TreeMesh` by the packing functions that already exist (`pipeline/surface/curve/pack.rs`). `Renderer::submit(&TreeMesh)` would become `submit_packed(&Packed)` after a pack, so the native and web paths stay one. `CurveGpu::new` would take the packed words and the demand instead of `&Curve`. Estimated at about 150 to 250 lines across `crates/telperion-core/src/mesh.rs`, `crates/telperion-render/src/{lib,curve,wood,foliage}.rs` and `web.rs`, plus a byte-identity test (`submit` and `submit_packed` leave the same device bytes; the resident digest already reads them). The Wasm size cost should be small, since the packers are already compiled into the render module; it is not measured.
2. **A producer in a worker.** There are two candidates:
   - (a) `telperion.wasm` (the field path's module) gains an output `{ upload: true }` on `TreeEngine.build`, with slots for the packed points, clusters, sections and organ. This is the one way the field path uses, and it brings the fn-369 named refusal with it. It adds the packers (and the wood's `Curve`, if the core binding does not already keep it) to `telperion.wasm`, which is under the 250,000-Brotli aim (P11). The size cost is not measured.
   - (b) `telperion-render.wasm` exports a free, device-free `buildPacked(family)`, and the worker loads the render module the page already caches. No new download, but no named refusal until the render module gets the fn-369 allocator, and that is a second way of growing beside the field path's.
3. **A consumer.** `WebRenderer.submitPacked(header, points, clusters, sections, leaves, organLeaves)` would copy the buffers into its memory and call `submit_packed`. Growth by year (`beginGrowth`/`growTo`/`drawGrowth`) holds a `GrowthState` between calls. Off the thread, that state lives in the worker, and every `growTo` or `draw` becomes a round trip.

## 4. Questions for the host (the stop)

- **Q1, the API's shape.** The design says "the same call returns the same promise", but `setTree`, `Growth.growTo` and `Growth.draw` return synchronously today. Off the thread they cannot keep that shape: the caller must await. Is `setTree(family): Promise<Submitted>` (and an async `Growth`) acceptable as 0.2.0's change, with the harness's four callers moved? Or should a new async method sit beside the synchronous one, which would be a second way of growing?
- **Q2, the producer.** Should the worker grow on (a) `telperion.wasm` with a packed-upload output (the field path's module, named refusal kept, the field path grows) or (b) the render module's own export (no new bytes, no named refusal)?
- **Q3, the wire.** Should the upload form (13.4 MB for the oak at 40, 146 MB for Telperion, transferable, one copy in each realm) be the wire? The f64 `Curve` would be 2.7 times larger and would need a `Curve` decoder on the page.
- **Q4, the worker's home.** Should the package ship the worker module and own its protocol (supersede by request id, errors by name, refusal as a typed message), or does the page bring its own worker?
- **Q5, the unused surface.** Should the renderer stop building the CPU wood surface it only counts? This is separate from the worker, but it is 1.18 GB of Telperion's build.

## 4b. Question 5 built? No: the surface feeds three values, and two cannot be had exactly without it (14 Nov, afternoon)

The host asked for this: the renderer stops building the CPU wood surface, and the two counts it reads come from the wood's layout, exactly. The reading stops at the host's own stop condition. Nothing was changed.

**What the renderer reads from the surface.** `Renderer::submit_at` (`crates/telperion-render/src/lib.rs:240-266`) reads three things from it, not two:

1. `Submitted.wood_vertices` (`mesh.wood_vertices()`).
2. `Submitted.wood_triangles` (`mesh.wood_triangles()`).
3. **`mesh.bounds`**, the union of the surface's box (`SurfaceMesh::bounds`, the box of the float32 vertices, `surface/build.rs:418`) with the leaves' and flowers' boxes (`mesh.rs`, `assembled`). The renderer draws with it: the scale figure's placement (`lib.rs:248`), `self.bounds`, which the sun's caster box reads (`lib.rs:398, 569`) and the hero camera reads (`web.rs:301`), and `Submitted.bounds`.

**Can each be had exactly without the surface?**

- Vertices: yes. The layout gives them (`Layout::extent().positions / 3`). The sizing arithmetic exists as `surface::build::extent` (`#[cfg(test)]`), which runs the paths, `rings::rank` and `Layout::new` and builds no mesh.
- Triangles: **not by layout alone.** `triangles()` is `indices.len() / 3` after the mesh step leaves out every triangle whose float32 corners span no area (`dropped`; `surface/build.rs:107`, `normals::accumulate`). The layout gives `extent.indices / 3`, which equals the count only where `dropped == 0`. Knowing `dropped` needs every ring's float32 positions, which is the sweep. On the six shipped presets without the Two Trees at their seed 42, `dropped` is 0, so the layout count happens to be exact there. That is a measurement, not a rule; the code allows up to `DROPPED_RINGS * 2 * segments`.
- The box: **no.** The curve's box (`Curve::bounds`, the analytic boxes of its clusters with lobes included) is not the polygon's float32 box. With the leaves and flowers joined to each, the tree's box differs on **every** shipped preset measured: ordinary, oak, spruce, beech, birch and date palm, seed 42, the Two Trees not run. The difference is about 1e-9 to 2.1e-5 m, mostly in `min.y` (the root's foot, `-0.0491461455821991` against `-0.04914614486694336` for the ordinary) and in one `max` (ordinary `max.y` 12.289375 against 12.289396). So any change of the box's source moves the figure, the sun's caster box and the hero pose by micrometres. What is drawn is then no longer byte-identical.

**Every reader of the two counts** (src, harness, tests, scripts, crates):

- Page: `src/browser/render.ts:29-30` (the `Submitted` type) and **`harness/GrowerDev.tsx:376`**, a text note ("N wood tris, N wood verts"). That is the only page consumer. Nothing in `src/`, `tests/browser` or `scripts/` reads the renderer's `Submitted` counts.
- Renderer: `crates/telperion-render/src/web.rs:460-461` (JSON out), `submit.rs:20-21` (the struct), `examples/headless.rs:93-94` (printed), `tests/headless.rs:17-18` and `tests/submit.rs:252-255` (equal to the tree's own counts), `tests/conformance.rs:141-143` (`wood_triangles > 0`, a check that something was submitted).
- The rest read `TreeMesh::wood_*` from `mesh::build`, not the renderer: `crates/telperion-jev/src/runner/pins.rs:54` (catalogue pins), then `scripts/catalogue-check.mjs` and `catalogue-pages.mjs`, then the core's examples and suites. They keep the surface on their own path, and this change would not touch them.

So the counts have one display reader and can be deleted, but deleting them does not free the surface. The box is still read from it, and the box is drawn.

**Options for the host** (unranked, none built):

- (a) Delete `woodVertices`/`woodTriangles` from `Submitted`, the harness note and the two equality tests. Take the box from the curve (with the leaves and flowers). The picture moves by up to 21 µm at the figure, the sun's caster box and the hero pose. This needs a byte-change acceptance under "Generator evolution", with visual evidence.
- (b) The same deletion, and the wood's box from a sweep that keeps only the float32 positions: no normals, coords or indices, and no index pass. The box stays exact and the picture byte-identical. The positions are still built, which is about 445 MB of Telperion's 1.18 GB. This is a second consumer of the rings with a new output, so it is a design call.
- (c) Keep the box and the counts, and build the surface less often. That is not a fix.

**Not measured:** the before and after peak resident memory and seconds, and the render Wasm's size, because nothing was changed. One extra observation: running every shipped preset, with Telperion at its own age, in one process under the 4 GB cap was killed (`telperion-heavy`, exit 137) after the date palm. Telperion alone ran inside 4 GB in the morning's measurement. The full-surface peak is the likely cause; it was not measured.

## 4c. Option (a) built (host decision, 14 Nov afternoon)

**Change.**
- `mesh::drawn(family)` and `growth::drawn(state, at, family)` are `mesh::build` and `growth::mesh` with `Request::drawn()`: curve and leaves, no wood. `TreeMesh.wood` is `Option<SurfaceMesh>` (`Some` from `build`). `wood_vertices()` and `wood_triangles()` return `Option<usize>`.
- `assembled` takes the tree's box from `Curve::bounds()` joined with the leaves' and flowers', for every mesh.
- `WebRenderer.setTree`, `drawGrowth` and the `headless` example build through `drawn`.
- `Submitted` loses `wood_vertices` and `wood_triangles`, and so do its JSON (`web.rs`), the TypeScript `Submitted` (`src/browser/render.ts`) and the harness note (`harness/GrowerDev.tsx`).
- `tests/headless.rs` and `tests/submit.rs` drop the two equalities, with a comment. In `tests/conformance.rs`, `drew_the_tree` checks `renderer.curve_bytes() > 0` instead, and the shipped-family test submits `mesh::drawn`.
- `pipeline/drawn.rs` traces a curve asked for without the wood beside the leaves, as the wood was. The first build ran it after the leaves, which made Telperion about 2 s slower than before (20.4 and 20.9 s against 19.1 and 18.5 s). Concurrent tracing removed that.
- CHANGELOG has a line under 0.2.0 "Removed".

**(1) The box, before (CPU surface box ∪ leaves ∪ flowers) and after (curve box ∪ the same), seed 42 at each preset's own age.** In every row, the curve, the leaves, the flowers and the box from `drawn` equal `build`'s.

| preset | largest component difference (m) | component | CPU wood vertices outside the new box (farthest) |
|---|---|---|---|
| ordinary | 2.095e-5 | max.y (12.289375 → 12.289396) | 21 (7.2e-10 m) |
| oregon-white-oak | 5.618e-8 | max.z | 21 (7.7e-10 m) |
| norway-spruce | 2.713e-7 | max.y (11.6368484 → 11.6368482) | 1 (2.7e-7 m) |
| european-beech | 1.093e-5 | max.y | 21 (5.1e-10 m) |
| silver-birch | 1.760e-5 | max.x | 0 |
| date-palm | 5.560e-10 | min.y | 21 (5.6e-10 m) |
| telperion (82) | 1.788e-9 | min.y | 21 (1.8e-9 m) |
| laurelin (80) | 1.192e-9 | min.y | 0 |

**(2) Stills at the hero pose** (`examples/headless --preset <id> --seed 42`, 1024², 4 samples). Raw PNGs are in `raw/wood-surface/`. The sheet is `wood-surface/sheet.jpg`.

| tree | pixels differing | largest channel difference |
|---|---|---|
| oak at 40 | 14 of 1,048,576 | 1/255 |
| Telperion at 82 | 0 | 0 |

The before and after binaries each reproduce their own still byte for byte across runs.

**(3) Native headless submit, whole process** (build, upload, one frame and the PNG), seed 42, through `telperion-heavy --gpu 2`, peak from `ru_maxrss`. Runs 4 and 5 are the final code, back to back on a quiet desk.

| tree | before: s, peak MiB | after: s, peak MiB |
|---|---|---|
| oak at 40 | 1.28, 560 / 1.29, 559 | 1.23, 470 / 1.30, 473 |
| Telperion at 82 | 19.58, 5,466 / 19.56, 5,466 | 17.71, 3,611 / 18.01, 3,606 |

Telperion's peak falls by 1,855 MiB (34 %) and its time by about 1.7 s. The oak's peak falls by 88 MiB.

**Render Wasm size:** `dist/telperion-render.wasm` is 2,181,291 bytes and `dist/telperion.wasm` 1,226,690 (ceilings 3,700,000 and 1,338,000). The growth path is 352,578 Brotli bytes. I did not build a before on the same base. The candidate checkout's build from 12:07, at another commit, is 2,180,768 and 1,226,042.

**Re-pinned:** `catalogue/date-palm/pins.json`, through `telperion_jev::runner::pins::write`, the runner's writer. Only `min[1]` moved: -0.059595685452222824 → -0.059595686267089845, the box. No other pin is checked by a test.

**Tests run (narrow).**
- core lib `pipeline::tests pipeline::executor suite::leaf_bases suite::leaf_base_lattice suite::codominance suite::sweep suite::mesh`: 36 passed, **1 failed** (below).
- core `native_entry`, `ground_reach`: passed.
- jev `runner_accept` (after the re-pin) and `runner_catalogue`: passed.
- render `--lib compile_size`, plus `headless submit conformance curve`, every `bark_*`, `mature_bark colour_cavity smooth_bark person_frame canopy_light lobe_shade`: all passed, on the RTX 3080.
- `npx tsc --noEmit`: 0 errors. vitest `src/browser/render.test.ts` and `module.test.ts`: 9 passed.
- `npm run build`, then `node scripts/artifact-budgets.mjs`: within every ceiling.

**The failing test, a question for the host.** `suite::mesh::every_preset_builds_a_mesh_whose_counts_and_bounds_describe_its_buffers` asserts that every CPU wood vertex lies inside `m.bounds` (`suite/mesh.rs:88`). With the box from the curve, some vertices lie outside it, on six of eight presets (table above). Most are at the root's foot, under 2e-9 m (below one float32 step at that height). The spruce's tip vertex is outside by 2.7e-7 m. The CPU surface is no longer what the box describes, but the test promises "the bounds enclose both parts". I left it as it is: changing it would loosen or redefine a bound. The candidates:
- (i) Check the curve's cluster boxes and the leaves against `m.bounds`: what the renderer uploads.
- (ii) Keep the surface check against a box widened by a stated tolerance: a loosened bound.
- (iii) Join the surface's box into `bounds` where `build` made a surface. The renderer's box would then differ between `build` and `drawn`.

## 5. Tests, sizes, browser steps

No code was changed, so no tests were run beyond the scratch measurement, which built and ran in 19 s through `telperion-heavy 4 --threads 4`. `npm ci`, `npm run build` and `node scripts/artifact-budgets.mjs` were not run, because no shipped artifact changed. Browser steps for the owner are written once a design lands: a check list now would describe code that does not exist.

## 6. Unknowns

- The added Wasm size of option (a) or (b) and of `submit_packed` (not built).
- Whether `telperion.wasm`'s build already holds the `Curve` in memory beside its surface output (not read).
- The page-side copy time for 146 MB into the render module's memory, and whether the module's 4 GiB holds Telperion's packed tree beside its other buffers once growth leaves it (not measured).
