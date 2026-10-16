---
satisfies: [R1, R2, R3, R4]
---
# fn-166-the-owner-looks-at-a-runs-kept-tree-in.1 Implement The owner looks at a run's kept tree in the harness

## Description
Add `species <id> --look` and a harness view of a run's kept tree beside the shipped preset.

## Acceptance
- R1-R4 of the spec.

## Done summary
`species <id> --look` writes the kept tree to the ignored `harness/looks/<id>.json` and prints `?look=<id>&seed=<n>`. The harness lays the overlay over the preset, refuses a family that differs from core's, names run, revision and rounds, and toggles look and preset in place. A missing kept tree or an unknown overlay row is refused by path. Runbook and add-species skill updated. fn-157's beech overlay is refused on master (retired fn-170 rows); the fixture drops them. Impl-review: one NEEDS_WORK round fixed, then SHIP.

## Evidence
- Commits: fbc982d93559ce96ef839c85d5590a75aff408bf, 1b7095921eb69f9ce12ce225a7d02432b1f5da1e, 03af1681121d49cf4d04de95e1090380aab0237c
- Tests: npm test, cargo test --profile ci --workspace --no-fail-fast
- PRs:
