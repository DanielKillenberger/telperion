# FN8: complete Rust generator measurement

Recorded 2026-09-05. The final browser builder is **2.91× faster for Telperion, 2.71× for Laurelin and 2.81× for comparison** than finished FN6, including Wasm request/transfer and Three.js materialization. Every measured build preserves node, vertex, triangle, placed/retained leaf, twig and handoff counts. This is a CPU generation improvement; it does not meet the separate 2 ms GPU rendering target or establish a total-memory reduction.

## Matched workload and protocol

Reference: final FN6 `fdafb099b1495519de75a6b9a66d37f7d07e47bd`. The reference server's source, harness and lockfile diff against that revision was empty; [context](fn6-context.json) records its served HEAD. Rust starts from integrated task6 `8a56a41c925ad4a7f926e3e2decb4aa3d0438a34`, with the profile-distance optimization described below and removal of unused legacy code. [Final production source hashes](source-hashes.json) identify the resulting implementation.

Both browser runs use headless Chromium 151, real NVIDIA RTX 3080 graphics, AMD Ryzen 9 5950X, 1600×1000 CSS pixels, raw/native DPR1, authored preset seeds/parameters, surface and foliage enabled, neutral clay, and the same framing. The machine is a shared interactive Linux workstation, without CPU pinning or clock control. Agent-heavy work was paused during CPU/GPU and native timing. This is one paired session, not a statistical confidence claim.

Protocol: one complete CPU warmup followed by five measured builds per subject; GPU queries use eight warmup frames and twenty samples at each DPR 2, 1, 0.7, 0.5 and 0.25. Vsync-off flags match FN6. Raw samples include warmups, stage timings, renderer/driver/host metadata and real query times: [JS](fn6-headless.json), [Rust](rust-headless.json). Unsupported timer behavior is exercised and reports no frame-time substitute. Earlier headed observations are supplemental only; paired headless runs replace them because the user requested no further visible browser windows.

| Subject | Nodes | Wood vertices | Wood triangles | Retained leaves |
|---|---:|---:|---:|---:|
| Telperion | 175,035 | 6,554,490 | 12,888,512 | 1,349,630 |
| Laurelin | 104,207 | 2,246,930 | 4,365,056 | 798,553 |
| Comparison | 279,242 | 8,801,420 | 17,253,568 | 2,148,183 |

## Complete browser latency

Medians in milliseconds, excluding warmup. `setTree` additionally includes previous-scene disposal, replacement and room measurement. Comparison builder statistics sum the two subjects, matching FN6; `setTree` also observes their group assembly.

| Subject | JS builder | Rust builder | Speedup | JS setTree | Rust setTree |
|---|---:|---:|---:|---:|---:|
| Telperion | 4719.9 | 1620.8 | 2.91× | 4927.2 | 1621.4 |
| Laurelin | 2190.4 | 809.3 | 2.71× | 2306.7 | 809.3 |
| Comparison | 7232.6 | 2577.9 | 2.81× | 7558.4 | 2579.8 |

Rust's adapter uses native immutable bounds and directly adopts owned copied buffers. This removes the previous giant instance-bounds rescan and duplicate matrix allocation. The full builder includes that integration change; the gain cannot be assigned solely to language or compiler.

| Subject | Growth | Surface | Foliage including exact bounds | Core assembly | Output transfer | Three materialization |
|---|---:|---:|---:|---:|---:|---:|
| Telperion | 323.1 | 469.2 | 761.6 | 1556.4 | 55.9 | 0.3 |
| Laurelin | 173.7 | 169.8 | 437.2 | 780.8 | 28.5 | 0.3 |
| Comparison | 508.9 | 651.9 | 1219.4 | 2387.6 | 209.4 | 0.2 |

These are independent medians and need not add. Core assembly includes diagnostics; output transfer includes metadata parsing and copying typed arrays. Request encoding/copy and binding parsing/serialization belong to the outer build, not transfer. Field is not requested in these render workloads and costs zero. No cold-start or deployment startup gain is claimed.

### Investigated regression

The first complete [unoptimized Rust run](rust-before.json) had builder medians 4642.5 / 2514.8 / 7234.2 ms: essentially flat for Telperion/comparison and 15% slower for Laurelin. Telperion foliage alone took about 3.36 s. A sampled [Wasm CPU profile summary](profile-summary.json) ([raw profile](wasm-profile.json)) identified software `hypot` and its software fused multiply-add implementation as the dominant cost.

`envelope::distance_to_profile` previously called `hypot` for every one of 128 profile segments. It now minimizes squared distances and takes one square root; exceptional overflow/underflow uses the robust `hypot` path. The same kernel serves crown sampling, branch shedding and leaf culling. No geometry, sampling resolution, shell rule or foliage count was reduced. `profile_distance.rs` checks projection, duplicates, empty profiles and tiny/huge finite distances against the prior arithmetic; foliage tests and complete frozen-reference browser checks validate retention. The property test passed before and after this performance change; the red evidence is the measured Laurelin performance regression, not a fabricated failing correctness test.

One optimized measurement attempt failed before collecting samples because Vite HMR exposed two module instances after regenerated metadata changed. Restarting the server resolved it. That attempt is inconclusive and excluded, with no production workaround. The earlier interrupted headed FN6 comparison was likewise excluded; the headline data contains complete paired headless runs.

## Rendering remains a separate limit

Real GPU-query medians in milliseconds. Native DPR1 is the budget comparison; DPR2 is a separate high-DPI workload. Complete sweeps and all twenty samples per point are in the raw JSON.

| Subject | JS DPR1 | Rust DPR1 | JS DPR2 | Rust DPR2 |
|---|---:|---:|---:|---:|
| Telperion | 6.066 | 6.277 | 7.027 | 7.304 |
| Laurelin | 3.665 | 3.723 | 4.547 | 4.841 |
| Comparison | 9.714 | 9.632 | 10.854 | 10.663 |

Telperion and Laurelin regress modestly in this GPU observation (about 3.5%/1.6% at DPR1, 4.0%/6.5% at DPR2); comparison improves slightly. No rendering change was made to erase these observations, and this session does not isolate their cause. Lower DPR does not bring any full-tree subject under 2 ms. CPU gains do not imply a GPU-frame gain.

## Native and mesh-free consumers

Native release execution uses the same core, one warmup and five samples in one separate process per case. Full native builds include growth/radii, surface/normals, foliage placement/cull and exact bounds; they exclude process startup, JS binding, rendering and final drop. The ordinary native family uses its authored default step 0.022; the frozen ordinary comparison fixture uses 0.02 and is a separate correctness workload.

| Native subject | Full build ms | Growth | Surface | Placement | Cull | Bounds | Process peak RSS KiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| Ordinary | 73.36 | 21.95 | 20.23 | 10.11 | 17.81 | 3.32 | 30,728 |
| Telperion | 1224.80 | 293.36 | 361.28 | 184.27 | 312.06 | 72.73 | 518,448 |
| Laurelin | 611.46 | 152.77 | 128.08 | 111.67 | 174.39 | 43.10 | 255,748 |

[Native raw samples and RSS](native/) come from `crates/telperion-core/examples/measure.rs`. `/proc/self/status` VmHWM is whole-process peak RSS across six builds, including their allocations and benchmark bookkeeping; VmRSS is sampled after drops. Neither is comparable to Wasm capacity or total browser memory.

Mesh-free measurements request wood+foliage fields only: no wood surface, render buffers or exact rendered-foliage bounds scan. Both native and browser consumers query 32³ closed cubic cells spanning field bounds, obtaining nonzero wood and foliage occupancy. The browser query time includes packing transfer and copying result flags; native queries return directly.

| Consumer / subject | Full field build ms | Field index ms | 32,768 queries ms | Owned field capacity bytes |
|---|---:|---:|---:|---:|
| Native ordinary | 63.70 | 14.86 | 2.85 | 7,867,816 |
| Browser ordinary | 90.00 | 28.30 | 4.30 | 7,867,816 |
| Native Telperion | 1131.22 | 351.79 | 5.36 | 151,471,384 |
| Browser Telperion | 1638.50 | 677.10 | 6.40 | 151,471,384 |

Field index time excludes required growth and retained-leaf construction; full field time includes them. [Browser field samples](browser-field.json) record stage selection, query counts and capacity. Native field process peaks were 20,756 KiB ordinary and 353,340 KiB Telperion. Occupancy is the core's tapered-wood/leaf-support approximation, not exact voxelization of the lobed surface.

## Memory domains and release

Separate headless passes use CDP `HeapProfiler.collectGarbage` then `Runtime.getHeapUsage`, after rendered frames, with three build/scene-dispose cycles per subject and a fresh page for each. Forced GC is never part of latency samples. Raw domain records: [JS](fn6-memory.json), [Rust](rust-memory.json).

| Subject | JS retained backing bytes | Rust retained backing bytes | Rust Wasm capacity bytes |
|---|---:|---:|---:|
| Telperion | 400,846,004 | 400,394,455 | 551,944,192 |
| Laurelin | 159,914,164 | 159,462,615 | 274,792,448 |
| Comparison | 558,260,916 | 557,809,367 | 551,944,192 |

Backing storage and Wasm capacity remained exactly stable across all three equal build/dispose cycles. After scene disposal, backing storage returned to 2,499,252 bytes JS and 2,047,703 bytes Rust. Observed V8 used heap was about 5.1–5.4 MB JS and 3.3–3.5 MB Rust, including harness/module state; exact used/allocated/embedder domains are retained in the raw data. Small fixed differences include harness/module overhead and are not presented as geometry savings.

Wasm retains about **526 MiB** capacity for Telperion/comparison and **262 MiB** for Laurelin after native output release. That is an additional persistent capacity domain; the shared engine reuses it. CDP backing storage here does not include that capacity: dropping the engine leaves that CDP number unchanged. `engine.dispose()` removes the instance reference and later calls fail; physical reclamation is eligible for host GC but not directly measured by this probe.

Total browser/process peak, live Wasm allocator bytes and GPU allocation bytes are unavailable. Scene replacement intentionally overlaps old and new representations until success; post-GC snapshots miss that transient peak. No total-memory savings or leak-free lifetime theorem follows from three stable cycles. The extra Wasm capacity and broadly unchanged owned render payload must remain visible when assessing the migration's memory cost.

## Lean core and verification

There is one production generator. The deleted TypeScript code survives only in Git archives for reproducible reference exports. [Test ownership and explicit retirements](../../../tests/migration/README.md#retained-test-ownership) map the removed suites to native and consumer invariants. The same guide demonstrates a botanical branch-law change and an independent surface-resolution change through their owning modules; no compatibility generator or speculative public option was added.

[Raw line counts](loc.json) include comments and blanks, with generated code separated: FN6 core 5,837 lines; final Rust core 3,564 (39% fewer raw core lines), Rust binding 518, browser adapter/public entry 164, generated preset metadata 348. Tests are separate: native integration tests 1,996, native inline tests 64, browser integration runners 237, harness tests 1,494. Native examples add 106. This is not a 39% whole-project reduction, a semantic code-LOC metric, or a target used to shape the implementation.

Clean `npm ci`, local pinned-toolchain native/Wasm release builds, release workspace tests, rustfmt, strict clippy, all 100 harness tests, typecheck, packaged library build, actual headless binding/UI checks and complete ordinary/Telperion/Laurelin/comparison migration checks are recorded in [verification logs](verification.json). The release suite has 39 passing tests and four optional archived-fixture tests ignored; native growth and foliage comparisons were then explicitly run on all seven frozen classes, in addition to the full browser fixture run. [Current captures and numeric records](browser/migration.json) retain the visual evidence. Numerical comparison retains exact topology/indices and leaf membership, with previously diagnosed small float32 position/normal drift from FN6. Broader species realism, lifecycle simulation and GPU geometry reduction remain outside FN8.

## Reproduction

Follow the root README's pinned Rust/npm setup. Start the finished FN6 archive in one checkout/server and this Rust checkout in another. Both benchmark URLs must serve their respective source harness; restart Vite after regenerating Wasm before sampling.

```sh
IMPLEMENTATION=fn6 BENCHMARK_URL=http://127.0.0.1:5181 BENCHMARK_OUTPUT=/tmp/fn8-headless-fn6-baseline node .flow/evidence/fn8/measure.mjs
BENCHMARK_URL=http://127.0.0.1:5185 BENCHMARK_OUTPUT=/tmp/fn8-headless-rust-final node .flow/evidence/fn8/measure.mjs
IMPLEMENTATION=fn6 BROWSER_URL=http://127.0.0.1:5181 MEMORY_OUTPUT=/tmp/fn8-headless-fn6-memory node .flow/evidence/fn8/memory.mjs
BROWSER_URL=http://127.0.0.1:5185 MEMORY_OUTPUT=/tmp/fn8-rust-memory node .flow/evidence/fn8/memory.mjs
BROWSER_URL=http://127.0.0.1:5185 node .flow/evidence/fn8/field.mjs
cargo build --release -p telperion-core --example measure
target/release/examples/measure telperion
target/release/examples/measure laurelin
target/release/examples/measure ordinary
target/release/examples/measure telperion --field
target/release/examples/measure ordinary --field
```

The GPU runner deliberately requires an RTX 3080 with the actual timer extension for this named-machine comparison. Other hardware must receive a separately labelled report. `PLAYWRIGHT_MODULE` and `CHROMIUM_EXECUTABLE` are optional host overrides; all launches are headless and no permission auto-accept or global browser settings are used. Reference fixture export and full browser comparison commands are documented in the migration guide; large binary fixtures are regenerated, not committed.
