---
satisfies: [R1, R2, R3]
---
# fn-25-a-scenic-cut-of-the-walk-for-x.1 The eased walk, the camera, and the graded clip

## Description
Build the scenic walk on the headless target and the assembly script, prove them at proof size, then render the one full-size clip for the owner (R1, R2; R3 is the owner's).

## Acceptance
- `--walk`, `--hold`, `--sweep`; eased blend and camera between both hero poses; leaf view walks too; invalid values refused naming the flag
- `--frames` path unchanged, asserted by a test on the frame schedule
- `scripts/scenic-cut.mjs` writes a 1920x1080 H.264 yuv420p 24 fps clip under 24 s plus `scenic.json`

## Done summary
`walk_pose` in `camera.rs` eases between both ends' hero poses; schedule in `examples/headless/walk.rs`, asserted by the new `tests/walk.rs` (example-target tests never run under cargo test). `--frames` keeps its own branch, pinned by a test. Full cut: 384 whole-tree + 120 leaf frames, h264 1920x1080, 24 fps, 20.5 s, 52 MB; the owner accepted it on 2026-09-10.

## Evidence
- Commits: e64e7b23daea90b7640cb2d780be8f795a744675, a0a35bacebc0eff1781f04783e963f5cbfa9c751, 6b2e8ad7b29ad17a12da8583e8316e82ce74cf92
- Tests: cargo test --release -p telperion-render (55 tests, green), cargo fmt --check -p telperion-render, cargo clippy --release -p telperion-render --all-targets, ...
- PRs:
