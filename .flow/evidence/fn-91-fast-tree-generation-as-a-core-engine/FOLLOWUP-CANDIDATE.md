# Angular cache and standalone CPU output

Invocation 4 starts at 880db513. The shared surface helper caches the existing fixed sine/cosine expressions once per build and the existing profile expression for zero twist. Nonzero twist retains per-ring phase/profile evaluation. The helper uses checked reservation; its transient allocation is 40 bytes per angular sample on this native target (three f64 values and Option<f64>), released when each surface/attachment build returns. No output buffers or GPU algorithms change.

`Generator::for_cpu_output` initializes only the device's compute pipelines and rejects resident delivery before generating a skeleton. Renderer-bound construction and identity checks remain intact. The native benchmark now uses this constructor for gpu-output; initialization is recorded separately and process RSS includes it.

The comparison uses the saved first-GPU executable and a fresh candidate executable, the same four native modes, one cold plus three warm specimens per fresh process, seed 1 oak/spruce. Warm specimens are regenerated. Resident totals include hero-camera rendering at 1280×720 and device completion. Initialization is excluded from per-build totals but included in process elapsed time and RSS. Explicit allocation counters omit allocator, shader/compiler, driver, textures, pipelines and other device domains; process RSS also cannot measure all device-local VRAM. The saved baseline executable predates the signed-zero correction and engine-neutral Family move; these do not intentionally change these ordinary fixtures. No new visual qualification is claimed.

Results and focused gate outcomes are appended after measurement. Acceptance remains open; this checkpoint does not claim the 10×/100 ms or no-memory-rise requirements.

## Measured result

Warm p50 in milliseconds, three regenerated samples after one cold build; maxima, cold samples, initialization, raw stages and process elapsed times are retained in `followup-summary.json` and JSONL. This is not a statistically robust tail estimate.

| Specimen/mode | Previous ms | Candidate ms | Previous peak KiB | Candidate peak KiB |
|---|---:|---:|---:|---:|
| oregon-white-oak cpu-output | 642.29 | 557.79 | 254540 | 256224 |
| oregon-white-oak gpu-output | 358.97 | 270.31 | 508356 | 482132 |
| oregon-white-oak cpu-render | 872.83 | 755.12 | 584096 | 586388 |
| oregon-white-oak gpu-render | 526.04 | 426.18 | 529396 | 529100 |
| norway-spruce cpu-output | 4776.65 | 4586.11 | 403640 | 405716 |
| norway-spruce gpu-output | 489.67 | 363.33 | 500028 | 494636 |
| norway-spruce cpu-render | 5291.96 | 5178.79 | 619592 | 617940 |
| norway-spruce gpu-render | 636.39 | 463.33 | 485300 | 485044 |

The saved GPU candidate is the comparison baseline here, not the original pre-fn91 generator. The cache improves both species and all measured modes. The standalone constructor reduces initialization/renderer allocation but does not establish no-memory-rise against pure CPU delivery. A zero-build fresh-process diagnostic measured 128748 KiB for standalone GPU-output and 135296 KiB for GPU-render (the latter includes a Frame); neither is subtracted from the full peak.

RSS varies across matrices: prior oak GPU-render peak was 621992 KiB; this matched matrix observed 529396 KiB before and 529100 KiB after. Shader disk caches and process allocator state remain uncontrolled. These are raw observations, not qualification or evidence that the difference belongs entirely to this change. Device-local memory remains only partially observable through the explicit allocation counters.

Candidate warm stage medians (ms):

| Specimen/mode | Skeleton | Descriptors | Wood | GPU wait | Readback |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak gpu-output | 64.60 | 22.11 | 178.93 | 1.29 | 1.62 |
| oregon-white-oak gpu-render | 59.23 | 20.07 | 170.83 | 1.31 | 0.00 |
| norway-spruce gpu-output | 44.11 | 65.31 | 144.66 | 12.66 | 74.90 |
| norway-spruce gpu-render | 42.56 | 63.99 | 139.80 | 12.38 | 0.00 |

The remaining GPU placement, compaction, mass and transfer costs are all retained separately in the JSON. The existing explicit CPU/GPU live-buffer counters are unchanged; the angular cache adds 800 transient CPU bytes (20 samples × 40 bytes) during each surface/attachment build, not included in those older counters. CPU and GPU peaks are not interchangeable or additive without residency/lifetime evidence.

## Verification

Seven selected isolated tests passed in 3.723 seconds: exact angular old-formula parity across zero/nonzero twist, zero/nonzero lobes and signed zero; standalone GPU-backed owned output and early resident rejection; all five existing generation tests, including culling, spatial contacts, repeatability, renderer identity and signed-zero extrema. Only core/render library test binaries were compiled (44.34 seconds); benchmark examples built in 43.87 seconds. No broad workspace or renderer integration suite was repeated.

Existing native CPU specimen fingerprints (wood positions/indices, packed foliage words, element positions/indices, and foliage bounds; the prior runner does not hash wood normals/coords or the reference separately) match all four prior artifacts:

- oregon-white-oak seed 1: `94abae63f92f82d3` (unchanged).
- oregon-white-oak seed 7: `08e12dde55361de5` (unchanged).
- norway-spruce seed 1: `9de204f696703019` (unchanged).
- norway-spruce seed 7: `ce62d7a2658b2d1b` (unchanged).

Wasm buildability (`cargo check --release --target wasm32-unknown-unknown -p telperion-wasm --lib`) passed in 1.65 seconds. Scoped Rust formatting passed. The task remains in progress. Invocation 4 adds one checkpoint; acceptance, broader views/motion, original-baseline desktop qualification and phone evidence remain with the host. Saved candidate executables and both previous hashes are listed in `followup-binaries.sha256`.
