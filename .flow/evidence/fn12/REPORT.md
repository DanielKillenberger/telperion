# FN12: retain CPU generation and field queries

No production GPU path qualifies. The benchmark-only WebGPU indexed query candidate
fails every cold timing gate and the CPU field's occupancy contract. The optional
owned CPU `field.snapshot()` boundary remains; the rejected GPU implementation,
runner, dedicated tests and operating guide were removed on September 7. The
snapshot API and CPU measurement helpers support fn18's field-generation checks.
Generation, geometry, botanical inputs, synchronous queries
and default output selection retain their existing behavior. No asynchronous GPU
package entry point or automatic backend switching was added.

## Decision by requirement and stage

| Requirement / stage | Ordinary | Telperion giant | Decision |
|---|---|---|---|
| R1: skeleton growth | CPU median 11.3 ms | CPU median 312.0 ms | GPU performance inconclusive; dependency screening only |
| R1: foliage prerequisite | CPU median 25.2 ms | CPU median 640.8 ms | GPU performance inconclusive; no construction experiment |
| R1: field index construction | CPU median 23.0 ms | CPU median 662.5 ms | GPU performance inconclusive; candidate consumes CPU-built indices |
| R1/R2: cold field queries | All measured batches slower | All measured batches slower | Reject adoption |
| R2: occupancy / repeatability | 64³ and contact mismatches repeat | Contact mismatches repeat | Reject uncorrected f32 semantics |
| R3: support / failure | Explicit complete success or failure in experiment | Same contract | Retain CPU production API; no implicit fallback |

Growth includes radius solving and local branches. The CPU baseline and
[feasibility analysis](../../../scripts/benchmarks/generation.md#skeleton-feasibility)
identify ordered attractor reductions, parent-ordered appends, shared budgets and
successive round dependencies. No per-round costs, transfer floors, GPU growth
substeps or GPU field construction were measured. The spec's per-round profiling
ambition remains unresolved; the query measurements cannot answer it. The snapshot
boundary is architecturally portable and engine-independent, but no external engine
integration was demonstrated.

## Matched query evidence

The [authoritative raw run](gpu-results.json) preserves one warmup and five paired
measured samples for each full, unmodified preset, alternating specimen and CPU/GPU
query order. All twelve builds completed without caps. Field-only output retains
wood and foliage, without render meshes. [Independent input verification](gpu-input-verification.json)
checks parameters, snapshot arrays, grid/contact cells and CPU flags against the
[CPU baseline](../../../scripts/benchmarks/generation-baseline.json). All repeat
hashes agree, including the GPU's incorrect cells; repeatability is not correctness.

Times are milliseconds, median (maximum). Cold CPU is the separately paired indexed
Rust/Wasm query, including input and copied output. Cold GPU is a direct stopwatch
from fresh snapshot extraction through packing, device/pipeline setup, upload,
submission, synchronization, readback, copied output and disposal. Cell generation
is outside both query timings; whole-build totals also account for it and the
shared CPU prerequisites. Five-sample p95 is the maximum, not a population estimate.

| Batch | CPU cold | GPU cold | CPU resident | GPU resident | Different cells |
|---|---:|---:|---:|---:|---:|
| Ordinary 32³ | 3.9 (4.1) | 102.2 (104.4) | 3.6 (4.0) | 4.8 (9.1) | 0 |
| Ordinary 64³ | 21.3 (22.1) | 101.1 (102.4) | 19.9 (20.2) | 12.1 (13.7) | 2 |
| Ordinary contacts | 0.3 (0.5) | 87.5 (101.1) | 0.2 (0.3) | 3.3 (3.5) | 65 |
| Telperion 32³ | 6.9 (7.6) | 244.8 (292.5) | 5.9 (6.2) | 5.0 (9.0) | 0 |
| Telperion 64³ | 26.3 (27.0) | 261.4 (318.1) | 25.4 (26.1) | 11.8 (18.0) | 0 |
| Telperion contacts | 0.6 (0.7) | 245.1 (370.1) | 0.3 (0.4) | 3.4 (3.4) | 54 |

The fixed gates require at least 20% median gain, GPU maximum below CPU minimum,
no worse maximum and exact, repeatable occupancy with memory accounted. Every cold
workload and every two-query amortization estimate fails. The latter are summed
estimates, not uninterrupted lifecycle measurements. Whole-build CPU medians are
60.0/1615.4 ms; adding GPU cold preparation cannot yield an end-to-end gain.

The final giant 64³ source-resident subset passes its timing and exact-grid gates
(25.4 to 11.8 ms median). Keep this as a research direction for reused, precision-
corrected data. It does not qualify a general query API: giant contacts have 54
mismatches, including six false-negative cells. Ordinary contacts have 65 mismatches,
including 21 false-negative cells; Ordinary 64³ has two foliage mismatches, including
one false negative. Raw cells and flags are retained. Both wood and foliage contact
predicates drift; no epsilon relaxation, boundary correction or workload reduction
was applied.

Preliminary and internal-timing runs failed every resident timing qualification.
Their superseded dumps remain in Git history at `6f3adb2`, alongside the rejected
implementation; the working tree retains the authoritative final raw run. The final
run supersedes their cold sums with a direct caller stopwatch. This observed
variability limits performance claims; no slow sample was discarded. The CPU
baseline's interrupted export attempt is documented in its protocol and is
inconclusive, not counted as a successful sample.

## Support, precision, ownership and resources

Production CPU field computations retain f64 semantics and existing deterministic
seeded generation. Snapshot schema 1 copies exact f64 wood/bounds and u32 BVH topology;
its [layout and lifecycle](../../../scripts/benchmarks/generation.md#portable-snapshot-contract)
are documented. Snapshot extraction is optional, with zero snapshot copies in ordinary
builds/queries. It requires a live revision; native rebuild (even failed), release
and disposal invalidate the handle. Owned arrays survive and are caller-mutable
without changing native data. Errors throw without partial snapshots; Wasm staging
is released in success and failure paths.

The measured GPU configuration is Linux 7.1.9-arch1-2, Ryzen 9 5950X, Chromium
151.0.7922.173, RTX 3080/NVIDIA Ampere, non-fallback Vulkan/ANGLE adapter with timestamp
queries. Exact launch flags, adapter limits and metadata are in the raw samples.
This is one configuration, not a supported hardware matrix. The evidence does not
pin a driver package version or prove other drivers/browsers. Software adapters,
other platforms and cross-device repeatability remain unqualified. Timestamps are
optional and supplement caller wall time.

The experimental WGSL f32 conversion cannot guarantee f64 occupancy. Closed cube
wood/foliage contacts, zero-extent points, empty/outside batches, malformed,
nonfinite, negative, overflow-scale and large-coordinate inputs are covered by the
[GPU tests](gpu-tests.json). Large f32-unsafe cells are zero only if f64 root bounds
prove them outside both fields; potentially overlapping cells fail `precision-range`.
No partial flags or silent CPU fallback are returned.

Missing adapters, unsupported limits, shader/device/allocation/submission/readback
failure, traversal overflow, device loss and timeout produce explicit whole-request
errors. Preparation owns copied source data independently of later engine revisions.
Sessions own devices/buffers; disposal is idempotent and post-disposal queries fail.
Concurrent queries fail `busy`. Invalid input leaves a reusable session; resource or
execution failure destroys it, and recovery requires a new session. The
[test receipt](gpu-tests.json) labels controlled API doubles separately from actual
WGSL execution and real `GPUDevice.destroy()` loss. Injected failures do not establish
that those hardware failures occurred during timing.

The giant has 177,543 wood primitives and 1,360,279 foliage boxes. Its f64 snapshot
is 129,078,104 bytes; packed arrays and GPU source buffers are each 72,333,272 bytes.
Its named cold JS/GPU allowance peaks at 347,257,564 bytes, separately from
380,174,336 bytes of Wasm high-water. Ordinary 64³ uses 32,752,880 named JS/GPU bytes
and 32,440,320 Wasm bytes. Source, packed, validation scratch, query/output, staging,
mapped readback and timestamps are accounted in each raw row. Native field/staging
are within Wasm high-water and must not be added twice. Driver staging payload is an
allowance; opaque driver allocations, GC retention and browser overhead prevent an
OS peak-memory claim. Buffer/dispatch limits, topology validation, a 64-entry traversal
stack and visit guard bound the experimental request.

## Reproduction and validation

Build with the pinned Rust toolchain and `npm ci`, then `npm run wasm:build` and
serve Vite on port 5188. The [CPU protocol](../../../scripts/benchmarks/generation.md)
gives the retained runner commands, binary bundle layouts and local
Playwright/Chromium overrides. Historical GPU reproduction requires a separate
checkout of `6f3adb2`; its `scripts/benchmarks/generation-gpu.md` records the removed
runner's commands and configuration. GPU timing requires exclusive GPU access and
no competing CPU benchmark/test load. No additional GPU run was needed for the
rejection or removal.

Native workspace tests, formatting and clippy ran after the snapshot Rust change;
no Rust code changed afterward. Durable receipts: [native tests](native-tests.log)
(69 passed, six ignored), [format](native-fmt.log) and [clippy](native-clippy.log).
The final task also checks typecheck, harness tests, package build and CPU-only browser
bindings. Browser snapshot checks cover exact reference flags, mutation isolation,
stale/released/disposed handles, empty snapshots and staging cleanup. GPU hardware
and controlled failure checks remain those of task 2. Final command outcomes are
recorded in [validation](validation.json) with accompanying logs.

R1 is resolved by measured CPU stages and matched query evidence, with GPU growth
and construction explicitly inconclusive. R2 rejects this candidate and retains CPU
behavior. R3 documents the production snapshot boundary and experimental failures,
support and precision limits. Per owner instruction, no plan, implementation or
completion review was dispatched; the parent spec remains open.
