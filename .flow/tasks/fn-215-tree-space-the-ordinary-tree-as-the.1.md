---
satisfies: [R1, R2, R3, R4]
---
# fn-215-tree-space-the-ordinary-tree-as-the.1 Implement the ordinary tree as the space's centroid

## Description
TBD

## Acceptance
Every R-ID in the parent spec's requirements is satisfied; judge this task against the spec's criteria directly. R1 is as amended: the corners' centroid, sized to their mean. R3 is as amended: the walks pass at age 40, and the age-80 walks are deferred to fn-230, which owns them (host, 2026-10-08).
## Done summary
The ordinary tree is the equal-weight centroid of the European beech, the Oregon white oak and the Norway spruce, sized to their mean (spec R1, as amended by the host, 2026-10-08). `examples/centroid.rs` derives the rows from the corner presets. Every age's lateral probabilities are multiplied by one factor, 0.742, which makes the tree's light-neutral expected kept phytomers at 80 the corners' geometric mean, 3,559,562. A rerun of the tool reproduces the preset byte for byte. `space_presets` tests the centroid, the factor and the size.

The tree builds natively and in Wasm at 40 and 80. At 80 it keeps 1.84M to 3.31M phytomers and grows at most 15M, under the 20M budget. It grows in 1.2 to 1.6 s at 40 and 22 to 43 s at 80. The TypeScript field test passes in 41.4 s against its unchanged 60 s limit.

The sheet covers ages 40 and 80, seeds 1, 7 and 42, bare and in leaf, beside the corners' sheets, all viewed (SHEET.md). The walks pass at 40. The age-80 walks are deferred to fn-230, which owns them (spec R3 amended).

Open for the owner: the build time at 80 and the default dressing. Recorded for the host: the early refusal (`foreseen`), not extended. FRICTION.md has three entries.

stage: impl-review - ran (codex fan-out NEEDS_WORK on R3 at 80; re-review SHIP after the host deferred it to fn-230)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: fc3a22c2a8e6df93b049d7f32e52925fdd3e3e30, e25bd047b0cf4b956b80a795aea9c6fd2c19b433, f4310ac83d2f1ebf4df07e2ca67d7594706c8b83, 7a6096039ea8e588d04c2a9e8358d7bd508f2d41, 4edc4a49ac2e187a1210422b412b6b6fd0fd5df8, a5d21fcbab7e1a7b92de82a42909d2a42f59c1c6, ef33eec44186f60d8de4eac0616280b9c06d0148
- Tests: cargo test --profile ci --workspace --no-fail-fast (per package; render first under the GPU lock), cargo test --profile ci -p telperion-core --test space_presets, npx vitest run src/field/field.test.ts harness/parity.test.ts
- PRs: