> Historical raw-output references: see the [archive and recovery instructions](README.md).

# Host boundary analysis before wood profiling

Existing browser completed-frame results include preparation, submission and queue completion. Subtracting independently computed medians below is only a rough stage-budget illustration, not a matched-sample decomposition or a prediction.

| Fixture | Completed p50 ms | Preparation p50 ms | Wood p50 ms | Approximate remainder after preparation ms | CPU wood buffer capacity MiB |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak 1 | 484.2 | 335.0 | 228.8 | 149.2 | 197.2 |
| oregon-white-oak 7 | 542.2 | 374.2 | 255.4 | 168.0 | 225.6 |
| norway-spruce 1 | 500.8 | 366.9 | 172.4 | 133.9 | 161.3 |
| norway-spruce 7 | 477.2 | 342.9 | 165.1 | 134.3 | 152.7 |

Oak still spends roughly 150–170 ms beyond preparation, and its CPU wood arrays occupy about 197–226 MiB by existing capacity accounting. Removing just the CPU wood build while leaving submission unchanged would not reach the 157/186 ms oak completed-frame targets. A GPU wood candidate should therefore replace CPU expansion and retain the generated vertex/index buffers through rendering, not read the full mesh back and upload it again. CPU-owned output must remain a separately measured representation.

Task .3 will profile wood preparation, vertex/index emission and normal computation before the host selects that boundary. A CPU compact-descriptor or position-only upload could preserve exact contact geometry while parallelizing remaining expansion, but its usefulness is unproven until that split is measured. General GPU wood expansion must still handle caps, collapsed-triangle rejection, run spans/order, finite normals, bounds and leaf contact correctness. These contracts are not waived by the accepted pixel differences.

This is source-grounded design context and stage-budget arithmetic, not a new performance result, strategy change, acceptance relaxation or an implementation task for a GPU wood pipeline.
