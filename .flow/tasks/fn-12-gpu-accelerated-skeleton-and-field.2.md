---
satisfies: [R1, R2, R3]
---
# fn-12-gpu-accelerated-skeleton-and-field.2 Measure a WebGPU indexed field candidate against the CPU reference

## Description
Evaluate the bounded candidate from task 1 against R1–R3; no automatic adoption.

**Size:** M
**Files:** `scripts/benchmarks/generation.mjs`, `scripts/benchmarks/generation-gpu.mjs`, `tests/browser/generation-gpu.mjs`, `.flow/evidence/fn12/`
**Touches:** [scripts/benchmarks/generation*, tests/browser/generation*, .flow/evidence/fn12/**]

### Approach
Use raw WebGPU independent of Three; portable snapshot supplies indexed tapered wood and foliage AABBs. Follow W3C buffer mapping and webgpu-samples timestampQuery patterns, bounds-check dispatches and adapter limits, account upload and copied result readback. Run paired indexed CPU/GPU batches with same inputs/browser. Evaluate the predeclared 20% threshold, tails and precision without changing the denominator. Preserve failure receipts, including optional timestamps unavailable. Do not call approximation exact; any correction costs belong in total time. Record field-construction/skeleton candidate screening rather than implying their CPU stages ran on GPU.

### Quick commands
`node --check scripts/benchmarks/generation-gpu.mjs`; browser candidate tests and paired benchmark under an exclusive GPU lease.

## Acceptance
- [ ] Both field occupancy domains execute on the GPU for ordinary/giant grids and boundary cases, or a reproducible capability/resource failure explicitly explains inability; same-browser CPU reference and repeatability checked.
- [ ] Raw cold/resident/end-to-end timings, actual output parity, all source/staging/peak buffer bytes and adapter/browser metadata recorded; preparation/CPU prerequisites never hidden.
- [ ] Unavailable adapter/timestamps, bad inputs, exceeded limits, shader/device/readback failures and disposal produce explicit complete-result/failure outcomes, with cleanup and reusable testable failure paths.
- [ ] Evidence classifies each candidate/workload qualified, rejected or inconclusive against fixed gates, including growth/construction investigation findings.


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
