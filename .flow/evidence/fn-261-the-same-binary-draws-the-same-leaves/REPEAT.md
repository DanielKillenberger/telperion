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

Two other atomic slot allocations sit upstream: the grower's overflow queue (`record.wgsl`) orders the regrowth of chains that outgrew their region, and the fold's crown sift (`fold.wgsl`, `sift`) writes its kept nodes in atomic order. Neither shows in the outputs: the tree agrees, and the segments and leaves agree as sets. Whether the device's node numbering is stable is not read directly; the fix below keys runs by node index, so its repeat test going green over ten builds is that check. The remaining atomics in the fine executor are min/max folds, flags and counts read back as totals, all order-free.

## The fix (R2)

The runs are numbered by a key: `keys` writes each run's (stub, first node) at a slot the count hands out, a bitonic sort orders them (`sort.wgsl`, one integer comparison, no float, no atomic), and `starts` numbers the runs in the sorted order. A root's runs stay together and the roots in their stubs' order, as host decision 3 set; within a root the runs are in their first nodes' order. The `owned` pass and its two words a node are gone.
