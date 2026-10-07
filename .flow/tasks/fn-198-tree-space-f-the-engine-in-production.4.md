---
satisfies: [R1, R2]
---
# fn-198-tree-space-f-the-engine-in-production.4 Fine roots on the GPU in the pipeline and the harness

## Description
INTEGRATION.md decision 2's door (ii), after fn-209 task 8 (owner: "super high performing generator with the new growth algo integrated into the harness").

1. Merge branch fn-209-tree-space-f1-fine-orders-grown-in-the (F1 through task 8: `grow` through the partition with the count-only predictor, canonical lineage order, the `settle` driver with CPU and GPU executors for fine roots under realized mechanics, the GPU buffer pool) into this branch. Every engine tree through the pipeline identical by lineage to F1's branch; tests green.
2. Stage 2 grows through F1's `settle` driver: the GPU executor where the pipeline has a device (native and the browser's WebGPU), the CPU executor otherwise; the same tree either way, by lineage, geometry within the screen. The oak stays on its current path until fn-211's stepped GPU executor is merged (it is being built now).
3. Host decision 15: the browser's memory refusal happens before growth, from F1's count-only predictor (expected kept phytomers against the 4,000,000 ceiling measured in task 3), so an over-ceiling tree is refused in milliseconds.
4. Time in the harness, as task 3 did (fresh pages, the same driver): seed change at 40 and age 40 to 80 for beech, spruce, oak and palm, before and after; native `Generator::prepare` timing beside it. Screenshots of the four species at 40 and 80 where they fit, viewed.

## Acceptance
- F1 merged; trees identical by lineage to F1's branch. - Stage 2 through settle with the GPU executor natively and in the browser; CPU and GPU trees identical by lineage, geometry within the screen. - Over-ceiling trees refused before growth. - Harness and native timings before and after; screenshots viewed. - npm test and the workspace gate green.

## Done summary
F1 merged (8a1eafc1): every engine tree identical to F1's branch (32 digests). Decision 15 built (f4c2ad12): expected_kept refuses an over-ceiling tree before growth (spruce 80 refused in 322 ms instead of 8.3 s); light-reading species still refused after growth (the predictor ignores light). Native growth on all cores about 2x faster (spruce 80 2.1 to 1.1-1.2 s, beech 80 400 to 190 ms). Screenshots of the four species at 40 and 80 (spruce 70) viewed. Gate and npm test green.

Acceptance amended by the host (2026-10-07, decision 16): stage 2 through the GPU executor is withdrawn from this task; nothing after stage 2 reads fine roots from the device, readback is excluded by fn-209 decision 5, and the device-side readers are a spec for the owner (option c). The one-core regression found here (9-41 %) is task 5.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8a1eafc1, f4c2ad12, fab45ab8, e905dcdd, da65120e
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: