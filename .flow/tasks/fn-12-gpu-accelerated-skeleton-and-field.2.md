---
satisfies: [R1, R2, R3]
---
# fn-12-gpu-accelerated-skeleton-and-field.2 Measure a WebGPU indexed field candidate against the CPU reference

## Description
Measure a benchmark-only WebGPU indexed field-query candidate against the CPU reference; no automatic adoption.

## Acceptance
- Both occupancy domains run on the GPU for ordinary/giant grids and boundary cases, or fail explicitly; CPU reference and repeatability checked.
- Raw cold/resident/end-to-end timings, parity, failures and memory recorded.

## Done summary
Indexed WGSL candidate for wood and foliage with paired full-preset measurements, deterministic disposal and explicit failures. Every cold workload rejects adoption; Ordinary 64³ differs in 2 foliage cells, contact matrices in 65/54 cells, all repeatably. Telperion 64³ source-resident passes timing (25.4 vs 11.8 ms) but the general contract fails on precision.

## Evidence
- Commits: 5659b41301b6e415b28827894c172c3dec3156ed
- Tests: cargo test --release -p telperion-core --test field; npm run typecheck; node tests/browser/generation-gpu.mjs
