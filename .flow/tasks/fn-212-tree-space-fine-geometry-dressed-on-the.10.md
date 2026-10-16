---
satisfies: [R3, R4, R5a]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.10 The device path in the browser harness

## Description
The spec's R4 (browser) and R5a: the same resident path through WebGPU in the harness (setTreeGpu), async completion, allocation and binding limits, device errors, repeated large-to-small rebuilds; a measured pass criterion for the spruce at 80 in the browser; build latency (a parameter change to a drawn tree) and steady-frame cost reported apart, in the harness and natively, before and after; screenshots viewed. The harness drives npm run dev, never a full-forest capture.

## Acceptance
- The harness draws beech and spruce at 80 through the device path in the browser, within the CPU reference's tolerance.
- Spruce 80's browser pass criterion measured (memory, limits, completion); large-to-small rebuilds survive.
- Build latency and frame cost before/after, harness and native; screenshots viewed.
- Gate and npm test green.

## Done summary
The fine roots' device path is the production path for every resident tree whose fine roots come apart (beech, spruce), natively and in the browser. The oak and the palm keep the CPU executor; `executor::apart` names why.

- **`Roots` is async.** One driver serves both targets: natively it blocks, the browser awaits, and the CPU reference returns ready steps.
- **One `Owned` buffer type.** Every renderer buffer is destroyed after the next submit once it is let go.
- **Frames.** They are paced on GPU completion and keep their own error scopes.
- **Fine stations** are numbered by root.
- **The slender bound** is decided in whole micrometres.
- **The grower's regions** trim to each build's need.
- **A stated browser budget** of 8.5 GiB, calibrated on this card, refuses builds by name.

Spruce 80 seed 1, native: from a parameter change to the drawn tree, 9 to 11 s → 1.4 to 2.5 s, and peak RSS from 4.8 GB to 0.8 GB. Steady frame 50 against 47 ms; the gap goes to fn-176.

In the browser (headless Chromium):
- the spruce at 80 draws in 3.4 s; before, it was refused;
- beech 40 → 80 takes 1.4 s against 5.7 s;
- large-to-small and back rebuilds succeed;
- a second 80 beside an 80 is refused by name while the old tree keeps drawing.

The render module's ceiling is raised to 2,800,000 bytes, to flag in the PR.

## Evidence
- Commits: 60028e27, 0f7383e1, 5f10ba8d, b059de8c
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, npm test
- PRs: