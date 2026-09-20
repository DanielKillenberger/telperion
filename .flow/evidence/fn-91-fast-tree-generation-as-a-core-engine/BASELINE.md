# Mature generation baseline — native checkpoint

Revision `507d3ef1`; release Rust on Linux / Ryzen 9 5950X. Exact parameters, binary hash, runtime and raw samples are beside this report. Full CPU wood geometry and compact foliage placements, without rendering. One first-process build followed by five warmed builds per case. Shared machine; no CPU isolation. No Quick commands are defined in the spec (`baseline: none`).

Times are milliseconds, warm median (maximum); five samples do not establish a population p95. Cold is first build in a fresh native process, excluding process startup and parameter parsing.

| Species / seed | Cold build | Warm total | Growth | Wood | Placement + attachment | Culling | Bounds |
|---|---:|---:|---:|---:|---:|---:|---:|
| norway-spruce / 1 | 5607.81 | 5581.80 (5590.10) | 39.12 (42.23) | 205.88 (208.27) | 4030.32 (4038.13) | 360.31 (361.21) | 943.29 (944.87) |
| norway-spruce / 7 | 5336.75 | 5338.82 (5400.39) | 38.08 (41.04) | 197.33 (198.89) | 3849.14 (3879.03) | 344.12 (353.57) | 901.36 (933.31) |
| oregon-white-oak / 1 | 953.95 | 942.79 (947.07) | 59.47 (60.62) | 252.21 (253.13) | 217.04 (217.68) | 99.77 (99.98) | 316.40 (316.74) |
| oregon-white-oak / 7 | 1139.02 | 1136.90 (1146.57) | 66.88 (75.91) | 293.22 (300.61) | 263.96 (268.14) | 121.76 (123.28) | 388.51 (389.19) |

All 24 builds report complete; all wood dropped counts are zero. Oak seeds retain 715,065 and 869,310 leaves; spruce retains 7,353,754 instances at seed 1 (seed 7 is recorded in the raw rows). The timing runner follows the public mature assembly stages; the final constant-size wood/foliage bounds union is omitted. Element/reference preparation is separately recorded and below 0.3 ms in these cases. No generator behavior changed.

R1 remains incomplete: native rendering upload, canopy mass construction, GPU completion, cold process startup, phone hardware and peak memory domains are unmeasured. Attachment construction is included in placement. Browser measurements are a separate checkpoint. These CPU measurements cannot establish time to first completed frame.

Stage selection: foliage bounds consume about 34% of oak generation. Placement plus attachment consumes about 72% of spruce generation. The host is evaluating bounded candidates; no optimization or acceptance verdict is claimed.

## Reproduction and browser scope

Build native: `cargo build --release -p telperion-core --example generation_stages`.
Run each case in its own process: `target/release/examples/generation_stages oregon-white-oak 1` (also seeds 7 and `norway-spruce`). The default is six builds: the first is reported separately and five warm samples follow. `GENERATION_SAMPLES` overrides the count. `GENERATION_NO_CONTACT=1` is a diagnostic changed workload, never a qualifying optimization.

Build browser: `npm run wasm:build && npm run render:build`.
Run: `GENERATION_OUTPUT=NEW_FILE timeout 240s node scripts/benchmarks/mature-generation.mjs` with a hardware-capable Chromium and display. The current run uses Vite development serving. Initialization starts after navigation, includes module import/initialization and device creation, and is affected by Vite transform and browser cache warming. It is not cold website startup. The recorded frame submissions use the default camera, without framing the tree. There is no GPU completion fence: successive CPU builds may overlap prior GPU work. Frame submission is a lower bound, not first completed frame evidence. Future comparisons must state camera/protocol changes rather than treating these rows as completed-frame baselines.

Per-case native parameters and counts, browser diagnostics/counts, and compiled Wasm/native binary hashes are retained. Output-array digests were not captured; exact byte comparisons remain work for a behavior-preserving candidate. Wasm linear-memory capacity is recorded for owned CPU outputs; it is not peak browser process memory and cannot be added to overlapping native allocations.

## Browser measurements

Chromium 152.0.7977.82, viewport 1280 × 720 CSS pixels, hardware/flags in JSON. All four cases completed. Warm values are median (maximum) milliseconds.

| Species / seed | Module + renderer init | First setTree | Warm setTree | Owned CPU build | CPU Wasm capacity bytes |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak / 1 | 2862.3 | 2325.4 | 1428.7 (1434.0) | 1391.4 (1395.9) | 261816320 |
| oregon-white-oak / 7 | 208.7 | 2756.8 | 1680.1 (1709.3) | 1660.8 (1663.3) | 328990720 |
| norway-spruce / 1 | 182.2 | 9676.6 | 7335.6 (7342.9) | 7247.6 (7256.2) | 472973312 |
| norway-spruce / 7 | 219.0 | 9172.8 | 7011.8 (7041.3) | 6903.0 (6909.6) | 455016448 |

## Contact diagnostic

Oak already has `surface_contact=0`; its diagnostic is an unchanged-workload control. Spruce changes `surface_contact` from one to zero, so its result is diagnostic only: attachment construction and projection queries both disappear, and placement positions change. One first build and one warm build per seed-1 species; no distribution claim and no acceptance result.

| Species | Preset placement warm median | No-contact placement warm sample | No-contact total warm sample |
|---|---:|---:|---:|
| oregon-white-oak | 217.04 | 217.69 | 952.67 |
| norway-spruce | 4030.32 | 2063.24 | 3633.35 |

This bounds the combined contact workload only. It does not distinguish attachment preparation from per-station projection. No public profiling API or generator change was added.

Verification: native example release build, Rust formatting check, JavaScript syntax check, four CLI error controls, 24 complete native preset builds, four complete native diagnostic builds, and 48 complete browser builds. The initial invalid-seed control expected a prose error string, while Rust emits `InvalidDigit`; the corrected control passed. Full workspace suites were not run for this measurement-only checkpoint.

Task remains in progress. Candidate selection/tuning awaits owner acceptance bounds and host direction. `stage: impl-review - skipped(config: REVIEW_MODE=none)`
