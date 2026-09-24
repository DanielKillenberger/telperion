---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-135-a-species-first-tuned-tree-starts-from.1 Implement fn-135-a-species-first-tuned-tree-starts-from

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The conductor now derives the tree a species' tuning starts from out of its sourced profile. `crates/telperion-jev/data/profile-to-preset.json` (no species named; conditions read family values only) and `conductor/derive.rs` turn every appearance range keyed by a material field into its midpoint, and map height, trunk diameter over height, crown width over twice the height (no rosette), frond length to the rachis (rosette) and leaflet or leaf sizes over the canopy size to wire values. Each value is clamped to its dial in `data/dials.json`. `conductor/overlay.rs` writes them into the tuning config's `initial_overrides` before a fresh revision, with `<config>.derived.json` beside it holding each value's source, citations and formula. A manual entry wins. The same step makes a tuning-profile gating metric the measurer cannot read (`derive::MEASURED`) contextual.

Both unknowns are confirmed in the spec: the measurer's metric keys, and `spread` as the widest radius over the height. Palm fixtures were copied from the fn-80 worktree at cfead8c7, with the tuning config trimmed to the fields the derivation reads. Docs updated: `docs/species-conductor.md` ("The tree tuning starts from") and `docs/tuning-loop.md`.

Gate: RED, and not from this diff. 1031 passed, 4 failed, 21 ignored. The failures are the four telperion-core `tests/species.rs` memory-ceiling tests (peak resident 6.36 GB over 6.11 GB). Alone they pass 14/14, and the diff touches no core file. Recorded in FRICTION.md.

Judgment calls the host should check:
- a folder with no `packet/profile.json` derives nothing, which keeps the existing conductor tests' shape;
- the trunk radius uses breast-height diameter for the root radius;
- the frond length maps to the rachis alone;
- `identity` is implemented, but no row uses it yet.

Follow-up: nothing builds the tuning profile from the packet profile. The step only downgrades unreadable metrics in the file the tuning config names.

Tier: session (host-settled design)
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: dff2cfbd
- Tests: cargo test --profile ci --workspace --no-fail-fast (1031 passed, 4 failed: core memory ceilings under a concurrent gate; tests/species.rs 14/14 on a quiet rerun)
- PRs: