---
satisfies: [R1, R2, R3]
---
# fn-204-leaf-base-clothing-runs-in-expansion.1 Leaf-base clothing moves to the start of expansion

## Description
Move `branching::clothe_leaf_bases` from the end of the skeleton stage to the start of expansion, so a tree handed to `executor::expand` is clothed; every shipped preset byte-identical.

## Acceptance
- R2 test red on base, green after.
- Digest comparison of every preset before and after recorded.
- Crate tests; Codex review SHIP.

## Done summary
Clothing moved to `pipeline::clothe`, called by `pipeline::build` after the skeleton and by `Grown::expansion`. All eight presets' artifact digests identical before and after (date palm `8d7a61d2bdc8c9d2`). Handed-tree test red on the base. Codex SHIP; its P2 (re-clothing a clothed tree) was then fixed on the host's call. Workspace gate 1,044 passed, 0 failed.

## Evidence
- Commits: 038e8856, d71a0721, 31cc9738
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs:
