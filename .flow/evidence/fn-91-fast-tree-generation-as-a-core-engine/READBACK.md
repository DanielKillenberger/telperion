> Historical raw-output references: see the [archive and recovery instructions](README.md).

# Bounded leaf readback

Task .2 replaces both CPU delivery and explicit resident verification readback with one shared asynchronous helper. It reserves the final `Vec<[u32; 3]>` fallibly once, copies sequential chunks into one reusable staging buffer, and decodes mapped words directly into the final vector. The production chunk is 4,194,300 bytes (4 MiB rounded down to a multiple of 12), further limited by the source length and device buffer limit. Zero leaves returns before allocation or GPU work. Source size/usage, allocation, mapping, device loss and scoped GPU errors return an error; the owned partial vector never escapes. Dropping the future drops its local output and staging resources. Native polling is cfg-excluded from Wasm.

## Matched measurements

Baseline source is c8b45f53, freshly built before the implementation; candidate is this task's change on that revision. Both native release example builds use identical Cargo settings (LTO, one codegen unit). Binary SHA-256 values are in `readback-rss.json`; preserved executables are `/tmp/telperion-fn91-tools/generation-gpu-readback-{baseline,candidate}`. Hardware: Ryzen 9 5950X, NVIDIA GeForce RTX 3080, Linux x86-64, rustc 1.98.1. Each species/revision runs in a fresh process: mature seed 1, standalone GPU engine, owned CPU output, one cold plus three regenerated warm samples. No render/frame/viewport work is included. No compilation or host benchmark ran concurrently. OS background work and shader disk caches are uncontrolled.

| Specimen | Warm p50 before → after ms | Readback p50 ms | Peak RSS before → after KiB | Process elapsed seconds |
|---|---:|---:|---:|---:|
| Oak 1 | 259.27 → 255.65 | 1.594 → 1.206 | 488540 → 486380 | 1.506 → 1.431 |
| Spruce 1 | 351.73 → 297.03 | 74.733 → 22.575 | 495860 → 493452 | 1.673 → 1.463 |

Warm p50 improves by 3.62 ms (1.40%) for oak and 54.69 ms (15.55%) for spruce. Cold specimen time is 275.96 → 278.00 ms for oak (a 2.04 ms regression), and 367.01 → 315.81 ms for spruce. Engine initialization is separate: oak 387.14 → 320.81 ms; spruce 180.80 → 190.51 ms. These small sample counts do not establish tail latency or statistical attribution of small differences. Full stages, bounds, counts, cold/warm samples and initialization are retained in the JSONL; summary values are in `readback-summary.json`. `readback-measure.py` records wait4 peak RSS including initialization, generation and teardown.

## Explicit memory accounting

Oak has 715065 leaves: final requested leaf storage is 8,580,780 bytes. Spruce has 7353754 leaves: 88,245,048 bytes. New readback requests that final storage once plus at most 4,194,300 staging bytes: 12,775,080 and 92,439,348 bytes respectively. Allocator rounding/metadata are excluded. The source GPU buffer remains live through readback in both implementations.

Previously the full staging allocation coexisted with a full byte vector; staging was dropped before that byte vector was decoded into the final leaf vector. Thus the old readback peak has two full result-sized copies, not three simultaneously: 17,161,560 bytes for oak and 176,490,096 for spruce, excluding the source and other domains. The new explicit transient reduction is 4,386,480 and 84,050,748 bytes. Mapped staging residency and driver allocations cannot be equated with ordinary heap memory. Whole-process RSS falls only 2160 and 2408 KiB in this run; the larger compute/wood/allocator/driver lifetimes still govern the process peak. Device-local VRAM, browser memory and phone measurements remain unmeasured here. This result does not qualify parent R5's whole-path memory or latency requirements.

## Correctness and gates

Separate untimed verification processes run two regenerated samples per species/revision. Packed leaf word hashes agree within and across implementations: oak `79cca656bd60f673`, spruce `7ee559575850f9b2`. Counts also match. No shader, packing, placement, mass, wood or renderer-delivery algorithm changed.

Baseline: green, seven isolated native generation tests, renderer Wasm check and scoped formatting. The initial new test build failed because the helper API did not yet exist (`readback-red.log`); this is an API compile-red, not a behavioral regression reproduction. The final ten-test isolated generation run passed, including existing geometry, spatial contacts, repeatability and ownership tests. New tests check known words for zero, one, exact chunk, multiple exact chunks and a tail; insufficient source size/usage/chunk limits, destroyed source, allocation overflow and destroyed device return errors. Zero uses a destroyed source and zero chunk limit to verify the early return. Browser runtime cancellation was not exercised; ownership prevents a dropped helper future from returning partial data.

Native library tests and benchmark compile, `cargo check -p telperion-render --target wasm32-unknown-unknown`, scoped rustfmt and diff whitespace checks pass. No workspace gate, screenshots or unrelated LTO build was run. Task .1 and the parent spec remain open. Host review found no blocker before completion. The build queue friction entry records one under-one-minute Cargo-lock wait.
