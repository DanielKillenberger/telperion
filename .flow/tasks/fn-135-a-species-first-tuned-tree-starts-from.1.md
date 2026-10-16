---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-135-a-species-first-tuned-tree-starts-from.1 Implement fn-135-a-species-first-tuned-tree-starts-from

## Description
Derive a species' starting tuning overlay from its sourced profile.

## Acceptance
Spec R1-R5.

## Done summary
`data/profile-to-preset.json` and `conductor/derive.rs` map appearance midpoints and sizes to clamped wire values; `conductor/overlay.rs` writes `initial_overrides` and a `<config>.derived.json` provenance file before a fresh revision (manual wins); unreadable gating metrics become contextual. Docs updated. Gate: 1031 passed, 4 core memory-ceiling failures from a concurrent gate (pass alone).

## Evidence
- Commits: dff2cfbd
- Tests: cargo test --profile ci --workspace --no-fail-fast (1031 passed, 4 memory-ceiling failures under load; 14/14 alone)
- PRs:
