# Wood stage profile at b0c38c25

The native control builds wood in 144–205 ms warm across the four fixtures. Normal accumulation/normalization is the largest measured single group (49–72 ms instrumented); vertex/coordinate plus index/cap emission together is comparable (52–76 ms). Moving only normal computation leaves most construction and submission work. The host retains the next boundary decision. This diagnostic does not qualify parent R1, R2 or R5.

## Protocol and limits

Revision `b0c38c259bb733f1b85b137f4cf36916e4352724`, native Linux Ryzen 9 5950X, Rust release with LTO and one codegen unit. `environment.json` retains exact compiler and hardware details. Each preset/seed generates its solved tree once, then constructs full CPU wood four times. Sample 0 is the first wood build after generation; samples 1–3 supply warm medians. These are initialized-process timings, excluding cold process startup. No foliage expansion, device, upload, frame, screenshots or phone measurement ran.

The same example and actual source revision built first as control, then with scratch-only timers. Both release builds passed (~15 seconds each). All four control processes ran before all four instrumented processes on a shared machine without CPU isolation. This order, allocator/cache state and compiler optimization effects confound pure timer cost. Observed warm overhead is material (6.5–21.3%); it is reported separately and never charged as production work. No tuning reruns were made. Three warm observations cannot establish a p95.

| Fixture | Control first | Control warm | Instrumented first | Instrumented warm | Observed delta | Radius control warm |
|---|---:|---:|---:|---:|---:|---:|
| oregon-white-oak 1 | 176.60 | 176.56 | 203.82 | 214.24 | +37.68 (21.3%) | 14.59 |
| oregon-white-oak 7 | 217.07 | 205.08 | 231.07 | 223.71 | +18.62 (9.1%) | 17.85 |
| norway-spruce 1 | 167.65 | 150.25 | 165.61 | 163.58 | +13.33 (8.9%) | 12.58 |
| norway-spruce 7 | 141.67 | 143.73 | 156.23 | 153.07 | +9.34 (6.5%) | 12.08 |

All times are milliseconds. Radius recovery calls the existing `telperion-render/src/wood/radius.rs::radii` directly, after surface construction and before hashing. It allocates a separate f32 vector; its time is outside wood totals. Instrumented radius warm medians are 15.65/16.16/12.27/11.47 ms in table order; their variation also cautions against treating all control deltas as timer overhead.

## Instrumented stage medians

| Stage | Oak 1 | Oak 7 | Spruce 1 | Spruce 7 |
|---|---:|---:|---:|---:|
| validation_paths_allocation | 11.17 | 7.81 | 5.66 | 5.26 |
| ranking_samples | 7.61 | 6.48 | 4.66 | 4.31 |
| ranking_sort | 1.86 | 1.70 | 1.67 | 1.50 |
| emission_samples_frames | 29.81 | 31.96 | 25.03 | 21.46 |
| vertices_coords | 38.76 | 40.50 | 29.37 | 27.30 |
| indices_caps | 32.48 | 35.07 | 24.95 | 24.28 |
| normal_buffer_resize | 6.25 | 6.73 | 4.21 | 4.74 |
| normal_accumulation | 48.31 | 52.25 | 37.76 | 35.41 |
| normal_normalization | 17.78 | 19.39 | 14.00 | 13.21 |
| run_table | 1.72 | 1.39 | 1.59 | 1.42 |
| final_bounds | 5.17 | 5.69 | 4.06 | 3.85 |
| Median sum of stage durations | 200.91 | 209.01 | 152.41 | 142.49 |
| Median outer total minus stage sum | 13.32 | 15.22 | 11.17 | 10.58 |

Per-run timings aggregate without printing inside the loop. The outer total includes scope exit and scratch-buffer destruction. Timer calls and bookkeeping between measured intervals, loop overhead and scratch destruction contribute to the unassigned 10.6–15.2 ms; they cannot be independently separated here. Stage medians need not sum to the median total. Timers also perturb measured intervals and code generation, so these stage costs and shares are approximate. `surface.patch` preserves geometry operation order. Normal accumulation and normalization have separate clocks inside the existing shade function.

## Output and allocation checks

| Fixture | Vertices | Triangles | Runs | Full surface FNV-1a64 | Surface capacity MiB | Radius MiB |
|---|---:|---:|---:|---|---:|---:|
| oregon-white-oak 1 | 3,724,874 | 7,227,360 | 55,597 | `b82b2cf44372d4bf` | 197.23 | 14.21 |
| oregon-white-oak 7 | 4,262,170 | 8,255,000 | 67,335 | `d2b8c64f7c86ebeb` | 225.57 | 16.26 |
| norway-spruce 1 | 3,050,636 | 5,893,480 | 51,948 | `6fdd44ed9ad69f53` | 161.34 | 11.64 |
| norway-spruce 7 | 2,888,144 | 5,580,040 | 49,062 | `8471722f3923dd42` | 152.75 | 11.02 |

All 32 builds match their fixture hash, including first and warm runs; every dropped count is zero. The digest length-prefixes positions, normals, coordinates and indices, then includes optional bounds, run count, dropped count and every ordered run-table field. Floats use exact little-endian bit representations. FNV-1a64 is a deterministic all-field digest, not a collision-proof byte comparison. Hashing and JSON emission run outside the measured sections.

Source-visible allocation lifetime matters. Preparation reserves output positions, normals, coordinates, indices and run-table capacity, plus paths, node distances and longest-run scratch. Ranking samples each path into reusable scratch and retains an ordered path/radius vector. Emission samples each path again and reuses sample, segment and frame vectors. Index/cap emission includes cap positions/coordinates. Normal resize zero-fills new vertices, accumulation reads float32 triangles into f64 arithmetic while compacting dropped triangles, and normalization writes f32 normals. Bounds scans all positions. Paths, distances, ordering and scratch live during build; output arrays survive return. Radius recovery adds its separate output while the full wood remains alive. Capacity figures exclude those temporary allocations, allocator metadata and GPU allocations; they are not a measured peak-memory claim.

## Upper bounds for host selection

Zero-cost removal of one stage cannot save more than that stage’s production cost. These timers cannot identify that cost exactly. As an illustrative proportional budget only, normals accumulation plus normalization consume 31–32% of instrumented wood total; vertex/coordinate plus index/cap emission consume 33–35%. The known positive observer overhead prevents interpreting their raw instrumented milliseconds as achieved or precisely removable latency.

The existing completed-frame browser matrix in `../WOOD-BOUNDARY.md` reports wood as 47.3%, 47.1%, 34.4% and 34.6% of completed-frame time in fixture order. Those whole-wood fractions are upper bounds for any wood-only removal at that recorded browser workload, assuming unchanged remaining stages and zero replacement cost. A normals-only path is bounded below that ceiling. Multiplying the approximate native normals shares by browser wood fractions gives an illustrative 14.6%, 15.1%, 10.9%, 11.0% end-to-end budget, not a browser measurement or speedup promise. Cross-runtime shares can differ. A fresh matched end-to-end run is required for any candidate.

The existing host boundary note establishes that removing wood construction alone leaves roughly 255/287 ms for oak, including other preparation and submission, above its 157/186 ms completed-frame targets. A candidate intended to meet that target therefore has to replace CPU expansion and the relevant upload/submission work while preserving rendered output contracts; accelerating an isolated normal stage cannot establish qualification. Run order, cap emission, collapsed-triangle handling, bounds and CPU-owned output remain part of the existing contracts. No implementation boundary is selected by this report.

## Reproduce and verify

From the repository root, run `bash .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile/run.sh`, then `python3 .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile/analyze.py`. The driver archives the recorded commit into a fresh temporary directory and shares the repository target directory by default. Reserve an idle build/timing window; `WOOD_TARGET` selects another target directory. Existing evidence output files are overwritten by a reproduction, so copy this evidence directory first to preserve this run. `WOOD_SCRATCH` is for an already prepared, uninstrumented archive only. No worktree is removed.

`run.sh`, `wood_profile.rs`, `instrument.py` and `surface.patch` retain the diagnostic. Raw JSONL, summary JSON, binary SHA-256 digests and build logs retain the observations. The analyzer verifies 32 selected samples, one identical full-surface hash per fixture, and positive stage sums bounded by outer totals. Baseline was none because the task defines no Quick commands and excludes whole-workspace suites. Product source, APIs and defaults remain unchanged.
