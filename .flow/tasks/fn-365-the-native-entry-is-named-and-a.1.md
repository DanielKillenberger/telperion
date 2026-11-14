# fn-365-the-native-entry-is-named-and-a.1 The native entry is named and a headless test grows the oak

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The oak is grown from parameter text and a seed through `pipeline::growth` to 20 and on to 40, with no GPU, browser or harness (`crates/telperion-core/tests/native_entry.rs`). The test checks the tree and mesh at each age, and checks that the tree at 20 on the way to 40 is the tree at 20 alone, by its digest. It passed in 3.8 s at a peak of 756 MiB. `docs/native-entry.md`, linked from `docs/pipeline.md`, names the entry and the test behind each promise. The R1 reading is in `.flow/evidence/fn-365-the-native-entry-is-named-and-a/READING.md`.

The parameter text is rendered in the test's setup from the oak's value file through `params::metadata`. No fixture is committed, so the text cannot drift from the shipped table.

R4, returned to the host and not removed here:
- The `params` module also exports `preset(id)`, `by_identity`, `CATALOGUE` and `IN_WORK`.
- `parse` starts every family from `preset(0)`.

Also found: `growth::mesh` places leaves by the family's `skeleton.seed` row, not by `begin`'s seed.

stage: impl-review - skipped(policy: host-deferred - conductor owns the gate)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: ad6212ac0c6865b3ab8de97f6f8a5cdc7393bb65
- Tests: cargo test --profile ci -p telperion-core --test native_entry (1 passed, 3.8 s, peak 756 MiB), cargo test --profile ci -p telperion-core --test generation_limit_guard (2 passed), cargo test --profile ci -p telperion-render --lib compile_size (2 passed)
- PRs: