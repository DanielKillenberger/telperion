# Resident wood experiment

Native and browser consumers that explicitly request GPU-resident delivery now keep expanded wood buffers on the GPU. The final native seed-1 completed-frame medians improve from 449.55 to 232.95 ms for oak and 458.84 to 262.88 ms for spruce. All four browser fixtures also improve over the prior GPU foliage candidate. The default CPU surface retains all four complete-field hashes. Task .4's implementation and measurements are complete; the host's scoped visual capture is complete and the owner verdict is pending in [visual-lit/VERDICT.md](visual-lit/VERDICT.md). Task .1 and parent R1-R5 remain open.

## Comparable measurements

Both native executables use the release profile, LTO and one codegen unit on the existing Ryzen 9 5950X / NVIDIA RTX 3080 / Linux desktop. Each process regenerates mature seed 1 once first and three times warm. Resident delivery includes preparation, submission, a 1280×720 hero frame and actual device completion. Process `wait4` RSS includes initialization and teardown. No build or other task measurement ran concurrently. Background OS activity and disk shader caches remain uncontrolled.

The preserved baseline executable is `/tmp/telperion-fn91-tools/generation-gpu-readback-candidate`, SHA-256 `712018723addd5f642781d55086f073b4bc9c9b4da95d08b0cc7341a8d256a81`. Its provenance is in `../READBACK.md`; task .3 changed no product code. The first candidate executable ends in `generation-gpu-resident-wood-candidate`, SHA-256 `a1d044d8675b257d8cc78ac967dc5bb85360d2d58f2162b0248d11edd3a917aa`. The final executable ends in `generation-gpu-resident-wood-final`, SHA-256 `f70e94cee6e07b2c972abfe5cc78c1e6c4874940046418fec116d9021decb07a`. The final build includes the fit-count fix and reporting changes in checkpoint `8f025f80`; the surface and shader algorithms are the same as the first candidate.

| Native experiment | Warm completed-frame p50 before → after ms | Improvement | Peak RSS before → after KiB |
|---|---:|---:|---:|
| First candidate, oak 1 | 432.48 → 224.48 | 48.10% | 579424 → 352592 |
| First candidate, spruce 1 | 468.04 → 259.08 | 44.65% | 484052 → 387988 |
| Final candidate, oak 1 | 449.55 → 232.95 | 48.18% | 529196 → 353356 |
| Final candidate, spruce 1 | 458.84 → 262.88 | 42.71% | 483636 → 391008 |

The first experiment cleared the required greater-than-20% improvement for both species before browser work began. `resident-*.jsonl` retains that decision; `final-resident-*.jsonl` retains the final executable comparison. The two RSS sets demonstrate run variability, especially in the baseline oak process. Three warm samples cannot establish tail latency or statistical attribution of small changes. `summary.json` contains cold timings, initialization, warm minima/maxima and every sample; the JSONL retains full stages and counts.

Browser measurements use Chromium with the same Vulkan/WebGPU flags, viewport, hero pose and actual renderer-queue completion protocol as `../browser-completed-gpu.json`. Each fixture gets one first build and five regenerated warm builds. The existing baseline predates this experiment on the same desktop; it is not a simultaneously rerun browser baseline. `browser-completed.json` records the new Wasm hash, browser version, hardware, initialization, first builds, warm distributions and explicit allocation counters.

| Browser fixture | Prior GPU → resident wood warm p50 ms | Improvement | New Wasm memory bytes | Speedup over original pre-GPU baseline |
|---|---:|---:|---:|---:|
| Oak 1 | 484.2 → 325.5 | 32.78% | 101187584 | 4.82× |
| Oak 7 | 542.2 → 383.7 | 29.23% | 189267968 | 4.85× |
| Spruce 1 | 500.8 → 384.0 | 23.32% | 216006656 | 19.87× |
| Spruce 7 | 477.2 → 366.5 | 23.20% | 209715200 | 20.15× |

Browser oak remains below the parent 10× target. Every fixture remains above 100 ms, by 225.5–283.7 ms in the browser and 132.95–162.88 ms in the final native resident run. Module/renderer initialization and first-build pipeline work are separate fields; neither protocol measures full public-page startup or compositor presentation. Phone-class hardware remains unavailable.

## Retained CPU output

`cpu-measure.py` runs both ordinary CPU-owned geometry and standalone GPU generation with owned CPU output in separate fresh processes, one first plus three warm samples each. Standalone CPU-output generators create no wood pipeline. These observations measure regressions without claiming every difference is caused by this change.

| Representation / specimen | Warm p50 before → after ms | Change |
|---|---:|---:|
| CPU API, oak 1 | 564.72 → 597.04 | +5.72% |
| CPU API, spruce 1 | 4571.33 → 4584.13 | +0.28% |
| GPU engine owned CPU, oak 1 | 267.44 → 255.74 | -4.37% |
| GPU engine owned CPU, spruce 1 | 301.60 → 304.90 | +1.09% |

The direct-CPU oak regression is recorded and unresolved as an attribution question. The separately timed CPU wood stage in GPU-owned output improves from 180.16 to 172.01 ms for oak and changes from 138.42 to 138.81 ms for spruce. This evidence does not isolate the direct-CPU oak difference to wood. `cpu-rss.json` and the output JSONL retain RSS, initialization, cold results and stages. No retuning or weakened target followed these results.

## Geometry and lifecycle proof

Core preparation reuses path sampling, frames, angular profiles and vertex emission with the ordinary builder. It emits canonical float32 positions, rounded per-ring distance/radius, rounded angular values, bounds and ordered run metadata. Full index, normal, coordinate and per-vertex radius arrays are absent from CPU preparation. CPU float64 checks inspect every procedural triangle; collapse or unsupported cross-product range requests CPU wood. The default builder retains its fast strip/cap loops and allocation strategy. Its ring capacity now correctly counts every buried trunk in a clump, avoiding unchecked growth for that case.

The GPU generates final renderable indices and attributes and gathers adjacent triangles in the original order per vertex. Ring vertices gather bounded adjacent triangles; cap centres gather cap faces, with no float atomics. Small status readback rejects unusable normals before adoption. Compute-only storage/dispatch limits fall back to CPU wood; ordinary renderer limits still reject truly unrenderable output. Renderer identity, stale requests and disposal gate adoption. Actual buffer usage flags survive resident ownership and later CPU writes, including growth to a larger allocation. Resident counts, bounds and spans are explicit metadata used by fit and submission checks.

The focused mature-fixture comparison checks every GPU position, index, coordinate and radius against CPU output, plus bounds/run spans and finite unit normals. Complete CPU hashes match task .3 exactly.

| Fixture | Complete CPU surface FNV-1a64 | Maximum normal angle radians |
|---|---|---:|
| Oak 1 | b82b2cf44372d4bf | 2.16452e-7 |
| Oak 7 | d2b8c64f7c86ebeb | 2.62362e-7 |
| Spruce 1 | 6fdd44ed9ad69f53 | 2.15938e-7 |
| Spruce 7 | 8471722f3923dd42 | 2.14908e-7 |

The tolerance was fixed at 0.001 radians before implementation. Other focused checks cover curved/twisted capped runs, 1,025 rings, 65,536 runs across dispatch rows, clumps, empty/root-only surfaces, collapsed triangles, unusable GPU normals, compute limits, destroyed devices, foreign prepared buffers and subsequent CPU allocation reuse/growth. Existing foliage contact/repeatability and request-state checks remain green. Browser smoke verifies resident wood reporting, submitted counts, explicit full CPU fallback, invalid JSON, overlap, synchronous replacement preserving rendered stats, disposal and calls after disposal. `browser-smoke.json` holds the result.

## Allocation accounting and remaining limits

In the first native run, prepared wood CPU capacity is 48,145,532 bytes for oak and 39,656,536 for spruce. Additional packed metadata capacity is 2,335,104 / 2,009,944 bytes. New wood GPU peak counters are 223,158,952 / 182,554,664 bytes, including final buffers, metadata, configuration, status and the 16-byte status staging allocation. The four-byte host status result is separate. Temporary CPU inputs drop after submission; temporary GPU metadata drops after the completion/status check.

Each replacement reports the actual previous live tree's buffers. In the first candidate's warm oak replacement, 254,810,296 previous bytes coexist with 223,158,952 new wood bytes and 8,850,092 new foliage bytes, totaling 486,819,340 explicitly counted GPU bytes. Spruce totals 688,000,908 bytes from 416,821,772 previous, 182,554,664 wood and 88,624,472 foliage. Foliage compute occurs earlier, so its separate peak plus the previous tree must also be considered. The counters do not sum sequential stages as simultaneous allocations.

These are explicit allocation counts, not whole peak-memory qualification. CPU preparation scratch, triangle-rejection preparation discarded before a CPU fallback, allocator overhead, upload staging inside wgpu, pipeline/compiler/driver allocations, deferred destruction and device-local residency remain incompletely measured. Prepared-capacity accounting is recorded before compute-limit fallback. Wasm linear memory is a module high-water capacity, and native RSS includes several memory domains. Lower RSS and retained GPU allocations do not prove the parent's no-peak-rise requirement across all domains.

## Gates and outstanding acceptance

Baseline was green for 14 core tests and 12 renderer generation/wood tests, plus Wasm check and scoped formatting. The new preparation API test first failed compilation because the API was absent. The first GPU test then exposed the WGSL reserved-name error; its correction and rebuild friction are recorded in `../FRICTION.md`. No assertion or baseline was weakened.

Final focused gates pass 17 core tests (one pre-existing skipped reference test) and 17 renderer tests. Build receipts are in the task handover and accompanying logs. Wasm/glue build, Wasm check, TypeScript, browser lifecycle smoke and the completed-frame matrix pass. The historical unrelated full-workspace Vulkan/LTO test failure remains outside this task's explicit gates and is not reported green.

The host captured and inspected four scoped images and opened the comparison gallery. The remaining task .4 acceptance is the recorded owner verdict in [visual-lit/VERDICT.md](visual-lit/VERDICT.md). Earlier owner approvals cover the previous candidate only. Supported motion, wider views, phone hardware, full cold startup and whole CPU/GPU peak memory remain parent qualification gaps. Task .1 and parent targets are unchanged.

The owner rejected the first interior shaded capture set as unjudgeable. The host replaced it with four exterior bare-wood images under camera-side lighting, inspected them and opened a full-width slider gallery. The replacement has 12 changed oak pixels and 8 changed spruce pixels out of 921,600 each. The owner verdict is still pending; the failed set is retained.
