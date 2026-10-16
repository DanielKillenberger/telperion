---
satisfies: [R3, R5]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.7 The resident handoff and the 3x verdict

## Description
The contract's probe, last part: the submit path takes device buffers (no per-node readback); the timing harness from settled geometry to draw-ready resident artifacts, five builds a side alternately; the 3x gate at both spruce-80 seeds against baseline B (about 8.1 s; target about 2.7 s); final-output parity; a close-camera look at the seams (D4) for the host.

## Acceptance
- The 3x verdict with its profile.
- Final-output parity with the CPU reference.
- Seam close-ups viewed.
- Two-part gate green.

## Done summary
The resident handoff on the real path, and the 3x verdict.

- The pipeline converts only the structure above the cut and the landed roots, from the device settle (executor::settle, Settling::part, Part::expansion); the device draws its roots from Tree::attachments, packs its curve into the renderer's form after the CPU block (handoff.wgsl), and places its leaves at an offset in the one leaf buffer with one mass grid; submit_prepared takes the device curve. The probe's pruned stand-in is deleted.
- Host option 3: a bearer reads its fine roots through their attachments (kind counts each root, fork from the phase-2 base radius) on both paths.
- 3x on the real path: 7.7x (seed 1, load 7 to 9) and 7.1x (seed 7, loaded); the stations' serial `bases` pass is 248 of their 284 ms.
- Parity by named class at 30 and 80: every run matched, kinds and forks equal, leaves equal apart from task 6's rounding class; seams viewed at a close camera, device and CPU indistinguishable.
- Gate: render half 162 passed, CPU half 988 passed.

## Evidence
- Commits: 4f7411c3, 3aa10064, 7a022853, 496e4c46, 71b3d2ee, 0e583ab0
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render three_x -- --ignored, cargo test --profile ci -p telperion-render real_path_at_80 -- --ignored
- PRs: