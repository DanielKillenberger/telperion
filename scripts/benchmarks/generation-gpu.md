# Indexed WebGPU field experiment

`generation-gpu.mjs` is a benchmark-only asynchronous candidate. It is not exported
by the package or selected by the engine. It copies the schema-1 snapshot into f32
storage buffers and traverses the existing wood and foliage BVHs in WGSL. Wood
retains the tapered sphere-sweep predicate and cube circumsphere inflation;
foliage retains closed AABB overlap. A 64-entry traversal stack, validated topology,
node-visit guard, dispatch bounds and device limits bound execution.

Reproduce after `npm run wasm:build` and serving this checkout with Vite:

```sh
PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs \
  BROWSER_URL=http://127.0.0.1:5188 \
  node tests/browser/generation-gpu.mjs --hardware

PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs \
  BROWSER_URL=http://127.0.0.1:5188 GENERATION_OUTPUT=/tmp/fn12-gpu \
  node scripts/benchmarks/generation-gpu-run.mjs
```

Omit `PLAYWRIGHT_MODULE` with a normal local Playwright installation. Set
`CHROMIUM_EXECUTABLE` for another Chromium binary. The runner's recorded Linux
Vulkan/ANGLE flags are part of this measured configuration, not a statement that
all browsers require them. WebGL2 initialization and a 500 ms wait preceded the
adapter request on this machine, because an immediate request could return null.
Only the isolated blank benchmark page is rendered. Acquire an exclusive GPU
measurement window, also avoiding CPU test/build load; do not run fn13 alongside it.

## Timing and comparison contract

The runner builds full Ordinary and Telperion presets, with their unmodified
parameters/seeds and field output only. It alternates subject order and CPU/GPU
query order, runs one warmup and five measured samples, and preserves every sample.
The CPU denominator is the indexed Rust/Wasm `field.query` call in the same browser,
including input transfer and copied result flags. No JavaScript traversal timing is
used as a denominator. Parameters, every source array, each canonical grid/contact
input and CPU flags are independently checked against task 1's hashes.

Each sample first measures a source-resident session with two CPU/GPU pairs at each
resolution. Input validation and packing, transient allocation, upload, submission,
synchronization, mapped readback, copied flags and transient destruction are charged
to the GPU query. The second pair supplies the resident comparison and deterministic
repeat check. GPU timestamps cover only the compute pass and are supplementary.

A separate cold probe for **each** resolution measures one direct caller wall-clock
interval: fresh snapshot extraction, validation/packing, adapter/device and pipeline
creation, source upload, query, complete result copying, and device disposal. No CPU
reference call or hashing runs inside that interval. Its CPU reference is separately
paired immediately before or after the probe. Original resident-session resources
are disposed first. Cell generation is outside both query-only denominators, and is
added alongside the common CPU build to the whole-build totals.

The two-query amortization values are explicitly **summed estimates** from the two
actual resident query calls plus one preparation/snapshot and measured session
teardown, divided by two. They are not direct uninterrupted lifecycle stopwatches.
They do not justify adoption. No benchmark IO or hashing time is charged to either
backend. Raw phase and caller timings remain visible; the direct cold caller total
is authoritative rather than a sum of internal timers.

Qualification remains fixed: at least 20% median end-to-end improvement, GPU maximum
below CPU minimum across measured pairs, no worse maximum (the five-sample p95),
exact occupancy bits and deterministic repeats, with explicit memory cost. Five
observations do not establish a population tail distribution. Precision failure
rejects the uncorrected candidate even if a timing subset improves.

## Precision and failure behavior

WGSL f32 rounding is not the CPU's f64 contract. No epsilon relaxation or contact
correction is used. Valid cells outside the safe f32 arithmetic range are checked
against both f64 root bounds; only conclusively outside cells become GPU zero flags.
This preprocessing count and time are recorded. A large cell that could overlap
returns `precision-range` for the whole request. Nonfinite/negative/malformed input
and f64 overflow return explicit errors.

The snapshot is copied synchronously during preparation, so its owned source can be
released independently. Source handles still obey the normal engine revision rules
when extracted; already owned snapshots have no dependency on later engine rebuilds.
The GPU session owns its device and buffers. Disposal is idempotent, and later queries
fail. Validation failures leave a usable session; allocation, execution, readback,
traversal, loss and timeout failures destroy it. Concurrent use reports `busy`.
No failure returns partial occupancy and no whole-request CPU fallback is implicit.

The tests distinguish actual hardware from controlled API doubles:

- Actual WGSL tests cover tapered wood and foliage, closed contacts, point/outside
  queries, empty fields/batches, large outside cells, repeats, reuse after bad input,
  disposal, timestamp queries disabled, and device loss induced by real
  `GPUDevice.destroy()`.
- Controlled doubles cover absent/null adapters, missing timestamp support, shader
  and pipeline failure, device/allocation/submission/readback failure, timeout,
  traversal failure and cleanup counts. A late-created device is destroyed even
  after its request times out. These are failure-path tests, not claims that the
  corresponding hardware failures occurred during measurement.
- Pure input tests cover malformed topology, nonfinite/negative/overflow cells,
  unsafe overlapping f32 coordinates, and buffer/dispatch limits.

Creating a fresh session is the recovery path after resource failure. The experiment
makes no universal support or exactness guarantee and remains outside production.

## Memory accounting

All sizes are bytes, not OS resident-memory measurements. Source f64 arrays and
packed f32/topology arrays coexist during upload. Validation scratch, GPU source
buffers, query cells, uniforms, storage output, MAP_READ staging, copied CPU/GPU
flags and timestamp resolve/readback buffers are explicit in each row. Two timestamp
query slots are recorded; their driver storage is opaque. `writeBuffer` staging is
also opaque: its payload bytes are recorded as a conservative payload allocation
allowance, not a measured total driver-memory bound. Garbage-collection retention,
browser overhead and driver allocations cannot be inferred from these counts.

`accountedColdBufferPeakBytes` and `accountedResidentBufferPeakBytes` are bounds for
the named allocations, with their maximum recorded separately. Native field and
snapshot/query staging are contained in Wasm high-water and must not be added to
that high-water a second time. The Wasm number is reported separately from JS/GPU
buffers. This distinction matters especially for the giant snapshot.

## Skeleton and field-construction screening

The baseline's measured CPU stage costs remain in `generation-baseline.json`; the
paired GPU run also records CPU growth, foliage prerequisite, index construction
and whole-build timings for every full specimen. These CPU stages **did not execute
on the GPU**. Growth's nearest/alive updates, ordered pull reductions, parent-ordered
appends, shared caps, radius propagation and successive round dependencies are a
code-level feasibility analysis documented in `generation.md`. Per-round counters,
transfer floors and GPU growth substeps were not measured: GPU growth performance
is **inconclusive**, and was not chosen as the bounded first prototype.

Leaf-bound transformation is independently parallelizable, but field construction
also needs median partitions and exact index membership before queries can run.
The query candidate reuses the CPU-built indices; its timings cannot establish a
GPU construction speedup. GPU field-construction performance is **inconclusive**.
The measurement rejects this query implementation under the fixed gates; it neither
proves that all GPU algorithms are slower nor establishes an accelerated growth or
construction backend.

## Recorded result: September 6, 2026

Chromium 151.0.7922.173 used the NVIDIA GeForce RTX 3080 through Vulkan/ANGLE;
WebGPU reported NVIDIA Ampere, a non-fallback adapter and available timestamp queries.
All twelve full-preset builds completed without caps. Every canonical source/input/CPU
hash matched task 1, and all repeated GPU hashes matched, including independently
created cold sessions. All differing cells and their exact CPU/GPU flags are in the
raw evidence. No sample was removed for being slow.

Times are milliseconds, median (maximum). Resident CPU is its separately paired
second CPU call; cold CPU is paired with the direct cold lifecycle.

| Specimen / batch | Cold CPU | Cold GPU including disposal | Resident CPU | Resident GPU | Mismatching cells |
|---|---:|---:|---:|---:|---:|
| ordinary / 32 | 3.9 (4.1) | 102.2 (104.4) | 3.6 (4.0) | 4.8 (9.1) | 0 |
| ordinary / 64 | 21.3 (22.1) | 101.1 (102.4) | 19.9 (20.2) | 12.1 (13.7) | 2 |
| ordinary / boundary | 0.3 (0.5) | 87.5 (101.1) | 0.2 (0.3) | 3.3 (3.5) | 65 |
| telperion / 32 | 6.9 (7.6) | 244.8 (292.5) | 5.9 (6.2) | 5.0 (9.0) | 0 |
| telperion / 64 | 26.3 (27.0) | 261.4 (318.1) | 25.4 (26.1) | 11.8 (18.0) | 0 |
| telperion / boundary | 0.6 (0.7) | 245.1 (370.1) | 0.3 (0.4) | 3.4 (3.4) | 54 |

Every **cold** workload fails the fixed timing gates. All two-query amortization
estimates also fail. The final giant 64³ resident submeasurement passes the local
20%/non-overlap/tail/grid-parity gates, and is retained as a positive observation.
It is not a qualified general API: giant contact probes have 54 mismatching cells
(six false-negative cells), and previous runs had variable resident timings. Ordinary
64³ differs in two foliage cells, one a false negative; Ordinary contacts differ in
65 cells, including 21 false-negative cells. Both wood and foliage contact predicates
show precision drift. No correction or production integration was attempted.

The giant source snapshot is 129,078,104 bytes; packed arrays and GPU source
buffers are each 72,333,272 bytes. Its named cold JS/GPU allocation allowance peaks
at 347,257,564 bytes, separately from 380,174,336 bytes of Wasm high-water. Ordinary
64³ peaks at 32,752,880 named JS/GPU bytes and 32,440,320 Wasm bytes. These are the
allocation domains described above, not physical memory measurements.

The final run also retains these **CPU** prerequisite medians:

| Stage | Ordinary ms | Telperion ms |
|---|---:|---:|
| Growth | 11.3 | 312.0 |
| Foliage prerequisite | 25.2 | 640.8 |
| Field index | 23.0 | 662.5 |
| Whole mesh-free build | 60.0 | 1615.4 |

Raw receipts under `.flow/evidence/fn12/`:

- `gpu-results.json`: authoritative direct cold caller timing, all source/input/output
  hashes, mismatching cells, raw phases, memory, hardware and final gate classifications.
- `gpu-input-verification.json`: independent comparison with task 1 and source hashes.
- `gpu-tests.json`: hardware checks and explicitly labelled controlled failures.
- `gpu-preliminary.json`: initial complete exploratory run, before extended accounting.
- `gpu-internal-timing.json`: next complete run, with internal cold stage sums but
  without the authoritative direct cold lifecycle stopwatch. Preserved as variability
  evidence; it is superseded for cold totals, not discarded as an inconvenient sample.

The first two runs failed every resident timing qualification; the final run passes
only the giant 64³ resident submeasurement. This variation limits performance claims.
Task 3 should retain CPU production behavior: no complete accelerated lifecycle
qualifies, and the uncorrected candidate fails the field precision contract.

Implementation references: [WebGPU buffer mapping and resource rules](https://www.w3.org/TR/webgpu/),
[WGSL floating-point evaluation](https://www.w3.org/TR/WGSL/#floating-point-evaluation),
and [official compute/timestamp samples](https://github.com/webgpu/webgpu-samples).
