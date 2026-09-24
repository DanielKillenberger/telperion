---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-102-one-build-pipeline-in-place-of-the.1 Implement One build pipeline in place of the copied chains

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
`telperion_core::pipeline` now runs the mature build. It names the stages and runs each one once, only where a requested output needs it. `mesh::build`, `mesh::assemble` and the Wasm binding call it, so the binding's copy of the recipe is gone (R1). The chain-equality test is deleted and nothing replaces it (R2).

Where wood and leaves are both wanted on a native target, they run concurrently. Results join in stage order and the earliest failing stage's error is returned (R8). A family whose leaves sit on the wood gets one ring sweep per request (R7). When wood and leaves run side by side, the sweep runs first and the wood reads it through `surface::build_swept`. When they run one after the other, the leaves read the rings from the wood's own vertices through `AttachmentSurface::of_wood`, which keeps base's Wasm allocation order.

Evidence is in `.flow/evidence/fn-102-one-build-pipeline-in-place-of-the/RESULTS.md`. Byte identity (R3) holds for every preset, both seeds and every binding output combination. The date palm's binding now equals the mesh build, the permitted exception. Peak Wasm memory is unchanged, apart from the palm's +0.4 MB from the fix.

R4 is met under the owner's ruling of 2026-09-23, written into the spec: the whole build must be no slower, and a concurrent stage's own wall time may rise. Whole builds are faster (ordinary 136 to 123 ms, spruce 4,910 to 4,889 ms, median of five at seed 1). The concurrent wood stage rose +25% and +24%; forced serial, every stage is within ±5% of base. The machine was shared during measurement and moved untouched code by 5 to 40%, so the spruce's -0.4% is within noise. The concurrent slowdown is deferred as review item J.

Tests: `pipeline::tests::one_sweep_and_one_element_serve_a_request` (R7), `every_family_builds_the_same_bytes_under_either_schedule` and `the_earliest_failing_stage_answers` (R8). Gate `cargo test --profile ci --workspace --no-fail-fast`: 927 passed. `npm test`: 124 passed. `npm run rust:test:wasm` passed.

Follow-up worth a spec: the native family wire encodes `maxInstances` as a 64-bit `usize::MAX`, which the wasm32 binding refuses (FRICTION.md).

stage: impl-review - ran [codex, one draw, single pass per owner scope]: NEEDS_WORK, 1 finding (R8 error precedence under concurrency), fixed in 7cfa4676 with a red-to-green test; no re-review, per the owner's one-pass instruction
Tier: session (jev intelligent 0.36)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 49f004e26f6bc7589351ddd5d824d8d35372c9f7, 72577e73d6fdfb208b4fb341c3e72e8567343e05, 7cfa4676ae0f5c35727e70fb9509e0639e548fbc, c1b4988904e45d1113a3be136d13276b05db9cda
- Tests: cargo test --profile ci --workspace --no-fail-fast (927 passed, 0 failed), cargo test --profile ci -p telperion-core --lib pipeline, npm test (vitest 124 passed), npm run rust:test:wasm, R3: tools/meshhash.rs base vs candidate, 16/16 identical, R3: tools/binding.mjs 8 presets x 2 seeds x 15 combos; 7 families identical, date palm changed per exception, R4: tools/r4native.sh, r4serial.sh, r4wasm.sh (see RESULTS.md)
- PRs: