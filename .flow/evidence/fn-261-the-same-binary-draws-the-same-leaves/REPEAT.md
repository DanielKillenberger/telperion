# fn-261: where the same binary's builds part

## R1: the repro

`generation::fine::repeat_tests::the_spruce_builds_the_same_leaves_twice` builds the Norway spruce at its default age and seed on the real path (`resident::build`) twice, each in a fresh context (device, generator, kernels and tools made anew), and compares four outputs byte for byte: the pipeline's grown tree (`expansion.tree().nodes`), the station segments of both blocks, the leaf buffer, and the mass grid.

On F4 (93579bf9) it is red: `["segments", "leaves"]` differ; the tree and the masses do not. Probed further with a comparison of each output sorted, two runs (a temporary probe in the test, not kept):

| | Apart in place | As a set |
|---|--:|---|
| Station segments, less their first leaf | 232,352-238,380 | equal |
| Leaves (12 bytes each) | 958,843-990,747 | equal |

The same segments and the same leaves are built every time; only their order changes. The masses, a count over the grid, cannot see order and agree.

## The first stage whose output differs: the stations' run numbering

Checked against the code (`crates/telperion-render/src/generation/fine/stations.wgsl`, F4):

- Upstream of the stations, nothing the build hands on is ordered by thread timing. The pipeline's tree agrees byte for byte. The sweep numbers its runs by a bitonic sort on (widest radius, first node), a total order (`sweep.wgsl`, `bitonic`).
- `owned` gives each bearing run its rank among its root's runs with `atomicAdd(&owners[s * OW], 1u)`, and `starts` writes the run at its root's first run plus that rank. A root's runs are contiguous and the roots in their stubs' order (fn-212 task 10, host decision 3), but within a root the order is whichever thread reached the atomic first. TASK10.md says as much: "Within a root the order is still the atomic rank's".
- `bases` and `emit` number each run's first station, first segment and first leaf by a scan in run order, so every leaf of a root after the first reordered run lands at another index of the leaf buffer.
- The renderer reads a leaf by that index: `foliage.wgsl` takes `id = list[instance]` and varies the leaf by `vary(id)`. A leaf at another index is drawn with another leaf's variation, which is the 0.08-0.16 % of pixels fn-258 saw, all on leaves. The shader header's "the runs may be numbered in any order" holds for the leaves' placement, not for their drawing.

The palm and the oak grow on the CPU executor, whose stations are numbered in a walk, and are byte-identical; this is the device's alone.

Two other atomic slot allocations sit upstream: the grower's overflow queue (`record.wgsl`) orders the regrowth of chains that outgrew their region, and the fold's crown sift (`fold.wgsl`, `sift`) writes its kept nodes in atomic order. Neither shows in the outputs: the tree agrees, and the segments and leaves agree as sets. Whether the device's node numbering is stable was not read when this section was written. It is not stable; see "Open" below. The remaining atomics in the fine executor are min/max folds, flags and counts read back as totals, all order-free.

## The fix (R2)

The runs are numbered by a key: `keys` writes each run's (stub, first node) at a slot the count hands out, a bitonic sort orders them (`sort.wgsl`, one integer comparison, no float, no atomic), and `starts` numbers the runs in the sorted order. A root's runs stay together and the roots in their stubs' order, as host decision 3 set; within a root the runs are in their first nodes' order. The `owned` pass and its two words a node are gone.

## R2: green, and what it costs

`every_device_preset_builds_the_same_leaves_each_time` takes every shipped preset that `executor::apart` sends to the device (the spruce, beech, birch, Telperion and Laurelin) at its default age and seed. With `REPEATS=10` each was built ten times in fresh contexts, and every build matched the first byte for byte in tree, segments, leaves and masses (72 s under the GPU lock). The gate runs it with two builds. The ten builds do not show that the device's node numbering is stable, and it is not (see "Open" below). Within each root, keys sorted by node index keep the same relative order under the renumbering seen so far.

Timings: native, RTX 3080, `RAYON_NUM_THREADS=4`, other sessions' load at about 4. Before and after were run alternately, three rounds each, and the medians are below (`raw/timing.jsonl`, `raw/stage.txt`).

| | Before | After |
|---|--:|--:|
| Spruce at 40: stations stage (ms, six builds each) | 9.6-10.8 | 13.4-23.2 |
| Spruce at 40: whole build, `prepare_timing` (ms) | 20,867 | 22,753 |
| Telperion at 82: whole build (ms) | 27,263 | 22,790 |
| Spruce frame, whole / close / bare-whole / bare-close (ms) | 8.51 / 8.08 / 3.36 / 3.96 | 8.41 / 8.06 / 3.36 / 3.96 |
| Telperion frame, whole / close / bare-whole / bare-close (ms) | 5.33 / 12.31 / 4.13 / 8.94 | 5.19 / 12.21 / 4.13 / 8.96 |

The sort adds about 3.5 ms to the stations stage at the least: 171 dispatches over the spruce's runs, each with its own uniform and bind group. A whole build takes 20-27 s, and its run-to-run noise of ±3 s hides that cost. Frame time is unchanged or a little lower, since a root's runs now follow node order.

## R3: stills

For every one of the eight shipped presets, `generation_gpu <id> gpu-render` (resident delivery, hero pose, 1280×720, `GENERATION_CAPTURE`) was run twice, each run its own process under the GPU lock. All eight pairs are byte-identical PNGs. As a control, the pre-fix binary (b0a613c3) drew two spruce stills that differ in 31,574 pixels in some channel and in 2,533 by more than 8/255, out of 921,600. Pixel counts come from a thresholded difference image, not from `compare -metric AE`.

Before the fix against after it, the spruce differs in 53,384 pixels in some channel and in 5,725 by more than 8/255. These are the leaves taking other indices and so other variations. Viewed side by side, whole and as a crop of the upper crown, the two cannot be told apart (`raw/side.png`, `raw/crop.png`).

## Open: the device-grown nodes are not numbered the same run to run

The codex review (round 1, all three reviewers) found that R1's "grown tree" check compared only the pipeline's part (`expansion.tree()`), never the device-grown nodes. A probe read the sweep's node table (`swept.nodes`, every device node, 33 words each) back before its release and compared it byte for byte. The patch is `raw/grown-probe-and-queue-experiment.patch` and is not committed. Over 6-build runs:

- The spruce, birch and Telperion matched in every build.
- The beech failed intermittently: about 390,000 of its 391,673 node slots differed, with the same count of live nodes in each build. The leaves, segments, masses and stills stayed identical.
- Laurelin did the same: about 3.7 M of its 4.5 M slots differed, in every 6-build run tried.

Experiment (not kept): the beech queues chains that outgrow their region (`record.wgsl`, `queue[atomicAdd(&queued[0], 1u)]`; 44 and 5 chains at level 0). The regrown chains are numbered in queue order. With the queue sorted into wave order before `gather`, the beech matched across 12 builds, but Laurelin still did not. Growth therefore has at least one more source of order beyond the queue, still unnamed.

This lies in growth (`roots.rs`, `record.wgsl`, and whatever else numbers Laurelin's nodes), which fn-254 is rewriting. It is outside the stage this task fixed, so it is handed to the host as NEEDS_HUMAN.
