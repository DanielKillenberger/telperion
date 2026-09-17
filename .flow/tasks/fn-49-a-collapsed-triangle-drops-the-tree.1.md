---
satisfies: [R1, R2, R3, R4]
---
# fn-49-a-collapsed-triangle-drops-the-tree.1 Implement A collapsed triangle drops, the tree still builds

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
A triangle whose float32 corners span no area is dropped from its run and counted (bound: four times the ring segments, 80 on the beech and birch), vertices left without a triangle take their ring's outward normal, a position float32 cannot hold is still an error, and the tree builds.

The cause, reproduced on a test-only beech at seed 266: the planner's resampling of a shortened limb pairs stations micrometres apart (11.39 um there), and at one segment the frame's tilt cancels the step so two vertices round to one float32 point. The resample rule was left unchanged, because any merge tolerance that caught these pairs would move shipped oaks that build today.

Previously failing trees drop 2 triangles each. 716 cases across every preset hash byte-identical; the identity pins and the 48-case protocol hold; species_measure records counts.wood_dropped. Host reviewed the range a0df57f..a3e56f2.
## Evidence
- Commits: b8e7ceb, a3e56f2
- Tests: cargo test --release -p telperion-core, cargo test --release -p telperion-render, cargo clippy --release -p telperion-core -p telperion-render --all-targets -- -D warnings, npm run typecheck, npm run rust:test:wasm, node tests/species.mjs --measure-only (48 cases)
- PRs: