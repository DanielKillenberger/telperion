---
satisfies: [R3, R4]
---
# fn-226-light-read-from-the-crowns-outline-per.3 Local light and local sharing

## Description
The spec's R3 and R4: first the R4 inventory and the written equations (evidence, before code), then the outline history recorded by the structure above the cut, local light and local sharing on the CPU, then the GPU executor inline in the growth step; beside today's model for the head-to-head only.
## Acceptance
- R4's inventory and equations written before code; missing expectations stopped for the host (done: R4-INVENTORY.md, host decisions S1-S8).
- R3 and R4 built per bud on the CPU, with no synchronisation added; the one-way history shown; pass count and sync points per cycle reported (R3-R4.md). The GPU half is out of scope (S8 (c)).
- Analytic tests of chords and transmission through a cone stack, and every bootstrap and degenerate case of R3.
- Measured for the head-to-head. The gate's one red walk is today's model's, carried into task 4 by the owner's verdict.
## Done summary
R3 and R4 built on the CPU (outline light, local sharing, one-way history), with analytic chord, transmission and degenerate-case tests and the executor's one-way test; measured head-to-head against today's model on the lit oak at 40 and 80, seeds 1, 7, 42 (R3-R4.md): 0 synchronisation points a cycle against 4, but a tree half as tall (8.2-11.9 m against 22.3-26.9 m at 80), a clear trunk of about 2 m against 4.7 m, 44-70 % slower at 40 and not faster per phytomer at 80; host decision 23's drift compounds along the stem.

Owner verdict (2026-10-08), on the host's recommendation to close with today's model: "ok". The code is deleted in task 4 (R6, today's model chosen).

stage: impl-review - ran (codex): round 1 NEEDS_WORK, four findings fixed; round 2 NEEDS_WORK on one P1 (the per-bud read as its own traversal).
OVERRIDDEN: the P1 concerns code the owner's verdict removes in task 4; owner: "ok".
Gate: render half green; workspace 1,039 passed, 1 failed (today's model's apical_control walk, deferred from task 2 and carried into task 4).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 923e927471c16ce9c758524e266d9ec60bc60cd5, 143ad512d8f2e4799fa02fd3f2f1e46fb6e37df8, dda49857aa11ea7a18ae9d85cbfeb81e9bf424cb, e03da4e55b576108e5238645483a69e604ddbee8, 7eb5aa0e6a39979f93332228d05e83fe5bf963a0
- Tests: cargo test --profile ci --workspace --no-fail-fast (two parts)
- PRs: