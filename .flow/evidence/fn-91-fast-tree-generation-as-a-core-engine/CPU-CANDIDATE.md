# Shared CPU candidate

The exact-bounds broad phase and station reuse improve all four native fixtures while preserving their output fingerprints. This is an incremental improvement, not the accepted 10x result. GPU construction is not implemented.

The baseline is in BASELINE.md. The candidate runs the same mature public stages, one first build plus five warm samples, release Rust on the same shared desktop. Bounds-only attribution used seed 1 for both species; the final candidate adds shared sine/cosine evaluation and segment-frame preparation together. No parallel compiler or other task benchmark ran during measurements.

| Native case | Baseline warm median | Candidate warm median (max) | Speedup | 10x target | 100 ms stretch distance |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak / 1 | 942.79 ms | 639.49 (644.11) ms | 1.47x | 94.28 ms | 539.49 ms over |
| oregon-white-oak / 7 | 1136.90 ms | 756.77 (758.33) ms | 1.50x | 113.69 ms | 656.77 ms over |
| norway-spruce / 1 | 5581.80 ms | 4657.69 (4728.32) ms | 1.20x | 558.18 ms | 4557.69 ms over |
| norway-spruce / 7 | 5338.82 ms | 4489.90 (4503.43) ms | 1.19x | 533.88 ms | 4389.90 ms over |

Bounds-only seed 1: oak bounds 316.40 → 23.08 ms, total 942.79 → 661.65 ms; spruce bounds 943.29 → 212.67 ms, total 5581.80 → 4838.15 ms. The station pair then reduces placement from 217.96 → 203.45 ms in oak and 4024.71 → 3844.62 ms in spruce. These two station edits were measured together; no isolated sine/cosine speedup is claimed.

| Final native seed 1 | Growth | Wood | Placement + contact | Culling | Bounds |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak | 61.31 ms (9.6%) | 251.09 ms (39.3%) | 203.45 ms (31.8%) | 99.62 ms (15.6%) | 23.28 ms (3.6%) |
| norway-spruce | 38.64 ms (0.8%) | 202.02 ms (4.3%) | 3844.62 ms (82.5%) | 359.57 ms (7.7%) | 211.32 ms (4.5%) |

## Exactness and memory

For a fixed finite matrix coefficient, multiplication is monotone after selecting the correct endpoint by its sign. Each row adds those products in the same left-associated order as transform_point, so the resulting local-AABB image encloses every transformed vertex. Only strict interior containment permits skipping; boundary cases retain the original vertex traversal and min/max tie behavior, including signed zero. Nonfinite or f32-overflowing enclosures always fall back to the original checked traversal.

Three bounds tests compare exact extrema bits with an independent full traversal across curved thin/wide elements, packed rotations and negative scales, reversed leaf order, empty/signed-zero cases, malformed scales and extreme coordinates. The enclosure test also uses arbitrary signed matrix coefficients and randomized vertices across powers of two from -120 to 120. A station test compares precomputed frames to the former per-station arithmetic across repeated, tiny and reversing segments.

Native FNV-1a 64-bit fingerprints match the original implementation in every sample of all four candidate cases, including the separate bounds-only cases. The fingerprint covers wood positions/indices, retained packed leaves, element positions/indices and exact foliage bounds. It is a reproducibility fingerprint, not a cryptographic proof; it excludes unchanged wood attributes and element metadata. Its computation is outside generation timing. Baseline binaries were built before either production change, and their fingerprints are retained in cpu-reference files.

The new bounds values use constant stack storage; no per-leaf allocation or persistent buffer is added. Segment preparation mutates the existing frame vector. This establishes unchanged heap allocation shape for these edits, not whole-process peak memory. Renderer/GPU/native process peaks and phone hardware remain named gaps.

No visual regression verdict is claimed. The exact-reference checks support unchanged native geometry; owner visual acceptance and completed-frame rendering qualification remain separate spec requirements.

## Browser comparison

Same desktop, viewport, species/seed order and submission-only protocol as the baseline. One failed zero-sample navigation attempt is retained as inconclusive. The retry added only a navigation-origin check outside timing; it completed all 48 builds. No GPU completion fence or owner visual verdict was added.

| Case | Baseline setTree warm median | Candidate setTree warm median (max) | Speedup | Candidate owned CPU build warm median | CPU Wasm capacity bytes |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak / 1 | 1428.7 ms | 901.4 (909.1) ms | 1.58x | 850.4 ms | 261816320 (unchanged) |
| oregon-white-oak / 7 | 1680.1 ms | 1025.6 (1042.4) ms | 1.64x | 1042.5 ms | 328990720 (unchanged) |
| norway-spruce / 1 | 7335.6 ms | 5980.0 (6018.7) ms | 1.23x | 5862.2 ms | 472973312 (unchanged) |
| norway-spruce / 7 | 7011.8 ms | 5686.4 (5733.8) ms | 1.23x | 5617.9 ms | 455016448 (unchanged) |

All browser submitted counts/bounds and owned CPU counts/bounds match baseline exactly. Browser output-array byte fingerprints were not recorded in the initial baseline, so native fingerprint equality is not presented as a browser byte-comparison result. The CPU Wasm linear-memory capacity is unchanged in every case; it does not account for the separate renderer Wasm, GPU or process peak. Raw cold/first-build observations remain in the JSON under the original limited initialization definition; they do not qualify cold page delivery.

## Gate correction and next measurement boundary

The first canonical workspace run stopped on generation_limit_guard: the new segment-frame iterator was missing its inventory classification. The host reviewed it as algorithmic because N polyline points have N−1 segments. The exact lexical site is now declared in docs/generation-limits-inventory.json; the guard and all its assertions are unchanged. The focused guard passes. The first failure log is retained separately from the final run.

The host added an opt-in GENERATION_COMPLETED=1 mode to the benchmark runner while the workspace suite ran. It captures the renderer's actual GPU device, frames the tree and waits for submitted work to complete. This mode has syntax validation only at this checkpoint; none of the tables above use it. The host will measure the old and new implementations under that matched completed-frame protocol separately.

stage: impl-review - skipped(config: REVIEW_MODE=none)

This is worker invocation 2, with two baseline commits previously made and one candidate checkpoint in this invocation. No subagents were dispatched. The task remains in progress; this candidate does not meet R5's 10x target, and the outstanding R1/R3/memory gaps remain explicit.


## Terminal verification: renderer validation blocked

The corrected canonical workspace run passed the core and Jev suites, then crashed in the renderer's bark_plates test process with SIGSEGV. The exact default-parallel binary replay also crashed. The core dump locates a null call during Vulkan extension enumeration inside libvulkan; another test thread is simultaneously creating a Vulkan instance through NVIDIA GLX. Concurrent initialization is a plausible cause, not a confirmed root cause. No generator function is on the crashing stack. Memory inspection found 21 GiB available and no OOM event in the inspected kernel-journal window.

The host selected the existing nextest isolated-process route as a fallback, with a stop condition if the runner was unavailable. cargo-nextest is not installed locally. No installation, third replay, driver change or gate assertion change was attempted. The host directed a coherent checkpoint and will assess the missing runner separately. The original workspace gate remains failed; remaining renderer/Wasm tests and doctests are not claimed passed.

Passed checks: 19 focused foliage unit tests, three bounds property/edge-case tests, two generation-limit guard tests, all core and Jev tests reached in the corrected workspace run, native example and both Wasm builds, Rust formatting, JavaScript syntax, all native fingerprint comparisons, and the browser comparison. The initial inventory failure and subsequent renderer crash logs remain distinct.

NEEDS_HUMAN

BLOCKED: TOOLING_FAILURE
Task: fn-91-fast-tree-generation-as-a-core-engine.1
Summary: Native renderer validation crashes during Vulkan initialization, and the selected isolated-process runner is unavailable.
Impact: Full renderer validation and the next GPU experiment await the host's tooling decision; the 10x objective is not complete.
Suggested resolution: Establish the existing pinned nextest runner or another host-approved stable renderer-test environment, then run remaining renderer/Wasm coverage without rerunning passed core suites.

Task status stays in_progress. No review verdict or flowctl done was issued.
