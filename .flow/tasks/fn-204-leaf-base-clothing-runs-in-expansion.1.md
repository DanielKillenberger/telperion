---
satisfies: [R1, R2, R3]
---
# fn-204-leaf-base-clothing-runs-in-expansion.1 Leaf-base clothing moves to the start of expansion

## Description
Move branching::clothe_leaf_bases from the end of the skeleton stage to the start of expansion, so a tree handed to executor::expand is clothed; every shipped preset byte-identical.

## Acceptance
- R2 test red on base, green after.
- Digest comparison of every preset before and after recorded.
- Crate tests; Codex review SHIP.

## Done summary
Leaf-base clothing moved from the end of the skeleton stage to the start of expansion (`pipeline::clothe`, called by `pipeline::build` after the skeleton and by `Grown::expansion`), so a tree handed to `executor::expand` is clothed. Every shipped preset's artifacts are byte-identical before and after (DIGESTS.md; date palm `8d7a61d2bdc8c9d2`). The handed-tree test is red on the base. Codex SHIP with one P2 left open for the host: an already clothed tree handed back to `expand` is clothed again (the behaviour is unchanged by spec; idempotence would be a design change). Workspace gate green (1044 passed, 0 failed).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 038e8856, d71a0721, 31cc9738
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs: