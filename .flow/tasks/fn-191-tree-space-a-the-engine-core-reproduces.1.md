---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-191-tree-space-a-the-engine-core-reproduces.1 Implement Tree space A: the engine core reproduces GreenLab

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
New crate `crates/telperion-space` is the tree space's engine core (R1): buds with a physiological age on a reference-axis automaton, zoned growth units with position-set lateral PA, viability and shedding, minimal geometry, and named refusals. It reproduces GreenLab: closed-form counts exactly on three sets (R2), and Letort's two simulators run unchanged in headless Chromium, exactly when deterministic and in distribution over 4,000 seeds when stochastic, worst 3.81 of 4.5 standard errors (R3). L-Py is chosen as the 3D reference, with Pałubicki 2009's figures beside it (R4, REFERENCE.md; the host to confirm). The side-by-side sheet is at raw/sheet/sheet.png, and timing runs about 50 ns per phytomer (R5, RESULT.md); the host has not yet viewed the sheet.

Tier: session (jev long_running 0.33)
baseline: none (new crate; no Quick commands)
gate: cargo test --profile ci --workspace --no-fail-fast green (990 passed)

stage: impl-review - ran (codex fan-out, three draws SHIP; three P2 findings fixed in ad640d7d)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 1865557e808e0a4da18e83decd209ebdab68f122, ad640d7d46552c08ba6a4e4885c39c0db6aa8500
- Tests: cargo test --profile ci -p telperion-space, cargo test --profile ci --workspace --no-fail-fast
- PRs: