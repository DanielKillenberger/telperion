---
satisfies: [R3]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.8 SHORTEST decided in grown micrometres

## Description
Host decision (2026-10-07) on TASK6's option: the conversion's SHORTEST merge is decided on the recorded grown length in whole micrometres, exact on both executors, removing the named SHORTEST class. Changes merges on every engine tree: measure merges per tree before and after, view hero and close-up stills of beech and spruce at 80, rerun the eight-tree parity probes with the class gone.

## Acceptance
- SHORTEST on grown micrometres on both executors; the SHORTEST class empty in the probes.
- Merge counts and stills before/after shown.
- Gate green.

## Done summary
SHORTEST decided in whole micrometres of grown wood on both executors (space.rs, dress.wgsl): a phytomer merges into the last node drawn while the wood grown since it rounds under 1,000 um. Merges per tree: beech +1 to +61, spruce +430 to +2,050 (0.04 % of the spruce's nodes at 80). Stills at 80 (beech and spruce, hero and close) viewed before and after: indistinguishable, 0.17 to 0.53 % pixels exact. The eight-tree probes (dress, sweep, stations, placement) and the real path at spruce 80: the SHORTEST class empty everywhere, the spruce 80 s7 parted run gone. Gate: render 162, CPU 988 passed.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 9bc9e02a
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-core merges_per_tree -- --ignored
- PRs: