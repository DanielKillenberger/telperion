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
