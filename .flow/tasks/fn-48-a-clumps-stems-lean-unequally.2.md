---
satisfies: [R1, R2, R3, R4]
---
# fn-48-a-clumps-stems-lean-unequally.2 The second stem parts at a stated height

## Description
Owner, round 13 (2026-09-15): "Can we also have the second trunk be moved up/down?" and "if we can add it with a short task to the existing spec we can add it. If it's nothing major." Add `skeleton.habit.stem_fork_height` (0 to 0.5 of crown base height, neutral 0): later stems are born from the first stem at that height.

## Acceptance
- [x] R1 row railed, wire, blend, harness, metadata; presets byte-identical at 0.
- [x] R2 later stems leave the first at that height as trunk runs with a fork socket; girth below is the pipe sum; DBH counts stems at breast height.
- [x] R3 birch fork height set, pairs rendered, visual QA.
- [x] R4 tests per the spec.

## Done summary
The row works, but the fork showed a ledge (the trunk run followed the wider leaning stem). The fix needed the stem-or-limb fact in the specimen's shoot-less reads, past this task's surfaces. Host decision (2026-09-15): not "nothing major"; a3a72a13 reverted the birch's fork height to 0 and the fix became task 3. b42f2ea9 also has `Tree::stem_radius` measure stems at the fork.

## Evidence
- Commits: 744f408c (branch: e06083da, b42f2ea9, 682077f0, 4ad41eda, a3a72a13)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
