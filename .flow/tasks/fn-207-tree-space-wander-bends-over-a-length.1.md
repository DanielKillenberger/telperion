# fn-207-tree-space-wander-bends-over-a-length.1 Wander bends over a length: curvature that persists, the oak's trial

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
`form.bend_length` gives wander a curvature that persists, relaxing over a set length. At 0 it is neutral: beech, spruce and oak are byte-identical.
- **Tests:** `tests/bend.rs` was red, then green. The walks hold, including two across a continuation that Codex asked for.
- **Oak trial:** recorded in fn-195.
- **Review and gate:** Codex returned SHIP on review 2. The gate passed 1,063 tests with 0 failures.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 48cf6d69, cccc881c, e9d4129f
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-space --test bend, cargo test --profile ci -p telperion-space --test walks every_setting_changes
- PRs: