---
satisfies: [R1, R2, R3, R4]
---
# fn-49-a-collapsed-triangle-drops-the-tree.1 Implement A collapsed triangle drops, the tree still builds

## Description
Reproduce the float32 triangle collapse, then drop a zero-area triangle from its run, count it, and give orphan vertices their ring normal.

## Acceptance
- [x] R1 cause reproduced and recorded.
- [x] R2 drop, index correction, ring-normal fallback, bound.
- [x] R3 byte identity across 716 cases; 48-case protocol.
- [x] R4 `tests/surface_collapse.rs`.

## Done summary
A triangle whose float32 corners span no area is dropped and counted (bound four times the ring segments, 80 on beech and birch); a position float32 cannot hold is still an error. Cause: the planner's resample of a shortened limb pairs stations micrometres apart (11.39 µm at test-only beech seed 266); the rule was left unchanged to keep shipped oaks identical. Previously failing trees drop 2 triangles each; 716 cases hash identical.

## Evidence
- Commits: b8e7ceb, a3e56f2
- Tests: cargo test --release -p telperion-core, cargo test --release -p telperion-render, clippy, npm run typecheck, npm run rust:test:wasm, node tests/species.mjs --measure-only (48 cases)
- PRs:
