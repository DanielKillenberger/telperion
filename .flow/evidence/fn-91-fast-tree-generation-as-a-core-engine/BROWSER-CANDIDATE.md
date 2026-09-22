> Historical raw-output references: see the [archive and recovery instructions](README.md).

# Shared async GPU generation in the browser

Invocation 5 begins at 1e30cf22. The shared GPU preparation, shaders, contact, stable compaction, bounds and canopy mass implementation now runs through awaited readback, queue completion and error-scope completion. Native synchronous entry points wrap this same implementation. The platform clock uses native Instant or global performance.now, including worker globals.

Experimental `setTreeGpu` is opt-in; `setTree` and default harness behavior remain unchanged. Parameters are parsed before registering a request. Registration happens synchronously before the Promise is returned, so an immediate synchronous replacement invalidates the older request. A guard keeps the request busy until it drops, including stale/disposed/error paths. Construction futures own cloned device/identity handles; no renderer RefCell borrow crosses an await. Successful synchronous tree and growth submissions invalidate the revision. Disposal invalidates, clears cached pipelines and releases the live renderer; pending work can retain its own device resources until resolution but cannot commit. The TypeScript wrapper explicitly rejects new calls after disposal rather than touching its freed Wasm object.

## Proof

Seven selected native generation tests passed in 3.524 seconds through the synchronous wrappers, including the new request-state lifecycle test and prior GPU correctness/ownership tests. Only the renderer library test binary was compiled (43.11 seconds). The renderer Wasm/glue build passed in 15.75 seconds. TypeScript and scoped Rust formatting passed. The initial Wasm check failed on native-only allocation-counter helpers; exposing the existing read-only helpers resolved it, with that first failure retained.

The browser smoke passed GPU count/bounds checks, explicit CPU fallback, invalid JSON, overlap rejection, synchronous replacement without yielding, pending disposal, and a new call after disposal. After stale rejection it compared the replacement tree's hero pose and rendered stats before/after, not only the error text. The fixture reduces authored attractors to 16, but still generates 807079 foliage instances; it is a bounded lifecycle smoke, not a claim of a tiny geometry workload. The final smoke adds the disposed-wrapper assertion; the earlier passing smoke is retained separately. No full workspace gate or unrelated renderer suite was repeated.

## Comparable desktop result

The four fixtures use the existing 1280×720 hero-camera completed-queue protocol, one first build plus five regenerated warm samples. Shader/pipeline construction is included in the first setTreeGpu build. Module/renderer initialization is reported separately and excludes navigation/full public-page download. No specimen output is cached. The actual renderer device queue is fenced after drawing. Compositor/display presentation remains unmeasured.

Comparison is against `browser-completed-baseline.json`, the original pre-optimization browser path, not native timings. The current shared-cache CPU browser matrix was omitted to bound the fifth invocation; this comparison measures the combined improvement, not GPU-only attribution.

| Fixture | Original warm p50 ms | GPU warm p50 ms | Speedup | Whole-module memory bytes |
|---|---:|---:|---:|---:|
| Oak 1 | 1570.0 | 484.2 | 3.24× | 262537216 |
| Oak 7 | 1862.5 | 542.2 | 3.44× | 392232960 |
| Spruce 1 | 7628.6 | 500.8 | 15.23× | 216006656 |
| Spruce 7 | 7384.0 | 477.2 | 15.47× | 209715200 |

All four submitted wood/foliage counts match. Bounds are not byte-identical: maximum coordinate difference is about 0.000024 metres (spruce 7). GPU f32 placement is intentionally a changed encoding/geometry path; native CPU hash parity from prior checkpoints is a separate result. Raw cold timings, warm maxima, per-stage costs and bounds are retained in `browser-completed-gpu.json` and `browser-gpu-summary.json`; five samples do not establish a robust p95.

Oak's warm wood construction remains 229–255 ms, skeleton 73–81 ms, descriptors 19–22 ms. Spruce wood is 165–172 ms, descriptors 72–77 ms and placement wait 33–42 ms. Browser transfer/draw/completion beyond preparation remains included in the headline totals.

Memory is the initialized renderer module's actual Wasm memory size, read from the cached module instance, not per-renderer allocation or total CPU/GPU peak. GPU counters retain explicit compute/live buffers and their existing omissions. Browser/driver/compiler/transient device allocations and all resident GPU memory are not fully measured. The previous native GPU CPU-output memory miss remains; these browser numbers do not waive it. The 100 ms stretch target is not met, and the required 10× gain is not met for either oak fixture.

Close-view capture and owner assessment, if supplied by the host, have separate provenance; the prior owner verdict covers native seed-1 hero images only. Phone hardware and supported motion/view coverage remain unqualified.
