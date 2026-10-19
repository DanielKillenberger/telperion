---
satisfies: [R1, R2]
---
# fn-8-lean-rust-tree-generation-core.2 Port crown colonization and structural generation state

## Description
Port crown colonization and structural generation state.

## Acceptance
- Deterministic colonization with bias, turn limits, termination, crown guards and cap diagnostics; matches pinned fn-6.

## Done summary
Ordinary/Telperion/Laurelin emit 736/522/1733 nodes with the reference topology; max position drift 7.04e-11 m; all 6 comparison cases pass.

## Evidence
- Commits: fa31bb60666e898ba54cb377c7055ff38d33673b
- Tests: cargo test (6 passed), pinned fn-6 comparison
- PRs:
