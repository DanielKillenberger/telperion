---
satisfies: [R2]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.2 Delete first

## Description
AUDIT.md's deletions with the host decisions (CONTRACT.md, 'Host decisions'): compact rings derived from the curve's points (equal to the bit) instead of a second sweep; the beech's unread rings not built; the foliage cull's rejection policy, ranks, serial prefix pass, raw buffer and scatter removed on both executors (the bounds reduction stays); the spruce at 80's undrawn ring-mesh index buffer not built, so it stays on the GPU ring path within binding limits. Also D6 (sowing on the calling thread on wasm32, the engine's clock everywhere) and the branch's compile fix (expansion.rs:405). Every drawn output unchanged (seated leaves, surfaced wood, hashes) except where a deletion is shown to remove nothing drawn. Then the new baseline (option B) by AUDIT.md's method: spruce and beech at 80, seeds 1 and 7.

## Acceptance
- Each deletion with its before/after time and the drawn output unchanged.
- The spruce at 80 on the GPU ring path.
- wasm32 builds and grows an engine tree in Node without trapping.
- The new baseline.
- npm test and the gate green.

## Done summary
Deletions, each its own commit, drawn output unchanged by the pipeline's hashes (GPU and CPU, four species at 40; beech and spruce at 80): compile fix; D6 (wasm32 grows); the foliage cull's rejection removed on both executors (GPU peak -258 MiB); no ring positions for species that seat nothing (beech 2.3 to 1.2 s); positions sized by what the pass binds, so the spruce at 80 takes the GPU ring path (15.5-18.0 to 7.3-9.2 s, peak 5.6 to 4.2 GB; 9,551 of 16.9M seated leaves moved one quantisation step, within fn-125's bound); compact rings read from the curve (about 7.7 to 6.2 s). Baseline B: spruce 80 8.15 / 8.09 s, beech 80 1.8 / 3.4 s; R3's 3x target is about 2.7 s on the spruce. npm test green; the gate's one red (generation_limits_are_classified) predates this task and is fixed on fn-198's tip (merged in task 3). Host: shellDepth's removal goes on fn-198's R0 list.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 188cfc5e, 7f0cade5, 1c3ccb93, f83e03f6, 471fb1d9, 90c2cb8c, 884a0bb7
- Tests: npm test, cargo test --profile ci --workspace --no-fail-fast (two parts)
- PRs: