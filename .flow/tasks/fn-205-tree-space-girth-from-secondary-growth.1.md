---
satisfies: [R1, R2, R3, R4]
---
# fn-205-tree-space-girth-from-secondary-growth.1 form.secondary: girth from secondary growth by share

## Description
Add form.secondary per PA, 1 neutral: radius = secondary * pipe radius + (1 - secondary) * established radius (the phytomer's pipe radius when laid down at the apex). Walk-tested; every existing tree byte-identical at 1.

## Acceptance
- Single-axis test at 0 and 1 (red first).
- Walk test within 30.
- Existing trees byte-identical (oracle, walks, beech/spruce).
- Codex review SHIP.

## Done summary
`form.secondary` per PA, neutral 1: radius = secondary x pipe-model radius + (1 - secondary) x the established radius, read as the phytomer's own pipe when its apex laid it down (EVIDENCE.md says why: the other reading gives a yearly sawtooth). Beech, spruce and palm structures byte-identical at 1; oracle and walks pass; column test red first; walked within 30 (9.53 on sagging limbs). Codex SHIP, no findings. Workspace gate green (1044 passed, 0 failed).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: f2073f90, 1de8a72c
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs: