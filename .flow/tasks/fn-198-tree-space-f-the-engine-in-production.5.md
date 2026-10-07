---
satisfies: [R2]
---
# fn-198-tree-space-f-the-engine-in-production.5 One core as fast as before, and one order

## Description
Host decisions after task 4 (2026-10-07): (16) stage 2 stays on the CPU partition (option d); fine-root growth on the GPU waits for device-side readers (option c, a spec for the owner). (17) Stage 2 adopts the engine's canonical lineage order (FINE-CONTRACT decision C) and drops its own re-sort (0.14 s at spruce 80); converted bytes change once, recorded.

The task: after merging F1, one core is 9-41 % slower on every tree, and the browser runs on one core, so the harness got slower (beech 40 to 80: 5.7 to 6.1 s; oak: 17.5 to 21.9 s). Profile `grow` on one core (natively, `taskset -c 0`, and in the Wasm build) before and after the merge (base 4ef2aebe's engine against now), stage by stage and with the integer arithmetic's share (Q32 presences, fixed-point thresholds and sums, the canonical sort, the predictor, the partition's handover); name the cause with numbers; then fix it without moving a bit (lineage digest and `measures hash 80` unchanged). Then decision 17, and the harness timings again with the task 3/4 driver.

## Acceptance
- The one-core regression's cause named with numbers. - One core no slower than before the F1 merge for beech, spruce, oak, palm at 40 and 80 (natively and in Wasm), every tree unchanged. - Stage 2's re-sort removed; bytes change recorded. - Harness timings after; npm test and the workspace gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
