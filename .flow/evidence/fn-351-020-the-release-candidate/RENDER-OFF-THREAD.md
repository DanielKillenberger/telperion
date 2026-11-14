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

## 5. Tests, sizes, browser steps

No code was changed, so no tests were run beyond the scratch measurement, which built and ran in 19 s through `telperion-heavy 4 --threads 4`. `npm ci`, `npm run build` and `node scripts/artifact-budgets.mjs` were not run, because no shipped artifact changed. Browser steps for the owner are written once a design lands: a check list now would describe code that does not exist.

## 6. Unknowns

- The added Wasm size of option (a) or (b) and of `submit_packed` (not built).
- Whether `telperion.wasm`'s build already holds the `Curve` in memory beside its surface output (not read).
- The page-side copy time for 146 MB into the render module's memory, and whether the module's 4 GiB holds Telperion's packed tree beside its other buffers once growth leaves it (not measured).
