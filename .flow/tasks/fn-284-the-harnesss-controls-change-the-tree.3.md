# fn-284-the-harnesss-controls-change-the-tree.3 The panel reads plainly

## Description
R3, R4. Groups by what a control does, most telling first; plain labels with units and one-line meaning; changed-from-preset marks and resets per control, group and tree; filter box; advanced group closed. Grow-in and play the year unchanged.

## Acceptance
R3, R4; owner verdict R5

## Done summary
Controls grouped by what they do (`catalogue/group.rs`, one source): size and age, trunk and form, branching, leaves and fronds, flowers and fruit, bark and colour, supernatural (dormant note while off), advanced (closed). Label with unit, one-line meaning, dormancy line, changed mark with reset per control, group and tree; filter box. Grow-in and play the year untouched. Owner verdict (2026-10-15): "sliders change the tree yes. Yes settings are clear. groups seem ok to."

## Evidence
- Commits: 3ec1b064d
- Tests: cargo test --profile ci --workspace --no-fail-fast (1409 passed, 0 failed, 101 ignored), npm test (18 files, 179 tests)
- PRs: