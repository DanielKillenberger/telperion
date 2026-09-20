# Admission traversal experiment

The host retained the small, allocation-free traversal change: four fixtures improve preparation by 9.69–11.97%, and native seed-1 completed-frame delivery improves by 3.51% for oak and 4.15% for spruce. Both completed-frame results miss the 5% aim; oak 1 and spruce 1 narrowly miss the 10% preparation aim. This hypothesis stops here. The host explicitly stopped expansion to browser measurements because the native gain missed 5%; no browser speedup is claimed. Task .1 and the parent 10×, 100 ms, cold-start, phone and whole-peak-memory requirements remain open.

## Preparation

The control comes from revision `852fdbd1`, using the release profile, LTO and one codegen unit. The candidate changes only sequential admission traversal. Both ran on the existing Ryzen 9 5950X / Linux desktop. `preparation.rs` solves each mature oak/spruce seed once, times one first preparation and five warm preparations with a whole-call timer, then calculates checksums outside timing. Builds and measurements run in separate windows. `run-preparation.sh` reproduces the source selection and protocol; build logs and SHA-256 files preserve executable provenance. Both executables are saved under `/tmp/telperion-fn91-tools/`. Background OS activity remains uncontrolled; these small sample sets do not establish tail latency or statistical confidence.

| Fixture | Control → candidate warm preparation median ms | Improvement |
|---|---:|---:|
| Oak 1 | 129.768 → 117.193 | 9.69% |
| Oak 7 | 145.301 → 130.116 | 10.45% |
| Spruce 1 | 96.460 → 87.089 | 9.72% |
| Spruce 7 | 93.389 → 82.207 | 11.97% |

`control.jsonl` and `candidate.jsonl` contain all first/warm samples and fixture parameters. Every fixture's 12 prepared-field Debug-text FNV-1a checksums agree across control/candidate and repeats. This checksum includes all fields (positions, ring metrics, angles, run metadata/table, bounds, segments, index count and fallback); capacities are checked separately. All four full CPU surface hashes equal task .4: `b82b2cf44372d4bf`, `d2b8c64f7c86ebeb`, `6fdd44ed9ad69f53`, `8471722f3923dd42`, in table order. Capacities agree exactly at 48,145,532 / 55,221,180 / 39,656,536 / 37,540,048 bytes.

## Native delivery and memory

`measure-native.py` compares the saved .4 final executable with this candidate, first plus three warm native GPU-resident generations at seed 1, including the 1280×720 hero frame and actual device completion on the NVIDIA RTX 3080. The JSONL retains initialization, first results, all stages and counters; `native-summary.json` records warm medians. Oak improves 227.834 → 219.845 ms (3.51%); spruce improves 262.196 → 251.315 ms (4.15%). These remain 119.845 and 151.315 ms above the 100 ms stretch goal.

Process peak RSS changes 308,636 → 350,812 KiB for oak and 406,888 → 387,588 KiB for spruce. Oak's increase is explicitly unresolved; process RSS includes initialization, driver/compiler and allocator variation and does not qualify no-peak-rise. The traversal allocates nothing and prepared capacities remain exact. The parent memory requirement remains unqualified. The prior default-CPU oak timing attribution also remains unresolved; this experiment checks CPU output hashes but does not repeat CPU timing or claim that regression resolved.

## Correctness and gates

The helper follows ordinary CPU emission's nested strip and interleaved-cap order. It visits every triangle even after an admission rejection and propagates errors unchanged; f64 admission, radius recovery, canonical positions, metadata, GPU shaders and the default CPU branch are unchanged. The host directly reviewed the diff and requested no changes. Exact prepared output and unchanged GPU code reuse task .4's accepted visual evidence; no screenshots or repeated owner approval were requested.

The new unit test compares every emitted triangle to the old procedural enumeration for minimum triangles, different segment counts, nonzero bases and a 1,025-ring run, including caps. It first failed for the missing helper. Baseline: 17 focused integration tests and scoped edition-2021 formatting passed. Final checks include that same integration suite, the surface unit tests (including the new enumeration test), scoped formatting, diff whitespace and Wasm compile check. `final-tests.log` is the 15-test surface-name-filtered unit selection; it does not stand in for the 17-test integration suite in `final-integration-tests.log`. No full workspace/LTO gate or browser run is claimed.

Tier: session (jev intelligent 0.41; explicit IMPLEMENTER preserved)
stage: impl-review - skipped(config: REVIEW_MODE=none)
