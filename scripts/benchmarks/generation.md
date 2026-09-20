# Generation baseline and snapshot input (schema 1)

Run `npm run wasm:build`, serve this checkout with Vite, then:

```sh
BROWSER_URL=http://127.0.0.1:5188 GENERATION_OUTPUT=/tmp/fn12-generation \
  node scripts/benchmarks/generation.mjs
```

`PLAYWRIGHT_MODULE` and `CHROMIUM_EXECUTABLE` can select local installations.
The runner uses headless Chromium with GPU disabled. It builds the full Ordinary
and Telperion presets without parameter overrides or render outputs. It alternates
subject order across paired rounds, keeps an independent Wasm instance per subject,
and records one warmup plus five measured samples. The exact parameters, seeds,
completeness/cap diagnostics, hardware, Wasm hash, raw timings and hashes live in
`generation-baseline.json`. That file records the implementation checkout as dirty;
the source commit is its base, while the Wasm SHA256 identifies the executed build.

The CPU denominator is indexed Rust/Wasm `field.query`: wall time includes input
copying, native traversal and owned result copying. Query-cell construction is
separate. The independent JavaScript snapshot traversal checks both full grids in
the warmup and contact probes in every sample; it is never the speed denominator.
Every sample hashes its exact source arrays, query cells and returned flags, and
requires equality with its specimen's warmup. Incomplete specimens or mismatches
fail explicitly after saving the observations.

## Recorded CPU result

The September 6 run used Chromium 151 on the CPU named in the JSON hardware block.
All twelve samples completed without caps. All verified flags and all repeated
hashes agreed. Times below are measured medians in milliseconds, with maximum in
parentheses; with five samples the nearest-rank p95 is the maximum, not a stable
population tail estimate.

| Stage | Ordinary | Telperion |
|---|---:|---:|
| Skeleton, including radius solves and local growth | 11.2 (12.5) | 314.5 (344.0) |
| Foliage prerequisite: placement and culling | 25.2 (32.1) | 636.5 (794.6) |
| Field index construction | 23.1 (26.2) | 656.4 (740.5) |
| Whole mesh-free build | 59.9 (71.3) | 1607.9 (1853.6) |
| Snapshot extraction, copying and staging release | 2.5 (3.1) | 44.1 (52.9) |
| Exploratory f32 packing plus topology copies | 1.6 (1.7) | 19.9 (21.5) |
| Indexed CPU queries, 32³ | 3.9 (4.1) | 6.9 (7.6) |
| Indexed CPU queries, 64³ | 20.3 (22.1) | 25.8 (28.1) |

The giant contains 177,543 wood primitives and 1,360,279 retained foliage boxes.
Its owned f64 snapshot is 129,078,104 bytes; exploratory packing allocates another
72,333,272 bytes. Native field heap capacity is 152,842,304 bytes, and Wasm reaches
380,174,336 bytes including reusable allocator capacity. These are distinct
accounting domains: native field and staging are contained in Wasm high-water,
not additive to it. Source and packed JS arrays coexist; query artifacts add
9,751,203 bytes per specimen. Hashing, artifact saving, browser/GC overhead and
native query scratch are described separately in each row. This is allocation
accounting, not an OS peak-memory measurement.

An initial export attempt hit Chromium's multiple-download suppression after ten
files; that run was stopped and remains inconclusive in `/tmp/fn12-generation`.
The completed run uses one binary bundle per specimen and is in
`/tmp/fn12-generation-v2`. Both completed bundles were independently checked from
disk against every recorded SHA256. Timed stages exclude artifact IO; each bundle's
save time is recorded. No GPU workload was run in this task.

## Portable snapshot contract

`output.field.snapshot()` is optional. Ordinary builds and queries allocate no
snapshot buffers. Extraction requires the same live revision as a field query;
release, rebuild (including failed rebuild), and disposal invalidate extraction.
Already copied arrays survive all of these and may be changed by the caller without
changing the CPU field. Errors return no partial snapshot. Native staging is freed
after copying, including failure paths. The f64 snapshot preserves every source
value and the CPU's exact median BVH order; it does not rebuild or simplify it.

| Array | Scalar type | Layout |
|---|---|---|
| `wood` | Float64 | Eight per segment: a.xyz, b.xyz, start radius, end radius; roots are degenerate segments |
| `woodIndex.bounds` / `woodBounds` | Float64 | Six min.xyz/max.xyz per BVH node, followed by six per indexed item |
| `woodIndex.topology` / `woodTopology` | Uint32 | Four per node: item start, exclusive item end, left child, right child; followed by one original segment ID per item |
| `leaves.bounds` / `leafBounds` | Float64 | Same node-then-item layout; item bounds are transformed leaf AABBs including connectors |
| `leaves.topology` / `leafTopology` | Uint32 | Same layout; trailing leaf primitive IDs are unused (zero) |

Each index is independent and has `nodeCount`; root is zero. Both children are
`0xffffffff` for a leaf, otherwise both are valid node indices. An empty index has
zero nodes and items. Bounds item count is `bounds.length / 6 - nodeCount`.
The field snapshot also carries schema `1`, revision, union bounds (null when empty)
and extraction/copy/total timings. The Wasm staging ABI uses slots 9–13 in table
order, `field_snapshot(revision)` and `field_snapshot_release()`; consumers should
use the owned browser boundary.

In the exported bundle, each sample-zero `artifacts.arrays` entry records name,
typed-array constructor, scalar length, byte size and byte offset; `artifacts.path`
names the single little-endian binary. The snapshot arrays come first, then
`cells-32`, `flags-32`, `cells-64`, `flags-64`, `cells-boundary`, `flags-boundary`.
Read through a byte slice/copy when constructing typed arrays because offsets after
u8 result buffers need not be naturally aligned. The JSON has source, cell and
flag SHA256s for independent verification. Task 2 can reuse these values and BVHs,
but must measure CPU and GPU together in its own browser run.

Queries are closed cubes packed as Float64 x/y/z/halfExtent; bits are wood=1 and
foliage=2. Grid cubes span the largest field dimension, anchored at its minimum.
The boundary matrix samples tapered segment endpoint contacts, leaf AABB corners,
nearby points, outside points and large finite coordinates. Zero extent is valid;
nonfinite, negative, malformed or overflow-scale requests fail. Wood uses the
cube's circumsphere inflation and the exact tapered sphere sweep predicate;
foliage uses closed AABB overlap. The f32 preparation is only an allocation/timing
probe: it has no precision guarantee and is not an accelerated API. In particular,
the valid large-coordinate probe exceeds f32; the next experiment must handle it
explicitly, and any unexplained final occupancy mismatch rejects adoption.

## Applying the fn-12 findings to new generation work

Policy updated 2026-09-20. The owner permits changes to generated specimens during active development. The historical analysis below describes the old implementation and its exact-output constraints; matching its append order, floating-point reductions or hashes is not a blanket requirement for a replacement. Prefer byte-identical output for performance improvements when practical because exact comparison simplifies verification; this preference must not block a worthwhile measured gain that passes visual and correctness checks. Retain exact comparisons for paths intended to remain unchanged; document and validate deliberate changes to tree structure or representation. A performance candidate may differ in bytes and still qualify through measured end-to-end gains, no perceptible visual regression at supported views and in motion, and preserved relevant correctness; exact equality is not a prerequisite for that comparison. Branch dependencies, resource limits and correct spatial predicates still need a sound implementation.

fn-12 tested GPU queries over CPU-built spatial indices, with output read back to the CPU. It did not test GPU leaf placement, wood expansion or field-index construction. Every cold query lost after extraction, packing, setup, upload and readback; the resident giant 64-cubed grid improved from 25.4 to 11.8 ms, while contact queries still had false negatives. Relaxing byte equality does not make missed contacts correct. The original report remains the historical evidence, not a ban on GPU generation.

A new rendering experiment should upload compact structural inputs, generate placements into buffers the renderer consumes, and avoid full-result readback on the timed display path. Measure the complete path to a finished frame, including CPU preparation, setup, allocation, upload and all GPU passes; report cold page startup separately from an initialized renderer. Compare multiple seeds on a broadleaf and a needle-bearing tree, including a phone-class device, and record peak memory and visual/attachment correctness. Readback for validation is allowed but must not be hidden if production requires it.

Start by profiling current wood construction, attachment preparation, leaf placement, culling, bounds and upload separately. Test foliage expansion before committing to GPU wood or botanical growth. Set the target and bounded experiment scope before building; if end-to-end latency fails to improve or fidelity fails, report the limiting stage and stop that candidate. A fast isolated kernel does not qualify the path. The later fn-91 resident-expansion measurements below establish desktop warm-delivery gains; near-instant cold startup remains unqualified.

## Skeleton feasibility

The measured `growthMs` includes the whole `branching::generate` chain, not just
colonization. It accounts for roughly 19–20% of either specimen's median build.
That leaves limited potential for reducing whole-build time before charging GPU
preparation and synchronization; a small substep cannot be assumed to qualify. Per-round counters and GPU growth
substeps were not instrumented, so there is no measured substep speedup claim.

`colonization.rs::Attraction::settle` updates nearest/alive state after newly
appended nodes. Distance searches and kill predicates can run independently per
attractor if ties retain the earliest CPU node. The current grid also limits CPU
work to nearby attractors; a GPU all-pairs replacement would change the workload.
The trunk loop depends on the previous tip and settles attraction after every
append. Crown rounds have separable per-parent heading/bias/turn calculations and
per-attractor closing tests, but the normalized pull reduction currently follows
attractor order. A parallel reduction or f32 conversion can change the next branch.

Each round then appends children in parent order, respects a shared node budget,
updates prior-direction history, and settles attraction before the next round.
Parallel candidates therefore need deterministic compaction and synchronization
between rounds, not independent whole-tree dispatch. The radius solver propagates
path loss parent-to-child, sums carried radii child-to-parent, then scales radii;
its local-branch start radii depend on already solved parent radii. Local growth
uses ordered frontiers and shared append budgets; shedding renumbers survivors
before the final radius solve. Uploading a changing frontier and reading decisions
back each round would add transfer and synchronization costs absent from the CPU.

Independent leaf-bound transforms and query batches are clearer first candidates.
Field construction also partitions by median recursively and needs exact bounds
and item membership. Task 2 starts with the already constructed indexed field and
charges extraction, packing, upload, synchronization and readback. At the recorded
giant sizes, snapshot plus packing already exceeds either single CPU query batch;
resident reuse may be worth testing, but cannot establish a cold end-to-end gain.

## Monthly growth cost (fn-11, native, 2026-09-12)

Command, from the worktree root:

```sh
FN11_MEASURE=1 cargo test --release -p telperion-core --lib monthly_cost_report -- --nocapture --test-threads=1
```

This uses the core's `Instant` timers, seed 7, no GPU, and skeletons only.
Mature and envelope figures are medians of three builds. Slice figures are
individual samples; identity/radius comparisons happen outside the timer.
Both families reach the final quantum in month 2073 (172.75 years).
The final run followed all five gates, without concurrent test/compiler work;
the machine was shared, without CPU isolation.

| Species | Slice after 20 y | Slice after 100 y | Final slice | Mature build | Envelope build |
|---|---:|---:|---:|---:|---:|
| Oak | 4.549 ms | 18.709 ms | 18.400 ms | 18,897.739 ms | 62.786 ms |
| Spruce | 3.521 ms | 8.357 ms | 5.194 ms | 9,148.164 ms | 35.582 ms |

The cost shape remains a problem for oak. These are the minimum and maximum
nonzero changed-radius counts among active slices starting above 50,000 nodes,
selected by changed count, not elapsed time. Changed includes births and exact
changes to distal, proximal or base radius; it does not mean only new nodes.

| Species / month | Starting nodes | Born | Changed or born | Slice | Width update | Storage repair |
|---|---:|---:|---:|---:|---:|---:|
| Oak / 587 | 211,095 | 38 | 1,698 | 13.693 ms | 0.498 ms | 8.138 ms |
| Oak / 2073 | 215,963 | 1 | 203,775 | 18.400 ms | 14.300 ms | 0.00033 ms |
| Spruce / 782 | 74,616 | 18 | 264 | 0.839 ms | 0.035 ms | 0.755 ms |
| Spruce / 2073 | 74,666 | 1 | 65,989 | 5.194 ms | 4.898 ms | 0.232 ms |

Spruce's dense slice costs 6.194 times its sparse slice on almost the same tree
size. Oak changes 120 times as much wood for only 1.344 times the slice cost:
its sparse slice still repairs 205,826 moved local nodes and spends 4.384 ms
retrying the frontier. Thus the whole-slice proportional-cost requirement is
**not met for oak**, despite the incremental width solve.

Fixed work includes crown-profile preparation (about 0.013 ms oak / 0.002 ms
spruce in these samples), frontier ordering and deferred-shoot retries.
Structural insertion also moves the contiguous local segment and updates its
storage references. Dense slices legitimately resize much of the tree: the
age-100 oak adds one node but changes 203,664 radius tuples. These costs leave
mature builds about 301 and 257 times the envelope build, respectively. The
closed-form alternative remains an owner decision; this measurement does not
justify production routing.

The saturated advance to one million years took 0.00033 ms for oak and
0.00023 ms for spruce. Snapshot round-trip and placement costs remain unmeasured:
those implementations are still pending. Populations remain 215,964 versus
139,040 envelope nodes for oak, and 74,667 versus 90,439 for spruce; convergence
and calibration are unchanged. Full samples, stage counters, commands and gate
results are in `/tmp/flow-handover-fn11/child-notes.md` and `cost-measure-final.log`.

## Annual slice choice (fn-11, native, 2026-09-13)

The command above retains its historical `monthly_cost_report` name, but now
measures annual slices. Seed 7, native `Instant` timers, no GPU. Full-build
figures are three-build medians; slice figures are individual samples. These
measurements cover skeleton growth before foliage reads or change records.
Both families reach the final work quantum at year 173 (previously month 2073,
172.75 years). The API keeps the original time resolution: whole years plus an
integer remainder in 12 billion ticks per year.

| Species / slice length | Internal slice after 20 y | After 100 y | Mature slice | Mature build | Envelope build |
|---|---:|---:|---:|---:|---:|
| Oak / monthly | 2.767 ms | 5.419 ms | 4.826 ms | 5906.159 ms | 78.494 ms |
| Spruce / monthly | 2.885 ms | 4.438 ms | 0.163 ms | 4535.222 ms | 35.271 ms |
| Oak / annual | 15.918 ms | 5.343 ms | 4.729 ms | 774.870 ms | 69.925 ms |
| Spruce / annual | 5.279 ms | 4.870 ms | 0.090 ms | 585.439 ms | 35.721 ms |

**R10 chooses annual:** the mature monthly oak exceeds approximately 500 ms.
Annual improves it substantially but still misses that target. The closed-form
alternative remains the owner's reserve; it has not been implemented. Continue
calibration and the remaining integration at annual length. Monthly oak slice
traversal overlapped compilation of a clock regression; the decision's three
mature-oak samples, spruce build samples and all annual measurements did not.
The host is shared and CPU affinity is not isolated.

The finalizer runs once per advance. Packing is paid lazily by a consumer read,
and both are separate from the internal slice timings above. Sparse/dense means
minimum/maximum nonzero changed-radius count among active slices starting with
more than 50,000 nodes; it includes resized survivors as well as births.

| Species / length / slice | Starting nodes | Born | Changed or born | Internal slice | Advance, excluding read | Read |
|---|---:|---:|---:|---:|---:|---:|
| Oak / monthly / 587 | 211095 | 38 | 1698 | 6.766 ms | 8.232 ms | 13.719 ms |
| Oak / monthly / 2073 | 215963 | 1 | 203775 | 4.826 ms | 27.251 ms | 10.273 ms |
| Spruce / monthly / 782 | 74616 | 18 | 264 | 0.141 ms | 0.231 ms | 1.106 ms |
| Spruce / monthly / 2073 | 74666 | 1 | 65989 | 0.163 ms | 7.829 ms | 1.195 ms |
| Oak / annual / 66 | 178501 | 129 | 1415 | 5.659 ms | 5.943 ms | 3.998 ms |
| Oak / annual / 173 | 179715 | 1 | 164965 | 4.729 ms | 18.220 ms | 4.224 ms |
| Spruce / annual / 67 | 76602 | 24 | 273 | 4.684 ms | 4.752 ms | 1.286 ms |
| Spruce / annual / 173 | 76627 | 1 | 60396 | 0.090 ms | 6.003 ms | 1.357 ms |

**Cost still does not scale solely with changed wood.** Annual sparse oak spends
5.072 ms in local retries/order/width queries and spruce spends 3.504 ms there
plus 1.129 ms sampling visited-shoot vigour. These are the remaining repeated
costs; structural insertion moves zero local nodes. A zero-birth middle spruce
slice nevertheless changes 60,395 radius tuples, so its 7.329 ms finalizer is
real thickening work. Empty local frontiers skip width queries; unchanged queues
keep their identity order. General radius-dependent failures still retry because
their acceptance is not a monotone function of the scalar growth curve alone.
Saturated annual advances cost 0.002770 ms oak and 0.001390 ms spruce.
Snapshot round-trip timing remains unavailable.

Annual growth is **not converged** with the envelope build. Oak has 179,716
versus 139,040 nodes (+29.255%) and 5,418 versus 3,602 structural nodes
(+50.416%). Spruce has 76,628 versus 90,439 nodes (−15.271%) and 21,442 versus
17,573 structural nodes (+22.017%). Bounds spans differ by at most 2.249% oak
and 5.983% spruce. Both thresholds remain zero; calibration, production routing
and the authorized re-pin remain pending.

A follow-up after lifetime foliage was added used the same command, with fresh
foliage reads performed after each species' entire skeleton timing traversal.
The initial slice-choice numbers above remain its decision record; these are the
follow-up samples (no concurrent tests/compiler during timing):

| Species | Internal slice after 20 y / 100 y / mature | Mature build median | Envelope median | Sparse / dense advance, excluding read |
|---|---:|---:|---:|---:|
| Oak | 11.455 / 5.718 / 4.836 ms | 778.624 ms | 70.232 ms | 6.533 / 18.644 ms |
| Spruce | 4.798 / 4.641 / 0.090 ms | 560.931 ms | 36.171 ms | 4.611 / 6.268 ms |

The remaining sparse local-retry/order/width-query stage is 5.547 ms oak and
3.398 ms spruce, plus visited-vigour sampling of 0.505 and 1.102 ms. The cost
shape and annual decision are unchanged. Snapshot timing is still unavailable.

| Species / age | Placements before culling | Fresh placement read |
|---|---:|---:|
| Oak / 20 | 59189 | 17.347 ms |
| Oak / 100 | 78 | 14.320 ms |
| Oak / 173 | 13 | 10.914 ms |
| Spruce / 20 | 1823229 | 862.152 ms |
| Spruce / 100 | 100 | 82.651 ms |
| Spruce / 173 | 120 | 82.986 ms |

**Mature foliage has not converged.** The literal one-year oak / six-year spruce
shoot lifetime expires leaves while the sigmoidal work budget brings new shoot
births toward zero. The mature timeline consequently has only 13 and 120 leaf
placements before culling. This does not establish R1 or R11's mature appearance.
No renewal, seasonal flush, altered lifetime, production routing or re-pin has
been added to conceal it. The placement reads include lazy skeleton packing,
validation and, for spruce, construction of the contact surface; these costs
are separate from internal slices and are not a change-record update benchmark.

The experimental shared GPU browser path is measured only by explicit opt-in:
`GENERATION_GPU=1 GENERATION_COMPLETED=1 GENERATION_OUTPUT=<file> node scripts/benchmarks/mature-generation.mjs`.
It awaits `setTreeGpu`, retains the existing hero/queue-completion boundary, and records backend, preparation stages, explicit buffer counts and whole-module Wasm memory. The default remains synchronous CPU generation. Run `GENERATION_OUTPUT=<file> node scripts/benchmarks/generation-gpu-smoke.mjs` for bounded browser lifecycle validation before the full fixture matrix.

### Bounded GPU CPU-output readback

GPU CPU delivery and explicit resident verification share a chunked asynchronous
readback. It reserves the final packed leaf vector once and uses staging capped at
4,194,300 bytes (4 MiB rounded down to 12-byte records, respecting device limits).
No full-size intermediate byte vector is retained. Count the final vector, staging,
and still-live source separately; these limits do not measure whole-process or
GPU-driver memory. The old staging was released before final decoding, so the old
peak had two full readback copies, not three simultaneously.

Task fn-91.2's matched seed-1 native comparison is recorded in
`.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/READBACK.md`, with the
fresh-process wait4 protocol in `readback-measure.py`. Hash verification runs in
separate processes so its extra verification clone is excluded from timing/RSS.

### Resident wood expansion

The experimental `setTreeGpu` and native `Delivery::Resident` paths also retain
wood on the device. Core preparation emits canonical float32 positions and
compact run/ring metadata. CPU float64 triangle checks select the ordinary CPU
wood builder for collapse or unsupported numeric cases. Compute-only storage
limits and unusable GPU normals select explicit CPU wood fallback. Device errors
remain errors. The default CPU APIs and owned CPU delivery use the CPU builder;
standalone CPU-output generators do not create wood compute pipelines.

Resident expansion copies pre-rounded coordinates/radii, emits ordered indices,
and gathers incident triangles per vertex into finite unit normals. Verification
readback stays outside delivery timing. Resident metadata supplies actual counts
and run spans; a replacement keeps the old tree alive until validation and adoption.
Final buffers retain their vertex/index/storage/copy usages for later CPU uploads.

Reports distinguish `woodBackend`, `woodFallback`, compact preparation, upload/
dispatch and status wait, prepared CPU capacity, packed metadata CPU capacity,
wood GPU buffers including status staging, retained buffers and each previous live
tree's buffers. At wood expansion, add the previous live tree, new foliage buffers
and wood compute buffers to account their coexistence. Compare this with foliage
compute plus the previous tree; the two compute stages run sequentially. CPU
preparation scratch, allocator overhead, queue upload staging, the four-byte status
result, pipeline/compiler/driver allocations and delayed destruction remain outside
these counters. Wasm linear-memory size and process peak RSS have separate meanings.
Task .4's native/browser measurements and coverage limits live in
`.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/resident-wood/REPORT.md`.

### Shared canonical contacts

Task .6 reuses prepared wood positions for resident foliage when surface contact
is enabled. It uploads packed xyz positions once, releases CPU positions, and
adopts that same GPU buffer for wood after foliage. Ordinary CPU preparation and
zero-contact requests retain their existing paths. The new shared preparation,
contact-map and retained-metadata counters identify overlapping CPU lifetimes;
the foliage GPU peak counts the shared position buffer once, including mass work.

The reproducible native comparison and four-fixture browser results are in
`.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/shared-rings/REPORT.md`.
Native comparison isolates task .6 against the saved .5 executable. Browser
comparison against the saved .4 module includes both .5 and .6. Whole-process
RSS and Wasm capacity remain distinct from simultaneous explicit buffer counts.

### Qualified GPU positions

Task .8 admits compact GPU positions before resident foliage. The initial fast
path covers unmodulated profiles (`lobes == 0` or `lobe_depth == 0`), centres
within 64 m per component, radii at most 32 m and radii at least
`max(1 m, max(abs(centre))) / 131072`. Other valid inputs retain the canonical
resident path; these are capability bounds, not engine parameter limits.

Position emission, pre-arithmetic triangle checks, normal admission and enclosing
bounds complete before contacts consume the buffer. Actual corner-derived ring
radii use offsets from the first corner to avoid large-coordinate cancellation.
The same immutable positions feed foliage and final wood; full attributes are
allocated only after foliage. Geometry rejection drops the candidate before
canonical preparation, while device errors preserve the existing request contract.
Standalone owned CPU output creates no position pipeline.

`gpuPositions` identifies actual candidate delivery; `positionFallback` names
rejected attempts even when the canonical resident path remains GPU-backed.
Position preparation/upload/completion and CPU/GPU capacities are separate from
foliage and final expansion. Retained GPU metadata is counted through foliage;
shared positions count once. See
`.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/position-integration/REPORT.md`
for paired delivery, CPU controls, numerical qualification, visual scope and
remaining cold/device/memory gaps.

### Native CPU surface expansion

Ordinary CPU wood construction on Linux x86_64 can use up to eight scoped workers
for independent runs, writing into disjoint slices of the final arrays. Admission
requires at least 250,000 vertices, multiple available workers and an explicit
capacity envelope no larger than the serial builder's. These are scheduling
conditions; smaller or unsupported requests still receive the serial builder.
Prepared surfaces and contact preparation retain their existing path.

Position/coordinate emission finishes before preparation storage is released and
normals/indices are allocated. Each run retains canonical calculation order.
Worker failure, collapsed triangles or unusable normals join all started workers
and release candidate arrays before one serial retry. Wasm requires no threads.
The accounting includes requested stacks and a runtime allowance, but does not
prove allocator, thread-cache or whole-process peak memory. Complete CPU-output
latency and observed RSS are reported separately from the isolated wood speedup.

### Shared station preparation and position overlap

Task .14 streams station frames and avoids per-node child lists. Zero-contact
resident requests submit GPU positions and their admission readback before CPU
station preparation, then join admission before consuming geometry. Contact-bearing
requests preserve their dependencies. Unsupported station capability and station
errors join submitted work before fallback or return.

The single paired desktop browser run measures completed frames at 158.1/180.8 ms
for oak seeds 1/7 and 178.9/164.2 ms for spruce, respectively 9.93/10.30/42.64/44.97×
the original baseline. Oak seed 1 misses the strict 10× threshold by 1.1 ms; no
repeat was used to turn that miss into a pass. This ends the optimization search.
All four full station-record comparisons are byte-identical for these fixtures.

Live joint-capacity envelopes remain unchanged, including overlapped preparation
and conservative output reallocation. Wasm linear-memory high-water is separate:
oak seed 7 rises 26,279,936 bytes, oak seed 1 falls 24,641,536 bytes, and spruce is
unchanged. This does not qualify whole-process memory nonincrease or cold/phone
performance. Raw samples, controls and lifecycle checks are in
`.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/stations/REPORT.md`.
