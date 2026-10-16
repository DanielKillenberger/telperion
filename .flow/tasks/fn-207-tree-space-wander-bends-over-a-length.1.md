# fn-207-tree-space-wander-bends-over-a-length.1 Wander bends over a length: curvature that persists, the oak's trial

## Description
Build the spec's Design: `form.bend_length`, a persistent curvature per axis; R1 to R5.

## Acceptance
R1 to R5 of the spec.

## Done summary
`form.bend_length` gives wander a curvature that relaxes over a set length; at 0 beech, spruce and oak are byte-identical. `tests/bend.rs` red then green; walks hold, including two across a continuation Codex asked for. Oak trial recorded in fn-195. Codex SHIP on review 2; gate 1,063 passed, 0 failed.

## Evidence
- Commits: 48cf6d69, cccc881c, e9d4129f
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-space --test bend, cargo test --profile ci -p telperion-space --test walks every_setting_changes
- PRs:
