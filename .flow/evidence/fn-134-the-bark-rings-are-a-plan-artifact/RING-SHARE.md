# Ring share of the wood build (fn-134, 2026-09-24)

This was measured at base `b8acb508`, built `--release`, at seed 1, on a 32-CPU machine; the parallel path used 8 workers. Each preset is one tree, and each number is the median of 5 runs after one warm-up.

A temporary `pub fn scratch_build` called `build_mode` with `parallel_allowed` forced true or false. Contacts were on only where the pipeline seats leaves on the wood, which is the spruce alone. Timers were `Instant` durations summed into static atomics. All of it has been reverted.

## What was counted

- **Ring** is the rank pass plus the per-run ring work:
  - rank pass: `sample_path` over every run (build.rs:167-171)
  - per run: `sample_path`, `frames`, `record_edges` and `emit_run` (build.rs:231-245)
  - on the parallel path: the rank pass, `record_edges` (parallel.rs:180-188) and phase 1 (parallel.rs:192-247)
- **Mesh** is everything else:
  - serial: indices, `normals::shade`, the run table, and `finish` with bounds
  - parallel: run layout, the normals and indices zero-fill, phase 2 (parallel.rs:249-314) and the tail
- **Prep** is counted in neither: validation, `paths()`, `distance` and sizing.
- `emit_run` writes coords in the same closure as positions, so coords are counted as ring.
- `alloc1` is the parallel path's zero-fill of positions and coords. It is counted in neither.
- Ring share = ring / (ring + mesh).

| preset | schedule | ring ms | mesh ms | prep / alloc1 ms | wall ms (timed) | wall ms (no timers) | ring share | load 1m |
|---|---|---|---|---|---|---|---|---|
| oregon-white-oak | serial | 46.8 | 100.3 | 8.5 | 159.0 | 164.9 | 31.8% | 0.60 |
| oregon-white-oak | parallel, wall | 14.4 | 35.9 | 8.4 / 7.4 | 66.5 | 70.3 | 28.7% | 0.60 |
| oregon-white-oak | parallel, CPU sum | 54.5 | 108.5 | | | | 33.4% | |
| norway-spruce | serial | 43.4 | 83.1 | 4.8 | 134.5 | 128.2 | 34.3% | 0.60 |
| norway-spruce | parallel, wall | 11.8 | 31.4 | 4.3 / 6.2 | 53.9 | 53.7 | 27.3% | 0.60 |
| norway-spruce | parallel, CPU sum | 45.0 | 94.7 | | | | 32.2% | |
| silver-birch | serial | 27.9 | 57.2 | 5.2 | 92.4 | 87.7 | 32.8% | 0.60 |
| silver-birch | parallel, wall | 6.9 | 21.5 | 5.1 / 6.6 | 40.3 | 40.9 | 24.2% | 0.60 |
| silver-birch | parallel, CPU sum | 25.5 | 64.1 | | | | 28.4% | |
| ordinary | serial | 4.1 | 9.3 | 0.3 | 14.0 | 13.8 | 30.7% | 0.60 |
| ordinary | parallel requested (ran serial) | 3.9 | 9.3 | 0.3 | 13.9 | 13.8 | 29.5% | 0.60 |

Notes on the table:

- Load was recorded before each preset. The untimed baseline pass saw 0.55 to 0.69.
- The timers cost less than the noise between runs: timed minus untimed ranges from -6 to +6 ms, with no consistent sign.
- The ordinary tree was not admitted to the parallel path by `parallel::admitted` (parallel.rs:41-68), so it ran serial.
- On the oak's parallel build, the normals and indices zero-fill (`alloc2`) took 17.8 ms of wall. That is longer than phase 2 itself, at 12.5 ms.
- Work is balanced across the workers. On the oak, phase 1 had a slowest worker of 6.4 ms against a sum of 46.7 ms; phase 2 had 12.4 ms against 85.2 ms.

## How ring and mesh work are interleaved

- **Serial** (build.rs:223-323): each run does its ring work and then its mesh work in the same loop iteration. `shade` reads the positions that run has just appended.
- **Parallel**: two phases of scoped threads over the same contiguous run partitions (parallel.rs:71-90).
  - Phase 1 writes only positions and coords, which is ring work only.
  - Phase 2 writes only indices and normals, which is mesh work only.
  - `record_edges` runs serially before phase 1.

## Can the mesh step take a precomputed float32 ring array as `positions` without copying?

**The parallel path already does.**

- Phase 2 reads `positions` as a borrowed slice split per worker (parallel.rs:253, 264, 274).
- The mesh returns the same `Vec` that phase 1 filled (parallel.rs:190, 325).
- Phase 2 needs only `segments` and each run's `base`, `rings`, `first_index` and `index_count` (parallel.rs:8-15, 277-283).
- It needs no frames. Any dropped triangle or zero normal makes the phase fail (parallel.rs:290-295), and the build then reruns serially (build.rs:200).

**The serial mesh step reads `mesh.positions` in place** (build.rs:259, 303-309). Besides the positions, it needs:

- `frame`, for the `facing` fallback (build.rs:308, 387-401). The fallback is used only for a vertex left with no triangle.
- `samples.len()`, as the run's ring count (build.rs:250-251, 289).
- `largest_radius` from the rank pass (build.rs:169, 321), for the run table.

**What the other outputs depend on:**

- Indices depend only on `base`, `rings` and `segments` (prepared.rs:14-36).
- Bounds depend only on positions (build.rs:410-419).
- Dropped-triangle handling uses positions and indices (normals.rs:37-69). The limit is checked in `finish` (build.rs:404).
- Coords need each ring's `d` and each segment's `angle` (build.rs:376, 380-382). Today they are written during the ring step.
- Each sample's radius `r` reaches the mesh only through `largest_radius`.
- `record_edges` needs only `base`, `rings`, `segments` and the trunk offset (build.rs:335-351).

**The prepared path already keeps a ring-step product.** It stores positions, one `[d, mean radius]` pair per ring, one angle per segment, and the run descriptors (prepared.rs:66-77; build.rs:216-222, 264-271). It recomputes the ring radius from the float32 corners (prepared.rs:141-150).

## Facts

Across the four presets, the ring step takes 31-34% of serial ring+mesh time. On the parallel path it takes 24-29% of wall time and 28-33% of summed worker CPU. Mesh time is 2 to 2.5 times ring time in every row.

On the parallel path, the two zero-fill allocations take 18-25 ms combined, which is longer than the whole ring wall of 7-14 ms. Prep adds another 4-8 ms.

The serial path does each run's ring and mesh work in the same loop iteration. The parallel path already splits them into two phases, and phase 2 reads the finished float32 positions without a copy.

Only the serial mesh step depends on ring-step intermediates. It uses the per-run frames for the fallback normal of a vertex with no triangle, and the rank pass's largest radius for the run table.
