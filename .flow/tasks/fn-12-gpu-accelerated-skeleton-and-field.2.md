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
Implemented a benchmark-only indexed WGSL candidate for both wood and foliage, with paired full-preset measurements, complete failure outcomes, deterministic disposal, limit/input checks and timestamp/readback accounting. The authoritative direct cold stopwatch includes fresh snapshot extraction through device disposal; every cold workload and two-query amortization estimate rejects adoption. Ordinary 64³ differs in two foliage cells (one false negative); Ordinary/Telperion contact matrices differ in 65/54 cells. Every differing cell and CPU/GPU flags is preserved, and repeats reproduce those differences exactly.

The final Telperion 64³ source-resident submeasurement passes local timing/grid-parity gates (25.4 ms CPU versus 11.8 ms GPU medians), while earlier runs varied and the general uncorrected field contract fails contact precision. No production API or automatic backend was added. GPU growth/construction performance remains explicitly inconclusive: their CPU prerequisites were measured and dependency screening was code analysis, not GPU execution.

R1/R2 evidence: `.flow/evidence/fn12/gpu-results.json`, `gpu-input-verification.json`, preserved preliminary/internal-timing runs, and `scripts/benchmarks/generation-gpu.md`. R3 tests: `tests/browser/generation-gpu.mjs` plus `gpu-tests.json` distinguish real hardware execution/device.destroy loss from controlled shader/device/allocation/submission/readback/traversal/timeout/capability failures; they verify whole-request errors, cleanup counts and reuse/disposal behavior.

Baseline: green (five Rust field tests and typecheck). The new helper reproduction failed first for the absent module. Final Rust field tests (five), typecheck, syntax, helper/failure tests and actual GPU checks passed. All twelve direct-lifecycle benchmark observations completed without caps; canonical parameter/source/input/CPU hashes match task 1. Source and staging/mapped buffers, logical memory bounds, Wasm high-water and opaque driver allowances are recorded separately.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 5659b41301b6e415b28827894c172c3dec3156ed
- Tests: baseline: green (pre-edit Rust field suite: 5 passed; npm run typecheck: passed), red reproduction: node tests/browser/generation-gpu.mjs failed with ERR_MODULE_NOT_FOUND before candidate implementation (/tmp/fn12-task2-red.log), /home/daniel/.cargo/bin/cargo test --release -p telperion-core --test field: 5 passed (/tmp/fn12-task2-verify-rust.log), npm run typecheck: passed (/tmp/fn12-task2-verify-ts.log), node --check scripts/benchmarks/generation-gpu.mjs: passed, node --check scripts/benchmarks/generation-gpu-run.mjs: passed, node tests/browser/generation-gpu.mjs: helper and controlled failure tests passed (/tmp/fn12-gpu-helper-final.log), PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs timeout 600 node tests/browser/generation-gpu.mjs --hardware: passed (.flow/evidence/fn12/gpu-tests.json), PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs GENERATION_OUTPUT=/tmp/fn12-gpu-lifecycle timeout 600 node scripts/benchmarks/generation-gpu-run.mjs: exit 0, 12 complete repeated full-preset observations; all mismatches preserved; cold rejected, giant64 resident submeasurement positive (.flow/evidence/fn12/gpu-results.json), Independent canonical source/input/CPU flag SHA256 comparison with task1: passed (.flow/evidence/fn12/gpu-input-verification.json), git diff --check: passed
- PRs: