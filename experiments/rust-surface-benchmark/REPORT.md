# Surface benchmark: measured decision

The browser Wasm prototype materially reduces surface-build latency compared with
the preallocated TypeScript baseline on both shipped presets. This supports a
bounded surface-core follow-up, but does **not** yet justify a broader Rust core
migration: comparable total peak memory and full-lifecycle latency remain unmeasured.

## Recorded run

Recorded 2026-09-05T13:00:47.464Z. Raw samples, fixture parameters/hashes, output hashes,
source hashes and build metadata are in [measurements.json](results/measurements.json).
Reproduction and timer boundaries are in [README.md](README.md).

Host: AMD Ryzen 9 5950X 16-Core Processor, 32 logical CPUs, 31.26 GiB RAM;
Linux 7.1.9-arch1-2; Node v26.8.1; Three 0.185.1;
rustc 1.98.1 (48a229cea 2026-09-01); cargo 1.98.1 (797e8a9bc 2026-08-05); Chromium 151.0.7922.173 Arch Linux.
Both Rust targets use release optimization, LTO, one codegen unit, and warnings
denied. Browser timing used actual headless Chromium with GPU disabled. Native
ran in separate Linux processes. This is one machine/session, without CPU pinning
or clock control; no statistical confidence interval is claimed.

| Input | Nodes | Vertices | Triangles | Packed input bytes | Output bytes |
|---|---:|---:|---:|---:|---:|
| telperion | 15,826 | 624,088 | 1,224,048 | 759,736 | 22,177,632 |
| laurelin | 43,191 | 983,340 | 1,901,728 | 2,073,256 | 34,620,816 |

## Browser latency

Milliseconds, median / p95 of ten measured samples after three warmups per path.
Order rotates by repetition. At n=10, nearest-rank p95 is the maximum sample.

| Input | Implementation | Compute median / p95 | Caller median / p95 |
|---|---|---:|---:|
| telperion | reference | 101.23 / 110.55 | 103.35 / 129.35 |
| telperion | optimized | 70.64 / 83.93 | 74.10 / 82.71 |
| telperion | wasm | 22.17 / 29.57 | 27.08 / 36.56 |
| laurelin | reference | 179.28 / 261.33 | 170.50 / 249.21 |
| laurelin | optimized | 129.74 / 147.52 | 126.29 / 435.12 |
| laurelin | wasm | 36.70 / 116.85 | 45.73 / 83.31 |

TS compute and caller have the same boundary and independently sampled timings;
their difference is run noise, not binding overhead. The final run contains large
tail samples (including a 435 ms optimized Laurelin caller build); all samples are
retained. Scheduling and GC contributions were not isolated, so these p95 values
are observations, not stable tail-latency guarantees. Wasm compute excludes input
packing/copy and output copying, while its caller measurement includes all of them
and returns independently owned JS arrays, matching TS output ownership.

For telperion, preallocation improves the reference caller median by 1.39×. Wasm
then improves the optimized caller median by 2.74×, saving 47.02 ms.

For laurelin, preallocation improves the reference caller median by 1.35×. Wasm
then improves the optimized caller median by 2.76×, saving 80.56 ms.

The extra Wasm caller cost remains visible; compute-only ratios would overstate
the practical gain. Rust also changes internal representation and allocations,
so the remaining difference cannot be assigned solely to language or compiler.

One cold loopback fetch/compile/instantiate observation was 3.53 ms.
That excludes application startup and is not a deployment cold-start prediction.

## Native latency (separate environment)

| Input | Compute median / p95, ms | Caller median / p95, ms | Process peak RSS, KiB |
|---|---:|---:|---:|
| telperion | 18.20 / 20.21 | 18.54 / 24.62 | 50,192 |
| laurelin | 30.53 / 32.93 | 31.00 / 33.89 | 82,652 |

Native compute uses already decoded packed input. Caller includes decoding bytes
and the shared build, returning owned Rust vectors. Neither includes process
startup, file I/O or final output serialization. Three build warmups precede ten
samples per path. These are not browser migration speedups.

## Memory evidence and limits

| Input | Observed JS heap maximum, MiB | Wasm linear-memory high-water capacity, MiB | Explicit output, MiB |
|---|---:|---:|---:|
| telperion | 419.90 | 60.00 | 21.15 |
| laurelin | 471.19 | 103.62 | 33.02 |

Native RSS is Linux `/proc/self/status` VmHWM, read after correctness-output
serialization; it includes the process, input, mesher and serialization storage,
not an isolated compute allocation peak. Browser JS heap is sampled via Chromium
`performance.memory.usedJSHeapSize` after timed calls across all implementations.
It includes retained correctness meshes/harness work, misses transient peaks, and
cannot attribute memory savings to an implementation. Wasm linear-memory capacity
is monotonic over the same module and includes prior cases plus its allocator;
it is neither used bytes nor total browser memory. Caller-owned output copies
also live outside Wasm. Explicit buffer byte counts describe payloads, not peaks.

Comparable per-implementation total browser peak memory is **unavailable**
(`totalBrowserPeak: null`). No native-RSS-versus-Wasm-capacity comparison is valid.
This experiment establishes no total-memory reduction.

## Correctness and recommendation

All 13 cases passed cross-implementation counts, exact indices/connectivity/winding,
finite positions, bounds, target determinism and scale-aware positional tolerance.
Both preset outputs had zero observed positional difference; the independent
`8 * 2^-23 * max(1, maxAbsInputCoordinate)` threshold was retained. Small geometry
also passed closed-edge orientation checks. Frozen source hashes match the named
Git revision; rebuilding the complete fixture manifest produced identical bytes.

Keep the production core unchanged for now. The measured CPU gain supports a
follow-up evaluating the same coarse surface boundary in the actual consumer,
including packaging, cancellation/lifetime handling and comparable total memory.
The optimized TS result is a useful lower-risk baseline for that decision.
Only after end-to-end lifecycle and memory evidence should the project commit
to a broader Rust migration. Growth, radius solving, foliage, renderer upload,
GPU frame time and native engine integration were not benchmarked here. Surface
CPU gains imply no GPU-frame or full-lifecycle gain.

## 2026-09-05: FN8 architecture decision

FN8 replaces the complete production generator with one Rust core, exposed through
native modules and a Wasm browser binding. Structure, wood surface, foliage and
mesh-free occupancy have separate consumers. The TypeScript generator is removed;
this experiment and its frozen reference remain historical artifacts. The earlier
recommendation above belongs to the surface-only experiment and is superseded by
that architecture decision. Its measured results have not been changed.

The [FN8 report](../../.flow/evidence/fn8/REPORT.md) owns the matched full-builder,
transfer, memory-domain and GPU observations. None of the surface-only numbers in
this report establish a whole-engine speedup, total-memory reduction or GPU win.
