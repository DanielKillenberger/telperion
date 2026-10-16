---
satisfies: [R1, R2, R3, R4]
---
# fn-205-tree-space-girth-from-secondary-growth.1 form.secondary: girth from secondary growth by share

## Description
Add `form.secondary` per PA, 1 neutral: radius = secondary * pipe radius + (1 - secondary) * established radius. Walk-tested; every existing tree byte-identical at 1.

## Acceptance
- Single-axis test at 0 and 1 (red first).
- Walk test within 30.
- Existing trees byte-identical; Codex review SHIP.

## Done summary
`form.secondary` per PA, neutral 1, the established radius read as the phytomer's own pipe when laid down (the other reading gives a yearly sawtooth). Beech, spruce and palm structures byte-identical at 1; column test red first; walked within 30 (9.53 on sagging limbs). Codex SHIP. Gate 1,044 passed, 0 failed.

## Evidence
- Commits: f2073f90, 1de8a72c
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs:
