---
satisfies: [R2]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.7 A cut predictor that costs what the cut reads

## Description
Host decision 9 (2026-10-07), after task 6's decision 7: `grow` through the partition is 2x faster on all cores but slower on one core (beech 80: 1.03 to 1.48 s; spruce 80: 6.4 to 7.9 s), because the cut's predictor builds the whole closed-form table (0.7-1.2 s on one core) while the cut reads only each bud's expected kept phytomer count. The browser runs on one core.

Build a dedicated predictor that computes only that count, by its own recursion, iteratively (removing the deep-stack workaround `cores::deep` where only the predictor needed it). The classification it yields must equal today's predictor's for every bud (same cut decisions), so every tree stays identical; prove it with the lineage digest and `measures hash 80` (all species, seeds 1 and 7, neutral and lit). Time `grow` on 1 and all cores, ages 40 and 80, before and after; the bar is one core no slower than `grow_whole`.

## Acceptance
- Cut decisions identical to today's predictor for every bud; every tree identical (lineage digest and measures hash). - Iterative; no deep stack needed for the predictor. - grow on one core no slower than grow_whole at ages 40 and 80 for beech and spruce; all cores no slower than now. - Workspace gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
